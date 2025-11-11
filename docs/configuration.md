# Configuration

This document describes all configuration options available for the MongoDB to SQLite migration tool.

## Environment Variables

### MongoDB Configuration

#### `MONGODB_URI`
- **Description**: MongoDB connection string
- **Default**: `mongodb://localhost:27017`
- **Format**: Standard MongoDB connection URI
- **Examples**:
  ```bash
  # Local MongoDB
  export MONGODB_URI="mongodb://localhost:27017"
  
  # Remote MongoDB with authentication
  export MONGODB_URI="mongodb://user:password@mongo.example.com:27017"
  
  # MongoDB Atlas
  export MONGODB_URI="mongodb+srv://user:pass@cluster.mongodb.net"
  
  # Replica Set
  export MONGODB_URI="mongodb://host1:27017,host2:27017,host3:27017/?replicaSet=myReplSet"
  ```

### Turso/LibSQL Configuration

#### `TURSO_DATABASE_URL`
- **Description**: Turso cloud database URL
- **Default**: None (uses local file if not set)
- **Format**: `libsql://[database-name].[org-name].turso.io`
- **Example**:
  ```bash
  export TURSO_DATABASE_URL="libsql://my-database-acme.turso.io"
  ```

#### `TURSO_AUTH_TOKEN`
- **Description**: Turso authentication token
- **Default**: None
- **Format**: JWT token from Turso CLI
- **Example**:
  ```bash
  export TURSO_AUTH_TOKEN="eyJhbGciOiJFZERTQSIsInR5cCI6IkpXVCJ9..."
  ```
- **How to Get**: Use the Turso CLI
  ```bash
  turso db tokens create [database-name]
  ```

### Logging Configuration

#### `RUST_LOG`
- **Description**: Controls logging verbosity
- **Default**: `info`
- **Values**:
  - `error` - Only errors
  - `warn` - Warnings and errors
  - `info` - Informational messages (recommended)
  - `debug` - Detailed debugging information
  - `trace` - Very verbose debugging
- **Examples**:
  ```bash
  # Info level (default)
  export RUST_LOG=info
  
  # Debug everything
  export RUST_LOG=debug
  
  # Debug specific modules
  export RUST_LOG=mongo_to_sqlite::migration=debug
  
  # Multiple modules
  export RUST_LOG=mongo_to_sqlite::migration=debug,mongo_to_sqlite::schema=trace
  ```

## Command-Line Arguments

### Required Arguments

#### `--database <DATABASE>`
- **Description**: MongoDB database name to migrate
- **Required**: Yes
- **Example**: `--database myapp`

### Table Selection (One Required)

#### `--table <TABLE>`
- **Description**: Migrate a specific collection
- **Conflicts With**: `--all-tables`
- **Example**: `--table users`

#### `--all-tables`
- **Description**: Migrate all collections in the database
- **Conflicts With**: `--table`
- **Example**: `--all-tables`

### Optional Arguments

#### `--mongodb-uri <URI>`
- **Description**: MongoDB connection URI (overrides `MONGODB_URI` env var)
- **Default**: Value of `MONGODB_URI` or `mongodb://localhost:27017`
- **Example**: `--mongodb-uri "mongodb://localhost:27017"`

#### `--output <PATH>`
- **Description**: Output SQLite database file path
- **Default**: `output.db`
- **Ignored When**: `TURSO_DATABASE_URL` and `TURSO_AUTH_TOKEN` are set
- **Example**: `--output ./data/mydb.db`

#### `--batch-size <SIZE>`
- **Description**: Number of documents to insert per batch
- **Default**: `1000`
- **Range**: 1 - 100000 (recommended: 100 - 10000)
- **Memory Impact**: Higher values use more memory but are faster
- **Example**: `--batch-size 5000`

#### `--sample-size <SIZE>`
- **Description**: Number of documents to sample for schema inference
- **Default**: `100`
- **Range**: 1 - 10000 (recommended: 50 - 500)
- **Impact**: More samples = more accurate schema but slower analysis
- **Example**: `--sample-size 500`

### Migration Mode Flags

#### `--schema-only`
- **Description**: Only migrate schema (CREATE TABLE statements)
- **Conflicts With**: `--data-only`
- **Use Case**: Preview schema before full migration
- **Example**: `--schema-only`

#### `--data-only`
- **Description**: Only migrate data (assumes tables exist)
- **Conflicts With**: `--schema-only`
- **Use Case**: Add data to existing schema
- **Example**: `--data-only`

## Configuration Files

### `.env` File

Create a `.env` file in the project root for local development:

```bash
# .env
MONGODB_URI=mongodb://localhost:27017
TURSO_DATABASE_URL=libsql://dev-database.turso.io
TURSO_AUTH_TOKEN=your-token-here
RUST_LOG=info
```

**Note**: Add `.env` to `.gitignore` to avoid committing secrets.

