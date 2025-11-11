use anyhow::Result;
use colored::Colorize;
use futures::stream::TryStreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use tracing::{debug, info, warn};

use crate::{
    converter::document_to_sql_values,
    libsql_client::LibSqlClient,
    mongodb_client::MongoClient,
    schema::SchemaInferrer,
};

/// Migration mode determines what gets migrated
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MigrationMode {
    /// Migrate both schema and data
    Full,
    /// Migrate schema only
    SchemaOnly,
    /// Migrate data only (assumes schema exists)
    DataOnly,
}

impl MigrationMode {
    /// Create migration mode from command-line flags
    ///
    /// # Arguments
    /// * `schema_only` - Whether --schema-only flag was set
    /// * `data_only` - Whether --data-only flag was set
    ///
    /// # Returns
    /// Appropriate MigrationMode
    pub fn from_args(schema_only: bool, data_only: bool) -> Self {
        match (schema_only, data_only) {
            (true, false) => MigrationMode::SchemaOnly,
            (false, true) => MigrationMode::DataOnly,
            _ => MigrationMode::Full,
        }
    }
}

/// Orchestrates the migration process
pub struct Migrator {
    mongo_client: MongoClient,
    libsql_client: LibSqlClient,
    database_name: String,
    batch_size: usize,
    sample_size: usize,
}

impl Migrator {
    /// Create a new Migrator
    ///
    /// # Arguments
    /// * `mongo_client` - MongoDB client
    /// * `libsql_client` - LibSQL client
    /// * `database_name` - Name of MongoDB database to migrate
    /// * `batch_size` - Number of documents to insert per batch
    /// * `sample_size` - Number of documents to sample for schema inference
    ///
    /// # Returns
    /// A new Migrator instance
    pub fn new(
        mongo_client: MongoClient,
        libsql_client: LibSqlClient,
        database_name: String,
        batch_size: usize,
        sample_size: usize,
    ) -> Self {
        Self {
            mongo_client,
            libsql_client,
            database_name,
            batch_size,
            sample_size,
        }
    }

    /// Migrate collections from MongoDB to SQLite
    ///
    /// # Arguments
    /// * `collections` - List of collection names to migrate
    /// * `mode` - Migration mode (full, schema only, or data only)
    /// * `truncate` - If true, delete existing data before inserting (only for data-only mode)
    /// * `drop_tables` - If true, drop tables before creating schema
    ///
    /// # Returns
    /// Total number of documents migrated
    pub async fn migrate(
        &self,
        collections: Vec<String>,
        mode: MigrationMode,
        truncate: bool,
        drop_tables: bool,
    ) -> Result<usize> {
        info!("Starting migration of {} collection(s)", collections.len());
        
        let mut total_documents = 0;

        // Drop tables if requested (before schema migration)
        if drop_tables && (mode == MigrationMode::Full || mode == MigrationMode::SchemaOnly) {
            println!("\n{}", "🗑️  Dropping existing tables...".yellow());
            self.drop_tables(&collections).await?;
        }

        // Migrate schema if needed
        if mode == MigrationMode::Full || mode == MigrationMode::SchemaOnly {
            println!("\n{}", "📋 Migrating schema...".yellow());
            self.migrate_schemas(&collections).await?;
        }

        // Truncate tables if requested (only for data-only mode)
        if truncate && mode == MigrationMode::DataOnly {
            println!("\n{}", "🗑️  Truncating existing tables...".yellow());
            self.truncate_tables(&collections).await?;
        }

        // Migrate data if needed
        if mode == MigrationMode::Full || mode == MigrationMode::DataOnly {
            println!("\n{}", "📦 Migrating data...".yellow());
            total_documents = self.migrate_data(&collections).await?;
        }

        Ok(total_documents)
    }

    /// Drop tables completely (removes schema and data)
    async fn drop_tables(&self, collections: &[String]) -> Result<()> {
        for collection_name in collections {
            let sql = format!("DROP TABLE IF EXISTS \"{}\"", collection_name.replace('"', "\"\""));
            debug!("Dropping table: {}", collection_name);
            
            match self.libsql_client.execute(&sql).await {
                Ok(_) => {
                    println!("  {} Dropped table: {}", 
                        "✓".green(), 
                        collection_name.cyan()
                    );
                }
                Err(e) => {
                    warn!("Failed to drop table {}: {}", collection_name, e);
                    // Continue with other tables even if one fails
                }
            }
        }
        Ok(())
    }

    /// Truncate (delete all data from) tables
    async fn truncate_tables(&self, collections: &[String]) -> Result<()> {
        for collection_name in collections {
            let sql = format!("DELETE FROM \"{}\"", collection_name.replace('"', "\"\""));
            debug!("Truncating table: {}", collection_name);
            
            match self.libsql_client.execute(&sql).await {
                Ok(affected) => {
                    println!("  {} Truncated table: {} ({} rows deleted)", 
                        "✓".green(), 
                        collection_name.cyan(),
                        affected
                    );
                }
                Err(e) => {
                    warn!("Failed to truncate table {}: {}", collection_name, e);
                    // Continue with other tables even if one fails
                }
            }
        }
        Ok(())
    }

