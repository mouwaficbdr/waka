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
///
/// When offline: `⚠ offline  ·  Last updated: ...`
/// When loading: `⠹ Loading...  ·  Last updated: ...`
pub fn render_status_bar(
    frame: &mut Frame,
    area: Rect,
    time_until_refresh: Duration,
    last_update_str: &str,
    offline: bool,
    loading: bool,
    spinner_state: usize,
) {
    let spinner_frames = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
    let spinner = spinner_frames[spinner_state % spinner_frames.len()];

    let mut spans = vec![];

    // Show offline indicator or loading spinner
    if offline {
        spans.push(Span::styled("⚠ offline", Style::default().fg(Color::Red)));
        spans.push(Span::raw("  ·  "));
    } else if loading {
        spans.push(Span::styled(
            format!("{spinner} Loading..."),
            Style::default().fg(Color::Yellow),
        ));
        spans.push(Span::raw("  ·  "));
    }

    let refresh_mins = time_until_refresh.as_secs() / 60;
    let refresh_secs = time_until_refresh.as_secs() % 60;
    let refresh_str = format!("{refresh_mins}m {refresh_secs}s");

    spans.extend(vec![
        Span::raw(" Last updated: "),
        Span::styled(last_update_str, Style::default().fg(Color::Cyan)),
        Span::raw("  ·  Auto-refresh in "),
        Span::styled(refresh_str, Style::default().fg(Color::Yellow)),
        Span::raw("  ·  Tab: switch view"),
    ]);

    let line = Line::from(spans);
    let para = Paragraph::new(line).style(Style::default().fg(Color::Gray));
    frame.render_widget(para, area);
}
