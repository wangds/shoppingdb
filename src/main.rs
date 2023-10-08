#![macro_use]
extern crate rusqlite;

mod app;
mod ui;

use crate::app::DbItem;
use ratatui::prelude::{CrosstermBackend, Terminal};
use rusqlite::{params, Connection, Result};
use tui_textarea::TextArea;

const DATABASE_FILE: &str = "shopping.db";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    // Create app and run it
    let mut app = app::App::new();
    let conn = Connection::open(DATABASE_FILE)?;
    create_database(&conn)?;

    app.items = select_items(&conn)?;
    app.categories = select_categories(&conn)?;
    app.descriptions = select_descriptions(&conn)?;

    let mut textarea = TextArea::default();

    loop {
        terminal.draw(|f| ui::render_tui(f, &app, &textarea))?;

        if crossterm::event::poll(std::time::Duration::from_millis(250))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if key.code == crossterm::event::KeyCode::F(10) {
                    break;
                }

                if key.code == crossterm::event::KeyCode::Enter {
                    let lines = textarea.into_lines();
                    insert_item(&conn, "2023", "cat", &lines[0], 100)?;

                    app.items = select_items(&conn)?;
                    app.categories = select_categories(&conn)?;
                    app.descriptions = select_descriptions(&conn)?;
                    textarea = TextArea::default();
                } else {
                    textarea.input(key);
                }
            }
        }
    }

    // Restore terminal
    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

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
