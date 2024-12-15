#![macro_use]
extern crate rusqlite;

mod app;
mod ui;
mod util;

use crate::app::{App, AppState, DbItem};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::{CrosstermBackend, Terminal};
use rusqlite::{params, Connection, Result};
use tui_textarea::CursorMove;

const DATABASE_FILE: &str = "shopping.db";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    // Create app and run it
    let mut app = App::new();
    let mut conn = Connection::open(DATABASE_FILE)?;
    create_database(&conn)?;

    app.items = select_items(&conn)?;
    app.distinct_categories = select_categories(&conn)?;
    app.distinct_descriptions = select_descriptions(&conn)?;
    app.table_state.select(navigate_home(&app.items));

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
                    AppState::Browse => main_browse(&mut app, key, &mut conn)?,
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

fn main_browse(app: &mut App, key: KeyEvent, conn: &mut Connection) -> Result<()> {
    if key.code == KeyCode::Up {
        app.table_state
            .select(navigate_up(&app.items, app.table_state.selected(), 1));
    } else if key.code == KeyCode::Down {
        app.table_state
            .select(navigate_down(&app.items, app.table_state.selected(), 1));
    } else if key.code == KeyCode::PageUp {
        app.table_state
            .select(navigate_up(&app.items, app.table_state.selected(), 10));
    } else if key.code == KeyCode::PageDown {
        app.table_state
            .select(navigate_down(&app.items, app.table_state.selected(), 10));
    } else if key.code == KeyCode::Home {
        app.table_state.select(navigate_home(&app.items));
    } else if key.code == KeyCode::End {
        app.table_state.select(navigate_end(&app.items));
    } else if key.code == KeyCode::F(2) {
        sort_items(conn)?;
        app.items = select_items(conn)?;
    } else if key.code == KeyCode::F(4) {
        if let Some(i) = app.table_state.selected() {
            app.item_template = Some(app.items[i].clone());
            app.transition(AppState::InsertDate);
            app.textarea.insert_str(&app.items[i].date);
        }
    } else if key.code == KeyCode::F(7) {
        app.transition(AppState::InsertDate);
        app.textarea.insert_str(util::today());
    } else if key.code == KeyCode::F(8) {
        if let Some(i) = app.table_state.selected() {
            delete_item(conn, app.items[i].id)?;

            app.items = select_items(conn)?;
            app.distinct_categories = select_categories(conn)?;
            app.distinct_descriptions = select_descriptions(conn)?;
            app.table_state
                .select(navigate_down(&app.items, app.table_state.selected(), 0));
        }
    }

    Ok(())
}

fn main_insert_date(app: &mut App, key: KeyEvent) {
    if key.code == KeyCode::Enter {
        let line = app.get_text();
        if line.is_empty() {
            app.transition(AppState::Browse);
        } else if let Some(date) = util::parse_date(line) {
            app.new_item.date = date.format("%F").to_string();

            app.transition(AppState::InsertDescription);
            if let Some(item) = &app.item_template {
                app.textarea.insert_str(&item.description);
            }
            app.update_history();
        }
    } else {
        app.textarea.input(key);
    }
}

fn main_insert_description(app: &mut App, key: KeyEvent, conn: &Connection) -> Result<()> {
    if handle_history_input(app, key) {
        return Ok(());
    }

    if key.code == KeyCode::Enter {
        let line = app.get_text();
        if line.is_empty() {
            app.transition(AppState::Browse);
        } else {
            app.new_item.description = String::from(line);

            app.transition(AppState::InsertCategory);
            if let Some(item) = &app.item_template {
                app.textarea.insert_str(&item.category);
                app.update_history();
            } else if let Ok(autofill) = select_category(conn, &app.new_item.description) {
                app.textarea.insert_str(autofill);
                app.update_history();
            }
        }
    } else {
        app.textarea.input(key);
        app.update_history();
    }

    Ok(())
}

fn main_insert_category(app: &mut App, key: KeyEvent) {
    if handle_history_input(app, key) {
        return;
    }

    if key.code == KeyCode::Enter {
        let line = app.get_text();
        app.new_item.category = String::from(line);

        app.transition(AppState::InsertPrice);
        if let Some(item) = &app.item_template {
            app.textarea.insert_str(util::format_price(item.price));
        }
    } else {
        app.textarea.input(key);
        app.update_history();
    }
}

