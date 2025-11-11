# Examples

This document provides practical examples of using the MongoDB to SQLite migration tool in various scenarios.

## Basic Examples

### Example 1: Simple Local Migration

Migrate a single MongoDB collection to a local SQLite file:

```bash
# Start with a simple collection
mongo-to-sqlite \
  --database blog \
  --table posts \
  --output blog.db
```

**Use Case**: Perfect for development environments or backing up specific collections.

### Example 2: Migrate All Collections

Migrate an entire MongoDB database:

```bash
mongo-to-sqlite \
  --mongodb-uri "mongodb://localhost:27017" \
  --database myapp \
  --all-tables \
  --output myapp.db
```

**Use Case**: Creating a complete SQLite replica of a MongoDB database.

### Example 3: Preview Schema Before Migration

Check what the SQLite schema will look like without migrating data:

```bash
# Generate schema only
mongo-to-sqlite \
  --database myapp \
  --all-tables \
  --schema-only \
  --output preview.db

# Inspect with sqlite3
sqlite3 preview.db ".schema"
```

**Use Case**: Validating schema before committing to a full migration.

## Advanced Examples

### Example 4: Migrate to Turso Cloud

Deploy your MongoDB data to Turso's distributed database:

```bash
# Set Turso credentials
export TURSO_DATABASE_URL="libsql://my-database-org.turso.io"
export TURSO_AUTH_TOKEN="eyJhbGc..."

# Migrate (output path is ignored when using Turso)
mongo-to-sqlite \
  --database production \
  --all-tables
```

**Use Case**: Deploying to edge locations with Turso's global distribution.

### Example 5: Large Dataset with Custom Batch Size

Optimize memory usage and performance for large collections:

```bash
# Larger batch size for faster migration
mongo-to-sqlite \
  --database analytics \
  --table events \
  --batch-size 5000 \
  --sample-size 500 \
  --output events.db
```

**Use Case**: Migrating millions of documents efficiently.

### Example 6: Remote MongoDB to Local SQLite

Connect to a remote MongoDB instance:

```bash
mongo-to-sqlite \
  --mongodb-uri "mongodb+srv://user:pass@cluster.mongodb.net" \
  --database production \
  --table users \
  --output users_backup.db
```

**Use Case**: Creating local backups from cloud MongoDB instances.

### Example 7: Incremental Table Migration

Migrate tables one at a time to the same database:

```bash
# First table (includes schema)
mongo-to-sqlite \
  --database ecommerce \
  --table products \
  --output ecommerce.db

# Additional tables (data only to same file)
mongo-to-sqlite \
  --database ecommerce \
  --table orders \
  --data-only \
  --output ecommerce.db

mongo-to-sqlite \
  --database ecommerce \
  --table customers \
  --data-only \
  --output ecommerce.db
```

**Use Case**: Controlled migration with testing between each collection.

### Example 8: Development to Production Workflow

Test locally before deploying to production:

```bash
# Step 1: Test with development database
mongo-to-sqlite \
  --database dev_myapp \
  --all-tables \
  --output dev-test.db

# Step 2: Verify data
sqlite3 dev-test.db "SELECT COUNT(*) FROM users;"

# Step 3: Deploy to production Turso
export TURSO_DATABASE_URL="libsql://prod-myapp.turso.io"
export TURSO_AUTH_TOKEN="$PROD_TOKEN"

mongo-to-sqlite \
  --mongodb-uri "$PROD_MONGO_URI" \
  --database prod_myapp \
  --all-tables
```

**Use Case**: Safe production deployments with testing.

## Real-World Scenarios

### Scenario 1: E-Commerce Application

```bash
# Migrate product catalog for edge search
mongo-to-sqlite \
  --mongodb-uri "mongodb://mongo.example.com:27017" \
  --database ecommerce \
  --table products \
  --sample-size 1000 \
  --batch-size 2000 \
  --output products.db

# Products are now queryable in SQLite
sqlite3 products.db "CREATE INDEX idx_product_category ON products(category);"
```

### Scenario 2: Analytics Data Archive

```bash
# Archive old analytics data from MongoDB to SQLite
mongo-to-sqlite \
  --mongodb-uri "mongodb://analytics.internal" \
  --database analytics \
  --table events_2023 \
  --batch-size 10000 \
  --output archives/analytics_2023.db
```

### Scenario 3: Multi-Tenant Application

```bash
# Migrate each tenant's data separately
for tenant in tenant1 tenant2 tenant3; do
  mongo-to-sqlite \
    --database "$tenant" \
    --all-tables \
    --output "tenants/${tenant}.db"
done
```

### Scenario 4: Microservices Data Consolidation

