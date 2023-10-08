use crate::app::{App, DbItem};
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

pub fn render_tui<B: Backend>(frame: &mut Frame<B>, app: &App, textarea: &TextArea) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Min(0),
            Constraint::Length(1), // text area
            Constraint::Length(1), // key bar
        ])
        .split(frame.size());

    render_table(frame, layout[0], &app.items);
    render_text_area(frame, layout[1], textarea);
    render_key_bar(frame, layout[2]);
}

fn render_table<B: Backend>(frame: &mut Frame<B>, layout: Rect, items: &Vec<DbItem>) {
    let header = Row::new(vec![
        Cell::from(Line::from("Id").alignment(Alignment::Center)),
        Cell::from(Line::from("Date").alignment(Alignment::Center)),
        Cell::from(Line::from("Category")),
        Cell::from(Line::from("Description")),
        Cell::from(Line::from("Price").alignment(Alignment::Center)),
    ])
    .style(Style::default().fg(Color::LightYellow));

    let mut body: Vec<Row> = Vec::new();

    for item in items {
        body.push(make_table_row(&item));
    }

    let table = Table::new(body)
        .block(Block::default().borders(Borders::ALL))
        .header(header)
        .style(Style::default().fg(Color::White).bg(Color::Blue))
        .widths(&[
            Constraint::Length(4),
            Constraint::Length(4 + 1 + 2 + 1 + 2),
            Constraint::Percentage(40),
            Constraint::Percentage(40),
            Constraint::Length(5 + 1 + 2),
        ]);

    frame.render_widget(table, layout);
}

fn make_table_row<'a>(item: &DbItem) -> Row<'a> {
    let id = format!("{}", item.id);
    let price = format!("{}.{:02}", item.price / 100, item.price % 100);

    Row::new(vec![
        Cell::from(Line::from(id).alignment(Alignment::Right)),
        Cell::from(item.date.clone()),
        Cell::from(item.category.clone()),
        Cell::from(item.description.clone()),
        Cell::from(Line::from(price).alignment(Alignment::Right)),
    ])
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
