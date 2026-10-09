//! `waka report`: productivity reports (Markdown, HTML, JSON, CSV).

use super::*;

// ─── report ───────────────────────────────────────────────────────────────────

// `needless_pass_by_value`: cmd is consumed by the match for exhaustive checking.
#[allow(clippy::needless_pass_by_value)]
pub(super) async fn report(cmd: ReportCommands, global: &GlobalOpts) -> Result<()> {
    match cmd {
        ReportCommands::Generate {
            from,
            to,
            output,
            output_format,
        } => report_generate(from, to, output, output_format, global).await,
        ReportCommands::Summary { period } => report_summary(period, global).await,
    }
}

/// Generates a productivity report for a date range.
async fn report_generate(
    from: String,
    to: String,
    output: Option<std::path::PathBuf>,
    format: ReportFormat,
    global: &GlobalOpts,
) -> Result<()> {
    use chrono::NaiveDate;

    // Parse dates
    let start_date = NaiveDate::parse_from_str(&from, "%Y-%m-%d")
        .with_context(|| format!("invalid start date '{from}' (expected YYYY-MM-DD)"))?;
    let end_date = NaiveDate::parse_from_str(&to, "%Y-%m-%d")
        .with_context(|| format!("invalid end date '{to}' (expected YYYY-MM-DD)"))?;

    if end_date < start_date {
        bail!("end date must be after start date");
    }

    // Build client
    let config = load_config()?;
    let profile = resolve_profile(global, &config);
    let client = build_api_client(&profile, &config)?;

    // Fetch data for the period
    let pb = stats_spinner("Fetching data for report...");
    let params = waka_api::SummaryParams::for_range(start_date, end_date);
    let summary = client.summaries(params).await?;

    // Fetch goals (may fail if user has no goals - that's ok)
    let goals = client.goals().await.ok();

    pb.finish_and_clear();

    // Generate report content
    let content = match format {
        ReportFormat::Md => {
            generate_report_markdown(&summary, goals.as_ref(), start_date, end_date)
        }
        ReportFormat::Html => generate_report_html(&summary, goals.as_ref(), start_date, end_date),
        ReportFormat::Json => generate_report_json(&summary, goals.as_ref(), start_date, end_date)?,
        ReportFormat::Csv => generate_report_csv(&summary, start_date, end_date),
    };

    // Write output
    if let Some(path) = output {
        std::fs::write(&path, content)
            .with_context(|| format!("failed to write report to {}", path.display()))?;
        if !global.quiet {
            eprintln!("Report written to {}", path.display());
        }
    } else {
        print!("{content}");
    }

    Ok(())
}

/// Shows a brief productivity summary.
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss
)]
async fn report_summary(period: SummaryPeriod, global: &GlobalOpts) -> Result<()> {
    let config = load_config()?;
    let profile = resolve_profile(global, &config);
    let client = build_api_client(&profile, &config)?;

    let today = chrono::Local::now().date_naive();
    let params = match period {
        SummaryPeriod::Week => {
            let start = today - chrono::Duration::days(6);
            waka_api::SummaryParams::for_range(start, today)
        }
        SummaryPeriod::Month => {
            let start = today - chrono::Duration::days(29);
            waka_api::SummaryParams::for_range(start, today)
        }
    };

    let pb = stats_spinner("Fetching summary...");
    let summary = client.summaries(params).await?;
    pb.finish_and_clear();

    // Calculate totals
    let total_seconds: f64 = summary
        .data
        .iter()
        .map(|d| d.grand_total.total_seconds)
        .sum();
    let hours = (total_seconds / 3600.0).floor() as u64;
    let minutes = ((total_seconds % 3600.0) / 60.0).floor() as u64;

    let days = summary.data.len();
    let avg_seconds = if days > 0 {
        total_seconds / days as f64
    } else {
        0.0
    };
    let avg_hours = (avg_seconds / 3600.0).floor() as u64;
    let avg_minutes = ((avg_seconds % 3600.0) / 60.0).floor() as u64;

    let period_name = match period {
        SummaryPeriod::Week => "Last 7 days",
        SummaryPeriod::Month => "Last 30 days",
    };

    println!("\n{period_name} Summary\n");
    println!("  Total:   {hours}h {minutes}m");
    println!("  Average: {avg_hours}h {avg_minutes}m per day");
    println!("  Days:    {days}\n");

    Ok(())
}

