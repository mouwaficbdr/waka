//! Interactive TUI dashboard for `waka`.
//!
//! Built on [`ratatui`] and [`crossterm`]. Implements its own rendering
//! pipeline — it does **not** depend on `waka-render`.

mod app;
mod event;
mod ui;
mod widgets;

use std::io;
use std::time::Duration;

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use tokio::sync::mpsc;
use waka_api::WakaClient;

pub use app::App;
pub use event::Event;

/// Runs the TUI dashboard.
///
/// This function initializes the terminal, spawns the event loop tasks
/// (input, ticker, data fetcher), and runs the main rendering loop until
/// the user quits.
///
/// # Errors
/// Returns an error if the terminal cannot be initialized or if rendering fails.
pub async fn run(client: WakaClient, refresh_interval: Duration) -> Result<(), io::Error> {
    const EVENT_CHANNEL_CAPACITY: usize = 100;

    // Set up terminal.
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state.
    let mut app = App::new(client.clone(), refresh_interval);

    // Create event channel.
    let (tx, mut rx) = mpsc::channel(EVENT_CHANNEL_CAPACITY);

    // Spawn background tasks.
    event::spawn_input_handler(tx.clone());
    event::spawn_ticker(tx.clone(), Duration::from_millis(250));
    event::spawn_data_fetcher(tx, client, refresh_interval);

    // Main event loop.
    while app.running {
        terminal.draw(|f| ui::render(f, &app))?;

        if let Some(ev) = rx.recv().await {
            match ev {
                Event::Tick => { /* no-op for now */ }
                Event::Key(key) => event::handle_key_event(&mut app, key),
                Event::SummaryUpdate(summary) => {
                    app.summary_today = Some(*summary);
                    app.last_update = Some(std::time::Instant::now());
                    app.loading = false;
                }
                Event::WeeklyUpdate(summary) => {
                    app.summary_week = Some(*summary);
                }
                Event::ActivityUpdate(summary) => {
                    app.activity_30d = Some(*summary);
                }
                Event::GoalsUpdate(goals) => {
                    app.goals = Some(*goals);
                }
                Event::Error(msg) => {
                    app.error = Some(msg);
                    app.loading = false;
                }
            }
        }
    }

    // Restore terminal.
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
