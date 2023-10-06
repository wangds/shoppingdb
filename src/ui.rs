use ratatui::{prelude::*, widgets::*};

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
    let text = vec![
        Span::styled("10", Style::default().fg(Color::White).bg(Color::Black)),
        Span::styled("Quit", Style::default().fg(Color::Black).bg(Color::Cyan)),
    ];

    frame.render_widget(Paragraph::new(Line::from(text)), layout);
}
