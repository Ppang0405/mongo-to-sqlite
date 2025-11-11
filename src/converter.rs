use bson::{Bson, Document};
use chrono::{DateTime, Utc};
use libsql::Value as SqlValue;
use serde_json;
use tracing::warn;

/// Convert a BSON value to a SQLite value
///
/// This function handles the conversion of MongoDB BSON types to SQLite types.
/// Complex types (arrays, nested documents) are serialized as JSON strings.
///
/// # Arguments
/// * `bson` - The BSON value to convert
///
/// # Returns
/// A SQLite Value that can be used in queries
pub fn bson_to_sql_value(bson: &Bson) -> SqlValue {
    match bson {
        Bson::Double(v) => SqlValue::Real(*v),
        Bson::String(v) => SqlValue::Text(v.clone()),
        Bson::Document(doc) => {
            // Serialize nested documents as JSON
            match serde_json::to_string(doc) {
                Ok(json) => SqlValue::Text(json),
                Err(e) => {
                    warn!("Failed to serialize document to JSON: {}", e);
                    SqlValue::Null
                }
            }
        }
        Bson::Array(arr) => {
            // Serialize arrays as JSON
            match serde_json::to_string(arr) {
                Ok(json) => SqlValue::Text(json),
                Err(e) => {
                    warn!("Failed to serialize array to JSON: {}", e);
                    SqlValue::Null
                }
            }
        }
        Bson::Binary(_) => {
            // Convert binary to base64 text for now
            // TODO: Could store as BLOB if needed
            match serde_json::to_string(bson) {
                Ok(json) => SqlValue::Text(json),
                Err(e) => {
                    warn!("Failed to serialize binary to JSON: {}", e);
                    SqlValue::Null
                }
            }
        }
        Bson::ObjectId(oid) => SqlValue::Text(oid.to_hex()),
        Bson::Boolean(v) => SqlValue::Integer(if *v { 1 } else { 0 }),
        Bson::DateTime(dt) => {
            // Convert to ISO 8601 string
            let datetime: DateTime<Utc> = (*dt).into();
            SqlValue::Text(datetime.to_rfc3339())
        }
        Bson::Null => SqlValue::Null,
        Bson::RegularExpression(regex) => {
            // Store regex pattern and options as JSON
            let json = serde_json::json!({
                "pattern": regex.pattern,
                "options": regex.options
            });
            SqlValue::Text(json.to_string())
        }
        Bson::JavaScriptCode(code) => SqlValue::Text(code.clone()),
        Bson::JavaScriptCodeWithScope(code_with_scope) => {
            let json = serde_json::json!({
                "code": code_with_scope.code,
                "scope": code_with_scope.scope
            });
            SqlValue::Text(json.to_string())
        }
        Bson::Int32(v) => SqlValue::Integer(*v as i64),
        Bson::Int64(v) => SqlValue::Integer(*v),
        Bson::Timestamp(ts) => SqlValue::Integer(ts.time as i64),
        Bson::Decimal128(dec) => {
            // Convert Decimal128 to string for precision
            SqlValue::Text(dec.to_string())
        }
        Bson::Undefined => SqlValue::Null,
        Bson::MaxKey => SqlValue::Text("$maxKey".to_string()),
        Bson::MinKey => SqlValue::Text("$minKey".to_string()),
        Bson::DbPointer(_) => {
            warn!("DbPointer type is deprecated, storing as null");
            SqlValue::Null
        }
        Bson::Symbol(s) => SqlValue::Text(s.clone()),
    }
}

/// Infer SQLite type from BSON value
///
/// # Arguments
/// * `bson` - The BSON value to analyze
///
/// # Returns
/// SQLite type as a string (TEXT, INTEGER, REAL, BLOB, NULL)
pub fn infer_sqlite_type(bson: &Bson) -> &'static str {
    match bson {
        Bson::Double(_) => "REAL",
        Bson::String(_) => "TEXT",
        Bson::Document(_) => "TEXT", // JSON
        Bson::Array(_) => "TEXT",     // JSON
        Bson::Binary(_) => "BLOB",
        Bson::ObjectId(_) => "TEXT",
        Bson::Boolean(_) => "INTEGER",
        Bson::DateTime(_) => "TEXT",
        Bson::Null | Bson::Undefined => "NULL",
        Bson::RegularExpression(_) => "TEXT",
        Bson::JavaScriptCode(_) => "TEXT",
        Bson::JavaScriptCodeWithScope(_) => "TEXT",
        Bson::Int32(_) | Bson::Int64(_) => "INTEGER",
        Bson::Timestamp(_) => "INTEGER",
        Bson::Decimal128(_) => "TEXT", // Store as string for precision
        Bson::MaxKey | Bson::MinKey => "TEXT",
        Bson::DbPointer(_) => "NULL",
        Bson::Symbol(_) => "TEXT",
    }
}

