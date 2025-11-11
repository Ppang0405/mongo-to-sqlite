use bson::{Bson, Document};
use std::collections::HashMap;
use tracing::{debug, info};

use crate::converter::{escape_identifier, infer_sqlite_type};

/// Represents a field in a MongoDB collection
#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub sql_type: String,
    pub nullable: bool,
    pub is_primary_key: bool,
}

/// Represents the schema of a MongoDB collection
#[derive(Debug, Clone)]
pub struct CollectionSchema {
    pub collection_name: String,
    pub fields: Vec<Field>,
}

impl CollectionSchema {
    /// Generate a CREATE TABLE statement for this schema
    ///
    /// # Returns
    /// SQL CREATE TABLE statement
    pub fn to_create_table_sql(&self) -> String {
        let table_name = escape_identifier(&self.collection_name);
        
        let field_defs: Vec<String> = self.fields.iter().map(|field| {
            let field_name = escape_identifier(&field.name);
            let mut def = format!("{} {}", field_name, field.sql_type);
            
            if field.is_primary_key {
                def.push_str(" PRIMARY KEY");
            }
            
            if !field.nullable && !field.is_primary_key {
                def.push_str(" NOT NULL");
            }
            
            def
        }).collect();
        
        format!(
            "CREATE TABLE IF NOT EXISTS {} (\n  {}\n)",
            table_name,
            field_defs.join(",\n  ")
        )
    }

    /// Get ordered list of field names
    ///
    /// # Returns
    /// Vector of field names in the order they appear in the schema
    pub fn field_names(&self) -> Vec<String> {
        self.fields.iter().map(|f| f.name.clone()).collect()
    }

    /// Generate INSERT statement template with placeholders
    ///
    /// # Returns
    /// SQL INSERT statement with ? placeholders
    pub fn to_insert_sql(&self) -> String {
        let table_name = escape_identifier(&self.collection_name);
        let field_names: Vec<String> = self.fields
            .iter()
            .map(|f| escape_identifier(&f.name))
            .collect();
        
        let placeholders = vec!["?"; self.fields.len()].join(", ");
        
        format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table_name,
            field_names.join(", "),
            placeholders
        )
    }
}

/// Schema inference engine
pub struct SchemaInferrer;

impl SchemaInferrer {
    /// Infer schema from a collection of sample documents
    ///
    /// # Arguments
    /// * `collection_name` - Name of the collection
    /// * `documents` - Sample documents to analyze
    ///
    /// # Returns
    /// Inferred schema for the collection
    pub fn infer_schema(
        collection_name: &str,
        documents: &[Document],
    ) -> CollectionSchema {
        info!("Inferring schema for collection: {}", collection_name);
        
        if documents.is_empty() {
            debug!("No documents to analyze, creating minimal schema");
            return Self::create_empty_schema(collection_name);
        }

        // Collect field information across all documents
        let mut field_info = Self::analyze_documents(documents);
        
        // Build field definitions
        let mut fields = Vec::new();
        
        // MongoDB's _id is always present and becomes the primary key
        if let Some(info) = field_info.remove("_id") {
            fields.push(Field {
                name: "_id".to_string(),
                sql_type: info.most_common_type,
                nullable: false,
                is_primary_key: true,
            });
        }
        
        // Add remaining fields, sorted by name for consistency
        let mut field_names: Vec<_> = field_info.keys().cloned().collect();
        field_names.sort();
        
        for field_name in field_names {
            let info = &field_info[&field_name];
            fields.push(Field {
                name: field_name.clone(),
                sql_type: info.most_common_type.clone(),
                // Always nullable except for _id - MongoDB is schema-less
                // and fields can be missing in documents outside our sample
                nullable: true,
                is_primary_key: false,
            });
        }
        
        debug!("Inferred {} fields for {}", fields.len(), collection_name);
        
        CollectionSchema {
            collection_name: collection_name.to_string(),
            fields,
        }
    }

    /// Create an empty schema with just _id field
    fn create_empty_schema(collection_name: &str) -> CollectionSchema {
        CollectionSchema {
            collection_name: collection_name.to_string(),
            fields: vec![Field {
                name: "_id".to_string(),
                sql_type: "TEXT".to_string(),
                nullable: false,
                is_primary_key: true,
            }],
        }
    }

    /// Analyze documents to collect field information
    fn analyze_documents(documents: &[Document]) -> HashMap<String, FieldInfo> {
        let mut field_info: HashMap<String, FieldInfo> = HashMap::new();
        
        for doc in documents {
            for (key, value) in doc.iter() {
                let info = field_info.entry(key.clone()).or_insert_with(|| {
                    FieldInfo::new()
                });
                
                info.record_value(value);
            }
        }
        
        // Determine most common type for each field
        for info in field_info.values_mut() {
            info.finalize();
        }
        
        field_info
    }
}

/// Information collected about a field during analysis
#[derive(Debug)]
struct FieldInfo {
    type_counts: HashMap<String, usize>,
    presence_count: usize,
    most_common_type: String,
}

