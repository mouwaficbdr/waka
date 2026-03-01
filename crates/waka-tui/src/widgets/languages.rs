//! Languages breakdown widget.

#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss
)]

use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use waka_api::SummaryResponse;

/// Renders the "Languages" breakdown showing today's top languages.
///
/// Expected layout:
/// ```text
/// ┌─ Languages ────────────────────┐
/// │  Go          52%  ██████████   │
/// │  TypeScript  28%  █████░░░░░   │
/// │  Bash        12%  ██░░░░░░░░   │
/// │  YAML         8%  █░░░░░░░░░   │
/// └────────────────────────────────┘
/// ```
pub fn render_languages(frame: &mut Frame, area: Rect, summary: &SummaryResponse) {
    let block = Block::default().title(" Languages ").borders(Borders::ALL);

    let Some(today) = summary.data.first() else {
        let placeholder = Paragraph::new("No data")
            .block(block)
            .alignment(Alignment::Center);
        frame.render_widget(placeholder, area);
        return;
    };

    // Sort languages by total_seconds descending, limit to 4
    let mut langs = today.languages.clone();
    langs.sort_by(|a, b| b.total_seconds.partial_cmp(&a.total_seconds).unwrap());
    let top4 = &langs[..langs.len().min(4)];

    if top4.is_empty() {
        let placeholder = Paragraph::new("No languages today")
            .block(block)
            .alignment(Alignment::Center);
        frame.render_widget(placeholder, area);
        return;
    }

    let lines: Vec<Line> = top4
        .iter()
        .map(|lang| {
            // Build mini bar: 10 chars wide
            let bar_filled = ((lang.percent / 100.0) * 10.0).floor() as usize;
            let bar_empty = 10 - bar_filled;
            let bar = format!("{}{}", "█".repeat(bar_filled), "░".repeat(bar_empty));

            Line::from(vec![
                Span::raw("  "),
                Span::raw(format!("{:<12}", lang.name)),
                Span::styled(
                    format!("{:>3.0}%", lang.percent),
                    Style::default().fg(Color::Yellow),
                ),
                Span::raw("  "),
                Span::styled(bar, Style::default().fg(Color::Green)),
            ])
        })
        .collect();

    let para = Paragraph::new(lines).block(block);
    frame.render_widget(para, area);
}
