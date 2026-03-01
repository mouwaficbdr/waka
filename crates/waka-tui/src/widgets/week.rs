//! This Week panel widget.

#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss
)]

use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use waka_api::SummaryResponse;

/// Renders the "This Week" panel showing the last 7 days' total and average.
///
/// Expected layout:
/// ```text
/// ┌─ This Week ──────────────────────┐
/// │  32h 14m   Avg: 4h 36m           │
/// │  ▁▃▅▇▄▂█  Mon-Sun                │
/// └──────────────────────────────────┘
/// ```
pub fn render_week(frame: &mut Frame, area: Rect, summary_week: &SummaryResponse) {
    let block = Block::default().title(" This Week ").borders(Borders::ALL);

    if summary_week.data.is_empty() {
        let placeholder = Paragraph::new("No weekly data")
            .block(block)
            .alignment(Alignment::Center);
        frame.render_widget(placeholder, area);
        return;
    }

    // Sum all 7 days
    let total_secs: f64 = summary_week
        .data
        .iter()
        .map(|day| day.grand_total.total_seconds)
        .sum();
    let count = summary_week.data.len() as f64;
    let avg_secs = total_secs / count.max(1.0);

    let total_hours = (total_secs / 3600.0).floor() as u64;
    let total_minutes = ((total_secs % 3600.0) / 60.0).floor() as u64;
    let avg_hours = (avg_secs / 3600.0).floor() as u64;
    let avg_minutes = ((avg_secs % 3600.0) / 60.0).floor() as u64;

    let total_str = format!("{total_hours}h {total_minutes:02}m");
    let avg_str = format!("Avg: {avg_hours}h {avg_minutes:02}m");

    // Build sparkline: map each day's seconds to a bar character
    let max_secs = summary_week
        .data
        .iter()
        .map(|d| d.grand_total.total_seconds)
        .fold(0.0_f64, f64::max);

    let sparkline_chars = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
    let sparkline: String = summary_week
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

    // The spec shows day labels "Mon-Sun" — we'll just hardcode a generic text
    let labels = if summary_week.data.len() == 7 {
        "Mon-Sun"
    } else {
        "Last 7 days"
    };

    let line1 = Line::from(vec![
        Span::raw("  "),
        Span::styled(
            total_str,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("   "),
        Span::styled(avg_str, Style::default().fg(Color::Gray)),
    ]);

    let line2 = Line::from(vec![
        Span::raw("  "),
        Span::styled(sparkline, Style::default().fg(Color::Green)),
        Span::raw("  "),
        Span::raw(labels),
    ]);

    let para = Paragraph::new(vec![line1, line2]).block(block);
    frame.render_widget(para, area);
}