// ─── report generation ────────────────────────────────────────────────────────

/// Generates a Markdown report.
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::too_many_lines,
    clippy::redundant_closure_for_method_calls
)]
fn generate_report_markdown(
    summary: &waka_api::SummaryResponse,
    goals: Option<&waka_api::GoalsResponse>,
    start_date: chrono::NaiveDate,
    end_date: chrono::NaiveDate,
) -> String {
    use chrono::Datelike as _;
    use std::fmt::Write as _;

    let mut output = String::new();

    // Header
    output.push_str("# Productivity Report\n\n");
    let _ = writeln!(output, "**Period:** {start_date} to {end_date}\n");

    // Calculate totals
    let total_seconds: f64 = summary
        .data
        .iter()
        .map(|d| d.grand_total.total_seconds)
        .sum();
    let hours = (total_seconds / 3600.0).floor() as u64;
    let minutes = ((total_seconds % 3600.0) / 60.0).floor() as u64;

    let _ = writeln!(output, "**Total Time:** {hours}h {minutes}m\n");

    let days_with_data = summary
        .data
        .iter()
        .filter(|d| d.grand_total.total_seconds > 0.0)
        .count();
    if days_with_data > 0 {
        let avg_seconds = total_seconds / days_with_data as f64;
        let avg_hours = (avg_seconds / 3600.0).floor() as u64;
        let avg_minutes = ((avg_seconds % 3600.0) / 60.0).floor() as u64;
        let _ = writeln!(
            output,
            "**Average per day:** {avg_hours}h {avg_minutes}m (across {days_with_data} active days)\n"
        );
    }

    output.push_str("---\n\n");

    // Projects breakdown
    output.push_str("## Projects\n\n");

    let mut project_totals: std::collections::HashMap<String, f64> =
        std::collections::HashMap::new();
    for day in &summary.data {
        for project in &day.projects {
            *project_totals.entry(project.name.clone()).or_insert(0.0) += project.total_seconds;
        }
    }

    if project_totals.is_empty() {
        output.push_str("*No project data available.*\n\n");
    } else {
        let mut projects: Vec<_> = project_totals.into_iter().collect();
        projects.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        output.push_str("| Project | Time | Percentage |\n");
        output.push_str("|---------|------|------------|\n");

        for (name, secs) in projects.iter().take(10) {
            let hrs = (secs / 3600.0).floor() as u64;
            let mins = ((secs % 3600.0) / 60.0).floor() as u64;
            let pct = if total_seconds > 0.0 {
                (secs / total_seconds * 100.0).round() as u64
            } else {
                0
            };
            let _ = writeln!(output, "| {name} | {hrs}h {mins}m | {pct}% |");
        }
        output.push('\n');
    }

    // Languages breakdown
    output.push_str("## Languages\n\n");

    let mut language_totals: std::collections::HashMap<String, f64> =
        std::collections::HashMap::new();
    for day in &summary.data {
        for lang in &day.languages {
            *language_totals.entry(lang.name.clone()).or_insert(0.0) += lang.total_seconds;
        }
    }

    if language_totals.is_empty() {
        output.push_str("*No language data available.*\n\n");
    } else {
        let mut languages: Vec<_> = language_totals.into_iter().collect();
        languages.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        output.push_str("| Language | Time | Percentage |\n");
        output.push_str("|----------|------|------------|\n");

        for (name, secs) in languages.iter().take(10) {
            let hrs = (secs / 3600.0).floor() as u64;
            let mins = ((secs % 3600.0) / 60.0).floor() as u64;
            let pct = if total_seconds > 0.0 {
                (secs / total_seconds * 100.0).round() as u64
            } else {
                0
            };
            let _ = writeln!(output, "| {name} | {hrs}h {mins}m | {pct}% |");
        }
        output.push('\n');
    }

    // Editors breakdown
    output.push_str("## Editors\n\n");

    let mut editor_totals: std::collections::HashMap<String, f64> =
        std::collections::HashMap::new();
    for day in &summary.data {
        for editor in &day.editors {
            *editor_totals.entry(editor.name.clone()).or_insert(0.0) += editor.total_seconds;
        }
    }

    if editor_totals.is_empty() {
        output.push_str("*No editor data available.*\n\n");
    } else {
        let mut editors: Vec<_> = editor_totals.into_iter().collect();
        editors.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        output.push_str("| Editor | Time | Percentage |\n");
        output.push_str("|--------|------|------------|\n");

        for (name, secs) in &editors {
            let hrs = (secs / 3600.0).floor() as u64;
            let mins = ((secs % 3600.0) / 60.0).floor() as u64;
            let pct = if total_seconds > 0.0 {
                (secs / total_seconds * 100.0).round() as u64
            } else {
                0
            };
            let _ = writeln!(output, "| {name} | {hrs}h {mins}m | {pct}% |");
        }
        output.push('\n');
    }

    // Daily activity
    output.push_str("## Daily Activity\n\n");
    output.push_str("| Date | Day | Time |\n");
    output.push_str("|------|-----|------|\n");

    for day_data in &summary.data {
        let date_str = day_data.range.date.as_deref().unwrap_or("N/A");
        let date = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok();
        let day_name = match date.as_ref().map(|d| d.weekday()) {
            Some(chrono::Weekday::Mon) => "Mon",
            Some(chrono::Weekday::Tue) => "Tue",
            Some(chrono::Weekday::Wed) => "Wed",
            Some(chrono::Weekday::Thu) => "Thu",
            Some(chrono::Weekday::Fri) => "Fri",
            Some(chrono::Weekday::Sat) => "Sat",
            Some(chrono::Weekday::Sun) => "Sun",
            None => "?",
        };
        let secs = day_data.grand_total.total_seconds;
        let hrs = (secs / 3600.0).floor() as u64;
        let mins = ((secs % 3600.0) / 60.0).floor() as u64;
        let _ = writeln!(output, "| {date_str} | {day_name} | {hrs}h {mins}m |");
    }
    output.push('\n');

    // Goals achieved
    if let Some(goals_resp) = goals {
        if !goals_resp.data.is_empty() {
            output.push_str("## Goals\n\n");

            for goal in &goals_resp.data {
                let status = if goal.status == "success" {
                    "✓ Achieved"
                } else if goal.status == "pending" {
                    "⋯ In Progress"
                } else {
                    "✗ Not Achieved"
                };
                let title = &goal.title;
                let _ = writeln!(output, "- **{title}**: {status}");
            }
            output.push('\n');
        }
    }

    output.push_str("---\n\n");
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M");
    let _ = writeln!(output, "*Generated on {now} by waka*");

    output
}

