# MongoDB to SQLite Migration Tool

Welcome to the documentation for the MongoDB to SQLite migration tool.

## Overview

This tool provides a robust, efficient way to migrate MongoDB databases to SQLite/LibSQL with automatic schema inference and intelligent type mapping. It supports both local SQLite files and Turso cloud databases.

## Quick Links

- [Architecture](architecture.md) - System design and components
- [Type Mappings](type_mappings.md) - How MongoDB types map to SQLite
- [Usage Guide](usage.md) - Detailed usage examples and workflows
- [Configuration](configuration.md) - Environment variables and CLI options
- [Examples](examples.md) - Real-world usage examples

## Features

✨ **Automatic Schema Inference** - Analyzes MongoDB collections and generates appropriate SQLite schemas

🔄 **Flexible Migration Modes** - Migrate schema only, data only, or both

🎯 **Selective Migration** - Migrate specific tables or all tables at once

☁️ **Turso Cloud Support** - Write directly to Turso cloud databases or local SQLite files

🚀 **Efficient Batch Processing** - Handles large datasets with configurable batch sizes

📊 **Progress Tracking** - Real-time progress updates with beautiful CLI output

🛡️ **Type Safety** - Intelligent type mapping from MongoDB BSON to SQLite types

## Getting Started

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

### Quick Start

```bash
# Migrate all collections to local SQLite file
mongo-to-sqlite --database mydb --all-tables --output mydb.db

# Migrate to Turso cloud
export TURSO_DATABASE_URL="libsql://your-database.turso.io"
export TURSO_AUTH_TOKEN="your-auth-token"
mongo-to-sqlite --database mydb --all-tables
```

## Documentation Sections

### [Architecture](architecture.md)
Learn about the internal architecture, components, and data flow of the migration tool.

### [Type Mappings](type_mappings.md)
Understand how MongoDB BSON types are converted to SQLite types, including handling of complex nested data.

### [Usage Guide](usage.md)
Comprehensive guide covering all command-line options, common workflows, and best practices.

### [Configuration](configuration.md)
Complete reference for environment variables, command-line arguments, and performance tuning.

### [Examples](examples.md)
Real-world examples and use cases, from simple migrations to complex production deployments.

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

## Support

- **Issues**: [GitHub Issues](https://github.com/yourusername/mongo-to-sqlite/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourusername/mongo-to-sqlite/discussions)
- **Documentation**: [Full Documentation](https://yourusername.github.io/mongo-to-sqlite/)

## Contributing

Contributions are welcome! Please see our [Contributing Guide](../README.md#contributing) for details.

## License

This project is licensed under the MIT License.

---

**Next Steps**: Check out the [Usage Guide](usage.md) for detailed examples, or jump straight to [Examples](examples.md) for practical use cases.

