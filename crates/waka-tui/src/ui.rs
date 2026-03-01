//! Rendering for the TUI application.

use chrono::Local;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::{App, View};
use crate::widgets::{activity, goals_panel, languages, projects, status_bar, today, week};

/// Renders the entire TUI screen.
pub fn render(f: &mut Frame, app: &App) {
    let size = f.area();

    // If we have an error and no data, show error message
    if app.error.is_some() && app.summary_today.is_none() {
        render_error_state(f, size, app);
    } else if app.loading && app.summary_today.is_none() {
        render_loading_state(f, size);
    } else {
        match app.view {
            View::Main => render_main_view(f, size, app),
            View::Projects => render_projects_view(f, size, app),
            View::Languages => render_languages_view(f, size, app),
            View::Goals => render_goals_view(f, size, app),
            View::Activity => render_activity_view(f, size, app),
        }
    }

    // Render help overlay if visible.
    if app.show_help {
        render_help_overlay(f, size);
    }
}

/// Renders the main dashboard view.
fn render_main_view(f: &mut Frame, area: Rect, app: &App) {
    // Overall layout: main content + status bar at bottom
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(area);

    let main_area = chunks[0];
    let status_area = chunks[1];

    // Split main area into rows:
    // - Row 1: Today (left) + This Week (right)
    // - Row 2: Top Projects (full width)
    // - Row 3: Languages (left) + Goals (right)
    // - Row 4: Activity (full width)

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // Row 1
            Constraint::Length(7), // Row 2 (projects table)
            Constraint::Length(6), // Row 3
            Constraint::Min(1),    // Row 4 (activity)
        ])
        .split(main_area);

    // Row 1: Today (left) + This Week (right)
    let today_week_cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(rows[0]);

    // Row 2: Top Projects (full width)
    let projects_area = rows[1];

    // Row 3: Languages (left) + Goals (right)
    let langs_goals_cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(rows[2]);

    // Row 4: Activity
    let activity_area = rows[3];

    // Render each widget if data is available
    if let Some(ref summary_today) = app.summary_today {
        if let Some(ref goals) = app.goals {
            today::render_today(f, today_week_cols[0], summary_today, goals);
        } else {
            render_placeholder_block(f, today_week_cols[0], "Today", "No goals data");
        }

        if let Some(ref summary_week) = app.summary_week {
            week::render_week(f, today_week_cols[1], summary_week);
        } else {
            render_placeholder_block(f, today_week_cols[1], "This Week", "No weekly data");
        }

        projects::render_projects(f, projects_area, summary_today);
        languages::render_languages(f, langs_goals_cols[0], summary_today);

        if let Some(ref goals) = app.goals {
            goals_panel::render_goals_panel(f, langs_goals_cols[1], goals);
        } else {
            render_placeholder_block(f, langs_goals_cols[1], "Goals", "No goals data");
        }
    } else {
        render_placeholder_block(f, today_week_cols[0], "Today", "No data");
        render_placeholder_block(f, today_week_cols[1], "This Week", "No data");
        render_placeholder_block(f, projects_area, "Top Projects", "No data");
        render_placeholder_block(f, langs_goals_cols[0], "Languages", "No data");
        render_placeholder_block(f, langs_goals_cols[1], "Goals", "No data");
    }

    if let Some(ref activity_30d) = app.activity_30d {
        activity::render_activity(f, activity_area, activity_30d);
    } else {
        render_placeholder_block(
            f,
            activity_area,
            "Activity (last 30 days)",
            "No activity data",
        );
    }

    // Render status bar
    let time_until_refresh = app.time_until_refresh();
    let last_update_str = app.last_update.map_or_else(
        || "Never".to_string(),
        |_instant| Local::now().format("%H:%M:%S").to_string(),
    );
    status_bar::render_status_bar(f, status_area, time_until_refresh, &last_update_str);
}

/// Renders the Projects detail view (View 2).
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss
)]
fn render_projects_view(f: &mut Frame, area: Rect, app: &App) {
    // Layout: main content + status bar
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(area);

    let main_area = chunks[0];
    let status_area = chunks[1];

    let block = Block::default()
        .title(" Projects — ↑↓: navigate, Enter: details ")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));

    let Some(ref summary_today) = app.summary_today else {
        let para = Paragraph::new("No project data available").block(block);
        f.render_widget(para, main_area);
        render_status_bar_simple(f, status_area);
        return;
    };

    // Sort projects by time descending
    let mut projects = summary_today
        .data
        .first()
        .map_or_else(Vec::new, |d| d.projects.clone());
    projects.sort_by(|a, b| b.total_seconds.partial_cmp(&a.total_seconds).unwrap());

    if projects.is_empty() {
        let para = Paragraph::new("No projects found").block(block);
        f.render_widget(para, main_area);
        render_status_bar_simple(f, status_area);
        return;
    }

    // Build lines with selection highlight
    let mut lines = vec![];
    for (idx, proj) in projects.iter().enumerate() {
        let hours = (proj.total_seconds / 3600.0).floor() as u64;
        let minutes = ((proj.total_seconds % 3600.0) / 60.0).floor() as u64;
        let time_str = format!("{hours}h {minutes:02}m");
        let percent_str = format!("{:.1}%", proj.percent);

        let bar_filled = ((proj.percent / 100.0) * 30.0).floor() as usize;
        let bar_empty = 30 - bar_filled;
        let bar = format!("{}{}", "█".repeat(bar_filled), "░".repeat(bar_empty));

        let style = if idx == app.list_index {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };

        let line = Line::from(vec![
            Span::raw(if idx == app.list_index {
                " ▶ "
            } else {
                "   "
            }),
            Span::styled(format!("{:<20}", proj.name), style),
            Span::styled(bar, Style::default().fg(Color::Green)),
            Span::raw("  "),
            Span::styled(format!("{time_str:>10}"), Style::default().fg(Color::Cyan)),
            Span::raw("  "),
            Span::styled(
                format!("{percent_str:>6}"),
                Style::default().fg(Color::Yellow),
            ),
        ]);
        lines.push(line);
    }

    let para = Paragraph::new(lines).block(block);
    f.render_widget(para, main_area);
    render_status_bar_simple(f, status_area);
}