/// Generates an HTML report with inline CSS.
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::too_many_lines,
    clippy::redundant_closure_for_method_calls
)]
fn generate_report_html(
    summary: &waka_api::SummaryResponse,
    goals: Option<&waka_api::GoalsResponse>,
    start_date: chrono::NaiveDate,
    end_date: chrono::NaiveDate,
) -> String {
    use chrono::Datelike as _;
    use std::fmt::Write as _;

    // Calculate totals
    let total_seconds: f64 = summary
        .data
        .iter()
        .map(|d| d.grand_total.total_seconds)
        .sum();
    let hours = (total_seconds / 3600.0).floor() as u64;
    let minutes = ((total_seconds % 3600.0) / 60.0).floor() as u64;

    let days_with_data = summary
        .data
        .iter()
        .filter(|d| d.grand_total.total_seconds > 0.0)
        .count();
    let avg_display = if days_with_data > 0 {
        let avg_seconds = total_seconds / days_with_data as f64;
        let avg_hours = (avg_seconds / 3600.0).floor() as u64;
        let avg_minutes = ((avg_seconds % 3600.0) / 60.0).floor() as u64;
        format!("{avg_hours}h {avg_minutes}m (across {days_with_data} active days)")
    } else {
        "N/A".to_string()
    };

    let mut html = String::new();
    html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
    html.push_str("<meta charset=\"UTF-8\">\n");
    html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
    html.push_str("<title>Productivity Report</title>\n");
    html.push_str("<style>\n");
    html.push_str("body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; max-width: 900px; margin: 40px auto; padding: 20px; background: #f5f5f5; }\n");
    html.push_str("h1 { color: #333; border-bottom: 3px solid #4CAF50; padding-bottom: 10px; }\n");
    html.push_str("h2 { color: #555; margin-top: 40px; border-bottom: 2px solid #ddd; padding-bottom: 8px; }\n");
    html.push_str(".header { background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); margin-bottom: 20px; }\n");
    html.push_str(".stat { font-size: 18px; margin: 10px 0; }\n");
    html.push_str(".stat strong { color: #4CAF50; }\n");
    html.push_str("table { width: 100%; border-collapse: collapse; background: white; margin: 20px 0; border-radius: 8px; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }\n");
    html.push_str("th { background: #4CAF50; color: white; padding: 12px; text-align: left; }\n");
    html.push_str("td { padding: 12px; border-bottom: 1px solid #ddd; }\n");
    html.push_str("tr:last-child td { border-bottom: none; }\n");
    html.push_str("tr:hover { background: #f9f9f9; }\n");
    html.push_str(
        ".footer { text-align: center; margin-top: 40px; color: #888; font-size: 14px; }\n",
    );
    html.push_str(".goal-list { background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }\n");
    html.push_str(".goal-item { padding: 10px 0; border-bottom: 1px solid #eee; }\n");
    html.push_str(".goal-item:last-child { border-bottom: none; }\n");
    html.push_str("</style>\n");
    html.push_str("</head>\n<body>\n");

    // Header
    html.push_str("<div class=\"header\">\n");
    html.push_str("<h1>Productivity Report</h1>\n");
    let _ = writeln!(
        html,
        "<div class=\"stat\"><strong>Period:</strong> {start_date} to {end_date}</div>"
    );
    let _ = writeln!(
        html,
        "<div class=\"stat\"><strong>Total Time:</strong> {hours}h {minutes}m</div>"
    );
    let _ = writeln!(
        html,
        "<div class=\"stat\"><strong>Average per day:</strong> {avg_display}</div>"
    );
    html.push_str("</div>\n");

    // Projects
    html.push_str("<h2>Projects</h2>\n");
    let mut project_totals: std::collections::HashMap<String, f64> =
        std::collections::HashMap::new();
    for day in &summary.data {
        for project in &day.projects {
            *project_totals.entry(project.name.clone()).or_insert(0.0) += project.total_seconds;
        }
    }

    if project_totals.is_empty() {
        html.push_str("<p><em>No project data available.</em></p>\n");
    } else {
        let mut projects: Vec<_> = project_totals.into_iter().collect();
        projects.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        html.push_str(
            "<table>\n<thead><tr><th>Project</th><th>Time</th><th>Percentage</th></tr></thead>\n<tbody>\n",
        );
        for (name, secs) in projects.iter().take(10) {
            let hrs = (secs / 3600.0).floor() as u64;
            let mins = ((secs % 3600.0) / 60.0).floor() as u64;
            let pct = if total_seconds > 0.0 {
                (secs / total_seconds * 100.0).round() as u64
            } else {
                0
            };
            let _ = writeln!(
                html,
                "<tr><td>{}</td><td>{hrs}h {mins}m</td><td>{pct}%</td></tr>",
                html_escape(name)
            );
        }
        html.push_str("</tbody>\n</table>\n");
    }

    // Languages
    html.push_str("<h2>Languages</h2>\n");
    let mut language_totals: std::collections::HashMap<String, f64> =
        std::collections::HashMap::new();
    for day in &summary.data {
        for lang in &day.languages {
            *language_totals.entry(lang.name.clone()).or_insert(0.0) += lang.total_seconds;
        }
    }

    if language_totals.is_empty() {
        html.push_str("<p><em>No language data available.</em></p>\n");
    } else {
        let mut languages: Vec<_> = language_totals.into_iter().collect();
        languages.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        html.push_str(
            "<table>\n<thead><tr><th>Language</th><th>Time</th><th>Percentage</th></tr></thead>\n<tbody>\n",
        );
        for (name, secs) in languages.iter().take(10) {
            let hrs = (secs / 3600.0).floor() as u64;
            let mins = ((secs % 3600.0) / 60.0).floor() as u64;
            let pct = if total_seconds > 0.0 {
                (secs / total_seconds * 100.0).round() as u64
            } else {
                0
            };
            let _ = writeln!(
                html,
                "<tr><td>{}</td><td>{hrs}h {mins}m</td><td>{pct}%</td></tr>",
                html_escape(name)
            );
        }
        html.push_str("</tbody>\n</table>\n");
    }

    // Editors
    html.push_str("<h2>Editors</h2>\n");
    let mut editor_totals: std::collections::HashMap<String, f64> =
        std::collections::HashMap::new();
    for day in &summary.data {
        for editor in &day.editors {
            *editor_totals.entry(editor.name.clone()).or_insert(0.0) += editor.total_seconds;
        }
    }

    if editor_totals.is_empty() {
        html.push_str("<p><em>No editor data available.</em></p>\n");
    } else {
        let mut editors: Vec<_> = editor_totals.into_iter().collect();
        editors.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        html.push_str(
            "<table>\n<thead><tr><th>Editor</th><th>Time</th><th>Percentage</th></tr></thead>\n<tbody>\n",
        );
        for (name, secs) in &editors {
            let hrs = (secs / 3600.0).floor() as u64;
            let mins = ((secs % 3600.0) / 60.0).floor() as u64;
            let pct = if total_seconds > 0.0 {
                (secs / total_seconds * 100.0).round() as u64
            } else {
                0
            };
            let _ = writeln!(
                html,
                "<tr><td>{}</td><td>{hrs}h {mins}m</td><td>{pct}%</td></tr>",
                html_escape(name)
            );
        }
        html.push_str("</tbody>\n</table>\n");
    }

    // Daily activity
    html.push_str("<h2>Daily Activity</h2>\n");
    html.push_str(
        "<table>\n<thead><tr><th>Date</th><th>Day</th><th>Time</th></tr></thead>\n<tbody>\n",
    );
    for day_data in &summary.data {
        let date_str = day_data.range.date.as_deref().unwrap_or("N/A");
        let date = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok();
        let day_name = match date.as_ref().map(|d| d.weekday()) {
            Some(chrono::Weekday::Mon) => "Mon",
            Some(chrono::Weekday::Tue) => "Tue",
            Some(chrono::Weekday::Wed) => "Wed",
            Some(chrono::Weekday::Thu) => "Thu",
            Some(chrono::Weekday::Fri) => "Fri",
            Some(chrono::Weekday::Sat) => "Sat",
            Some(chrono::Weekday::Sun) => "Sun",
            None => "?",
        };
        let secs = day_data.grand_total.total_seconds;
        let hrs = (secs / 3600.0).floor() as u64;
        let mins = ((secs % 3600.0) / 60.0).floor() as u64;
        let _ = writeln!(
            html,
            "<tr><td>{date_str}</td><td>{day_name}</td><td>{hrs}h {mins}m</td></tr>"
        );
    }
    html.push_str("</tbody>\n</table>\n");

    // Goals
    if let Some(goals_resp) = goals {
        if !goals_resp.data.is_empty() {
            html.push_str("<h2>Goals</h2>\n<div class=\"goal-list\">\n");
            for goal in &goals_resp.data {
                let status = if goal.status == "success" {
                    "✓ Achieved"
                } else if goal.status == "pending" {
                    "⋯ In Progress"
                } else {
                    "✗ Not Achieved"
                };
                let title = html_escape(&goal.title);
                let _ = writeln!(
                    html,
                    "<div class=\"goal-item\"><strong>{title}:</strong> {status}</div>"
                );
            }
            html.push_str("</div>\n");
        }
    }

    // Footer
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M");
    let _ = writeln!(
        html,
        "<div class=\"footer\">Generated on {now} by waka</div>"
    );
    html.push_str("</body>\n</html>");

    html
}

