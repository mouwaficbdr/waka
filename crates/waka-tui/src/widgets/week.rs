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

/// Renders the "This Week" panel showing the last 7 days with per-day columns.
///
/// Expected layout:
/// ```text
/// ┌─ This Week ──────────────────────┐
/// │  32h 14m   Avg: 4h 36m           │
/// │                                  │
/// │  Mon   Tue   Wed   Thu   Fri   …  │
/// │   ▅     ▇     █     ▃     ▆   …  │
/// │  1h    3h    4h   45m   2h    …  │
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

    // Sum all days
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

    // Per-day columns
    let max_secs = summary_week
        .data
        .iter()
        .map(|d| d.grand_total.total_seconds)
        .fold(0.0_f64, f64::max);
    let sparkline_chars: [char; 8] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

    let mut name_spans: Vec<Span<'_>> = vec![Span::raw("  ")];
    let mut bar_spans: Vec<Span<'_>> = vec![Span::raw("  ")];
    let mut dur_spans: Vec<Span<'_>> = vec![Span::raw("  ")];

    for day in &summary_week.data {
        // Day name from range.date ("YYYY-MM-DD" → weekday abbreviation)
        let day_name = day
            .range
            .date
            .as_deref()
            .and_then(|s| s.parse::<chrono::NaiveDate>().ok())
            .map_or_else(
                || "---".to_owned(),
                |d| {
                    use chrono::Datelike as _;
                    match d.weekday() {
                        chrono::Weekday::Mon => "Mon",
                        chrono::Weekday::Tue => "Tue",
                        chrono::Weekday::Wed => "Wed",
                        chrono::Weekday::Thu => "Thu",
                        chrono::Weekday::Fri => "Fri",
                        chrono::Weekday::Sat => "Sat",
                        chrono::Weekday::Sun => "Sun",
                    }
                    .to_owned()
                },
            );

        // Bar char proportional to day's coding time
        let ratio = if max_secs > 0.0 {
            (day.grand_total.total_seconds / max_secs).clamp(0.0, 1.0)
        } else {
            0.0
        };
        let bar_ch = sparkline_chars[(ratio * (sparkline_chars.len() - 1) as f64).floor() as usize];

        // Short duration string
        let secs = day.grand_total.total_seconds;
        let h = (secs / 3600.0).floor() as u64;
        let m = ((secs % 3600.0) / 60.0).floor() as u64;
        let dur_str = if h > 0 && m > 0 {
            format!("{h}h{m}m")
        } else if h > 0 {
            format!("{h}h")
        } else {
            format!("{m}m")
        };

        name_spans.push(Span::styled(
            format!("{day_name:<6}"),
            Style::default().fg(Color::Gray),
        ));
        bar_spans.push(Span::styled(
            format!("{bar_ch:<6}"),
            Style::default().fg(Color::Green),
        ));
        dur_spans.push(Span::styled(
            format!("{dur_str:<6}"),
            Style::default().fg(Color::Cyan),
        ));
    }

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

    let para = Paragraph::new(vec![
        line1,
        Line::from(""),
        Line::from(name_spans),
        Line::from(bar_spans),
        Line::from(dur_spans),
    ])
    .block(block);

    frame.render_widget(para, area);
}
