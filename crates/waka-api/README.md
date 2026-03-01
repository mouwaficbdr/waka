# waka-api

[![Crates.io Version](https://img.shields.io/crates/v/waka-api.svg)](https://crates.io/crates/waka-api)
[![docs.rs](https://docs.rs/waka-api/badge.svg)](https://docs.rs/waka-api)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

WakaTime HTTP client library for Rust.

Provides typed request/response structs and an async HTTP client for the
[WakaTime API v1](https://wakatime.com/developers). Used internally by the
[waka CLI](https://github.com/mouwaficbdr/waka).

---

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
waka-api = "1"
tokio = { version = "1", features = ["full"] }
```

### Example

```rust
use waka_api::WakaClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = WakaClient::new("your-wakatime-api-key")?;

    let user = client.current_user().await?;
    println!("Logged in as: {}", user.data.display_name);

    let stats = client.stats("last_7_days", None).await?;
    println!("Total last 7 days: {}", stats.data.human_readable_total);

    Ok(())
}
```

---

## Documentation

Full API reference: [docs.rs/waka-api](https://docs.rs/waka-api)

---

## Part of the waka project

This crate is the HTTP layer for the
[waka CLI](https://github.com/mouwaficbdr/waka) — a fast, beautiful
WakaTime command-line client for your terminal.

---

## License

MIT — see [LICENSE](https://github.com/mouwaficbdr/waka/blob/main/LICENSE).
