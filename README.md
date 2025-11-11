# MongoDB to SQLite Migration Tool

A robust command-line tool for migrating MongoDB databases to SQLite/LibSQL with automatic schema inference and type mapping.

## Features

✨ **Automatic Schema Inference** - Analyzes MongoDB collections and generates appropriate SQLite schemas

🔄 **Flexible Migration Modes** - Migrate schema only, data only, or both

🎯 **Selective Migration** - Migrate specific tables or all tables at once

☁️ **Turso Cloud Support** - Write directly to Turso cloud databases or local SQLite files

🚀 **Efficient Batch Processing** - Handles large datasets with configurable batch sizes

📊 **Progress Tracking** - Real-time progress updates with beautiful CLI output

🛡️ **Type Safety** - Intelligent type mapping from MongoDB BSON to SQLite types

## Quick Start

### Prerequisites

- Rust 1.70+ (for building from source)
- MongoDB instance (local or remote)
- Optional: Turso account for cloud deployments

### Installation

```bash
cargo install mongo-to-sqlite
```

Or build from source:

```bash
git clone https://github.com/yourusername/mongo-to-sqlite
cd mongo-to-sqlite
cargo build --release
```

### Basic Usage

```bash
# Migrate all collections to local SQLite file
mongo-to-sqlite --database mydb --all-tables --output mydb.db

# Migrate specific collection
mongo-to-sqlite --database mydb --table users --output users.db

# Schema only (preview)
mongo-to-sqlite --database mydb --all-tables --schema-only --output schema.db

# Migrate to Turso cloud
export TURSO_DATABASE_URL="libsql://your-database.turso.io"
export TURSO_AUTH_TOKEN="your-auth-token"
mongo-to-sqlite --database mydb --all-tables
```

## Command-Line Options

```
Usage: mongo-to-sqlite [OPTIONS] --database <DATABASE>

Options:
  -d, --database <DATABASE>          MongoDB database name
      --mongodb-uri <URI>            MongoDB connection string [default: mongodb://localhost:27017]
  -t, --table <TABLE>                Migrate specific table/collection
      --all-tables                   Migrate all tables/collections
      --schema-only                  Migrate schema only (no data)
      --data-only                    Migrate data only (assumes schema exists)
  -o, --output <OUTPUT>              Output SQLite file path [default: output.db]
      --batch-size <SIZE>            Batch size for inserts [default: 1000]
      --sample-size <SIZE>           Number of documents to sample for schema [default: 100]
  -h, --help                         Print help
  -V, --version                      Print version
```

## Documentation

- [Architecture](docs/architecture.md) - System design and components
- [Type Mappings](docs/type_mappings.md) - How MongoDB types map to SQLite
- [Usage Guide](docs/usage.md) - Detailed usage examples and workflows
- [Environment Setup](docs/environment_setup.md) - Configure with .env files

## Examples

### Example 1: Local Development

```bash
# Migrate development database to local file
mongo-to-sqlite \
  --mongodb-uri "mongodb://localhost:27017" \
  --database myapp_dev \
  --all-tables \
  --output ./dev.db
```

### Example 2: Production Migration

```bash
# First preview the schema
mongo-to-sqlite \
  --mongodb-uri "$PROD_MONGODB_URI" \
  --database production \
  --all-tables \
  --schema-only \
  --output schema-preview.db

# Review the schema
sqlite3 schema-preview.db ".schema"

# Then perform full migration to Turso
export TURSO_DATABASE_URL="libsql://prod-db.turso.io"
export TURSO_AUTH_TOKEN="$PROD_TURSO_TOKEN"
mongo-to-sqlite \
  --mongodb-uri "$PROD_MONGODB_URI" \
  --database production \
  --all-tables
```

### Example 3: Incremental Migration

```bash
# Migrate tables one at a time for large databases
mongo-to-sqlite --database mydb --table users --output app.db
mongo-to-sqlite --database mydb --table posts --data-only --output app.db
mongo-to-sqlite --database mydb --table comments --data-only --output app.db
```

## Type Mapping

| MongoDB Type | SQLite Type | Notes |
|--------------|-------------|-------|
| String       | TEXT        | Direct mapping |
| Int32/Int64  | INTEGER     | Direct mapping |
| Double       | REAL        | Direct mapping |
| Boolean      | INTEGER     | 0 = false, 1 = true |
| Date         | TEXT        | ISO 8601 format |
| ObjectId     | TEXT        | Hex string |
| Array        | TEXT        | JSON serialized |
| Object       | TEXT        | JSON serialized |
| Binary       | BLOB        | Direct mapping |

See [Type Mappings](docs/typeMappings.md) for complete details.

## Environment Variables

### Using .env File (Recommended)

The tool automatically loads variables from a `.env` file:

```bash
# 1. Copy the example
cp env.example .env

# 2. Edit with your values
nano .env

# 3. Run - variables are loaded automatically
mongo-to-sqlite --database mydb --all-tables
```

### Or Export Manually

```bash
# MongoDB connection (optional, can use --mongodb-uri instead)
export MONGODB_URI="mongodb://localhost:27017"

# Turso cloud connection (optional, uses local file if not set)
export TURSO_DATABASE_URL="libsql://your-database.turso.io"
export TURSO_AUTH_TOKEN="your-auth-token"

# Logging level
export RUST_LOG=info  # Options: trace, debug, info, warn, error
```

## Performance Tips

1. **Batch Size**: Increase `--batch-size` for faster migrations (uses more memory)
2. **Network Proximity**: Run the tool close to your MongoDB server
3. **Parallel Processing**: Migrate multiple tables in parallel using separate processes
4. **Indexes**: Add SQLite indexes after migration for better query performance

## Limitations

- **Nested Data**: Arrays and nested objects are stored as JSON TEXT
- **Indexes**: MongoDB indexes are not automatically migrated
- **GridFS**: GridFS files are not supported
- **Geospatial**: Geographic queries require SQLite extensions
- **Schema Validation**: MongoDB validators are not enforced in SQLite

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [libSQL](https://github.com/tursodatabase/libsql) - SQLite fork with enhanced capabilities
- [MongoDB Rust Driver](https://github.com/mongodb/mongo-rust-driver)
- [clap](https://github.com/clap-rs/clap) - Command-line argument parsing

## Support

- 📖 [Documentation](docs/)
- 🐛 [Issue Tracker](https://github.com/yourusername/mongo-to-sqlite/issues)
- 💬 [Discussions](https://github.com/yourusername/mongo-to-sqlite/discussions)
