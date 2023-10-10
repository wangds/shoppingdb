use ratatui::widgets::*;
use tui_textarea::TextArea;

pub struct App<'a> {
    pub state: AppState,

    // Table state.
    pub table_state: TableState,

    // Text area widget for entering Date, Category, Description.
    pub textarea: TextArea<'a>,

    // Date of items being entered.
    pub date: String,

    // Category of item being entered.
    pub category: String,

    // Description of item being entered.
    pub description: String,

    // Items queried from database, possibly incomplete.
    pub items: Vec<DbItem>,

    // Categories queried from database, possibly incomplete.
    pub distinct_categories: Vec<String>,

    // Descriptions queried from database, possibly incomplete.
    pub distinct_descriptions: Vec<String>,
}

impl App<'_> {
    pub fn new<'a>() -> App<'a> {
        App {
            state: AppState::Browse,
            table_state: TableState::default(),
            textarea: TextArea::<'a>::default(),

            date: String::new(),
            category: String::new(),
            description: String::new(),

            items: Vec::new(),
            distinct_categories: Vec::new(),
            distinct_descriptions: Vec::new(),
        }
    }

    pub fn select_first(&mut self) {
        if self.items.len() == 0 {
            self.table_state.select(None);
        } else {
            self.table_state.select(Some(0));
        }
    }

    pub fn select_last(&mut self) {
        if self.items.len() == 0 {
            self.table_state.select(None);
        } else {
            self.table_state.select(Some(self.items.len() - 1));
        }
    }

    pub fn select_prev(&mut self, delta: usize) {
        if self.items.len() == 0 {
            self.table_state.select(None);
        } else {
            let i = match self.table_state.selected() {
                Some(i) => {
                    if i <= delta {
                        0
                    } else {
                        i - delta
                    }
                }
                None => 0,
            };
            self.table_state.select(Some(i));
        }
    }

    pub fn select_next(&mut self, delta: usize) {
        if self.items.len() == 0 {
            self.table_state.select(None);
        } else {
            let i = match self.table_state.selected() {
                Some(i) => {
                    if i + delta >= self.items.len() - 1 {
                        self.items.len() - 1
                    } else {
                        i + delta
                    }
                }
                None => 0,
            };
            self.table_state.select(Some(i));
        }
    }

    pub fn get_text<'a>(&'a self) -> &'a str {
        &self.textarea.lines()[0].trim()
    }

    pub fn transition(&mut self, state: AppState) {
        if self.state == state {
            return;
        }

        self.state = state;
        self.textarea = TextArea::default();

        match state {
            AppState::InsertDate => self.textarea.set_placeholder_text("yyyy-mm-dd"),
            _ => (),
        };
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AppState {
    Browse,
    InsertDate,
    InsertDescription,
    InsertCategory,
    InsertPrice,
}

#[derive(Debug)]
pub struct DbItem {
    pub id: i64,
    pub date: String,
    pub category: String,
    pub description: String,
    pub price: i64,
}
