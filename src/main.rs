mod cli;
mod converter;
mod error;
mod libsql_client;
mod migration;
mod mongodb_client;
mod schema;

use anyhow::Result;
use cli::Args;
use clap::Parser;
use colored::Colorize;
use tracing_subscriber::{fmt, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file if it exists (ignore errors if not found)
    let _ = dotenvy::dotenv();

    // Initialize logging
    init_logging();

    // Parse command-line arguments
    let args = Args::parse();

    // Validate arguments
    args.validate()?;

    // Print banner
    print_banner();

    // Run migration
    match run_migration(args).await {
        Ok(stats) => {
            println!("\n{}", "✅ Migration completed successfully!".green().bold());
            println!("   Total documents migrated: {}", stats.total_documents.to_string().cyan());
            println!("   Tables migrated: {}", stats.tables_migrated.to_string().cyan());
            println!("   Time elapsed: {:.2}s", stats.elapsed_seconds.to_string().cyan());
            if let Some(output) = stats.output_path {
                println!("   Output: {}", output.cyan());
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("\n{}", "❌ Migration failed!".red().bold());
            eprintln!("   Error: {}", e.to_string().red());
            std::process::exit(1);
        }
    }
}

/// Initialize logging based on RUST_LOG environment variable
fn init_logging() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .init();
}

/// Print application banner
fn print_banner() {
    println!("{}", "╔════════════════════════════════════════════════╗".cyan());
    println!("{}", "║     MongoDB to SQLite Migration Tool          ║".cyan().bold());
    println!("{}", "║     Powered by LibSQL & Turso                  ║".cyan());
    println!("{}", "╚════════════════════════════════════════════════╝".cyan());
    println!();
}

/// Run the migration process
async fn run_migration(args: Args) -> Result<MigrationStats> {
    use std::time::Instant;
    let start = Instant::now();

    // Connect to MongoDB
    println!("{}", "🔍 Connecting to MongoDB...".yellow());
    let mongo_client = mongodb_client::MongoClient::new(&args.mongodb_uri).await?;
    println!("{}", "   ✓ Connected to MongoDB".green());

    // Get list of collections to migrate
    let collections = if args.all_tables {
        mongo_client.list_collections(&args.database).await?
    } else if let Some(ref table) = args.table {
        vec![table.clone()]
    } else {
        anyhow::bail!("Either --all-tables or --table must be specified");
    };

    if collections.is_empty() {
        anyhow::bail!("No collections found in database '{}'", args.database);
    }

    let collections_count = collections.len();
    let collections_display = collections.join(", ");
    
    println!("\n{} Found {} collection(s): {}", 
        "📊".yellow(), 
        collections_count.to_string().cyan().bold(),
        collections_display.cyan()
    );

    // Connect to LibSQL (local or remote)
    println!("\n{}", "🔗 Connecting to SQLite/LibSQL...".yellow());
    let libsql_client = libsql_client::LibSqlClient::new(args.output.as_deref()).await?;
    println!("{}", "   ✓ Connected to SQLite/LibSQL".green());

    // Run migration
    let migrator = migration::Migrator::new(
        mongo_client,
        libsql_client,
        args.database.clone(),
        args.batch_size,
        args.sample_size,
    );

    let mode = migration::MigrationMode::from_args(args.schema_only, args.data_only);
    let total_documents = migrator.migrate(collections, mode, args.truncate, args.drop_tables).await?;

    let elapsed = start.elapsed();
    
    Ok(MigrationStats {
        total_documents,
        tables_migrated: collections_count,
        elapsed_seconds: elapsed.as_secs_f64(),
        output_path: args.output,
    })
}

/// Statistics about the migration
struct MigrationStats {
    total_documents: usize,
    tables_migrated: usize,
    elapsed_seconds: f64,
    output_path: Option<String>,
}
