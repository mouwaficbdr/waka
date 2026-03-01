//! Event handling for the TUI application.

use std::time::Duration;

use crossterm::event::{self as crossterm_event, Event as CrosstermEvent, KeyCode, KeyEvent};
use tokio::sync::mpsc;
use waka_api::{ApiError, GoalsResponse, SummaryResponse};

/// Events that the TUI event loop handles.
#[derive(Debug)]
pub enum Event {
    /// Periodic tick (triggers UI refresh).
    Tick,
    /// User keyboard input.
    Key(KeyEvent),
    /// Summary data (today) fetched successfully.
    SummaryUpdate(Box<SummaryResponse>),
    /// Weekly summary data fetched successfully.
    WeeklyUpdate(Box<SummaryResponse>),
    /// 30-day activity data fetched successfully.
    ActivityUpdate(Box<SummaryResponse>),
    /// Goals data fetched successfully.
    GoalsUpdate(Box<GoalsResponse>),
    /// API fetch failed.
    Error(String),
}

/// Spawns a task that reads keyboard events from crossterm and sends them
/// as `Event::Key` to the given channel.
pub fn spawn_input_handler(tx: mpsc::Sender<Event>) {
    tokio::spawn(async move {
        loop {
            if let Ok(CrosstermEvent::Key(key)) = crossterm_event::read() {
                if tx.send(Event::Key(key)).await.is_err() {
                    break;
                }
            }
        }
    });
}

/// Spawns a task that sends `Event::Tick` every `tick_rate` to the given channel.
pub fn spawn_ticker(tx: mpsc::Sender<Event>, tick_rate: Duration) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tick_rate);
        loop {
            interval.tick().await;
            if tx.send(Event::Tick).await.is_err() {
                break;
            }
        }
    });
}

/// Spawns a task that fetches summary data every `interval` and sends
/// various update events to the given channel.
pub fn spawn_data_fetcher(
    tx: mpsc::Sender<Event>,
    client: waka_api::WakaClient,
    interval: Duration,
) {
    tokio::spawn(async move {
        let mut fetch_interval = tokio::time::interval(interval);
        // Trigger first fetch immediately.
        fetch_interval.tick().await;

        loop {
            fetch_interval.tick().await;

            // Fetch summary for today.
            let summary_res = client.summaries(waka_api::SummaryParams::today()).await;
            match summary_res {
                Ok(resp) => {
                    if tx.send(Event::SummaryUpdate(Box::new(resp))).await.is_err() {
                        break;
                    }
                }
                Err(e) => {
                    let msg = format_api_error(&e);
                    if tx.send(Event::Error(msg)).await.is_err() {
                        break;
                    }
                }
            }

            // Fetch summary for the last 7 days.
            let today = chrono::Local::now().date_naive();
            let week_start = today - chrono::Duration::days(6);
            let week_res = client
                .summaries(waka_api::SummaryParams::for_range(week_start, today))
                .await;
            match week_res {
                Ok(resp) => {
                    if tx.send(Event::WeeklyUpdate(Box::new(resp))).await.is_err() {
                        break;
                    }
                }
                Err(e) => {
                    let msg = format_api_error(&e);
                    if tx.send(Event::Error(msg)).await.is_err() {
                        break;
                    }
                }
            }

            // Fetch activity for the last 30 days.
            let activity_start = today - chrono::Duration::days(29);
            let activity_res = client
                .summaries(waka_api::SummaryParams::for_range(activity_start, today))
                .await;
            match activity_res {
                Ok(resp) => {
                    if tx
                        .send(Event::ActivityUpdate(Box::new(resp)))
                        .await
                        .is_err()
                    {
                        break;
                    }
                }
                Err(e) => {
                    let msg = format_api_error(&e);
                    if tx.send(Event::Error(msg)).await.is_err() {
                        break;
                    }
                }
            }

            // Fetch goals.
            let goals_res = client.goals().await;
            match goals_res {
                Ok(resp) => {
                    if tx.send(Event::GoalsUpdate(Box::new(resp))).await.is_err() {
                        break;
                    }
                }
                Err(e) => {
                    let msg = format_api_error(&e);
                    if tx.send(Event::Error(msg)).await.is_err() {
                        break;
                    }
                }
            }
        }
    });
}

/// Converts an `ApiError` to a human-readable error message.
fn format_api_error(err: &ApiError) -> String {
    format!("API error: {err}")
}

/// Handles a single `Event::Key` and updates `App` state accordingly.
pub fn handle_key_event(app: &mut crate::app::App, key: KeyEvent) {
    use KeyCode::{Char, Down, Esc, Tab, Up};

    // Global keys (work in all contexts).
    match key.code {
        Char('q') | Esc => {
            app.quit();
            return;
        }
        Char('?') => {
            app.toggle_help();
            return;
        }
        _ => {}
    }

    // If help is visible, any other key closes it.
    if app.show_help {
        app.toggle_help();
        return;
    }

    // View-specific keys.
    match key.code {
        Tab | Char('1' | '2' | '3' | '4' | '5') => {
            if let Char(c) = key.code {
                // Direct jump to view by number.
                match c {
                    '1' => app.view = crate::app::View::Main,
                    '2' => app.view = crate::app::View::Projects,
                    '3' => app.view = crate::app::View::Languages,
                    '4' => app.view = crate::app::View::Goals,
                    '5' => app.view = crate::app::View::Activity,
                    _ => {}
                }
            } else {
                // Tab cycles through views.
                app.next_view();
            }
            app.reset_list();
        }
        Char('r') => {
            // Trigger manual refresh (handled by spawning a new fetch task).
            // For now, just clear error.
            app.error = None;
        }
        Up => {
            app.list_up();
        }
        Down => {
            // Get the max for the current view's data.
            let max = app.current_view_items_count();
            app.list_down(max);
        }
        _ => {}
    }
}
