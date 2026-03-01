//! Goals panel widget.

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
use waka_api::GoalsResponse;

/// Renders the "Goals" panel showing goal progress.
///
/// Expected layout:
/// ```text
/// ┌─ Goals ─────────────────────────┐
/// │  ✓ Daily    6h 42m / 8h  ████  │
/// │  ✗ Python   3h / 10h     ███░  │
/// │  ✓ Streak   12 days            │
/// └─────────────────────────────────┘
/// ```
pub fn render_goals_panel(frame: &mut Frame, area: Rect, goals: &GoalsResponse) {
    let block = Block::default().title(" Goals ").borders(Borders::ALL);

    if goals.data.is_empty() {
        let placeholder = Paragraph::new("No goals configured")
            .block(block)
            .alignment(Alignment::Center);
        frame.render_widget(placeholder, area);
        return;
    }

    // Limit to top 3 goals
    let top3 = &goals.data[..goals.data.len().min(3)];

    let lines: Vec<Line> = top3
        .iter()
        .map(|goal| {
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

            // Compute progress
            let target_secs = goal.seconds;
            let current_secs = goal.seconds; // Use reported value if available
            let progress_str = if target_secs > 0.0 {
                let current_hrs = (current_secs / 3600.0).floor() as u64;
                let target_hrs = (target_secs / 3600.0).floor() as u64;
                let ratio = (current_secs / target_secs).clamp(0.0, 1.0);
                let bar_filled = (ratio * 10.0).floor() as usize;
                let bar_empty = 10 - bar_filled;
                let bar = format!("{}{}", "█".repeat(bar_filled), "░".repeat(bar_empty));
                format!("{current_hrs}h / {target_hrs}h  {bar}")
            } else {
                // No target — just show title (e.g., streak)
                String::new()
            };

            let title = goal.title.clone();

            Line::from(vec![
                Span::raw("  "),
                Span::styled(status_symbol, Style::default().fg(color)),
                Span::raw(" "),
                Span::raw(format!("{title:<10}")),
                Span::styled(progress_str, Style::default().fg(Color::Cyan)),
            ])
        })
        .collect();

    let para = Paragraph::new(lines).block(block);
    frame.render_widget(para, area);
}
