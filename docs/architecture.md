# Architecture

This document describes the architecture and design decisions of the MongoDB to SQLite migration tool.

## Overview

The tool is structured as a modular Rust application with clear separation of concerns:

```
┌─────────────────────────────────────────────────────────┐
│                      CLI Layer                          │
│                    (clap parser)                        │
└──────────────────────┬──────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────┐
│                  Configuration                          │
│              (Args → Config mapping)                    │
└──────────────────────┬──────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────┐
│                     Migrator                            │
│            (Orchestration & Control Flow)               │
└───────────┬────────────────────────────┬────────────────┘
            │                            │
            ▼                            ▼
┌──────────────────────┐    ┌──────────────────────────┐
│  Schema Migrator     │    │   Data Migrator          │
│  - Schema Analysis   │    │   - Batch Processing     │
│  - DDL Generation    │    │   - Type Conversion      │
│  - Table Creation    │    │   - Data Transfer        │
└──────────┬───────────┘    └───────────┬──────────────┘
           │                            │
           └────────────┬───────────────┘
                        │
           ┌────────────┴──────────────┐
           │                           │
           ▼                           ▼
┌─────────────────────┐    ┌─────────────────────────┐
│  MongoDB Client     │    │  SQLite/Turso Client    │
│  - Connection       │    │  - Connection           │
│  - Data Fetching    │    │  - DDL Execution        │
│  - Schema Sampling  │    │  - Data Insertion       │
└─────────────────────┘    └─────────────────────────┘
```

## Core Components

### 1. CLI Layer (`cli.rs`)

**Responsibilities:**
- Parse command-line arguments using clap
- Validate argument combinations
- Provide user-friendly help messages

**Key Types:**
- `Args`: Struct representing CLI arguments
- `MigrationScope`: Enum defining what to migrate (all or single collection)

### 2. Configuration (`config.rs`)

**Responsibilities:**
- Transform CLI arguments into application configuration
- Determine connection type (local SQLite vs. remote Turso)
- Set migration options (schema, data, or both)

**Key Types:**
- `Config`: Main configuration struct
- `SqliteConfig`: Enum for local/remote SQLite configuration

### 3. Database Clients

#### MongoDB Client (`db/mongodb_client.rs`)

**Responsibilities:**
- Establish and manage MongoDB connections
- List collections
- Sample documents for schema inference
- Fetch documents in batches for data migration

**Key Methods:**
- `new()`: Establishes connection
- `list_collections()`: Returns all collection names
- `sample_documents()`: Fetches sample for schema analysis
- `fetch_all_documents()`: Streams all documents with callback

#### SQLite Client (`db/sqlite_client.rs`)

**Responsibilities:**
- Manage SQLite/Turso connections
- Execute DDL statements
- Perform batch inserts
- Transaction management

**Key Methods:**
- `new()`: Creates connection (local or remote based on config)
- `execute()`: Executes single SQL statement
- `execute_batch()`: Executes multiple statements in transaction
- `insert_batch()`: Inserts multiple rows efficiently

### 4. Schema System

#### Schema Types (`schema/types.rs`)

**Core Types:**
- `SqliteType`: Enum of SQLite data types
- `ColumnDefinition`: Column schema with constraints
- `TableSchema`: Complete table definition
- `IndexDefinition`: Index specification
- `FieldStats`: Statistics for schema inference

**Type Mapping:**
- Intelligent BSON → SQLite type conversion
- Supports nested documents (flattened with dot notation)
- JSON columns for complex types

#### Schema Analyzer (`schema/analyzer.rs`)

**Responsibilities:**
- Analyze MongoDB collections
- Infer schema from sample documents
- Determine column types and constraints
- Calculate field frequencies for nullability

**Algorithm:**
1. Sample N documents (default: 1000)
2. Collect statistics on all fields
3. Determine most common type for each field
4. Set nullability based on field frequency
5. Generate table schema with appropriate columns

**Features:**
- Nested document flattening (up to 2 levels)
- Frequency-based nullability
- Type conflict resolution
- Metadata preservation

#### SQL Generator (`schema/generator.rs`)

**Responsibilities:**
- Generate CREATE TABLE statements
- Generate CREATE INDEX statements
- Generate INSERT statements
- Handle identifier quoting for safety

**SQL Safety:**
- Proper identifier quoting
- SQL injection prevention
- JSON validation constraints

### 5. Migration Orchestration

#### Main Migrator (`migrator/mod.rs`)

**Responsibilities:**
- Coordinate schema and data migration
- Manage migration scope (all or specific collections)
- Handle migration modes (schema-only, data-only, or both)

**Workflow:**
```
migrate_all()
    ↓
migrate_schema()
    ↓
    └─→ For each collection:
        └─→ Analyze schema
        └─→ Generate DDL
        └─→ Create table
        └─→ Create indexes
    ↓
migrate_data()
    ↓
    └─→ For each collection:
        └─→ Fetch documents in batches
        └─→ Convert to SQL rows
        └─→ Insert in transactions
```

