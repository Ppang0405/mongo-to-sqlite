use anyhow::Result;
use bson::{doc, Document};
use futures::stream::TryStreamExt;
use mongodb::{Client, options::ClientOptions};
use tracing::{debug, info};

/// MongoDB client wrapper for database operations
pub struct MongoClient {
    client: Client,
}

impl MongoClient {
    /// Create a new MongoDB client
    ///
    /// # Arguments
    /// * `uri` - MongoDB connection URI (e.g., "mongodb://localhost:27017")
    ///
    /// # Returns
    /// A new MongoClient instance
    pub async fn new(uri: &str) -> Result<Self> {
        info!("Connecting to MongoDB at: {}", uri);
        
        let mut client_options = ClientOptions::parse(uri).await?;
        client_options.app_name = Some("mongo-to-sqlite".to_string());
        
        let client = Client::with_options(client_options)?;
        
        // Test the connection
        client
            .database("admin")
            .run_command(doc! { "ping": 1 }, None)
            .await?;
        
        debug!("Successfully connected to MongoDB");
        
        Ok(Self { client })
    }

    /// List all collection names in a database
    ///
    /// # Arguments
    /// * `database_name` - Name of the database
    ///
    /// # Returns
    /// Vector of collection names
    pub async fn list_collections(&self, database_name: &str) -> Result<Vec<String>> {
        info!("Listing collections in database: {}", database_name);
        
        let db = self.client.database(database_name);
        let collections = db.list_collection_names(None).await?;
        
        debug!("Found {} collections", collections.len());
        
        Ok(collections)
    }

    /// Sample documents from a collection for schema inference
    ///
    /// # Arguments
    /// * `database_name` - Name of the database
    /// * `collection_name` - Name of the collection
    /// * `sample_size` - Maximum number of documents to sample
    ///
    /// # Returns
    /// Vector of sampled documents
    pub async fn sample_documents(
        &self,
        database_name: &str,
        collection_name: &str,
        sample_size: usize,
    ) -> Result<Vec<Document>> {
        debug!(
            "Sampling {} documents from {}.{}",
            sample_size, database_name, collection_name
        );

        let db = self.client.database(database_name);
        let collection = db.collection::<Document>(collection_name);

        // Use MongoDB's $sample aggregation stage for efficient random sampling
        let pipeline = vec![
            doc! { "$sample": { "size": sample_size as i64 } },
        ];

        let mut cursor = collection.aggregate(pipeline, None).await?;
        let mut documents = Vec::new();

        while let Some(doc) = cursor.try_next().await? {
            documents.push(doc);
        }

        debug!("Sampled {} documents", documents.len());

        Ok(documents)
    }

    /// Count documents in a collection
    ///
    /// # Arguments
    /// * `database_name` - Name of the database
    /// * `collection_name` - Name of the collection
    ///
    /// # Returns
    /// Number of documents in the collection
    pub async fn count_documents(
        &self,
        database_name: &str,
        collection_name: &str,
    ) -> Result<u64> {
        let db = self.client.database(database_name);
        let collection = db.collection::<Document>(collection_name);
        
        let count = collection.count_documents(doc! {}, None).await?;
        
        debug!("Collection {} has {} documents", collection_name, count);
        
        Ok(count)
    }

    /// Stream all documents from a collection
    ///
    /// # Arguments
    /// * `database_name` - Name of the database
    /// * `collection_name` - Name of the collection
    ///
    /// # Returns
    /// A cursor that can be used to iterate over documents
    pub async fn stream_documents(
        &self,
        database_name: &str,
        collection_name: &str,
    ) -> Result<mongodb::Cursor<Document>> {
        debug!("Creating document stream for {}.{}", database_name, collection_name);

        let db = self.client.database(database_name);
        let collection = db.collection::<Document>(collection_name);

        // Configure find options to prevent cursor timeout
        let find_options = mongodb::options::FindOptions::builder()
            .no_cursor_timeout(true)  // Prevent 10-minute cursor timeout
            .batch_size(1000)          // Process in batches
            .build();

        let cursor = collection.find(doc! {}, find_options).await?;

        Ok(cursor)
    }

    /// Check if a database exists
    ///
    /// # Arguments
    /// * `database_name` - Name of the database to check
    ///
    /// # Returns
    /// True if the database exists, false otherwise
    #[allow(dead_code)]
    pub async fn database_exists(&self, database_name: &str) -> Result<bool> {
        let db_names = self.client.list_database_names(doc! {}, None).await?;
        Ok(db_names.contains(&database_name.to_string()))
    }

    /// Check if a collection exists in a database
    ///
    /// # Arguments
    /// * `database_name` - Name of the database
    /// * `collection_name` - Name of the collection to check
    ///
    /// # Returns
    /// True if the collection exists, false otherwise
    #[allow(dead_code)]
    pub async fn collection_exists(
        &self,
        database_name: &str,
        collection_name: &str,
    ) -> Result<bool> {
        let collections = self.list_collections(database_name).await?;
        Ok(collections.contains(&collection_name.to_string()))
    }

    /// Get a reference to the underlying MongoDB client
    ///
    /// # Returns
    /// Reference to the MongoDB client
    #[allow(dead_code)]
    pub fn client(&self) -> &Client {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests require a running MongoDB instance
    // They are disabled by default and can be enabled with: cargo test -- --ignored

    #[tokio::test]
    #[ignore]
    async fn test_connect_to_mongodb() {
        let client = MongoClient::new("mongodb://localhost:27017").await;
        assert!(client.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_list_collections() {
        let client = MongoClient::new("mongodb://localhost:27017")
            .await
            .unwrap();
        let collections = client.list_collections("test").await;
        assert!(collections.is_ok());
    }
}

