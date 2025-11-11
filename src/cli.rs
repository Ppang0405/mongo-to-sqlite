use clap::Parser;
use anyhow::{Result, bail};

/// MongoDB to SQLite migration tool
///
/// This tool migrates MongoDB databases to SQLite/LibSQL with automatic schema inference.
/// It supports both local SQLite files and Turso cloud databases.
#[derive(Parser, Debug)]
#[command(name = "mongo-to-sqlite")]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// MongoDB database name to migrate
    #[arg(short, long, required = true)]
    pub database: String,

    /// MongoDB connection URI
    /// 
    /// If not specified, will use the MONGODB_URI environment variable,
    /// or default to mongodb://localhost:27017
    #[arg(long, env = "MONGODB_URI", default_value = "mongodb://localhost:27017")]
    pub mongodb_uri: String,

    /// Migrate a specific table/collection
    /// 
    /// Mutually exclusive with --all-tables
    #[arg(short, long, conflicts_with = "all_tables")]
    pub table: Option<String>,

    /// Migrate all tables/collections in the database
    /// 
    /// Mutually exclusive with --table
    #[arg(long, conflicts_with = "table")]
    pub all_tables: bool,

    /// Only migrate schema (CREATE TABLE statements), skip data migration
    /// 
    /// Useful for previewing the schema before migrating data
    #[arg(long, conflicts_with = "data_only")]
    pub schema_only: bool,

    /// Only migrate data, skip schema creation
    /// 
    /// Assumes tables already exist in the target database
    #[arg(long, conflicts_with = "schema_only")]
    pub data_only: bool,

    /// Truncate (delete all data from) existing tables before inserting
    /// 
    /// Only valid with --data-only flag. Useful for re-running migrations.
    #[arg(long, requires = "data_only")]
    pub truncate: bool,

    /// Drop existing tables before creating new schema
    /// 
    /// Use with caution! This will delete all existing data and schema.
    /// Only valid with --schema-only or full migration (no flags).
    #[arg(long, conflicts_with = "data_only")]
    pub drop_tables: bool,

    /// Output SQLite database file path
    /// 
    /// If TURSO_DATABASE_URL and TURSO_AUTH_TOKEN are set, this is ignored
    /// and data is written to the Turso cloud database instead.
    #[arg(short, long, default_value = "output.db")]
    pub output: Option<String>,

    /// Number of documents to insert per batch
    /// 
    /// Larger batches are faster but use more memory
    #[arg(long, default_value = "1000")]
    pub batch_size: usize,

    /// Number of documents to sample for schema inference
    /// 
    /// More samples produce more accurate schemas but take longer
    #[arg(long, default_value = "100")]
    pub sample_size: usize,
}

impl Args {
    /// Validate that the arguments are consistent and complete
    ///
    /// This function validates that:
    /// - Either --table or --all-tables is specified
    /// - batch_size and sample_size are greater than 0
    pub fn validate(&self) -> Result<()> {
        // Ensure either --table or --all-tables is specified
        if self.table.is_none() && !self.all_tables {
            bail!("Either --table <TABLE> or --all-tables must be specified");
        }

        // Validate batch size
        if self.batch_size == 0 {
            bail!("--batch-size must be greater than 0");
        }

        // Validate sample size
        if self.sample_size == 0 {
            bail!("--sample-size must be greater than 0");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_missing_table_flags() {
        let args = Args {
            database: "test".to_string(),
            mongodb_uri: "mongodb://localhost:27017".to_string(),
            table: None,
            all_tables: false,
            schema_only: false,
            data_only: false,
            output: Some("output.db".to_string()),
            batch_size: 1000,
            sample_size: 100,
        };

        assert!(args.validate().is_err());
    }

    #[test]
    fn test_validate_zero_batch_size() {
        let args = Args {
            database: "test".to_string(),
            mongodb_uri: "mongodb://localhost:27017".to_string(),
            table: Some("users".to_string()),
            all_tables: false,
            schema_only: false,
            data_only: false,
            output: Some("output.db".to_string()),
            batch_size: 0,
            sample_size: 100,
        };

        assert!(args.validate().is_err());
    }

    #[test]
    fn test_validate_valid_args() {
        let args = Args {
            database: "test".to_string(),
            mongodb_uri: "mongodb://localhost:27017".to_string(),
            table: Some("users".to_string()),
            all_tables: false,
            schema_only: false,
            data_only: false,
            output: Some("output.db".to_string()),
            batch_size: 1000,
            sample_size: 100,
        };

        assert!(args.validate().is_ok());
    }
}
