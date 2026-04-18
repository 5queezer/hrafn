use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

use super::theme;
use crate::session::SessionMeta;

/// Render the session picker overlay.
pub(crate) fn render_session_picker(
    frame: &mut Frame,
    area: Rect,
    query: &str,
    items: &[&SessionMeta],
    selected: usize,
) {
    let width = area.width.min(80);
    let height = area.height.min(20);
    let [a] = Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .areas(area);
    let [a] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(a);

    frame.render_widget(Clear, a);
    let block = Block::default()
        .title(" Sessions ")
        .title_style(theme::bold())
        .borders(Borders::ALL)
        .border_style(theme::dim());

    let mut lines: Vec<Line> = vec![
        Line::from(format!("> {query}_")).style(theme::style()),
        Line::from(""),
    ];

    let max_visible = (height as usize).saturating_sub(5);
    let start = selected.saturating_sub(max_visible.saturating_sub(1));
    for (i, m) in items.iter().enumerate().skip(start).take(max_visible) {
        let rel = relative_time(m.updated_at);
        let title = m.title.as_deref().unwrap_or("\u{2014}"); // em dash
        let style = if i == selected {
            theme::bold()
        } else {
            theme::dim()
        };
        let marker = if i == selected { "> " } else { "  " };
        lines.push(Line::from(vec![Span::styled(
            format!(
                "{marker}{rel:>8}  {}  {:>4}m  {title}",
                m.id.short(),
                m.counts.total
            ),
            style,
        )]));
    }

    if items.is_empty() {
        lines.push(Line::from("  (no sessions)").style(theme::dim()));
    }
    lines.push(Line::from(""));
    lines.push(
        Line::from("\u{2191}\u{2193} navigate  \u{23CE} select  esc close").style(theme::dim()),
    );

    frame.render_widget(Paragraph::new(lines).block(block), a);
}

/// Case-insensitive substring filter on `id || title`.
pub(crate) fn filter_sessions<'a>(query: &str, items: &'a [SessionMeta]) -> Vec<&'a SessionMeta> {
    if query.is_empty() {
        return items.iter().collect();
    }
    let q = query.to_lowercase();
    items
        .iter()
        .filter(|m| {
            m.id.as_str().to_lowercase().contains(&q)
                || m.title
                    .as_deref()
                    .map(|t| t.to_lowercase().contains(&q))
                    .unwrap_or(false)
        })
        .collect()
}

/// Relative time string. Duplicated from `commands::sessions` for convenience;
/// if it drifts, consolidate into a shared helper.
fn relative_time(then: chrono::DateTime<chrono::Utc>) -> String {
    let now = chrono::Utc::now();
    let d = now.signed_duration_since(then);
    let secs = d.num_seconds().max(0);
    if secs < 60 {
        format!("{secs}s")
    } else if secs < 3600 {
        format!("{}m", secs / 60)
    } else if secs < 86_400 {
        format!("{}h", secs / 3600)
    } else if secs < 604_800 {
        format!("{}d", secs / 86_400)
    } else {
        format!("{}w", secs / 604_800)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session::{MessageCounts, SessionId, SessionMeta};
    use std::path::PathBuf;
    use std::time::Duration;

    fn meta(id_str: &str, title: Option<&str>) -> SessionMeta {
        SessionMeta {
            id: SessionId::parse(id_str).unwrap(),
            title: title.map(str::to_string),
            title_explicit: title.is_some(),
            cwd: PathBuf::from("/tmp"),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            duration: Duration::ZERO,
            provider: None,
            model: None,
            counts: MessageCounts::default(),
        }
    }

    #[test]
    fn filter_matches_id() {
        let items = vec![
            meta("20260101_000000_aabbcc", Some("First")),
            meta("20260202_000000_ddeeff", Some("Second")),
        ];
        let hits = filter_sessions("aabb", &items);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].id.as_str(), "20260101_000000_aabbcc");
    }

    #[test]
    fn filter_matches_title_case_insensitive() {
        let items = vec![
            meta("20260101_000000_aabbcc", Some("Fix Auth")),
            meta("20260202_000000_ddeeff", Some("Tests")),
        ];
        let hits = filter_sessions("auth", &items);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].title.as_deref(), Some("Fix Auth"));
    }

    #[test]
    fn filter_empty_query_returns_all() {
        let items = vec![
            meta("20260101_000000_aabbcc", None),
            meta("20260202_000000_ddeeff", None),
        ];
        let hits = filter_sessions("", &items);
        assert_eq!(hits.len(), 2);
    }
}