    /// Migrate schemas for all collections
    async fn migrate_schemas(&self, collections: &[String]) -> Result<()> {
        for collection_name in collections {
            self.migrate_schema(collection_name).await?;
        }
        Ok(())
    }

    /// Migrate schema for a single collection
    async fn migrate_schema(&self, collection_name: &str) -> Result<()> {
        debug!("Migrating schema for collection: {}", collection_name);

        // Sample documents for schema inference
        let documents = self
            .mongo_client
            .sample_documents(&self.database_name, collection_name, self.sample_size)
            .await?;

        // Infer schema
        let schema = SchemaInferrer::infer_schema(collection_name, &documents);

        // Generate and execute CREATE TABLE statement
        let create_table_sql = schema.to_create_table_sql();
        debug!("CREATE TABLE SQL: {}", create_table_sql);

        self.libsql_client.execute(&create_table_sql).await?;

        println!(
            "  {} Created table: {} ({} columns)",
            "✓".green(),
            collection_name.cyan(),
            schema.fields.len().to_string().cyan()
        );

        Ok(())
    }

    /// Migrate data for all collections
    async fn migrate_data(&self, collections: &[String]) -> Result<usize> {
        let mut total_documents = 0;

        for collection_name in collections {
            let count = self.migrate_collection_data(collection_name).await?;
            total_documents += count;
        }

        Ok(total_documents)
    }

    /// Migrate data for a single collection
    async fn migrate_collection_data(&self, collection_name: &str) -> Result<usize> {
        debug!("Migrating data for collection: {}", collection_name);

        // Get total document count
        let total_count = self
            .mongo_client
            .count_documents(&self.database_name, collection_name)
            .await?;

        if total_count == 0 {
            println!(
                "  {} {}: No documents to migrate",
                "✓".green(),
                collection_name.cyan()
            );
            return Ok(0);
        }

        // Sample documents to infer schema (needed for field ordering)
        let sample_docs = self
            .mongo_client
            .sample_documents(&self.database_name, collection_name, self.sample_size)
            .await?;

        let schema = SchemaInferrer::infer_schema(collection_name, &sample_docs);
        let insert_sql = schema.to_insert_sql();
        let field_names = schema.field_names();

        // Create progress bar
        let pb = ProgressBar::new(total_count);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("  {msg} [{bar:40.cyan/blue}] {pos}/{len} ({percent}%)")
                .expect("Invalid progress bar template")
                .progress_chars("#>-"),
        );
        pb.set_message(format!("{}", collection_name.cyan()));

        // Stream documents and insert in batches
        let mut cursor = self
            .mongo_client
            .stream_documents(&self.database_name, collection_name)
            .await?;

        let mut batch = Vec::new();
        let mut total_migrated = 0;

        while let Some(doc) = cursor.try_next().await? {
            // Convert document to SQL values
            let values = document_to_sql_values(&doc, &field_names);
            batch.push(values);

            // Insert batch when it reaches the batch size
            if batch.len() >= self.batch_size {
                self.insert_batch(&insert_sql, &batch).await?;
                total_migrated += batch.len();
                pb.set_position(total_migrated as u64);
                batch.clear();
            }
        }

        // Insert remaining documents
        if !batch.is_empty() {
            self.insert_batch(&insert_sql, &batch).await?;
            total_migrated += batch.len();
            pb.set_position(total_migrated as u64);
        }

        pb.finish_with_message(format!("{} ✓", collection_name.cyan()));

        if total_migrated != total_count as usize {
            warn!(
                "Expected {} documents but migrated {} for collection {}",
                total_count, total_migrated, collection_name
            );
        }

        Ok(total_migrated)
    }

    /// Insert a batch of documents
    async fn insert_batch(
        &self,
        insert_sql: &str,
        batch: &[Vec<libsql::Value>],
    ) -> Result<()> {
        if batch.is_empty() {
            return Ok(());
        }

        // Insert each row individually within a transaction
        // Start transaction
        self.libsql_client
            .execute("BEGIN TRANSACTION")
            .await?;

        match self.insert_batch_inner(insert_sql, batch).await {
            Ok(()) => {
                self.libsql_client.execute("COMMIT").await?;
                Ok(())
            }
            Err(e) => {
                self.libsql_client.execute("ROLLBACK").await?;
                Err(e)
            }
        }
    }

    /// Inner function to insert batch rows
    async fn insert_batch_inner(
        &self,
        insert_sql: &str,
        batch: &[Vec<libsql::Value>],
    ) -> Result<()> {
        for values in batch {
            // Clone values to satisfy IntoValue trait bound
            let params = libsql::params_from_iter(values.iter().cloned());
            self.libsql_client
                .execute_with_params(insert_sql, params)
                .await?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migration_mode_from_args() {
        assert_eq!(
            MigrationMode::from_args(false, false),
            MigrationMode::Full
        );
        assert_eq!(
            MigrationMode::from_args(true, false),
            MigrationMode::SchemaOnly
        );
        assert_eq!(
            MigrationMode::from_args(false, true),
            MigrationMode::DataOnly
        );
    }
}