/// Renders the Languages detail view (View 3).
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss
)]
fn render_languages_view(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(area);

    let main_area = chunks[0];
    let status_area = chunks[1];

    let block = Block::default()
        .title(" Languages — ↑↓: navigate ")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));

    let Some(ref summary_today) = app.summary_today else {
        let para = Paragraph::new("No language data available").block(block);
        f.render_widget(para, main_area);
        render_status_bar_simple(f, status_area);
        return;
    };

    let mut langs = summary_today
        .data
        .first()
        .map_or_else(Vec::new, |d| d.languages.clone());
    langs.sort_by(|a, b| b.total_seconds.partial_cmp(&a.total_seconds).unwrap());

    if langs.is_empty() {
        let para = Paragraph::new("No languages found").block(block);
        f.render_widget(para, main_area);
        render_status_bar_simple(f, status_area);
        return;
    }

    let mut lines = vec![];
    for (idx, lang) in langs.iter().enumerate() {
        let hours = (lang.total_seconds / 3600.0).floor() as u64;
        let minutes = ((lang.total_seconds % 3600.0) / 60.0).floor() as u64;
        let time_str = format!("{hours}h {minutes:02}m");
        let percent_str = format!("{:.1}%", lang.percent);

        let bar_filled = ((lang.percent / 100.0) * 30.0).floor() as usize;
        let bar_empty = 30 - bar_filled;
        let bar = format!("{}{}", "█".repeat(bar_filled), "░".repeat(bar_empty));

        let style = if idx == app.list_index {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };

        let line = Line::from(vec![
            Span::raw(if idx == app.list_index {
                " ▶ "
            } else {
                "   "
            }),
            Span::styled(format!("{:<15}", lang.name), style),
            Span::styled(bar, Style::default().fg(Color::Green)),
            Span::raw("  "),
            Span::styled(format!("{time_str:>10}"), Style::default().fg(Color::Cyan)),
            Span::raw("  "),
            Span::styled(
                format!("{percent_str:>6}"),
                Style::default().fg(Color::Yellow),
            ),
        ]);
        lines.push(line);
    }

    let para = Paragraph::new(lines).block(block);
    f.render_widget(para, main_area);
    render_status_bar_simple(f, status_area);
}

/// Renders the Goals detail view (View 4).
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss
)]
fn render_goals_view(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(area);

    let main_area = chunks[0];
    let status_area = chunks[1];

    let block = Block::default()
        .title(" Goals — ↑↓: navigate ")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));

    let Some(ref goals) = app.goals else {
        let para = Paragraph::new("No goals data available").block(block);
        f.render_widget(para, main_area);
        render_status_bar_simple(f, status_area);
        return;
    };

    if goals.data.is_empty() {
        let para = Paragraph::new("No goals configured").block(block);
        f.render_widget(para, main_area);
        render_status_bar_simple(f, status_area);
        return;
    }

    let mut lines = vec![];
    for (idx, goal) in goals.data.iter().enumerate() {
        let status_symbol = match goal.status.as_str() {
            "success" => "✓",
            "failure" => "✗",
            "pending" => "⋯",
            _ => "?",
        };

        let color = match goal.status.as_str() {
            "success" => Color::Green,
            "failure" => Color::Red,
            "pending" => Color::Yellow,
            _ => Color::Gray,
        };

        let style = if idx == app.list_index {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };

        let target_secs = goal.seconds;
        let progress_str = if target_secs > 0.0 {
            let target_hrs = (target_secs / 3600.0).floor() as u64;
            format!("Target: {target_hrs}h")
        } else {
            "No target".to_string()
        };

        lines.push(Line::from(vec![
            Span::raw(if idx == app.list_index {
                " ▶ "
            } else {
                "   "
            }),
            Span::styled(status_symbol, Style::default().fg(color)),
            Span::raw(" "),
            Span::styled(format!("{:<25}", goal.title), style),
            Span::styled(progress_str, Style::default().fg(Color::Cyan)),
        ]));

        lines.push(Line::from(vec![
            Span::raw("     "),
            Span::raw(format!(
                "Period: {}  |  Status: {}",
                goal.delta, goal.status
            )),
        ]));
        lines.push(Line::from(""));
    }

    let para = Paragraph::new(lines).block(block);
    f.render_widget(para, main_area);
    render_status_bar_simple(f, status_area);
}

