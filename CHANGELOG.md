# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1]

### Added
- GitHub Actions workflow for automated releases
- Support for Windows, Linux, and macOS (Intel + Apple Silicon) binaries
- Automated checksum generation for releases

## [0.1.0] - 2025-11-11

### Added
- Initial release of MongoDB to SQLite migration tool
- Automatic schema inference from MongoDB collections
- Support for local SQLite files
- Support for Turso cloud databases
- CLI with comprehensive options:
  - `--schema-only` - Migrate schema only
  - `--data-only` - Migrate data only
  - `--all-tables` - Migrate all collections
  - `--table` - Migrate specific collection
  - `--drop-tables` - Drop existing tables before migration
  - `--truncate` - Truncate tables before data migration
  - `--batch-size` - Configure batch insert size
  - `--sample-size` - Configure schema inference sample size
- Automatic .env file loading
- Progress bars and beautiful CLI output
- Type mapping from MongoDB BSON to SQLite types
- JSON serialization for nested documents and arrays
- Batch processing for efficient data migration
- Transaction support for data integrity

### Documentation
- Comprehensive README
- Usage guide with examples
- Architecture documentation
- Type mappings reference
- Configuration guide
- Environment setup guide
- Quick start guide
- Build guide
- Deployment guide
- Contributing guidelines

### Infrastructure
- CI/CD pipeline for testing and building
- GitHub Actions for documentation deployment
- Support for GitHub Pages
- Multi-platform binary builds

[Unreleased]: https://github.com/Ppang0405/mongo-to-sqlite/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/Ppang0405/mongo-to-sqlite/releases/tag/v0.1.0

