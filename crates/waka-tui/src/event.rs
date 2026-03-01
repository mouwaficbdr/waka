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

/// Triggers a manual refresh by spawning a one-time fetch task.
///
/// This performs the same fetches as `spawn_data_fetcher` but only once,
/// immediately, instead of on an interval.
pub fn trigger_manual_refresh(tx: mpsc::Sender<Event>, client: waka_api::WakaClient) {
    tokio::spawn(async move {
        // Fetch summary for today.
        let summary_res = client.summaries(waka_api::SummaryParams::today()).await;
        match summary_res {
            Ok(resp) => {
                let _ = tx.send(Event::SummaryUpdate(Box::new(resp))).await;
            }
            Err(e) => {
                let msg = format_api_error(&e);
                let _ = tx.send(Event::Error(msg)).await;
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
                let _ = tx.send(Event::WeeklyUpdate(Box::new(resp))).await;
            }
            Err(e) => {
                let msg = format_api_error(&e);
                let _ = tx.send(Event::Error(msg)).await;
            }
        }

        // Fetch activity for the last 30 days.
        let activity_start = today - chrono::Duration::days(29);
        let activity_res = client
            .summaries(waka_api::SummaryParams::for_range(activity_start, today))
            .await;
        match activity_res {
            Ok(resp) => {
                let _ = tx.send(Event::ActivityUpdate(Box::new(resp))).await;
            }
            Err(e) => {
                let msg = format_api_error(&e);
                let _ = tx.send(Event::Error(msg)).await;
            }
        }

        // Fetch goals.
        let goals_res = client.goals().await;
        match goals_res {
            Ok(resp) => {
                let _ = tx.send(Event::GoalsUpdate(Box::new(resp))).await;
            }
            Err(e) => {
                let msg = format_api_error(&e);
                let _ = tx.send(Event::Error(msg)).await;
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

/// Exports the current view's data to a JSON file.
///
/// The export file is saved to `waka_export.json` in the current directory.
///
/// # Errors
/// Returns an error if the file cannot be written or if serialization fails.
fn export_current_view(app: &crate::app::App) -> Result<(), Box<dyn std::error::Error>> {
    use crate::app::View;
    use std::fs::File;
    use std::io::Write;

    let json_data = match app.view {
        View::Main => {
            serde_json::json!({
                "view": "main",
                "summary_today": app.summary_today,
                "summary_week": app.summary_week,
                "goals": app.goals,
            })
        }
        View::Projects => {
            serde_json::json!({
                "view": "projects",
                "projects": app.summary_week.as_ref().and_then(|s| s.data.first()).map(|d| &d.projects),
            })
        }
        View::Languages => {
            serde_json::json!({
                "view": "languages",
                "languages": app.summary_week.as_ref().and_then(|s| s.data.first()).map(|d| &d.languages),
            })
        }
        View::Goals => {
            serde_json::json!({
                "view": "goals",
                "goals": app.goals,
            })
        }
        View::Activity => {
            serde_json::json!({
                "view": "activity",
                "activity_30d": app.activity_30d,
            })
        }
    };

    let json_string = serde_json::to_string_pretty(&json_data)?;
    let mut file = File::create("waka_export.json")?;
    file.write_all(json_string.as_bytes())?;

    Ok(())
}

/// Handles a single `Event::Key` and updates `App` state accordingly.
pub fn handle_key_event(
    app: &mut crate::app::App,
    key: KeyEvent,
    tx: &mpsc::Sender<Event>,
    client: &waka_api::WakaClient,
) {
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
            // Trigger manual refresh by spawning a new fetch task.
            app.error = None;
            app.loading = true;
            trigger_manual_refresh(tx.clone(), client.clone());
        }
        Char('e') => {
            // Export current view.
            if let Err(e) = export_current_view(app) {
                app.error = Some(format!("Export failed: {e}"));
            } else {
                app.error = Some("Exported to waka_export.json".to_string());
            }
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
