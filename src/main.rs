#![macro_use]
extern crate rusqlite;

use rusqlite::{params, Connection, Result};

const DATABASE_FILE: &str = "shopping.db";

fn main() -> Result<()> {
    let conn = Connection::open(DATABASE_FILE)?;

    create_database(&conn)?;
    insert_item(&conn, "2023", "cat", "dog", 100)?;

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