impl FieldInfo {
    /// Create a new FieldInfo
    fn new() -> Self {
        Self {
            type_counts: HashMap::new(),
            presence_count: 0,
            most_common_type: "TEXT".to_string(), // Default fallback
        }
    }

    /// Record a value occurrence
    fn record_value(&mut self, value: &Bson) {
        self.presence_count += 1;
        
        let sql_type = infer_sqlite_type(value);
        *self.type_counts.entry(sql_type.to_string()).or_insert(0) += 1;
    }

    /// Finalize analysis and determine most common type
    fn finalize(&mut self) {
        if self.type_counts.is_empty() {
            self.most_common_type = "TEXT".to_string();
            return;
        }

        // Find the most common type
        let mut max_count = 0;
        let mut most_common = "TEXT".to_string();
        
        // Priority order: INTEGER, REAL, TEXT, BLOB, NULL
        // If there's a tie, prefer in this order
        let type_priority = vec!["INTEGER", "REAL", "TEXT", "BLOB", "NULL"];
        
        for prio_type in &type_priority {
            if let Some(&count) = self.type_counts.get(*prio_type) {
                if count > max_count {
                    max_count = count;
                    most_common = prio_type.to_string();
                }
            }
        }
        
        // If no priority type found, take the first one with max count
        if max_count == 0 {
            if let Some((type_name, _count)) = self.type_counts.iter().max_by_key(|(_, &c)| c) {
                most_common = type_name.clone();
            }
        }
        
        // Special case: if we see NULL and other types, prefer the non-NULL type
        if most_common == "NULL" && self.type_counts.len() > 1 {
            for (type_name, &count) in &self.type_counts {
                if type_name != "NULL" && count > 0 {
                    most_common = type_name.clone();
                    break;
                }
            }
        }
        
        self.most_common_type = most_common;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bson::doc;

    #[test]
    fn test_infer_schema_simple() {
        let docs = vec![
            doc! {
                "_id": "1",
                "name": "Alice",
                "age": 30,
            },
            doc! {
                "_id": "2",
                "name": "Bob",
                "age": 25,
            },
        ];

        let schema = SchemaInferrer::infer_schema("users", &docs);
        
        assert_eq!(schema.collection_name, "users");
        assert_eq!(schema.fields.len(), 3); // _id, name, age
        
        // Check that _id is primary key
        let id_field = schema.fields.iter().find(|f| f.name == "_id").unwrap();
        assert!(id_field.is_primary_key);
    }

    #[test]
    fn test_infer_schema_nullable_fields() {
        let docs = vec![
            doc! {
                "_id": "1",
                "name": "Alice",
                "email": "alice@example.com",
            },
            doc! {
                "_id": "2",
                "name": "Bob",
                // email is missing
            },
        ];

        let schema = SchemaInferrer::infer_schema("users", &docs);
        
        let email_field = schema.fields.iter().find(|f| f.name == "email").unwrap();
        assert!(email_field.nullable);
        
        let name_field = schema.fields.iter().find(|f| f.name == "name").unwrap();
        assert!(!name_field.nullable);
    }

    #[test]
    fn test_create_table_sql() {
        let schema = CollectionSchema {
            collection_name: "users".to_string(),
            fields: vec![
                Field {
                    name: "_id".to_string(),
                    sql_type: "TEXT".to_string(),
                    nullable: false,
                    is_primary_key: true,
                },
                Field {
                    name: "name".to_string(),
                    sql_type: "TEXT".to_string(),
                    nullable: false,
                    is_primary_key: false,
                },
                Field {
                    name: "age".to_string(),
                    sql_type: "INTEGER".to_string(),
                    nullable: true,
                    is_primary_key: false,
                },
            ],
        };

        let sql = schema.to_create_table_sql();
        assert!(sql.contains("CREATE TABLE"));
        assert!(sql.contains("users"));
        assert!(sql.contains("PRIMARY KEY"));
    }

    #[test]
    fn test_insert_sql() {
        let schema = CollectionSchema {
            collection_name: "users".to_string(),
            fields: vec![
                Field {
                    name: "_id".to_string(),
                    sql_type: "TEXT".to_string(),
                    nullable: false,
                    is_primary_key: true,
                },
                Field {
                    name: "name".to_string(),
                    sql_type: "TEXT".to_string(),
                    nullable: false,
                    is_primary_key: false,
                },
            ],
        };

        let sql = schema.to_insert_sql();
        assert!(sql.contains("INSERT INTO"));
        assert!(sql.contains("VALUES"));
        assert!(sql.contains("?"));
    }

    #[test]
    fn test_empty_schema() {
        let docs: Vec<Document> = vec![];
        let schema = SchemaInferrer::infer_schema("empty", &docs);
        
        assert_eq!(schema.fields.len(), 1); // Just _id
        assert_eq!(schema.fields[0].name, "_id");
    }
}

