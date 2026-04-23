use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use console::Style;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::Write;
use std::process::Command;
use tokio::process::Command as AsyncCommand;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use tokio::time::{sleep, Duration};

/// tri-tunnel — Tailscale Funnel for trios-server
#[derive(Parser, Debug)]
#[command(name = "tri-tunnel")]
#[command(about = "Tailscale Funnel management for trios-server", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Start the funnel
    Start {
        #[arg(short, long, default_value = "9005")]
        port: u16,
    },
    /// Stop the funnel
    Stop,
    /// Show status
    Status,
    /// Open dashboard in browser
    Open,
}

const TAILSCALE_CLI: &str = "/Applications/Tailscale.app/Contents/MacOS/Tailscale";

fn print_header() {
    let header = Style::new().bold().cyan();
    println!("\n{}", header.apply_to("╔═══════════════════════════════════════════════════════════════╗"));
    println!("{}     {}              ",
        Style::new().bold().cyan().apply_to("║"),
        Style::new().bold().white().apply_to("tri-tunnel — Tailscale Funnel")
    );
    println!("{}                 for trios-server {}",
        Style::new().bold().cyan().apply_to("║"),
        Style::new().bold().cyan().apply_to("║")
    );
    println!("{}\n", Style::new().bold().cyan().apply_to("╚═══════════════════════════════════════════════════════════════╝"));
}

fn print_success(msg: &str) {
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).ok();
    writeln!(stdout, "✅ {}", msg).ok();
    stdout.reset().ok();
}

fn print_error(msg: &str) {
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).ok();
    writeln!(stdout, "❌ {}", msg).ok();
    stdout.reset().ok();
}

fn print_info(msg: &str) {
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue))).ok();
    writeln!(stdout, "ℹ️  {}", msg).ok();
    stdout.reset().ok();
}

fn print_url(label: &str, url: &str) {
    println!("\n{} {}: ", Style::new().bold().yellow().apply_to("🌐"), label);
    println!("   {}", Style::new().underlined().cyan().apply_to(url));
}

fn check_tailscale() -> Result<()> {
    if !std::path::Path::new(TAILSCALE_CLI).exists() {
        bail!(
            "Tailscale CLI not found at {}\nInstall from App Store: https://apps.apple.com/app/tailscale/id1475387142",
            TAILSCALE_CLI
        );
    }
    Ok(())
}

async fn get_status_json() -> Result<serde_json::Value> {
    let output = AsyncCommand::new(TAILSCALE_CLI)
        .args(["status", "--json"])
        .output()
        .await
        .context("Failed to get Tailscale status")?;

    if !output.status.success() {
        bail!("Tailscale status command failed");
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(serde_json::from_str(&stdout)?)
}

async fn get_funnel_status() -> Result<(bool, Option<String>)> {
    let output = Command::new(TAILSCALE_CLI)
        .args(["funnel", "status", "--json"])
        .output();

    match output {
        Ok(result) if result.status.success() => {
            let stdout = String::from_utf8_lossy(&result.stdout);
            let json: serde_json::Value = serde_json::from_str(&stdout).unwrap_or(serde_json::json!({}));

            // Check for Web handlers
            if let Some(web) = json.get("Web").and_then(|v| v.as_object()) {
                for (domain, config) in web {
                    if let Some(handlers) = config.get("Handlers").and_then(|v| v.as_object()) {
                        if handlers.get("/").is_some() {
                            return Ok((true, Some(domain.clone())));
                        }
                    }
                }
            }
            Ok((false, None))
        }
        _ => Ok((false, None)),
    }
}

async fn start(port: u16) -> Result<()> {
    print_header();

    // Check if already running
    let (is_running, _) = get_funnel_status().await?;
    if is_running {
        print_info("Funnel is already running!");
        let _ = show_status_box().await;
        return Ok(());
    }

    // Check Tailscale
    check_tailscale()?;

    // Start funnel
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner());
    pb.set_message("Starting Tailscale Funnel...");
    pb.enable_steady_tick(Duration::from_millis(100));

    let output = AsyncCommand::new(TAILSCALE_CLI)
        .args(["funnel", "--bg", &port.to_string()])
        .output()
        .await
        .context("Failed to start funnel")?;

    pb.finish();

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        print_error(&format!("Failed to start funnel: {}", stderr));
        return Ok(());
    }

    // Wait and verify
    sleep(Duration::from_secs(3)).await;
    let (is_running, domain) = get_funnel_status().await?;

    if is_running {
        print_success("Funnel started successfully!");

        // Show status
        let _ = show_status_box().await;

        // Show URLs
        if let Some(d) = domain {
            print_url("Your trios-server is accessible at", &format!("https://{}/", d));
            print_url("Health check", &format!("https://{}/health", d));
            print_url("API status", &format!("https://{}/api/status", d));
        }
    } else {
        print_error("Funnel started but verification failed");
    }

    Ok(())
}

async fn stop() -> Result<()> {
    print_header();
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner());
    pb.set_message("Stopping Tailscale Funnel...");
    pb.enable_steady_tick(Duration::from_millis(100));

    let output = AsyncCommand::new(TAILSCALE_CLI)
        .args(["funnel", "--https=443", "off"])
        .output()
        .await
        .context("Failed to stop funnel")?;

    pb.finish();

    if output.status.success() {
        print_success("Funnel stopped");
    } else {
        print_error("Failed to stop funnel");
    }

    Ok(())
}

async fn status() -> Result<()> {
    print_header();
    let _ = show_status_box().await;
    Ok(())
}

async fn show_status_box() -> Result<()> {
    let status = get_status_json().await?;
    let (funnel_active, domain) = get_funnel_status().await?;

    let device_name = status
        .get("Self")
        .and_then(|v| v.get("HostName"))
        .and_then(|v| v.as_str())
        .unwrap_or("Unknown");

    println!("\n┌─────────────────────────────────────────────────────────────┐");
    println!("│  {}                    │", Style::new().bold().white().apply_to("STATUS"));
    println!("├─────────────────────────────────────────────────────────────┤");
    println!("│  Device:  {:<48} │", device_name);

    if funnel_active {
        println!("│  Funnel:  {:<48} │",
            Style::new().green().apply_to("ACTIVE ✅")
        );
        if let Some(d) = domain {
            println!("│  URL:     {:<48} │", d.trim_end_matches('.'));
        }
    } else {
        println!("│  Funnel:  {:<48} │",
            Style::new().red().apply_to("INACTIVE ❌")
        );
    }
    println!("└─────────────────────────────────────────────────────────────┘");

    Ok(())
}

async fn open_dashboard() -> Result<()> {
    let (_, domain) = get_funnel_status().await?;

    if let Some(d) = domain {
        let url = format!("https://{}/", d);
        print_info(&format!("Opening: {}", url));
        open::that(url)?;
        print_success("Dashboard opened in browser");
    } else {
        print_error("Funnel is not running. Start it first with: tri-tunnel start");
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Start { port } => start(port).await,
        Commands::Stop => stop().await,
        Commands::Status => status().await,
        Commands::Open => open_dashboard().await,
    };

    if let Err(e) = result {
        print_error(&e.to_string());
        std::process::exit(1);
    }

    Ok(())
}
