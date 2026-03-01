//! Today panel widget.

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
use waka_api::{GoalsResponse, SummaryResponse};

/// Renders the "Today" panel showing today's total and goal progress.
///
/// Expected layout:
/// ```text
/// ┌─ Today ──────────────────────────┐
/// │  6h 42m  ███████████░░░  84%     │
/// │  Goal: 8h   Streak: 12 days 🔥   │
/// └──────────────────────────────────┘
/// ```
pub fn render_today(
    frame: &mut Frame,
    area: Rect,
    summary: &SummaryResponse,
    goals: &GoalsResponse,
) {
    let block = Block::default().title(" Today ").borders(Borders::ALL);

    // Extract today's data (first entry in the response)
    let Some(today) = summary.data.first() else {
        let placeholder = Paragraph::new("No data available")
            .block(block)
            .alignment(Alignment::Center);
        frame.render_widget(placeholder, area);
        return;
    };

    let total_secs = today.grand_total.total_seconds;
    let hours = (total_secs / 3600.0).floor() as u64;
    let minutes = ((total_secs % 3600.0) / 60.0).floor() as u64;
    let time_str = format!("{hours}h {minutes:02}m");

    // Find the daily goal to compute progress
    let daily_goal = goals.data.iter().find(|g| g.delta == "day");
    let (progress_pct, goal_str, bar) = if let Some(goal) = daily_goal {
        let target_secs = goal.seconds;
        if target_secs > 0.0 {
            let pct = ((total_secs / target_secs) * 100.0).min(100.0);
            let goal_hours = (target_secs / 3600.0).floor() as u64;
            let bar_filled = ((pct / 100.0) * 20.0).floor() as usize;
            let bar_empty = 20 - bar_filled;
            let bar_str = format!("{}{}", "█".repeat(bar_filled), "░".repeat(bar_empty));
            (pct, format!("Goal: {goal_hours}h"), bar_str)
        } else {
            (0.0, "Goal: N/A".to_string(), "░".repeat(20))
        }
    } else {
        (0.0, "No goal set".to_string(), "░".repeat(20))
    };

    // Find the current streak (assuming a goal with type="coding_streak" or similar exists)
    let streak_days = goals
        .data
        .iter()
        .find(|g| g.goal_type == "coding_streak" || g.delta == "day")
        .map_or(0, |g| (g.seconds / 3600.0 / 24.0).floor() as u64);
    let streak_str = if streak_days > 0 {
        format!("Streak: {streak_days} days 🔥")
    } else {
        "Streak: 0 days".to_string()
    };

    let line1 = Line::from(vec![
        Span::raw("  "),
        Span::styled(
            time_str,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::styled(bar, Style::default().fg(Color::Green)),
        Span::raw("  "),
        Span::styled(
            format!("{progress_pct:.0}%"),
            Style::default().fg(Color::Yellow),
        ),
    ]);

    let line2 = Line::from(vec![
        Span::raw("  "),
        Span::raw(goal_str),
        Span::raw("   "),
        Span::styled(streak_str, Style::default().fg(Color::Magenta)),
    ]);

    let para = Paragraph::new(vec![line1, line2])
        .block(block)
        .alignment(Alignment::Left);

    frame.render_widget(para, area);
}
