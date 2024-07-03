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
}
