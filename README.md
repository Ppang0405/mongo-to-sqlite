# MongoDB to SQLite Migration Tool

A command-line tool for migrating MongoDB databases to SQLite/LibSQL with automatic schema inference and type mapping.

## Installation

Download the latest release for your platform from the [Releases page](https://github.com/Ppang0405/mongo-to-sqlite/releases).

Extract the binary and optionally add it to your PATH:

```bash
# macOS/Linux
tar -xzf mongo-to-sqlite-*.tar.gz
sudo mv mongo-to-sqlite /usr/local/bin/

# Or place it in your preferred location
mv mongo-to-sqlite ~/.local/bin/
```

## Environment Setup

The tool uses environment variables for configuration. Create a `.env` file in your project directory:

```bash
cp env.example .env
```

Edit the `.env` file with your configuration:

```bash
# MongoDB Connection
MONGODB_URI=mongodb://localhost:27017

# Turso Cloud (optional - omit to use local SQLite file)
TURSO_DATABASE_URL=libsql://your-database.turso.io
TURSO_AUTH_TOKEN=your-auth-token

# Logging Level (optional)
RUST_LOG=info
```

The tool automatically loads these variables at runtime.

## Usage

### Basic Commands

Migrate all collections to a local SQLite file:

```bash
mongo-to-sqlite --database mydb --all-tables --output mydb.db
```

Migrate a specific collection:

```bash
mongo-to-sqlite --database mydb --table users --output users.db
```

Preview schema without migrating data:

```bash
mongo-to-sqlite --database mydb --all-tables --schema-only --output schema.db
```

Migrate to Turso cloud (requires `TURSO_DATABASE_URL` and `TURSO_AUTH_TOKEN` in `.env`):

```bash
mongo-to-sqlite --database mydb --all-tables
```

### Command-Line Options

```
Options:
  -d, --database <DATABASE>          MongoDB database name (required)
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

## Documentation

- [Quick Start](docs/quickstart.md) - Get started in 5 minutes
- [Type Mappings](docs/type_mappings.md) - Complete type mapping details

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
