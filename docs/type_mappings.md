# Type Mappings: MongoDB to SQLite

## Overview

This document outlines how MongoDB BSON types are mapped to SQLite types during migration.

## Type Conversion Table

| MongoDB BSON Type | SQLite Type | Notes |
|-------------------|-------------|-------|
| String | TEXT | Direct mapping |
| Int32 | INTEGER | Direct mapping |
| Int64 | INTEGER | Direct mapping |
| Double | REAL | Direct mapping |
| Boolean | INTEGER | 0 for false, 1 for true |
| Date | TEXT | ISO 8601 format (YYYY-MM-DD HH:MM:SS) |
| ObjectId | TEXT | Hex string representation |
| Null | NULL | Direct mapping |
| Array | TEXT | Stored as JSON string |
| Object (Nested) | TEXT | Stored as JSON string |
| Binary | BLOB | Direct mapping |
| Decimal128 | TEXT | String representation for precision |
| Timestamp | INTEGER | Unix timestamp (seconds since epoch) |
| RegEx | TEXT | Pattern and flags as JSON |
| JavaScript | TEXT | Code as string |
| MinKey/MaxKey | TEXT | Special sentinel values |

## Schema Inference Strategy

Since MongoDB is schema-less, the migration tool will:

1. **Sample Documents**: Analyze a sample of documents from each collection to infer the schema
2. **Type Detection**: Determine the most common type for each field
3. **Nullable Fields**: All fields are nullable by default unless present in 100% of sampled documents
4. **Type Conflicts**: When a field has multiple types, use TEXT as the safest option

## Handling Nested Data

### Strategy 1: JSON Serialization (Default)
Nested objects and arrays are serialized as JSON TEXT:
- **Pros**: Simple, preserves structure, no data loss
- **Cons**: Can't query nested fields efficiently, no referential integrity

### Strategy 2: Normalization (Future Enhancement)
Create separate tables for nested documents:
- **Pros**: Proper relational structure, efficient queries
- **Cons**: Complex, may not fit all use cases

## Primary Keys

- MongoDB's `_id` field becomes the primary key
- If `_id` is ObjectId, it's stored as TEXT PRIMARY KEY
- If `_id` is another type, converted appropriately

## Indexes

- MongoDB indexes are NOT automatically migrated
- Users can create indexes manually after migration based on their query patterns

## Limitations

1. **No Array Queries**: Arrays stored as JSON can't be queried element-wise without JSON1 extension
2. **No Schema Validation**: SQLite doesn't enforce MongoDB's schema validators
3. **Limited Text Search**: MongoDB's text indexes aren't directly translatable
4. **No TTL**: MongoDB's TTL indexes aren't supported
5. **No Geospatial**: Geographic queries require SQLite extensions

## Best Practices

1. Use `--schema-only` first to review the generated schema
2. Test with a subset of data before full migration
3. Consider data size and available memory for large collections
4. Add appropriate indexes after migration for your query patterns

