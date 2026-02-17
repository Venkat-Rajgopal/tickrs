use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(feature = "cli")]
    {
        // Run CLI when feature is enabled
        tickrs::cli::run_cli().await?;
    }

    #[cfg(not(feature = "cli"))]
    {
        // This will be replaced with TUI later
        println!("tickrs 🦀");
        println!("TUI mode coming soon...");
        println!("\nTo use CLI mode, run with --features cli or:");
        println!("  cargo run -- time-series AAPL --start 2025-12-20 --end 2025-12-30");
    }

    Ok(())
}
