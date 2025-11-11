# MongoDB to SQLite Migration Tool

Welcome to the documentation for the MongoDB to SQLite migration tool.

## Overview

This tool provides a robust, efficient way to migrate MongoDB databases to SQLite/LibSQL with automatic schema inference and intelligent type mapping. It supports both local SQLite files and Turso cloud databases.

## Quick Links

- [Quick Start](quickstart.md) - Get started in 5 minutes
- [Type Mappings](type_mappings.md) - How MongoDB types map to SQLite

## Key Features

- Automatic schema inference from MongoDB collections
- Flexible migration modes (schema only, data only, or both)
- Selective migration (specific tables or all tables)
- Support for Turso cloud databases and local SQLite files
- Efficient batch processing for large datasets
- Real-time progress tracking
- Intelligent type mapping from MongoDB BSON to SQLite

## Installation

Download the latest release for your platform from the [Releases page](https://github.com/Ppang0405/mongo-to-sqlite/releases).

Extract and install:

```bash
tar -xzf mongo-to-sqlite-*.tar.gz
sudo mv mongo-to-sqlite /usr/local/bin/
```

## Quick Start

Migrate all collections to a local SQLite file:

```bash
mongo-to-sqlite --database mydb --all-tables --output mydb.db
```

Migrate to Turso cloud:

```bash
export TURSO_DATABASE_URL="libsql://your-database.turso.io"
export TURSO_AUTH_TOKEN="your-auth-token"
mongo-to-sqlite --database mydb --all-tables
```

## Documentation Sections

### [Quick Start](quickstart.md)
Get up and running in minutes with step-by-step instructions and common usage examples.

### [Type Mappings](type_mappings.md)
Understand how MongoDB BSON types are converted to SQLite types, including handling of complex nested data.

## Common Use Cases

### Local Development
```bash
mongo-to-sqlite --database dev_app --all-tables --output dev.db
```

### Production Backup
```bash
mongo-to-sqlite \
  --mongodb-uri "mongodb+srv://prod-cluster.mongodb.net" \
  --database production \
  --all-tables \
  --output backup-$(date +%Y%m%d).db
```

### Edge Deployment with Turso
```bash
export TURSO_DATABASE_URL="libsql://edge-db.turso.io"
export TURSO_AUTH_TOKEN="$TURSO_TOKEN"
mongo-to-sqlite --database app --all-tables
```

## License

This project is licensed under the MIT License.
