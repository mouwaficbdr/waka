//! Basic usage of `waka-api`.
//!
//! This example demonstrates authenticating with a `WakaTime` API key and
//! fetching the current user profile, today's coding summary, and 7-day stats.
//!
//! Run with:
//! ```sh
//! WAKA_API_KEY=waka_xxxx cargo run --example basic_usage -p waka-api
//! ```

use waka_api::{StatsRange, SummaryParams, WakaClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key =
        std::env::var("WAKA_API_KEY").expect("WAKA_API_KEY environment variable must be set");

    let client = WakaClient::new(&api_key);

    // ── Fetch the authenticated user profile ────────────────────────────────
    let me = client.me().await?;
    println!("Logged in as: {} ({})", me.display_name, me.username);
    println!("Timezone: {}", me.timezone);

    // ── Fetch today's coding summary ────────────────────────────────────────
    let today = SummaryParams::today();
    let summary = client.summaries(today).await?;
    println!("\nToday's coding summary:");
    for entry in &summary.data {
        let total = &entry.grand_total.text;
        let date = entry.range.date.as_deref().unwrap_or("?");
        println!("  {date}: {total}");
    }

    // ── Fetch 7-day stats ───────────────────────────────────────────────────
    let stats = client.stats(StatsRange::Last7Days).await?;
    let s = stats.data;
    println!("\nLast 7 days: {} total", s.human_readable_total);
    println!("Top languages:");
    for lang in s.languages.iter().take(5) {
        println!("  {} — {}", lang.name, lang.text);
    }

    // ── List projects ───────────────────────────────────────────────────────
    let projects = client.projects().await?;
    println!("\nProjects ({} total):", projects.data.len());
    for p in projects.data.iter().take(5) {
        println!("  {}", p.name);
    }

    Ok(())
}
