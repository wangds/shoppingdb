use ratatui::{prelude::*, widgets::*};
use tui_textarea::TextArea;

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

pub fn render_tui<B: Backend>(frame: &mut Frame<B>, textarea: &TextArea) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Min(0),
            Constraint::Length(1), // text area
            Constraint::Length(1), // key bar
        ])
        .split(frame.size());

    render_text_area(frame, layout[1], textarea);
    render_key_bar(frame, layout[2]);
}

fn render_text_area<B: Backend>(frame: &mut Frame<B>, layout: Rect, textarea: &TextArea) {
    let div = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Length(2), Constraint::Min(0)])
        .split(layout);

    frame.render_widget(Paragraph::new("> "), div[0]);
    frame.render_widget(textarea.widget(), div[1]);
}

fn render_key_bar<B: Backend>(frame: &mut Frame<B>, layout: Rect) {
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
