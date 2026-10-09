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

    // Set up terminal. The guard restores it on every exit path (normal
    // return, `?` error, or panic unwinding), and the panic hook restores it
    // before the panic message is printed so the message stays readable.
    install_panic_hook();
    let _guard = TerminalGuard::enter()?;
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    // Create app state.
    let mut app = App::new(client.clone(), refresh_interval);

    // Create event channel.
    let (tx, mut rx) = mpsc::channel(EVENT_CHANNEL_CAPACITY);

    // Spawn background tasks.
    event::spawn_input_handler(tx.clone());
    event::spawn_ticker(tx.clone(), Duration::from_millis(250));
    event::spawn_data_fetcher(tx.clone(), client.clone(), refresh_interval);

    // Main event loop.
    while app.running {
        terminal.draw(|f| ui::render(f, &app))?;

        if let Some(ev) = rx.recv().await {
            match ev {
                Event::Tick => {
                    // Advance spinner animation
                    app.spinner_state = (app.spinner_state + 1) % 10;
                }
                Event::Key(key) => event::handle_key_event(&mut app, key, &tx, &client),
                Event::SummaryUpdate(summary) => {
                    app.summary_today = Some(*summary);
                    app.last_update = Some(std::time::Instant::now());
                    app.loading = false;
                    app.offline = false;
                }
                Event::WeeklyUpdate(summary) => {
                    app.summary_week = Some(*summary);
                    app.offline = false;
                }
                Event::ActivityUpdate(summary) => {
                    app.activity_30d = Some(*summary);
                    app.offline = false;
                }
                Event::GoalsUpdate(goals) => {
                    app.goals = Some(*goals);
                    app.offline = false;
                }
                Event::Error(msg) => {
                    app.error = Some(msg);
                    app.loading = false;
                    app.offline = true;
                }
            }
        }
    }

    // The terminal is restored when `_guard` is dropped.
    Ok(())
}

/// Puts the terminal in raw mode on the alternate screen and restores it when
/// dropped.
struct TerminalGuard;

impl TerminalGuard {
    fn enter() -> io::Result<Self> {
        enable_raw_mode()?;
        if let Err(e) = execute!(io::stdout(), EnterAlternateScreen) {
            let _ = disable_raw_mode();
            return Err(e);
        }
        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        restore_terminal();
    }
}

/// Best-effort terminal restoration; safe to call more than once.
fn restore_terminal() {
    let _ = disable_raw_mode();
    let _ = execute!(io::stdout(), LeaveAlternateScreen, crossterm::cursor::Show);
}

/// Chains a panic hook that restores the terminal before the default hook
/// prints the panic message (otherwise it would be lost on the alternate
/// screen, and the shell left in raw mode).
fn install_panic_hook() {
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        restore_terminal();
        previous(info);
    }));
}
