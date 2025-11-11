# Quickstart Guide

Get up and running with MongoDB to SQLite migration in 5 minutes.

## Installation

### Option 1: Install from Source

```bash
git clone https://github.com/yourusername/mongo-to-sqlite
cd mongo-to-sqlite
cargo build --release
sudo cp target/release/mongo-to-sqlite /usr/local/bin/
```

### Option 2: Using Cargo (once published)

```bash
cargo install mongo-to-sqlite
```

## Prerequisites

- MongoDB instance running (local or remote)
- Rust 1.70+ (for building from source)

## Your First Migration

### Step 1: Start MongoDB (if running locally)

```bash
mongod --dbpath /path/to/data
```

### Step 2: Create Sample Data (optional)

If you don't have existing data, create some test data:

```bash
mongosh
```

```javascript
use testdb

db.users.insertMany([
  { name: "Alice", age: 30, email: "alice@example.com", active: true },
  { name: "Bob", age: 25, email: "bob@example.com", active: true },
  { name: "Charlie", age: 35, email: "charlie@example.com", active: false }
])

db.posts.insertMany([
  { title: "First Post", author: "Alice", content: "Hello World", tags: ["intro", "welcome"] },
  { title: "Second Post", author: "Bob", content: "Learning MongoDB", tags: ["tutorial"] }
])
```

### Step 3: Run Your First Migration

```bash
mongo-to-sqlite \
  --database testdb \
  --all-tables \
  --output testdb.db
```

You should see output like:

```
╔════════════════════════════════════════════════╗
║     MongoDB to SQLite Migration Tool          ║
║     Powered by LibSQL & Turso                  ║
╚════════════════════════════════════════════════╝

🔍 Connecting to MongoDB...
   ✓ Connected to MongoDB

📊 Found 2 collection(s): users, posts

🔗 Connecting to SQLite/LibSQL...
   ✓ Connected to SQLite/LibSQL

📋 Migrating schema...
  ✓ Created table: users (5 columns)
  ✓ Created table: posts (6 columns)

📦 Migrating data...
  users: 3/3 (100%) ✓
  posts: 2/2 (100%) ✓

✅ Migration completed successfully!
   Total documents migrated: 5
   Tables migrated: 2
   Time elapsed: 1.23s
   Output: testdb.db
```

### Step 4: Verify the Data

```bash
sqlite3 testdb.db
```

```sql
-- Check the schema
.schema

-- Query users
SELECT * FROM users;

-- Query posts
SELECT * FROM posts;

-- Check counts
SELECT COUNT(*) FROM users;
SELECT COUNT(*) FROM posts;
```

## Common Scenarios

### Migrate a Single Collection

```bash
mongo-to-sqlite \
  --database mydb \
  --table users \
  --output users.db
```

### Preview Schema First

```bash
# Generate schema without data
mongo-to-sqlite \
  --database mydb \
  --all-tables \
  --schema-only \
  --output preview.db

# Inspect it
sqlite3 preview.db ".schema"
```

### Connect to Remote MongoDB

```bash
mongo-to-sqlite \
  --mongodb-uri "mongodb://user:pass@remote-host:27017" \
  --database production \
  --table important_collection \
  --output backup.db
```

### Use with Turso Cloud

```bash
# Get your Turso credentials
turso db create my-database
turso db tokens create my-database

# Set environment variables
export TURSO_DATABASE_URL="libsql://my-database-org.turso.io"
export TURSO_AUTH_TOKEN="your-token-here"

# Run migration (output will go to Turso)
mongo-to-sqlite \
  --database mydb \
  --all-tables
```

## Configuration with Environment Variables

Create a `.env` file:

```bash
# .env
MONGODB_URI=mongodb://localhost:27017
TURSO_DATABASE_URL=libsql://my-db.turso.io
TURSO_AUTH_TOKEN=your-token
RUST_LOG=info
```

Then run:

```bash
source .env
mongo-to-sqlite --database mydb --all-tables
```

## Customizing the Migration

### Adjust Batch Size

For large collections, increase batch size for better performance:

```bash
mongo-to-sqlite \
  --database mydb \
  --table large_collection \
  --batch-size 5000 \
  --output large.db
```

### Improve Schema Inference

Sample more documents for better schema accuracy:

```bash
mongo-to-sqlite \
  --database mydb \
  --table varied_collection \
  --sample-size 500 \
  --output accurate.db
```

## Troubleshooting

### Connection Failed

**Issue**: Can't connect to MongoDB

**Solution**: Check MongoDB is running and URI is correct

```bash
# Test MongoDB connection
mongosh mongodb://localhost:27017 --eval "db.adminCommand('ping')"
```

### Permission Denied

**Issue**: Can't create output file

**Solution**: Ensure write permissions in output directory

```bash
# Create directory with proper permissions
mkdir -p output
chmod 755 output
mongo-to-sqlite --database mydb --all-tables --output output/mydb.db
```

### Out of Memory

**Issue**: Program crashes with large collections

**Solution**: Reduce batch size

```bash
mongo-to-sqlite \
  --database mydb \
  --table huge_collection \
  --batch-size 100 \
  --output huge.db
```

## Next Steps

Now that you've completed your first migration:

1. **Explore the Schema**: Use `sqlite3 <file>.db .schema` to see the generated tables
2. **Add Indexes**: Create indexes for better query performance
3. **Read the Full Documentation**: Check out the [Usage Guide](usage.md) for advanced features
4. **Try Examples**: See [Examples](examples.md) for real-world scenarios

## Common Commands Reference

```bash
# Basic migration
mongo-to-sqlite --database mydb --all-tables --output mydb.db

# Single table
mongo-to-sqlite --database mydb --table users --output users.db

# Schema only
mongo-to-sqlite --database mydb --all-tables --schema-only --output schema.db

# Remote MongoDB
mongo-to-sqlite --mongodb-uri "mongodb://host:27017" --database mydb --all-tables

# Turso cloud
export TURSO_DATABASE_URL="..." TURSO_AUTH_TOKEN="..."
mongo-to-sqlite --database mydb --all-tables

# Debug mode
RUST_LOG=debug mongo-to-sqlite --database mydb --table test --output test.db
```

## Getting Help

- **Documentation**: [Full Documentation](index.md)
- **Examples**: [Real-world Examples](examples.md)
- **Issues**: [GitHub Issues](https://github.com/yourusername/mongo-to-sqlite/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourusername/mongo-to-sqlite/discussions)

---

**Ready for more?** Check out the [Usage Guide](usage.md) for comprehensive documentation.

