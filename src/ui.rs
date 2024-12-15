use crate::app::{App, AppState, DbItem};
use crate::util;
use ratatui::{prelude::*, widgets::*};

const KEY_BAR_ITEMS: &[(&str, &str)] = &[
    (" 1", "Help"),
    (" 2", "Sort"),
    (" 3", "    "),
    (" 4", "Edit"),
    (" 5", "    "),
    (" 6", "    "),
    (" 7", "Insert"),
    (" 8", "Delete"),
    (" 9", "    "),
    ("10", "Quit"),
];

pub fn render_tui(frame: &mut Frame, app: &mut App) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Min(0),
            Constraint::Length(1), // text area
            Constraint::Length(1), // key bar
        ])
        .split(frame.area());

    render_table(frame, layout[0], app);
    render_text_area(frame, layout[1], app);
    render_key_bar(frame, layout[2]);
    render_text_completion(frame, app);
}

fn render_table(frame: &mut Frame, layout: Rect, app: &mut App) {
    let header = Row::new(vec![
        Cell::from(Line::from("Id").alignment(Alignment::Center)),
        Cell::from(Line::from("Date").alignment(Alignment::Center)),
        Cell::from(Line::from("Category")),
        Cell::from(Line::from("Description")),
        Cell::from(Line::from("Price").alignment(Alignment::Center)),
    ])
    .style(Style::default().fg(Color::LightYellow));

    let mut body: Vec<Row> = Vec::new();

    for item in &app.items {
        body.push(make_table_row(item));
    }

    let mut widths = vec![
        Constraint::Length(6),                 // id
        Constraint::Length(4 + 1 + 2 + 1 + 2), // date
        Constraint::Length(0),                 // category
        Constraint::Min(0),                    // description
        Constraint::Length(5 + 1 + 2),         // price
    ];

    let div = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(widths.clone())
        .split(layout);

    // Divide width between category and description
    widths[2] = Constraint::Max(div[3].width / 3);
    widths[3] = Constraint::Min(div[3].width * 2 / 3);

    let table = Table::new(body, widths)
        .block(Block::default().borders(Borders::ALL))
        .header(header)
        .style(Style::default().fg(Color::White).bg(Color::Blue))
        .row_highlight_style(Style::default().fg(Color::Black).bg(Color::Cyan));

    frame.render_stateful_widget(table, layout, &mut app.table_state);
}

fn make_table_row<'a>(item: &DbItem) -> Row<'a> {
    let id = format!("{}", item.id);
    let price = util::format_price(item.price);

    Row::new(vec![
        Cell::from(Line::from(id).alignment(Alignment::Right)),
        Cell::from(item.date.clone()),
        Cell::from(item.category.clone()),
        Cell::from(item.description.clone()),
        Cell::from(Line::from(price).alignment(Alignment::Right)),
    ])
}

fn render_text_area(frame: &mut Frame, layout: Rect, app: &mut App) {
    let prompt = Span::from(match app.state {
        AppState::Browse => "> ",
        AppState::InsertDate => "date> ",
        AppState::InsertDescription => "desc> ",
        AppState::InsertCategory => "cat…> ",
        AppState::InsertPrice => "cost> ",
    });

    let is_valid = match app.state {
        AppState::InsertDate => util::parse_date(app.get_text()).is_some(),
        AppState::InsertPrice => util::parse_price(app.get_text()).is_some(),
        _ => true,
    };

    if is_valid {
        app.textarea.set_style(Style::default())
    } else {
        app.textarea
            .set_style(Style::default().fg(Color::Red).bold())
    }

    let div = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Length(prompt.width() as u16),
            Constraint::Min(0),
        ])
        .split(layout);

    frame.render_widget(Paragraph::new(prompt), div[0]);
    frame.render_widget(&app.textarea, div[1]);
}

fn render_key_bar(frame: &mut Frame, layout: Rect) {
    let key_style = Style::default().fg(Color::White).bg(Color::Black);
    let text_style = Style::default().fg(Color::Black).bg(Color::Cyan);

    let div = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(10)].repeat(KEY_BAR_ITEMS.len()))
        .split(layout);

    for (i, &(key, text)) in KEY_BAR_ITEMS.iter().enumerate() {
        let hint = vec![Span::styled(key, key_style), Span::from(text)];
        frame.render_widget(Paragraph::new(Line::from(hint)).style(text_style), div[i]);
    }
}

fn render_text_completion(frame: &mut Frame, app: &mut App) {
    if app.history.is_empty() || frame.area().height <= 7 {
        return;
    }

    let items: Vec<ListItem> = app
        .history
        .iter()
        .map(|li| ListItem::new(String::from(li)))
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .padding(Padding::horizontal(1)),
        )
        .style(Style::default().fg(Color::White).bg(Color::Cyan))
        .highlight_style(Style::default().fg(Color::LightYellow).bg(Color::Black));

    let frame_width = frame.area().width;
    let frame_height = frame.area().height - 2;
    let height = std::cmp::min(list.len() as u16 + 2, frame_height - 3);
    let area = Rect::new(4, frame_height - height, frame_width - 2 * 4 - 1, height);
    frame.render_widget(Clear, area);
    frame.render_stateful_widget(list, area, &mut app.list_state);
}
