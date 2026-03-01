//! Status bar widget.

use std::time::Duration;

use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

/// Renders the status bar at the bottom of the screen.
///
/// Expected format:
/// ```text
/// Last updated: 14:32:01  ·  Auto-refresh in 4m 23s  ·  Tab: switch view
/// ```
pub fn render_status_bar(
    frame: &mut Frame,
    area: Rect,
    time_until_refresh: Duration,
    last_update_str: &str,
) {
    let refresh_mins = time_until_refresh.as_secs() / 60;
    let refresh_secs = time_until_refresh.as_secs() % 60;
    let refresh_str = format!("{refresh_mins}m {refresh_secs}s");

    let line = Line::from(vec![
        Span::raw(" Last updated: "),
        Span::styled(last_update_str, Style::default().fg(Color::Cyan)),
        Span::raw("  ·  Auto-refresh in "),
        Span::styled(refresh_str, Style::default().fg(Color::Yellow)),
        Span::raw("  ·  Tab: switch view"),
    ]);

    let para = Paragraph::new(line).style(Style::default().fg(Color::Gray));
    frame.render_widget(para, area);
}
