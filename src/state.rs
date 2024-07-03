use color_eyre::eyre::Result;
use ratatui::widgets::ListState;
use serde::{Deserialize, Serialize};

use crate::{
    dnote::{DnoteBook, DnotePage},
    mode::Mode,
};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    i
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    i
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct State {
    pub mode: Mode,
    pub books: StatefulList<DnoteBook>,
    pub pages: StatefulList<DnotePage>,
    pub page_content: Option<String>,
    pub status_line: String,
}

impl State {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_active_book(&self) -> Option<DnoteBook> {
        if let Some(book_index) = self.books.state.selected() {
            let selected_book = &self.books.items[book_index];
            Some(selected_book.clone())
        } else {
            None
        }
    }

    pub fn get_active_page(&self) -> Option<DnotePage> {
        if let Some(page_index) = self.pages.state.selected() {
            let selected_page = &self.pages.items[page_index];
            Some(selected_page.clone())
        } else {
            None
        }
    }

    pub fn select_next_book(&mut self) {
        self.books.next()
    }

    pub fn select_prev_book(&mut self) {
        self.books.previous()
    }

    pub fn select_next_page(&mut self) {
        self.pages.next()
    }

    pub fn select_prev_page(&mut self) {
        self.pages.previous()
    }
}
