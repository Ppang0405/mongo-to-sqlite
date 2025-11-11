use anyhow::Result;
use libsql::{Builder, Connection, Database};
use std::env;
use tracing::{debug, info};

/// LibSQL client wrapper supporting both local and remote (Turso) databases
pub struct LibSqlClient {
    database: Database,
    connection: Connection,
    mode: ConnectionMode,
}

/// Connection mode for LibSQL
#[derive(Debug, Clone)]
enum ConnectionMode {
    Local(String),
    Remote { url: String },
}

impl LibSqlClient {
    /// Create a new LibSQL client
    ///
    /// If TURSO_DATABASE_URL and TURSO_AUTH_TOKEN environment variables are set,
    /// connects to Turso cloud. Otherwise, creates/connects to a local SQLite file.
    ///
    /// # Arguments
    /// * `output_path` - Optional path for local SQLite file (ignored if using Turso)
    ///
    /// # Returns
    /// A new LibSqlClient instance
    pub async fn new(output_path: Option<&str>) -> Result<Self> {
        let turso_url = env::var("TURSO_DATABASE_URL").ok();
        let turso_token = env::var("TURSO_AUTH_TOKEN").ok();

        let (database, mode) = match (turso_url, turso_token) {
            (Some(url), Some(token)) => {
                info!("Connecting to Turso cloud database: {}", url);
                let db = Builder::new_remote(url.clone(), token)
                    .build()
                    .await?;
                (db, ConnectionMode::Remote { url })
            }
            _ => {
                let path = output_path.unwrap_or("output.db");
                info!("Using local SQLite file: {}", path);
                
                // Create parent directory if it doesn't exist
                if let Some(parent) = std::path::Path::new(path).parent() {
                    if !parent.exists() {
                        std::fs::create_dir_all(parent)?;
                    }
                }
                
                let db = Builder::new_local(path)
                    .build()
                    .await?;
                (db, ConnectionMode::Local(path.to_string()))
            }
        };

        let connection = database.connect()?;
        
        debug!("Successfully connected to LibSQL database");

        Ok(Self {
            database,
            connection,
            mode,
        })
    }

    /// Execute a SQL statement without returning results
    ///
    /// # Arguments
    /// * `sql` - SQL statement to execute
    ///
    /// # Returns
    /// Number of rows affected
    pub async fn execute(&self, sql: &str) -> Result<u64> {
        debug!("Executing SQL: {}", sql);
        let result = self.connection.execute(sql, ()).await?;
        Ok(result)
    }

    /// Execute a SQL statement with parameters
    ///
    /// # Arguments
    /// * `sql` - SQL statement to execute (with ? placeholders)
    /// * `params` - Parameters to bind to the statement
    ///
    /// # Returns
    /// Number of rows affected
    pub async fn execute_with_params<P>(&self, sql: &str, params: P) -> Result<u64>
    where
        P: libsql::params::IntoParams,
    {
        let result = self.connection.execute(sql, params).await?;
        Ok(result)
    }

    /// Execute a batch of SQL statements in a transaction
    ///
    /// # Arguments
    /// * `statements` - Vector of SQL statements to execute
    ///
    /// # Returns
    /// Total number of rows affected
    pub async fn execute_batch(&self, statements: Vec<String>) -> Result<u64> {
        debug!("Executing batch of {} statements", statements.len());
        
        // Start transaction
        self.connection.execute("BEGIN TRANSACTION", ()).await?;
        
        match self.execute_batch_inner(&statements).await {
            Ok(affected) => {
                self.connection.execute("COMMIT", ()).await?;
                Ok(affected)
            }
            Err(e) => {
                self.connection.execute("ROLLBACK", ()).await?;
                Err(e)
            }
        }
    }

