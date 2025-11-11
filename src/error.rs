use thiserror::Error;

/// Errors that can occur during migration
#[derive(Error, Debug)]
pub enum MigrationError {
    /// MongoDB connection error
    #[error("MongoDB connection error: {0}")]
    MongoConnectionError(#[from] mongodb::error::Error),

    /// LibSQL/SQLite error
    #[error("LibSQL error: {0}")]
    LibSqlError(#[from] libsql::Error),

    /// Schema inference error
    #[error("Schema inference error: {0}")]
    SchemaInferenceError(String),

    /// Type conversion error
    #[error("Type conversion error: {0}")]
    TypeConversionError(String),

    /// Collection not found
    #[error("Collection '{0}' not found in database '{1}'")]
    CollectionNotFound(String, String),

    /// Database not found
    #[error("Database '{0}' not found")]
    DatabaseNotFound(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// I/O error
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// BSON serialization/deserialization error
    #[error("BSON error: {0}")]
    BsonError(#[from] bson::ser::Error),

    /// BSON document error
    #[error("BSON document error: {0}")]
    BsonDocumentError(#[from] bson::document::ValueAccessError),

    /// Migration interrupted
    #[error("Migration was interrupted")]
    Interrupted,

    /// Generic error
    #[error("{0}")]
    Other(String),
}

/// Result type alias for migration operations
pub type MigrationResult<T> = Result<T, MigrationError>;

impl MigrationError {
    /// Create a new schema inference error
    pub fn schema_inference<S: Into<String>>(msg: S) -> Self {
        MigrationError::SchemaInferenceError(msg.into())
    }

    /// Create a new type conversion error
    pub fn type_conversion<S: Into<String>>(msg: S) -> Self {
        MigrationError::TypeConversionError(msg.into())
    }

    /// Create a new configuration error
    pub fn config<S: Into<String>>(msg: S) -> Self {
        MigrationError::ConfigError(msg.into())
    }

    /// Create a new generic error
    pub fn other<S: Into<String>>(msg: S) -> Self {
        MigrationError::Other(msg.into())
    }
}
