pub struct App {
    // Date of items being entered.
    pub date: String,

    // Items queried from database, possibly incomplete.
    pub items: Vec<DbItem>,

    // Categories queried from database, possibly incomplete.
    pub categories: Vec<String>,

    // Descriptions queried from database.
    pub descriptions: Vec<String>,
}

impl App {
    pub fn new() -> App {
        App {
            date: String::new(),
            items: Vec::new(),
            categories: Vec::new(),
            descriptions: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct DbItem {
    pub id: i64,
    pub date: String,
    pub category: String,
    pub description: String,
    pub price: i64,
}