/// Generates a JSON report.
fn generate_report_json(
    summary: &waka_api::SummaryResponse,
    goals: Option<&waka_api::GoalsResponse>,
    start_date: chrono::NaiveDate,
    end_date: chrono::NaiveDate,
) -> Result<String> {
    let total_seconds: f64 = summary
        .data
        .iter()
        .map(|d| d.grand_total.total_seconds)
        .sum();

    let report = serde_json::json!({
        "period": {
            "start": start_date.to_string(),
            "end": end_date.to_string(),
        },
        "summary": {
            "total_seconds": total_seconds,
            "total_hours": (total_seconds / 3600.0).floor(),
            "total_minutes": ((total_seconds % 3600.0) / 60.0).floor(),
        },
        "data": summary.data,
        "goals": goals.map(|g| &g.data),
    });

    serde_json::to_string_pretty(&report).context("failed to serialize report to JSON")
}

/// Generates a CSV report.
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::redundant_closure_for_method_calls
)]
fn generate_report_csv(
    summary: &waka_api::SummaryResponse,
    _start_date: chrono::NaiveDate,
    _end_date: chrono::NaiveDate,
) -> String {
    use chrono::Datelike as _;
    use std::fmt::Write as _;

    let mut output = String::new();

    // Header
    output.push_str("Date,Day,Total Time (hours),Projects,Languages,Editors\n");

    // Daily rows
    for day_data in &summary.data {
        let date_str = day_data.range.date.as_deref().unwrap_or("N/A");
        let date = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok();
        let day_name = match date.as_ref().map(|d| d.weekday()) {
            Some(chrono::Weekday::Mon) => "Mon",
            Some(chrono::Weekday::Tue) => "Tue",
            Some(chrono::Weekday::Wed) => "Wed",
            Some(chrono::Weekday::Thu) => "Thu",
            Some(chrono::Weekday::Fri) => "Fri",
            Some(chrono::Weekday::Sat) => "Sat",
            Some(chrono::Weekday::Sun) => "Sun",
            None => "?",
        };

        let total_hours = day_data.grand_total.total_seconds / 3600.0;

        let projects: Vec<String> = day_data
            .projects
            .iter()
            .map(|p| format!("{}({:.1}h)", p.name, p.total_seconds / 3600.0))
            .collect();
        let projects_str = projects.join("; ");

        let languages: Vec<String> = day_data
            .languages
            .iter()
            .map(|l| format!("{}({:.1}h)", l.name, l.total_seconds / 3600.0))
            .collect();
        let languages_str = languages.join("; ");

        let editors: Vec<String> = day_data
            .editors
            .iter()
            .map(|e| format!("{}({:.1}h)", e.name, e.total_seconds / 3600.0))
            .collect();
        let editors_str = editors.join("; ");

        let _ = writeln!(
            output,
            "{},{day_name},{total_hours:.2},{},{},{}",
            delimited_field(date_str, ','),
            delimited_field(&projects_str, ','),
            delimited_field(&languages_str, ','),
            delimited_field(&editors_str, ','),
        );
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── report escaping ───────────────────────────────────────────────────────

    fn summary_with_hostile_names() -> waka_api::SummaryResponse {
        let json = include_str!("../../../../tests/fixtures/summaries_today.json");
        let mut resp: waka_api::SummaryResponse =
            serde_json::from_str(json).expect("fixture is valid JSON");
        resp.data[0].projects[0].name = "<script>alert(1)</script>".to_owned();
        resp.data[0].languages[0].name = "C, \"quoted\"".to_owned();
        resp
    }

    #[test]
    fn html_report_escapes_names() {
        let resp = summary_with_hostile_names();
        let day = chrono::NaiveDate::from_ymd_opt(2025, 1, 13).unwrap();
        let html = generate_report_html(&resp, None, day, day);
        assert!(!html.contains("<script>alert(1)</script>"));
        assert!(html.contains("&lt;script&gt;alert(1)&lt;/script&gt;"));
    }

    #[test]
    fn csv_report_quotes_fields_per_rfc4180() {
        let resp = summary_with_hostile_names();
        let day = chrono::NaiveDate::from_ymd_opt(2025, 1, 13).unwrap();
        let csv = generate_report_csv(&resp, day, day);
        let row = csv.lines().nth(1).expect("one data row");
        // Inner quotes are doubled inside a quoted field.
        assert!(row.contains("C, \"\"quoted\"\""), "row: {row}");
        // Hours keep one decimal instead of rounding small entries to 0h.
        assert!(!row.contains("(0h)"), "row: {row}");
    }
}