### Environment-Specific Configuration

Create multiple environment files:

```bash
# .env.development
MONGODB_URI=mongodb://localhost:27017
RUST_LOG=debug

# .env.staging
MONGODB_URI=mongodb://staging-mongo.example.com:27017
TURSO_DATABASE_URL=libsql://staging-db.turso.io
TURSO_AUTH_TOKEN=staging-token
RUST_LOG=info

# .env.production
MONGODB_URI=mongodb://prod-mongo.example.com:27017
TURSO_DATABASE_URL=libsql://prod-db.turso.io
TURSO_AUTH_TOKEN=prod-token
RUST_LOG=warn
```

Load the appropriate environment:

```bash
# Development
source .env.development
mongo-to-sqlite --database myapp --all-tables

# Production
source .env.production
mongo-to-sqlite --database myapp --all-tables
```

## Configuration Priority

Settings are applied in the following order (later overrides earlier):

1. Default values (hardcoded)
2. Environment variables
3. Command-line arguments

### Example

```bash
# Environment variable
export MONGODB_URI="mongodb://localhost:27017"

# Command-line argument overrides environment variable
mongo-to-sqlite \
  --mongodb-uri "mongodb://remote:27017" \
  --database mydb \
  --all-tables
# Uses: mongodb://remote:27017
```

## Performance Tuning

### Small Collections (< 10,000 documents)
```bash
mongo-to-sqlite \
  --database mydb \
  --table small_collection \
  --batch-size 1000 \
  --sample-size 100 \
  --output small.db
```

### Medium Collections (10,000 - 1,000,000 documents)
```bash
mongo-to-sqlite \
  --database mydb \
  --table medium_collection \
  --batch-size 5000 \
  --sample-size 200 \
  --output medium.db
```

### Large Collections (> 1,000,000 documents)
```bash
mongo-to-sqlite \
  --database mydb \
  --table large_collection \
  --batch-size 10000 \
  --sample-size 500 \
  --output large.db
```

### Memory-Constrained Environments
```bash
mongo-to-sqlite \
  --database mydb \
  --table any_collection \
  --batch-size 100 \
  --sample-size 50 \
  --output constrained.db
```

## Security Considerations

### Storing Credentials

**❌ Don't**:
- Commit credentials to version control
- Pass credentials in command history
- Log credentials in application logs

**✅ Do**:
- Use environment variables
- Store in `.env` files (add to `.gitignore`)
- Use secret management tools (Vault, AWS Secrets Manager)
- Use IAM roles when possible

### Example: Secure Credential Management

```bash
#!/bin/bash
# secure-migrate.sh

# Load credentials from secure storage
MONGODB_URI=$(vault kv get -field=uri secret/mongo)
TURSO_AUTH_TOKEN=$(vault kv get -field=token secret/turso)

# Export for tool to use
export MONGODB_URI
export TURSO_AUTH_TOKEN

# Run migration (credentials not in command)
mongo-to-sqlite --database mydb --all-tables
```

## Validation

### Check Configuration

Before running a migration, validate your configuration:

```bash
# Test MongoDB connection
mongosh "$MONGODB_URI" --eval "db.adminCommand('ping')"

# Test Turso connection (requires turso CLI)
turso db shell "$TURSO_DATABASE_URL" ".tables"

# Verify environment variables
env | grep MONGODB_URI
env | grep TURSO_
```

### Dry Run

Use `--schema-only` to validate without migrating data:

```bash
mongo-to-sqlite \
  --database mydb \
  --all-tables \
  --schema-only \
  --output test.db

# Inspect the schema
sqlite3 test.db ".schema"
```

## Troubleshooting

### Issue: Cannot Connect to MongoDB

**Check**:
1. MongoDB URI format
2. Network connectivity
3. Authentication credentials
4. MongoDB is running

```bash
# Test connection
mongosh "$MONGODB_URI" --eval "db.adminCommand('ping')"
```

### Issue: Cannot Connect to Turso

**Check**:
1. TURSO_DATABASE_URL format
2. TURSO_AUTH_TOKEN validity
3. Network connectivity

```bash
# Test with Turso CLI
turso db show [database-name]
```

### Issue: Out of Memory

**Solutions**:
1. Reduce `--batch-size`
2. Migrate tables individually
3. Increase system memory
4. Use a machine with more RAM

```bash
# Lower memory usage
mongo-to-sqlite \
  --database mydb \
  --table large_collection \
  --batch-size 100 \
  --output large.db
```

### Issue: Slow Migration

**Solutions**:
1. Increase `--batch-size`
2. Run on same network as MongoDB
3. Use SSD for local storage
4. Reduce `--sample-size`

```bash
# Faster migration
mongo-to-sqlite \
  --database mydb \
  --table collection \
  --batch-size 10000 \
  --sample-size 100 \
  --output fast.db
```