/// Convert a MongoDB document to a vector of SQL values
///
/// # Arguments
/// * `doc` - The MongoDB document to convert
/// * `field_names` - Ordered list of field names to extract
///
/// # Returns
/// Vector of SQL values in the same order as field_names
pub fn document_to_sql_values(doc: &Document, field_names: &[String]) -> Vec<SqlValue> {
    field_names
        .iter()
        .map(|field_name| {
            doc.get(field_name)
                .map(bson_to_sql_value)
                .unwrap_or(SqlValue::Null)
        })
        .collect()
}

/// Escape SQL identifier (table or column name)
///
/// # Arguments
/// * `identifier` - The identifier to escape
///
/// # Returns
/// Escaped identifier safe for use in SQL
pub fn escape_identifier(identifier: &str) -> String {
    // SQLite uses double quotes for identifiers
    // Escape any existing double quotes by doubling them
    format!("\"{}\"", identifier.replace('"', "\"\""))
}

#[cfg(test)]
mod tests {
    use super::*;
    use bson::oid::ObjectId;

    #[test]
    fn test_bson_string_to_sql() {
        let bson = Bson::String("hello".to_string());
        match bson_to_sql_value(&bson) {
            SqlValue::Text(s) => assert_eq!(s, "hello"),
            _ => panic!("Expected Text value"),
        }
    }

    #[test]
    fn test_bson_int_to_sql() {
        let bson = Bson::Int32(42);
        match bson_to_sql_value(&bson) {
            SqlValue::Integer(i) => assert_eq!(i, 42),
            _ => panic!("Expected Integer value"),
        }
    }

    #[test]
    fn test_bson_bool_to_sql() {
        let bson = Bson::Boolean(true);
        match bson_to_sql_value(&bson) {
            SqlValue::Integer(i) => assert_eq!(i, 1),
            _ => panic!("Expected Integer value"),
        }
    }

    #[test]
    fn test_bson_null_to_sql() {
        let bson = Bson::Null;
        match bson_to_sql_value(&bson) {
            SqlValue::Null => (),
            _ => panic!("Expected Null value"),
        }
    }

    #[test]
    fn test_bson_objectid_to_sql() {
        let oid = ObjectId::new();
        let bson = Bson::ObjectId(oid);
        match bson_to_sql_value(&bson) {
            SqlValue::Text(s) => assert_eq!(s, oid.to_hex()),
            _ => panic!("Expected Text value"),
        }
    }

    #[test]
    fn test_infer_types() {
        assert_eq!(infer_sqlite_type(&Bson::String("test".into())), "TEXT");
        assert_eq!(infer_sqlite_type(&Bson::Int32(42)), "INTEGER");
        assert_eq!(infer_sqlite_type(&Bson::Double(3.14)), "REAL");
        assert_eq!(infer_sqlite_type(&Bson::Boolean(true)), "INTEGER");
        assert_eq!(infer_sqlite_type(&Bson::Null), "NULL");
    }

    #[test]
    fn test_escape_identifier() {
        assert_eq!(escape_identifier("users"), "\"users\"");
        assert_eq!(escape_identifier("user_name"), "\"user_name\"");
        assert_eq!(escape_identifier("user\"name"), "\"user\"\"name\"");
    }

    #[test]
    fn test_document_to_sql_values() {
        let mut doc = Document::new();
        doc.insert("name", "Alice");
        doc.insert("age", 30);
        doc.insert("active", true);

        let field_names = vec![
            "name".to_string(),
            "age".to_string(),
            "active".to_string(),
        ];

        let values = document_to_sql_values(&doc, &field_names);
        assert_eq!(values.len(), 3);
    }
}