fn main_insert_price(app: &mut App, key: KeyEvent, conn: &Connection) -> Result<()> {
    if key.code == KeyCode::Enter {
        let line = app.get_text();
        if let Some(price) = util::parse_price(line) {
            let rowid: i64;
            app.new_item.price = price;

            if let Some(item) = &app.item_template {
                rowid = item.id;
                update_item(conn, rowid, &app.new_item)?;
            } else {
                rowid = insert_item(conn, &app.new_item)?;
            }

            app.items = select_items(conn)?;
            app.distinct_categories = select_categories(conn)?;
            app.distinct_descriptions = select_descriptions(conn)?;
            app.table_state
                .select(app.items.iter().position(|item| item.id == rowid));

            if app.item_template.is_some() {
                app.transition(AppState::Browse);
            } else {
                app.transition(AppState::InsertDescription);
            }
        }
    } else {
        app.textarea.input(key);
    }

    Ok(())
}

fn handle_history_input(app: &mut App, key: KeyEvent) -> bool {
    if key.code == KeyCode::Up {
        app.list_state
            .select(navigate_up(&app.history, app.list_state.selected(), 1));
    } else if key.code == KeyCode::Down {
        app.list_state
            .select(navigate_down(&app.history, app.list_state.selected(), 1));
    } else if key.code == KeyCode::PageUp {
        app.list_state
            .select(navigate_up(&app.history, app.list_state.selected(), 10));
    } else if key.code == KeyCode::PageDown {
        app.list_state
            .select(navigate_down(&app.history, app.list_state.selected(), 10));
    } else if key.code == KeyCode::Home {
        app.list_state.select(navigate_home(&app.history));
    } else if key.code == KeyCode::End {
        app.list_state.select(navigate_end(&app.history));
    } else if key.code == KeyCode::Tab {
        if let Some(i) = app.list_state.selected() {
            if i < app.history.len() {
                let text = String::from(&app.history[i]);
                app.textarea.move_cursor(CursorMove::Head);
                app.textarea.delete_line_by_end();
                app.textarea.insert_str(text);
                app.update_history();
            }
        }
    } else {
        return false;
    }

    true
}

fn navigate_up<T>(list: &Vec<T>, selected: Option<usize>, delta: usize) -> Option<usize> {
    if list.is_empty() {
        None
    } else {
        match selected {
            Some(i) => {
                if i <= delta {
                    Some(0)
                } else {
                    Some(i - delta)
                }
            }
            None => Some(list.len() - 1),
        }
    }
}

fn navigate_down<T>(list: &Vec<T>, selected: Option<usize>, delta: usize) -> Option<usize> {
    if list.is_empty() {
        None
    } else {
        match selected {
            Some(i) => {
                if i + delta >= list.len() - 1 {
                    Some(list.len() - 1)
                } else {
                    Some(i + delta)
                }
            }
            None => Some(0),
        }
    }
}

fn navigate_home<T>(list: &Vec<T>) -> Option<usize> {
    if list.is_empty() {
        None
    } else {
        Some(0)
    }
}

fn navigate_end<T>(list: &Vec<T>) -> Option<usize> {
    if list.is_empty() {
        None
    } else {
        Some(list.len() - 1)
    }
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

fn insert_item(conn: &Connection, item: &DbItem) -> Result<i64> {
    let mut stmt = conn
        .prepare("INSERT INTO items(date, category, description, price) values (?1, ?2, ?3, ?4)")?;

    stmt.execute(params![
        item.date,
        item.category,
        item.description,
        item.price
    ])?;

    Ok(conn.last_insert_rowid())
}

fn update_item(conn: &Connection, id: i64, item: &DbItem) -> Result<()> {
    conn.execute(
        "UPDATE items SET date=?1, category=?2, description=?3, price=?4 WHERE id=?5",
        params![item.date, item.category, item.description, item.price, id],
    )?;

    Ok(())
}

fn delete_item(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("DELETE FROM items WHERE id=?1", params![id])?;

    Ok(())
}

fn sort_items(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;

    tx.execute("ALTER TABLE items RENAME TO items2", ())?;

    tx.execute(
        "CREATE TABLE IF NOT EXISTS items (
             id INTEGER PRIMARY KEY,
             date TEXT NOT NULL,
             category TEXT NOT NULL,
             description TEXT NOT NULL,
             price INTEGER NOT NULL
        )",
        (),
    )?;

    tx.execute(
        "INSERT INTO items(date, category, description, price)
            SELECT date, category, description, price
            FROM items2
            ORDER BY date",
        (),
    )?;

    tx.execute("DROP TABLE items2", ())?;

    tx.commit()?;

    conn.execute("VACUUM", ())?;

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
    let mut stmt = conn.prepare("SELECT DISTINCT category FROM items ORDER BY category")?;
    let mut rows = stmt.query([])?;
    let mut categories = Vec::new();

    while let Some(row) = rows.next()? {
        categories.push(row.get(0)?);
    }

    Ok(categories)
}

fn select_descriptions(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT DISTINCT description FROM items ORDER BY description")?;
    let mut rows = stmt.query([])?;
    let mut descriptions = Vec::new();

    while let Some(row) = rows.next()? {
        descriptions.push(row.get(0)?);
    }

    Ok(descriptions)
}
