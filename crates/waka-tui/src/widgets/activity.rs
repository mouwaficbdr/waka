//! Activity (30-day sparkline) widget.

#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss
)]

use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use waka_api::SummaryResponse;

/// Renders the "Activity (last 30 days)" sparkline.
///
/// Expected layout:
/// ```text
/// ┌─ Activity (last 30 days) ──────────────────────────────────────┐
/// │  ░▁▂▃▄▅▆▇█▇▆▅▄▃▂▁░▁▂▃▄▅▆▇█▇▆▅  Dec 13 ──────────── Jan 13      │
/// └────────────────────────────────────────────────────────────────┘
/// ```
pub fn render_activity(frame: &mut Frame, area: Rect, activity_30d: &SummaryResponse) {
    let block = Block::default()
        .title(" Activity (last 30 days) ")
        .borders(Borders::ALL);

    if activity_30d.data.is_empty() {
        let placeholder = Paragraph::new("No activity data")
            .block(block)
            .style(Style::default().fg(Color::Gray));
        frame.render_widget(placeholder, area);
        return;
    }

    // Build sparkline from all days
    let max_secs = activity_30d
        .data
        .iter()
        .map(|d| d.grand_total.total_seconds)
        .fold(0.0_f64, f64::max);

    let sparkline_chars = ['░', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
    let sparkline: String = activity_30d
        .data
        .iter()
        .map(|day| {
            let ratio = if max_secs > 0.0 {
                (day.grand_total.total_seconds / max_secs).clamp(0.0, 1.0)
            } else {
                0.0
            };
            let idx = (ratio * (sparkline_chars.len() - 1) as f64).floor() as usize;
            sparkline_chars[idx]
        })
        .collect();

    // Extract date range from start/end fields
    let start_date = activity_30d.start.split('T').next().unwrap_or("");
    let end_date = activity_30d.end.split('T').next().unwrap_or("");
    let date_range = format!("{start_date} ──────────── {end_date}");

    let line = Line::from(vec![
        Span::raw("  "),
        Span::styled(sparkline, Style::default().fg(Color::Green)),
        Span::raw("  "),
        Span::styled(date_range, Style::default().fg(Color::Gray)),
    ]);

    let para = Paragraph::new(vec![line]).block(block);
    frame.render_widget(para, area);
}
