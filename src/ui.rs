use ratatui::{prelude::*, widgets::*};

const KEY_BAR_ITEMS: &[(&'static str, &'static str)] = &[
    (" 1", "Help"),
    (" 2", "    "),
    (" 3", "    "),
    (" 4", "    "),
    (" 5", "    "),
    (" 6", "    "),
    (" 7", "    "),
    (" 8", "    "),
    (" 9", "    "),
    ("10", "Quit"),
];

pub fn render_tui<B: Backend>(frame: &mut Frame<B>) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Min(0),
            Constraint::Length(1), // text entry
            Constraint::Length(1), // key bar
        ])
        .split(frame.size());

    render_key_bar(frame, layout[2]);
}

pub fn render_key_bar<B: Backend>(frame: &mut Frame<B>, layout: Rect) {
    let key_style = Style::default().fg(Color::White).bg(Color::Black);
    let text_style = Style::default().fg(Color::Black).bg(Color::Cyan);

    let div = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(10)].repeat(KEY_BAR_ITEMS.len()))
        .split(layout);

    for (i, &(key, text)) in KEY_BAR_ITEMS.iter().enumerate() {
        let hint = vec![Span::styled(key, key_style), Span::from(text)];
        frame.render_widget(Paragraph::new(Line::from(hint)).style(text_style), div[i]);
    }
}
