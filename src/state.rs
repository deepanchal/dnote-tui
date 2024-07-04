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

    pub fn select_book(&mut self, book: DnoteBook) {
        if let Some(index) = self.books.items.iter().position(|b| *b == book) {
            self.books.state.select(Some(index));
        }
    }

    pub fn select_page(&mut self, page: DnotePage) {
        if let Some(index) = self.pages.items.iter().position(|p| *p == page) {
            self.pages.state.select(Some(index));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stateful_list_with_items_creates_list_with_given_items() {
        let items = vec![1, 2, 3];
        let list = StatefulList::with_items(items.clone());
        assert_eq!(list.items, items);
        assert_eq!(list.state.selected(), None);
    }

    #[test]
    fn stateful_list_next_wraps_around() {
        let items = vec![1, 2, 3];
        let mut list = StatefulList::with_items(items);

        list.next();
        assert_eq!(list.state.selected(), Some(0));

        list.next();
        list.next();
        list.next();
        assert_eq!(list.state.selected(), Some(2));

        list.next();
        assert_eq!(list.state.selected(), Some(2));
    }

    #[test]
    fn stateful_list_previous_wraps_around() {
        let items = vec![1, 2, 3];
        let mut list = StatefulList::with_items(items);

        list.previous();
        assert_eq!(list.state.selected(), Some(0));

        list.next();
        list.next();
        list.previous();
        assert_eq!(list.state.selected(), Some(1));

        list.previous();
        assert_eq!(list.state.selected(), Some(0));

        list.previous();
        assert_eq!(list.state.selected(), Some(0));
    }

    #[test]
    fn stateful_list_unselect_clears_selection() {
        let items = vec![1, 2, 3];
        let mut list = StatefulList::with_items(items);

        list.next();
        assert_eq!(list.state.selected(), Some(0));

        list.unselect();
        assert_eq!(list.state.selected(), None);
    }

    #[test]
    fn state_new_creates_default_state() {
        let state = State::new();
        assert_eq!(state, State::default());
    }

    #[test]
    fn state_get_active_book_returns_selected_book() {
        let books = vec![
            DnoteBook {
                name: String::from("Book1"),
            },
            DnoteBook {
                name: String::from("Book2"),
            },
        ];
        let mut state = State::new();
        state.books = StatefulList::with_items(books.clone());

        state.books.state.select(Some(1));
        assert_eq!(state.get_active_book(), Some(books[1].clone()));

        state.books.state.select(Some(0));
        assert_eq!(state.get_active_book(), Some(books[0].clone()));

        state.books.state.select(None);
        assert_eq!(state.get_active_book(), None);
    }

    #[test]
    fn state_get_active_page_returns_selected_page() {
        let pages = vec![
            DnotePage {
                id: 1,
                summary: String::from("Page1"),
            },
            DnotePage {
                id: 2,
                summary: String::from("Page2"),
            },
        ];
        let mut state = State::new();
        state.pages = StatefulList::with_items(pages.clone());

        state.pages.state.select(Some(1));
        assert_eq!(state.get_active_page(), Some(pages[1].clone()));

        state.pages.state.select(Some(0));
        assert_eq!(state.get_active_page(), Some(pages[0].clone()));

        state.pages.state.select(None);
        assert_eq!(state.get_active_page(), None);
    }

    #[test]
    fn state_select_next_book_navigates_correctly() {
        let books = vec![
            DnoteBook {
                name: String::from("Book1"),
            },
            DnoteBook {
                name: String::from("Book2"),
            },
        ];
        let mut state = State::new();
        state.books = StatefulList::with_items(books);

        state.select_next_book();
        assert_eq!(state.books.state.selected(), Some(0));

        state.select_next_book();
        assert_eq!(state.books.state.selected(), Some(1));

        state.select_next_book();
        assert_eq!(state.books.state.selected(), Some(1));
    }

    #[test]
    fn state_select_prev_book_navigates_correctly() {
        let books = vec![
            DnoteBook {
                name: String::from("Book1"),
            },
            DnoteBook {
                name: String::from("Book2"),
            },
        ];
        let mut state = State::new();
        state.books = StatefulList::with_items(books);

        state.select_prev_book();
        assert_eq!(state.books.state.selected(), Some(0));

        state.select_prev_book();
        assert_eq!(state.books.state.selected(), Some(0));
    }

    #[test]
    fn state_select_next_page_navigates_correctly() {
        let pages = vec![
            DnotePage {
                id: 1,
                summary: String::from("Page1"),
            },
            DnotePage {
                id: 2,
                summary: String::from("Page2"),
            },
        ];
        let mut state = State::new();
        state.pages = StatefulList::with_items(pages);

        state.select_next_page();
        assert_eq!(state.pages.state.selected(), Some(0));

        state.select_next_page();
        assert_eq!(state.pages.state.selected(), Some(1));

        state.select_next_page();
        assert_eq!(state.pages.state.selected(), Some(1));
    }

    #[test]
    fn state_select_prev_page_navigates_correctly() {
        let pages = vec![
            DnotePage {
                id: 1,
                summary: String::from("Page1"),
            },
            DnotePage {
                id: 2,
                summary: String::from("Page2"),
            },
        ];
        let mut state = State::new();
        state.pages = StatefulList::with_items(pages);

        state.select_prev_page();
        assert_eq!(state.pages.state.selected(), Some(0));

        state.select_prev_page();
        assert_eq!(state.pages.state.selected(), Some(0));
    }
}
