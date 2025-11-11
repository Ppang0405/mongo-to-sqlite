# Environment Setup

## Using .env Files

The tool automatically loads environment variables from a `.env` file in the project root (if it exists).

### Quick Setup

1. **Copy the example file:**
   ```bash
   cp env.example .env
   ```

2. **Edit with your values:**
   ```bash
   # Open in your editor
   nano .env
   # or
   vim .env
   # or
   code .env
   ```

3. **Run the tool** - it will automatically load the `.env` file:
   ```bash
   ./target/release/mongo-to-sqlite --database mydb --all-tables
   ```

## Environment Variables

### MongoDB Configuration

#### `MONGODB_URI`
MongoDB connection string. The tool reads this automatically from `.env` file.

**Format:**
```
mongodb://[username:password@]host[:port][/database][?options]
```

**Examples:**

```bash
# Local MongoDB
MONGODB_URI=mongodb://localhost:27017

# Remote with authentication
MONGODB_URI=mongodb://user:password@mongo.example.com:27017

# MongoDB Atlas
MONGODB_URI=mongodb+srv://user:pass@cluster.mongodb.net

# Replica Set
MONGODB_URI=mongodb://host1:27017,host2:27017/?replicaSet=myReplSet
```

### Turso Configuration

#### `TURSO_DATABASE_URL`
Your Turso cloud database URL.

```bash
TURSO_DATABASE_URL=libsql://your-database.turso.io
```

#### `TURSO_AUTH_TOKEN`
Your Turso authentication token.

```bash
# Get token with Turso CLI
turso db tokens create <database-name>

# Then set in .env
TURSO_AUTH_TOKEN=eyJhbGciOiJFZERTQSIsInR5cCI6IkpXVCJ9...
```

### Logging Configuration

#### `RUST_LOG`
Controls logging verbosity.

**Options:** `error`, `warn`, `info`, `debug`, `trace`

```bash
# Production - minimal logging
RUST_LOG=warn

# Development - detailed logs
RUST_LOG=debug

# Specific module debugging
RUST_LOG=mongo_to_sqlite::migration=debug,mongodb=info

# Very verbose
RUST_LOG=trace
```

## Multiple Environments

### Option 1: Multiple .env Files

Create environment-specific files:

```bash
# Create environment files
touch .env.development
touch .env.staging
touch .env.production
```

**.env.development:**
```bash
MONGODB_URI=mongodb://localhost:27017
RUST_LOG=debug
```

**.env.staging:**
```bash
MONGODB_URI=mongodb://staging-mongo.example.com:27017
TURSO_DATABASE_URL=libsql://staging-db.turso.io
TURSO_AUTH_TOKEN=staging-token
RUST_LOG=info
```

**.env.production:**
```bash
MONGODB_URI=mongodb://prod-mongo.example.com:27017
TURSO_DATABASE_URL=libsql://prod-db.turso.io
TURSO_AUTH_TOKEN=prod-token
RUST_LOG=warn
```

**Usage:**
```bash
# Development
cp .env.development .env
./target/release/mongo-to-sqlite ...

# Production
cp .env.production .env
./target/release/mongo-to-sqlite ...
```

### Option 2: Source Different Files

```bash
# Load development config
set -a && source .env.development && set +a
./target/release/mongo-to-sqlite ...

# Load production config
set -a && source .env.production && set +a
./target/release/mongo-to-sqlite ...
```

### Option 3: Command-Line Override

Environment variables can be overridden by command-line arguments:

```bash
# .env has MONGODB_URI=mongodb://localhost:27017
# But you can override it:
./target/release/mongo-to-sqlite \
  --mongodb-uri "mongodb://other-host:27017" \
  --database mydb \
  --all-tables
```

## Priority Order

Settings are applied in this order (later overrides earlier):

1. **Default values** (hardcoded)
2. **.env file** (loaded automatically)
3. **Environment variables** (from shell)
4. **Command-line arguments** (highest priority)

### Example

```bash
# In .env file
MONGODB_URI=mongodb://localhost:27017

# Override with environment variable
export MONGODB_URI=mongodb://staging:27017

# Override again with CLI argument
./target/release/mongo-to-sqlite \
  --mongodb-uri "mongodb://production:27017" \
  ...

# Uses: mongodb://production:27017 (CLI wins)
```

## Security Best Practices

