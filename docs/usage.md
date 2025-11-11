# Usage Guide

Comprehensive guide for using the MongoDB to SQLite migration tool.

## Table of Contents

- [Installation](#installation)
- [Basic Usage](#basic-usage)
- [Advanced Usage](#advanced-usage)
- [Common Scenarios](#common-scenarios)
- [Troubleshooting](#troubleshooting)
- [Best Practices](#best-practices)

## Installation

### Prerequisites

- Rust 1.70 or higher
- MongoDB 4.0+ (source database)
- SQLite 3.35+ (for JSON support)

### Building from Source

```bash
# Clone the repository
git clone <repository-url>
cd mongo-to-sqlite

# Build the release binary
cargo build --release

# The binary will be at target/release/mongo-to-sqlite
```

### Installing with Cargo

```bash
cargo install --path .
```

### Verify Installation

```bash
mongo-to-sqlite --version
```

## Basic Usage

### Migrate Entire Database

Migrate all collections from MongoDB to a local SQLite file:

```bash
mongo-to-sqlite \
  --mongodb-uri "mongodb://localhost:27017" \
  --database mydb \
  --output mydb.sqlite \
  --all-tables
```

### Migrate Single Collection

Migrate only one collection:

```bash
mongo-to-sqlite \
  --mongodb-uri "mongodb://localhost:27017" \
  --database mydb \
  --table users \
  --output users.sqlite
```

### Using Environment Variables

Set MongoDB URI via environment variable:

```bash
export MONGODB_URI="mongodb://localhost:27017"

mongo-to-sqlite \
  --database mydb \
  --all-tables
```

## Advanced Usage

### Schema Only Migration

Generate table structures without data:

```bash
mongo-to-sqlite \
  --mongodb-uri "mongodb://localhost:27017" \
  --database mydb \
  --schema-only \
  --all-tables
```

**Use cases:**
- Preview table structure before full migration
- Create schema in production before data load
- Schema documentation generation

### Data Only Migration

Migrate data into existing tables:

```bash
mongo-to-sqlite \
  --mongodb-uri "mongodb://localhost:27017" \
  --database mydb \
  --data-only \
  --all-tables
```

**Prerequisites:**
- Tables must already exist
- Schema must match (use same tool for schema creation)

**Use cases:**
- Reload data into existing tables
- Incremental data updates
- Separate schema and data migration phases

### Turso Remote Database

#### Using Environment Variables (Recommended)

```bash
export TURSO_DATABASE_URL="libsql://your-database.turso.io"
export TURSO_AUTH_TOKEN="your-auth-token-here"

mongo-to-sqlite \
  --mongodb-uri "mongodb://localhost:27017" \
  --database mydb \
  --all-tables
```

#### Using CLI Arguments

```bash
mongo-to-sqlite \
  --mongodb-uri "mongodb://localhost:27017" \
  --database mydb \
  --turso-url "libsql://your-database.turso.io" \
  --turso-token "your-auth-token-here" \
  --all-tables
```

### Custom Batch Size

Adjust batch size for memory/performance trade-off:

```bash
# Small batch size (less memory, more transactions)
mongo-to-sqlite \
  --mongodb-uri "mongodb://localhost:27017" \
  --database mydb \
  --all-tables \
  --batch-size 500

# Large batch size (more memory, fewer transactions)
mongo-to-sqlite \
  --mongodb-uri "mongodb://localhost:27017" \
  --database mydb \
  --all-tables \
  --batch-size 5000
```

**Recommendations:**
- Small collections: 5000-10000
- Medium collections: 1000-5000 (default)
- Large collections: 500-1000
- Limited memory: 100-500

### Verbose Logging

Enable detailed logging for debugging:

```bash
mongo-to-sqlite \
  --mongodb-uri "mongodb://localhost:27017" \
  --database mydb \
  --all-tables \
  --verbose
```

## Common Scenarios

### Scenario 1: Blog Database Migration

**MongoDB Collections:**
- `posts` - Blog posts with nested author and comments
- `users` - User accounts
- `categories` - Post categories

```bash
# Migrate all collections
mongo-to-sqlite \
  --mongodb-uri "mongodb://localhost:27017" \
  --database blog \
  --output blog.sqlite \
  --all-tables \
  --batch-size 2000
```

**Result:**
- Nested author info flattened to `posts.author.name`, `posts.author.email`
- Comments array stored as JSON in `posts.comments`
- All ObjectIds converted to hex strings

### Scenario 2: E-commerce Database

**Large collections with many fields:**

```bash
# Step 1: Migrate schema first
mongo-to-sqlite \
  --mongodb-uri "mongodb://localhost:27017" \
  --database shop \
  --schema-only \
  --all-tables \
  --output shop.sqlite

# Step 2: Review generated schema
sqlite3 shop.sqlite ".schema"

# Step 3: Migrate data
mongo-to-sqlite \
  --mongodb-uri "mongodb://localhost:27017" \
  --database shop \
  --data-only \
  --all-tables \
  --batch-size 500
```

### Scenario 3: Selective Collection Migration

**Migrate only specific collections:**

```bash
# Migrate users collection
mongo-to-sqlite \
  -m "mongodb://localhost:27017" \
  -d myapp \
  -t users \
  -o myapp_users.sqlite

# Migrate orders collection
mongo-to-sqlite \
  -m "mongodb://localhost:27017" \
  -d myapp \
  -t orders \
  -o myapp_orders.sqlite
```

### Scenario 4: Production Migration to Turso

**Safe production migration:**

```bash
# Step 1: Test with local SQLite first
mongo-to-sqlite \
  --mongodb-uri "mongodb://prod-mongo:27017" \
  --database production \
  --all-tables \
  --output test_migration.sqlite

# Step 2: Verify data integrity
sqlite3 test_migration.sqlite "SELECT COUNT(*) FROM users;"

# Step 3: Migrate to Turso
export TURSO_DATABASE_URL="libsql://prod-db.turso.io"
export TURSO_AUTH_TOKEN="<token>"

mongo-to-sqlite \
  --mongodb-uri "mongodb://prod-mongo:27017" \
  --database production \
  --all-tables \
  --batch-size 1000
```

### Scenario 5: Time-Series Data

**Large time-series collection:**

```bash
# Use smaller batch size for memory efficiency
mongo-to-sqlite \
  --mongodb-uri "mongodb://localhost:27017" \
  --database analytics \
  --table events \
  --output events.sqlite \
  --batch-size 500 \
  --verbose
```

## Troubleshooting

### Connection Issues

**Problem:** Cannot connect to MongoDB

```
Error: MongoDB connection error: ...
```

**Solutions:**
1. Verify MongoDB is running: `mongo --eval "db.runCommand({ ping: 1 })"`
2. Check connection string format
3. Verify authentication credentials
4. Check network/firewall settings

### Schema Inference Issues

**Problem:** Empty collection error

```
Error: Collection 'xyz' is empty, cannot infer schema
```

**Solutions:**
1. Ensure collection has at least one document
2. Use a different collection as template
3. Create schema manually first

### Memory Issues

**Problem:** Out of memory during migration

**Solutions:**
1. Reduce batch size: `--batch-size 100`
2. Migrate collections one at a time
3. Increase system memory
4. Use streaming mode (already implemented)

### Type Conversion Errors

**Problem:** Failed to convert document

```
WARN: Failed to convert document: ...
```

**Solutions:**
1. Check MongoDB data for corrupted documents
2. Review verbose logs: `--verbose`
3. Report issue with document structure

### Turso Connection Issues

**Problem:** Cannot connect to Turso

**Solutions:**
1. Verify TURSO_DATABASE_URL format: `libsql://your-db.turso.io`
2. Check auth token is valid
3. Test connection with Turso CLI: `turso db shell`
4. Fall back to local SQLite for testing

## Best Practices

### 1. Test Before Production

Always test migration on a copy first:

```bash
# Test with local SQLite
mongo-to-sqlite \
  --mongodb-uri "mongodb://prod:27017" \
  --database prod_db \
  --table users \
  --output test_users.sqlite

# Verify results
sqlite3 test_users.sqlite "SELECT COUNT(*) FROM users;"
```

### 2. Use Schema-First Approach

Review schema before data migration:

```bash
# Generate schema only
mongo-to-sqlite \
  --mongodb-uri "..." \
  --database mydb \
  --schema-only \
  --all-tables

# Review schema
sqlite3 output.db ".schema"

# Migrate data if schema looks good
mongo-to-sqlite \
  --mongodb-uri "..." \
  --database mydb \
  --data-only \
  --all-tables
```

### 3. Monitor Progress

Use verbose mode for long-running migrations:

```bash
mongo-to-sqlite \
  --mongodb-uri "..." \
  --database large_db \
  --all-tables \
  --verbose 2>&1 | tee migration.log
```

### 4. Optimize Batch Size

Test different batch sizes:

```bash
# Time migration with different batch sizes
time mongo-to-sqlite ... --batch-size 500
time mongo-to-sqlite ... --batch-size 1000
time mongo-to-sqlite ... --batch-size 2000
```

### 5. Handle Nested Documents

For deeply nested documents:
- Consider manual schema design
- Use JSON columns for complex structures
- Query JSON in SQLite using `json_extract()`

Example query:
```sql
SELECT 
  _id,
  json_extract(comments, '$[0].text') as first_comment
FROM posts;
```

### 6. Index Strategy

The tool creates indexes for `_id` fields. Add more indexes after migration:

```sql
-- Add indexes after migration
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_posts_created ON posts(created_at);
```

### 7. Validate Data

After migration, validate:

```bash
# Count documents
mongo mydb --eval "db.users.count()"
sqlite3 output.db "SELECT COUNT(*) FROM users;"

# Spot check records
mongo mydb --eval "db.users.findOne()"
sqlite3 output.db "SELECT * FROM users LIMIT 1;"
```

### 8. Backup Strategy

Always backup before data-only migrations:

```bash
# Backup existing SQLite file
cp production.db production.db.backup

# Then migrate data
mongo-to-sqlite ... --data-only ...
```

## Performance Tips

### Large Datasets

For datasets > 1GB:

1. Use appropriate batch size (500-1000)
2. Consider migrating collections in parallel (manual process)
3. Monitor system resources
4. Use SSD storage for SQLite file

### Network Optimization

For remote MongoDB:

1. Run tool close to MongoDB server
2. Use local network when possible
3. Consider batch size vs. network latency

### SQLite Optimization

After migration:

```sql
-- Analyze tables for query optimization
ANALYZE;

-- Vacuum to reclaim space
VACUUM;
```

## Examples

See the [examples](../examples/) directory for:
- Complete migration scripts
- Docker compose setups
- Advanced use cases
- Custom type handlers

## Getting Help

For issues or questions:

1. Check this documentation
2. Review error messages with `--verbose`
3. Check GitHub issues
4. Open a new issue with:
   - MongoDB version
   - Collection structure (sample document)
   - Command used
   - Full error message