#### Schema Migrator (`migrator/schema_migrator.rs`)

**Responsibilities:**
- Create tables based on analyzed schema
- Drop existing tables if needed
- Create indexes

**Process:**
1. Analyze collection schema
2. Check if table exists
3. Drop table if exists (recreate)
4. Generate CREATE TABLE SQL
5. Execute DDL
6. Create indexes

#### Data Migrator (`migrator/data_migrator.rs`)

**Responsibilities:**
- Transfer data from MongoDB to SQLite
- Batch processing for efficiency
- Type conversion for each field
- Handle nested documents

**Process:**
1. Get table schema
2. Fetch documents in batches
3. For each document:
   - Extract values for all columns
   - Convert BSON → SQL values
   - Handle nested fields (dot notation)
4. Insert batch in transaction
5. Repeat until all documents migrated

**Type Conversion:**
- Primitives: Direct conversion
- ObjectId: Hex string
- DateTime: ISO 8601 string
- Arrays/Documents: JSON string
- Binary: Hex-encoded blob
- Nested documents: Dot notation access

### 6. Error Handling (`error.rs`)

**Custom Error Types:**
- `MongoConnection`: MongoDB connection errors
- `SqliteConnection`: SQLite/Turso connection errors
- `SchemaMigration`: Schema migration failures
- `DataMigration`: Data migration failures
- `TypeConversion`: Type conversion errors
- `CollectionNotFound`: Invalid collection specified

**Error Strategy:**
- Comprehensive error types using `thiserror`
- Proper error propagation with `Result` types
- Graceful error handling with logging

## Design Decisions

### 1. Schema Inference

**Why sample-based?**
- MongoDB is schemaless; documents may vary
- Sampling provides good balance between accuracy and performance
- 1000 documents default covers most use cases

**Handling Schema Variations:**
- Use most common type for each field
- Set nullability based on field frequency (<100% = nullable)
- Store type metadata for debugging

### 2. Nested Document Handling

**Flattening Strategy:**
- Convert nested documents to dot notation (`user.address.city`)
- Limit nesting depth to 2 levels (prevents column explosion)
- Store deep nesting as JSON

**Trade-offs:**
- Pros: Maintains relational structure, easier querying
- Cons: Many columns for deeply nested documents

### 3. Type Mapping

**Conservative Approach:**
- Prioritize data preservation over perfect types
- Use TEXT as fallback for unknown types
- JSON for arrays and complex documents
- BLOB for binary data

### 4. Batch Processing

**Why batching?**
- Memory efficiency for large collections
- Better transaction performance
- Progress tracking capability

**Batch Size:**
- Default: 1000 documents
- Configurable via CLI
- Balance between memory and transaction overhead

### 5. Transaction Management

**Strategy:**
- Each batch in a transaction
- Rollback on batch failure
- Continue with next batch after error logging

**Benefits:**
- Atomic batch operations
- Better performance than single-row inserts
- Partial failure recovery

### 6. Turso Integration

**Connection Abstraction:**
- Unified interface for local and remote
- Environment variable detection
- Automatic fallback to local file

**Remote Benefits:**
- Leverage Turso's edge network
- No local storage required
- Native SQLite compatibility

## Performance Considerations

### Memory Usage

- Streaming document fetching (not all at once)
- Batch processing limits memory per batch
- Connection pooling for efficiency

### Network Efficiency

- Batch fetching from MongoDB
- Batch inserts to SQLite/Turso
- Configurable batch sizes

### Optimization Opportunities

1. **Parallel Collection Migration**: Migrate multiple collections concurrently
2. **Index Creation Timing**: Create indexes after data load
3. **Prepared Statements**: Cache INSERT statements
4. **Compression**: Compress JSON fields

## Extensibility

### Adding New Features

**Custom Type Handlers:**
```rust
impl SqliteType {
    pub fn from_custom(value: &CustomType) -> Self {
        // Custom conversion logic
    }
}
```

**Custom Analyzers:**
```rust
pub trait SchemaAnalyzer {
    fn analyze(&self, docs: &[Document]) -> Schema;
}
```

**Migration Hooks:**
```rust
pub trait MigrationHook {
    async fn before_migrate(&self) -> Result<()>;
    async fn after_migrate(&self) -> Result<()>;
}
```

## Testing Strategy

### Unit Tests

- Type conversions
- SQL generation
- Field statistics
- Nested field access

### Integration Tests

- End-to-end migration
- MongoDB → SQLite roundtrip
- Schema inference accuracy
- Error handling

### Performance Tests

- Large collection migration
- Batch size optimization
- Memory usage profiling

## Future Enhancements

1. **Incremental Migration**: Sync only changed documents
2. **Schema Migration**: Handle schema evolution
3. **Validation**: Compare source and destination data
4. **Compression**: Automatic data compression
5. **Indexes**: Smart index recommendation
6. **Constraints**: Infer foreign key relationships
7. **Parallel Processing**: Multi-threaded migration
8. **Resume Capability**: Checkpoint and resume migrations