### 1. Never Commit .env Files

Already configured in `.gitignore`:
```gitignore
.env
.env.local
```

### 2. Use Strong Credentials

```bash
# Good
MONGODB_URI=mongodb://admin:s3cur3P@ssw0rd!@prod:27017

# Bad
MONGODB_URI=mongodb://admin:admin@prod:27017
```

### 3. Restrict File Permissions

```bash
# Make .env readable only by you
chmod 600 .env

# Verify
ls -la .env
# Should show: -rw-------
```

### 4. Use Secret Management in Production

For production, consider:
- AWS Secrets Manager
- HashiCorp Vault
- Azure Key Vault
- Google Secret Manager

**Example with Vault:**
```bash
#!/bin/bash
# load-secrets.sh

export MONGODB_URI=$(vault kv get -field=uri secret/mongodb)
export TURSO_AUTH_TOKEN=$(vault kv get -field=token secret/turso)

./target/release/mongo-to-sqlite --database mydb --all-tables
```

### 5. Separate Credentials by Environment

Don't use production credentials in development:

```bash
# Development - use local/test credentials
.env.development

# Production - use secure vault or secrets manager
# Don't store in .env file
```

## Verification

### Check What's Loaded

```bash
# Show all environment variables
./target/release/mongo-to-sqlite --help

# The CLI will show default values and env var names
```

### Test .env Loading

```bash
# 1. Create test .env
cat > .env << 'EOF'
MONGODB_URI=mongodb://test:27017
RUST_LOG=debug
EOF

# 2. Run with --help (should use test MongoDB URI)
./target/release/mongo-to-sqlite \
  --database test \
  --table users \
  --output test.db

# 3. Check logs - should show connection to test:27017
```

### Debug Environment

```bash
# Enable debug logging to see what's loaded
RUST_LOG=debug ./target/release/mongo-to-sqlite --help
```

## Common Issues

### .env File Not Loading

**Problem:** Variables not being read from .env

**Solutions:**
1. Ensure .env is in the **same directory** as where you run the command
2. Check file permissions: `chmod 644 .env`
3. Verify no syntax errors in .env file
4. Make sure there are no spaces around `=`:
   ```bash
   # Wrong
   MONGODB_URI = mongodb://localhost:27017
   
   # Correct
   MONGODB_URI=mongodb://localhost:27017
   ```

### Wrong Values Being Used

**Problem:** Unexpected configuration values

**Solution:** Check priority order - CLI args override .env

```bash
# Debug what's being used
RUST_LOG=debug ./target/release/mongo-to-sqlite \
  --database test \
  --all-tables 2>&1 | grep -i "connect"
```

### Permission Denied

**Problem:** Can't read .env file

**Solution:**
```bash
# Fix permissions
chmod 644 .env

# Or if too restrictive
chmod 600 .env  # Only you can read
```

## Example Workflows

### Local Development

```bash
# 1. Create .env for local dev
cat > .env << 'EOF'
MONGODB_URI=mongodb://localhost:27017
RUST_LOG=debug
EOF

# 2. Run migrations
./target/release/mongo-to-sqlite -d mydb --all-tables
```

### CI/CD Pipeline

```bash
# Don't use .env files in CI
# Set variables in CI environment instead
export MONGODB_URI=$CI_MONGODB_URI
export TURSO_DATABASE_URL=$CI_TURSO_URL
export TURSO_AUTH_TOKEN=$CI_TURSO_TOKEN

./target/release/mongo-to-sqlite -d mydb --all-tables
```

### Docker Deployment

```dockerfile
# Dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/mongo-to-sqlite /usr/local/bin/
# Don't copy .env - use environment variables instead
CMD ["mongo-to-sqlite"]
```

```bash
# docker-compose.yml
services:
  migrator:
    image: mongo-to-sqlite
    environment:
      - MONGODB_URI=mongodb://mongo:27017
      - TURSO_DATABASE_URL=${TURSO_URL}
      - TURSO_AUTH_TOKEN=${TURSO_TOKEN}
```

## Summary

✅ Create `.env` file with your configuration
✅ Tool automatically loads it on startup
✅ CLI arguments override .env values
✅ Keep .env out of version control
✅ Use secret management for production

For more details, see:
- [Configuration Guide](configuration.md)
- [Usage Guide](usage.md)
- [Quick Start](quickstart.md)