```bash
# Consolidate data from multiple MongoDB databases
# into one SQLite file for reporting

# Users service
mongo-to-sqlite \
  --database users_service \
  --table users \
  --output consolidated.db

# Orders service
mongo-to-sqlite \
  --database orders_service \
  --table orders \
  --data-only \
  --output consolidated.db

# Products service
mongo-to-sqlite \
  --database products_service \
  --table products \
  --data-only \
  --output consolidated.db
```

## Using with Scripts

### Bash Script Example

```bash
#!/bin/bash
# migrate-all.sh - Migrate multiple databases

DATABASES=("app1" "app2" "app3")
MONGO_URI="mongodb://localhost:27017"

for db in "${DATABASES[@]}"; do
  echo "Migrating database: $db"
  
  mongo-to-sqlite \
    --mongodb-uri "$MONGO_URI" \
    --database "$db" \
    --all-tables \
    --output "output/${db}.db"
  
  if [ $? -eq 0 ]; then
    echo "✓ Successfully migrated $db"
  else
    echo "✗ Failed to migrate $db"
    exit 1
  fi
done

echo "All databases migrated successfully!"
```

### Python Script Example

```python
#!/usr/bin/env python3
# migrate_wrapper.py - Python wrapper for migration

import subprocess
import sys
from pathlib import Path

def migrate_collection(database, collection, output_dir):
    """Migrate a single collection"""
    output_file = Path(output_dir) / f"{database}_{collection}.db"
    
    cmd = [
        "mongo-to-sqlite",
        "--database", database,
        "--table", collection,
        "--output", str(output_file),
    ]
    
    try:
        result = subprocess.run(cmd, check=True, capture_output=True, text=True)
        print(f"✓ Migrated {database}.{collection}")
        return True
    except subprocess.CalledProcessError as e:
        print(f"✗ Failed to migrate {database}.{collection}: {e.stderr}")
        return False

def main():
    collections = [
        ("mydb", "users"),
        ("mydb", "posts"),
        ("mydb", "comments"),
    ]
    
    output_dir = Path("./output")
    output_dir.mkdir(exist_ok=True)
    
    success_count = 0
    for database, collection in collections:
        if migrate_collection(database, collection, output_dir):
            success_count += 1
    
    print(f"\nMigrated {success_count}/{len(collections)} collections")
    return 0 if success_count == len(collections) else 1

if __name__ == "__main__":
    sys.exit(main())
```

## Troubleshooting Examples

### Debug Mode

Enable verbose logging to troubleshoot issues:

```bash
RUST_LOG=debug mongo-to-sqlite \
  --database mydb \
  --table problematic_collection \
  --output debug.db
```

### Connection Testing

Test MongoDB connection before migration:

```bash
# Test connection
mongosh "$MONGODB_URI" --eval "db.adminCommand('ping')"

# List databases
mongosh "$MONGODB_URI" --eval "show databases"

# List collections
mongosh "$MONGODB_URI/mydb" --eval "show collections"
```

### Verify Migration

Check that data was migrated correctly:

```bash
# Count documents in MongoDB
mongosh "$MONGODB_URI/mydb" --eval "db.users.countDocuments()"

# Count rows in SQLite
sqlite3 output.db "SELECT COUNT(*) FROM users;"
```

## Performance Benchmarks

Based on typical hardware (M1 Mac, 16GB RAM):

| Document Count | Batch Size | Time     | Notes                    |
|----------------|------------|----------|--------------------------|
| 1,000          | 1,000      | ~2s      | Default settings         |
| 10,000         | 1,000      | ~15s     | Small collection         |
| 100,000        | 5,000      | ~90s     | Medium collection        |
| 1,000,000      | 10,000     | ~15min   | Large collection         |

**Tips for Better Performance:**
- Increase `--batch-size` for faster migrations (more memory usage)
- Run tool on same network as MongoDB for lower latency
- Use SSD for local SQLite files
- For very large datasets, consider migrating tables in parallel

## Next Steps

After migrating your data:

1. **Create Indexes**: Add appropriate indexes for your query patterns
   ```sql
   CREATE INDEX idx_users_email ON users(email);
   CREATE INDEX idx_posts_author ON posts(author_id);
   ```

2. **Optimize Database**: Run SQLite optimization
   ```sql
   VACUUM;
   ANALYZE;
   ```

3. **Verify Data**: Query your data to ensure accuracy
   ```sql
   SELECT COUNT(*) FROM users;
   SELECT * FROM users LIMIT 10;
   ```

4. **Backup**: Create backups of your SQLite files
   ```bash
   cp mydb.db mydb.db.backup
   ```

