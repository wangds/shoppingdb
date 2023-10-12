use ratatui::widgets::*;
use tui_textarea::TextArea;

pub struct App<'a> {
    pub state: AppState,

    // Cursor position in main table.
    pub table_state: TableState,

    // Cursor position in history list.
    pub list_state: ListState,

    // Text area widget for entering Date, Category, Description.
    pub textarea: TextArea<'a>,

    // Copy of DbItem being edited.
    pub item_template: Option<DbItem>,

    // Date of items being entered.
    pub date: String,

    // Category of item being entered.
    pub category: String,

    // Description of item being entered.
    pub description: String,

    // Items queried from database, possibly incomplete.
    pub items: Vec<DbItem>,

    // List of previously entered values, possibly incomplete.
    pub history: Vec<String>,

    // Categories queried from database.
    pub distinct_categories: Vec<String>,

    // Descriptions queried from database.
    pub distinct_descriptions: Vec<String>,
}

impl App<'_> {
    pub fn new<'a>() -> App<'a> {
        App {
            state: AppState::Browse,
            table_state: TableState::default(),
            list_state: ListState::default(),
            textarea: TextArea::<'a>::default(),

            item_template: None,
            date: String::new(),
            category: String::new(),
            description: String::new(),

            items: Vec::new(),
            history: Vec::new(),
            distinct_categories: Vec::new(),
            distinct_descriptions: Vec::new(),
        }
    }

    pub fn get_text<'a>(&'a self) -> &'a str {
        &self.textarea.lines()[0].trim()
    }

    pub fn update_history(&mut self) {
        let list = match self.state {
            AppState::InsertDescription => &self.distinct_descriptions,
            AppState::InsertCategory => &self.distinct_categories,
            _ => {
                if !self.history.is_empty() {
                    self.history = Vec::new();
                    self.list_state.select(None);
                }
                return;
            }
        };

        let text = self.get_text();

        self.history = list
            .iter()
            .filter(|li| li.starts_with(text))
            .cloned()
            .collect();

        if self.history.len() == 1 {
            self.list_state.select(Some(0));
        } else {
            self.list_state.select(None);
        }
    }

    pub fn transition(&mut self, state: AppState) {
        if self.state == state {
            return;
        }

        self.state = state;
        self.textarea = TextArea::default();

        match state {
            AppState::Browse => self.item_template = None,
            AppState::InsertDate => self.textarea.set_placeholder_text("yyyy-mm-dd"),
            _ => (),
        };

        self.update_history();
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AppState {
    Browse,

    // F4: Edit, F7: Insert
    InsertDate,
    InsertDescription,
    InsertCategory,
    InsertPrice,
}

#[derive(Clone, Debug)]
pub struct DbItem {
    pub id: i64,
    pub date: String,
    pub category: String,
    pub description: String,
    pub price: i64,
}