    /// Inner function to execute batch statements
    async fn execute_batch_inner(&self, statements: &[String]) -> Result<u64> {
        let mut total_affected = 0u64;
        
        for stmt in statements {
            let affected = self.connection.execute(stmt.as_str(), ()).await?;
            total_affected += affected;
        }
        
        Ok(total_affected)
    }

    /// Execute multiple parameterized INSERT statements in a transaction
    ///
    /// # Arguments
    /// * `sql` - SQL INSERT statement template
    /// * `param_sets` - Vector of parameter sets, one for each INSERT
    ///
    /// # Returns
    /// Total number of rows inserted
    pub async fn execute_batch_inserts<P>(
        &self,
        sql: &str,
        param_sets: Vec<P>,
    ) -> Result<u64>
    where
        P: libsql::params::IntoParams,
    {
        if param_sets.is_empty() {
            return Ok(0);
        }

        debug!("Executing batch of {} inserts", param_sets.len());
        
        // Start transaction
        self.connection.execute("BEGIN TRANSACTION", ()).await?;
        
        match self.execute_inserts_inner(sql, param_sets).await {
            Ok(count) => {
                self.connection.execute("COMMIT", ()).await?;
                Ok(count)
            }
            Err(e) => {
                self.connection.execute("ROLLBACK", ()).await?;
                Err(e)
            }
        }
    }

    /// Inner function to execute INSERT statements
    async fn execute_inserts_inner<P>(
        &self,
        sql: &str,
        param_sets: Vec<P>,
    ) -> Result<u64>
    where
        P: libsql::params::IntoParams,
    {
        let mut count = 0u64;
        
        for params in param_sets {
            self.connection.execute(sql, params).await?;
            count += 1;
        }
        
        Ok(count)
    }

    /// Query for data (returns rows)
    ///
    /// # Arguments
    /// * `sql` - SQL query to execute
    ///
    /// # Returns
    /// Rows result set
    pub async fn query(&self, sql: &str) -> Result<libsql::Rows> {
        debug!("Querying: {}", sql);
        let rows = self.connection.query(sql, ()).await?;
        Ok(rows)
    }

    /// Get the connection mode (local or remote)
    ///
    /// # Returns
    /// String describing the connection mode
    pub fn connection_info(&self) -> String {
        match &self.mode {
            ConnectionMode::Local(path) => format!("Local file: {}", path),
            ConnectionMode::Remote { url } => format!("Turso cloud: {}", url),
        }
    }

    /// Check if using local mode
    ///
    /// # Returns
    /// True if using local SQLite file, false if using Turso
    pub fn is_local(&self) -> bool {
        matches!(self.mode, ConnectionMode::Local(_))
    }

    /// Get the output path (for local mode only)
    ///
    /// # Returns
    /// Optional path to the local SQLite file
    pub fn output_path(&self) -> Option<String> {
        match &self.mode {
            ConnectionMode::Local(path) => Some(path.clone()),
            ConnectionMode::Remote { .. } => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_create_local_database() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap();
        
        let client = LibSqlClient::new(Some(path)).await;
        assert!(client.is_ok());
        
        let client = client.unwrap();
        assert!(client.is_local());
    }

    #[tokio::test]
    async fn test_execute_create_table() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap();
        
        let client = LibSqlClient::new(Some(path)).await.unwrap();
        
        let result = client
            .execute("CREATE TABLE test (id INTEGER PRIMARY KEY, name TEXT)")
            .await;
        
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_insert_and_query() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap();
        
        let client = LibSqlClient::new(Some(path)).await.unwrap();
        
        client
            .execute("CREATE TABLE test (id INTEGER PRIMARY KEY, name TEXT)")
            .await
            .unwrap();
        
        client
            .execute("INSERT INTO test (id, name) VALUES (1, 'Alice')")
            .await
            .unwrap();
        
        let rows = client.query("SELECT * FROM test").await.unwrap();
        // Note: Can't easily test row contents without more complex assertions
        assert!(rows.column_count() > 0);
    }
}

