#![macro_use]
extern crate rusqlite;

mod app;
mod ui;
mod util;

use crate::app::{App, AppState, DbItem};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::{CrosstermBackend, Terminal};
use rusqlite::{params, Connection, Result};

const DATABASE_FILE: &str = "shopping.db";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    // Create app and run it
    let mut app = App::new();
    let conn = Connection::open(DATABASE_FILE)?;
    create_database(&conn)?;

    app.items = select_items(&conn)?;
    app.distinct_categories = select_categories(&conn)?;
    app.distinct_descriptions = select_descriptions(&conn)?;

    loop {
        terminal.draw(|f| ui::render_tui(f, &mut app))?;

        if crossterm::event::poll(std::time::Duration::from_millis(250))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if key.code == KeyCode::F(10) {
                    break;
                } else if key.code == KeyCode::Esc {
                    app.transition(AppState::Browse);
                    continue;
                }

                match app.state {
                    AppState::Browse => main_browse(&mut app, key),
                    AppState::InsertDate => main_insert_date(&mut app, key),
                    AppState::InsertDescription => main_insert_description(&mut app, key, &conn)?,
                    AppState::InsertCategory => main_insert_category(&mut app, key),
                    AppState::InsertPrice => main_insert_price(&mut app, key, &conn)?,
                };
            }
        }
    }

    // Restore terminal
    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}

fn main_browse(app: &mut App, key: KeyEvent) {
    if key.code == KeyCode::F(7) {
        app.transition(AppState::InsertDate);
    }
}

fn main_insert_date<'a>(app: &mut App<'a>, key: KeyEvent) {
    if key.code == KeyCode::Enter {
        let line = app.get_text();
        if line.len() == 0 {
            app.transition(AppState::Browse);
        } else if let Some(date) = util::parse_date(&line) {
            app.date = date.format("%F").to_string();
            app.transition(AppState::InsertDescription);
        }
    } else {
        app.textarea.input(key);
    }
}

fn main_insert_description<'a>(app: &mut App<'a>, key: KeyEvent, conn: &Connection) -> Result<()> {
    if key.code == KeyCode::Enter {
        let line = app.get_text();
        if line.len() == 0 {
            app.transition(AppState::Browse);
        } else {
            app.description = String::from(line);
            app.transition(AppState::InsertCategory);
            if let Ok(autofill) = select_category(&conn, &app.description) {
                app.textarea.insert_str(autofill);
            }
        }
    } else {
        app.textarea.input(key);
    }

    Ok(())
}

fn main_insert_category<'a>(app: &mut App<'a>, key: KeyEvent) {
    if key.code == KeyCode::Enter {
        let line = app.get_text();
        app.category = String::from(line);
        app.transition(AppState::InsertPrice);
    } else {
        app.textarea.input(key);
    }
}

fn main_insert_price<'a>(app: &mut App<'a>, key: KeyEvent, conn: &Connection) -> Result<()> {
    if key.code == KeyCode::Enter {
        let line = app.get_text();
        if let Some(price) = util::parse_price(&line) {
            insert_item(&conn, &app.date, &app.category, &app.description, price)?;

            app.items = select_items(&conn)?;
            app.distinct_categories = select_categories(&conn)?;
            app.distinct_descriptions = select_descriptions(&conn)?;
            app.transition(AppState::InsertDescription);
        }
    } else {
        app.textarea.input(key);
    }

    Ok(())
}

fn create_database(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS items (
             id INTEGER PRIMARY KEY,
             date TEXT NOT NULL,
             category TEXT NOT NULL,
             description TEXT NOT NULL,
             price INTEGER NOT NULL
         )",
        (),
    )?;

    Ok(())
}

fn insert_item(
    conn: &Connection,
    date: &str,
    category: &str,
    description: &str,
    price: i64,
) -> Result<()> {
    let mut stmt = conn
        .prepare("INSERT INTO items(date, category, description, price) values (?1, ?2, ?3, ?4)")?;

    stmt.execute(params![date, category, description, price])?;

    Ok(())
}

fn select_items(conn: &Connection) -> Result<Vec<DbItem>> {
    let mut stmt = conn.prepare("SELECT id, date, category, description, price FROM items")?;
    let iter = stmt.query_map([], |row| {
        Ok(DbItem {
            id: row.get(0)?,
            date: row.get(1)?,
            category: row.get(2)?,
            description: row.get(3)?,
            price: row.get(4)?,
        })
    })?;

    Ok(iter.map(|item| item.unwrap()).collect())
}

fn select_category(conn: &Connection, description: &str) -> Result<String> {
    conn.query_row(
        "SELECT category FROM items WHERE description=?1 LIMIT 1",
        params![description],
        |row| row.get(0),
    )
}

fn select_categories(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT DISTINCT category FROM items")?;
    let mut rows = stmt.query([])?;
    let mut categories = Vec::new();

    while let Some(row) = rows.next()? {
        categories.push(row.get(0)?);
    }

    Ok(categories)
}

fn select_descriptions(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT DISTINCT description FROM items")?;
    let mut rows = stmt.query([])?;
    let mut descriptions = Vec::new();

    while let Some(row) = rows.next()? {
        descriptions.push(row.get(0)?);
    }

    Ok(descriptions)
}