/// Renders the Activity calendar view (View 5) — GitHub-style heatmap.
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss
)]
fn render_activity_view(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(area);

    let main_area = chunks[0];
    let status_area = chunks[1];

    let block = Block::default()
        .title(" Activity Calendar (Last 30 Days) ")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));

    let Some(ref activity_30d) = app.activity_30d else {
        let para = Paragraph::new("No activity data available").block(block);
        f.render_widget(para, main_area);
        render_status_bar_simple(f, status_area);
        return;
    };

    if activity_30d.data.is_empty() {
        let para = Paragraph::new("No activity data").block(block);
        f.render_widget(para, main_area);
        render_status_bar_simple(f, status_area);
        return;
    }

    // Build heatmap: 5 rows (weekdays), 6 columns (weeks)
    // Use block characters: ░ (0), ▒ (low), ▓ (med), █ (high)
    let max_secs = activity_30d
        .data
        .iter()
        .map(|d| d.grand_total.total_seconds)
        .fold(0.0_f64, f64::max);

    let mut lines = vec![Line::from("")];
    lines.push(Line::from(Span::styled(
        "  Each block represents one day:",
        Style::default().fg(Color::Gray),
    )));
    lines.push(Line::from(""));

    // Simple grid: show all 30 days in 6 rows of 5
    let mut day_index = 0;
    for _row in 0..6 {
        let mut row_spans = vec![Span::raw("  ")];
        for _col in 0..5 {
            if day_index >= activity_30d.data.len() {
                row_spans.push(Span::raw(" "));
                day_index += 1;
                continue;
            }

            let day = &activity_30d.data[day_index];
            let ratio = if max_secs > 0.0 {
                (day.grand_total.total_seconds / max_secs).clamp(0.0, 1.0)
            } else {
                0.0
            };

            let (ch, color) = if ratio < 0.25 {
                ('░', Color::DarkGray)
            } else if ratio < 0.5 {
                ('▒', Color::Green)
            } else if ratio < 0.75 {
                ('▓', Color::Green)
            } else {
                ('█', Color::Green)
            };

            row_spans.push(Span::styled(
                format!("{ch}{ch}"),
                Style::default().fg(color),
            ));
            row_spans.push(Span::raw(" "));
            day_index += 1;
        }
        lines.push(Line::from(row_spans));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::raw("  "),
        Span::styled("░░", Style::default().fg(Color::DarkGray)),
        Span::raw(" Less  "),
        Span::styled("██", Style::default().fg(Color::Green)),
        Span::raw(" More"),
    ]));

    let para = Paragraph::new(lines).block(block);
    f.render_widget(para, main_area);
    render_status_bar_simple(f, status_area);
}

/// Helper to render a simple status bar without refresh timer.
fn render_status_bar_simple(f: &mut Frame, area: Rect) {
    let line = Line::from(vec![Span::raw(" Tab: switch view  ·  ?: help  ·  q: quit")]);
    let para = Paragraph::new(line).style(Style::default().fg(Color::Gray));
    f.render_widget(para, area);
}

/// Renders an error state.
fn render_error_state(f: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .title("Error")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Red));

    let text = vec![
        Line::from(Span::styled(
            app.error.as_deref().unwrap_or("Unknown error"),
            Style::default().fg(Color::Red),
        )),
        Line::from(""),
        Line::from("Press r to retry, q to quit."),
    ];

    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, area);
}

/// Renders a loading state.
fn render_loading_state(f: &mut Frame, area: Rect) {
    let block = Block::default()
        .title("Loading...")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Yellow));

    let text = vec![Line::from("Fetching data from WakaTime API...")];

    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, area);
}

/// Renders a placeholder block with a message.
fn render_placeholder_block(f: &mut Frame, area: Rect, title: &str, message: &str) {
    let block = Block::default()
        .title(format!(" {title} "))
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Gray));

    let paragraph = Paragraph::new(message).block(block);
    f.render_widget(paragraph, area);
}

/// Renders the help overlay popup.
fn render_help_overlay(f: &mut Frame, area: Rect) {
    let block = Block::default()
        .title("Help")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black).fg(Color::White));

    let help_text = vec![
        Line::from(Span::styled(
            "Keyboard Shortcuts",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("q / Esc      Quit"),
        Line::from("?            Toggle this help"),
        Line::from("Tab / 1-5    Switch view"),
        Line::from("r            Refresh"),
        Line::from("↑ / ↓        Navigate list"),
        Line::from("Enter        View details"),
        Line::from(""),
        Line::from("Press any key to close this help."),
    ];

    let paragraph = Paragraph::new(help_text).block(block);

    // Center the popup (40x12 rect).
    let popup_area = centered_rect(40, 12, area);
    f.render_widget(paragraph, popup_area);
}

/// Returns a centered `Rect` with the given percentage width and height.
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
