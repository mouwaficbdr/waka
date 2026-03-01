# waka-api Crate

`waka-api` is the standalone Rust HTTP client library powering `waka`.
It is published independently on [crates.io](https://crates.io/crates/waka-api)
and can be used in your own Rust projects.

## Add to your project

```toml
[dependencies]
waka-api = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick start

```rust,no_run
use waka_api::{WakaClient, StatsRange, SummaryParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = WakaClient::new("waka_xxxx");

    // Fetch the authenticated user
    let me = client.me().await?;
    println!("Hello {}", me.display_name);

    // Today's summary
    let summary = client.summaries(SummaryParams::today()).await?;
    println!("Total today: {}", summary.data[0].grand_total.text);

    // 7-day stats
    let stats = client.stats(StatsRange::Last7Days).await?;
    println!("Last 7 days: {}", stats.data.human_readable_total);

    Ok(())
}
```

## API reference

Full API documentation is on [docs.rs/waka-api](https://docs.rs/waka-api).

### `WakaClient`

| Method                     | Description                      |
| -------------------------- | -------------------------------- |
| `WakaClient::new(api_key)` | Create a client (production API) |
| `client.me()`              | Fetch the current user profile   |
| `client.summaries(params)` | Fetch summaries for a date range |
| `client.stats(range)`      | Fetch aggregated stats           |
| `client.projects()`        | List projects                    |
| `client.goals()`           | List goals                       |
| `client.leaderboard(page)` | Browse the leaderboard           |

### `SummaryParams`

```rust,no_run
use waka_api::SummaryParams;
use chrono::NaiveDate;

// Today
let p = SummaryParams::today();

// This week (Mon → today)
let p = SummaryParams::this_week();

// Custom range
let p = SummaryParams::for_range(
    NaiveDate::from_ymd_opt(2025, 2, 1).unwrap(),
    NaiveDate::from_ymd_opt(2025, 2, 28).unwrap(),
).project("my-saas");
```

### `StatsRange`

```rust
use waka_api::StatsRange;

let ranges = [
    StatsRange::Last7Days,
    StatsRange::Last30Days,
    StatsRange::Last6Months,
    StatsRange::LastYear,
    StatsRange::AllTime,
];
```

## Error handling

```rust,no_run
use waka_api::{WakaClient, ApiError};

async fn fetch() {
    let client = WakaClient::new("waka_xxxx");
    match client.me().await {
        Ok(user) => println!("Hello {}", user.display_name),
        Err(ApiError::Unauthorized) => eprintln!("Invalid API key"),
        Err(ApiError::RateLimit { retry_after }) => {
            eprintln!("Rate limited. Retry after {:?}s", retry_after);
        }
        Err(ApiError::NetworkError(e)) => eprintln!("Network error: {e}"),
        Err(e) => eprintln!("Error: {e}"),
    }
}
```

## License

MIT — same as the `waka` CLI.
