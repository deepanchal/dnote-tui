use std::error;

use tui::widgets::ListState;

use crate::dnote_lib::{DnoteBook, DnoteClient, DnotePage, DnotePageInfo};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
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
                    self.items.len() - 1
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TuiSection {
    BOOKS,
    PAGES,
    CONTENT,
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Dnote Client instance
    pub dnote_client: DnoteClient,
    /// Order of TuiChunk
    pub selected_chunk: TuiSection,
    /// Books list
    pub books: StatefulList<DnoteBook>,
    /// Pages List
    pub pages: StatefulList<DnotePage>,
    /// Page Info
    pub page_info: DnotePageInfo,
}

impl Default for App {
    fn default() -> Self {
        let client = DnoteClient {};
        let books_result = client.get_books();
        match books_result {
            Ok(books) => Self {
                running: true,
                dnote_client: DnoteClient {},
                selected_chunk: TuiSection::BOOKS,
                books: StatefulList::with_items(books),
                pages: StatefulList::with_items(vec![]),
                page_info: DnotePageInfo {
                    content: String::from(""),
                },
            },
            Err(e) => {
                println!("Something went wrong {:?}", e);
                Self {
                    running: true,
                    dnote_client: DnoteClient {},
                    selected_chunk: TuiSection::BOOKS,
                    books: StatefulList::with_items(vec![]),
                    pages: StatefulList::with_items(vec![]),
                    page_info: DnotePageInfo {
                        content: String::from(""),
                    },
                }
            }
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn get_books(&mut self) -> StatefulList<DnoteBook> {
        let books_result = self.dnote_client.get_books();
        match books_result {
            Ok(books) => self.books.items = books,
            Err(e) => println!("Error getting books {:?}", e),
        }
        self.books.clone()
    }

    pub fn get_pages(&mut self) -> StatefulList<DnotePage> {
        let books = self.get_books();
        let selected_book_index = books.state.selected();
        if selected_book_index.is_none() {
            return self.pages.clone();
        }
        let selected_book = &books.items[selected_book_index.unwrap()];
        let dnote_pages = self.dnote_client.get_pages(&selected_book.name);
        match dnote_pages {
            Ok(pages) => {
                self.pages.items = pages;
            }
            Err(e) => {
                println!("Error getting pages {:?}", e);
            }
        }
        self.pages.clone()
    }

    pub fn get_page_content(&mut self) -> DnotePageInfo {
        let pages = self.get_pages();
        let selected_page_index = pages.state.selected();
        if selected_page_index.is_none() {
            return DnotePageInfo {
                content: String::from(""),
            };
        }
        let selected_page = &pages.items[selected_page_index.unwrap()];
        let selected_page_info = self.dnote_client.get_page_content(selected_page.id);
        match selected_page_info {
            Ok(page_info) => {
                self.page_info = page_info;
            }
            Err(e) => {
                println!("Error getting page content {:?}", e);
            }
        }
        self.page_info.clone()
    }

    pub fn select_next_chunk(&mut self) {
        let new_chunk = match self.selected_chunk {
            TuiSection::BOOKS => TuiSection::PAGES,
            TuiSection::PAGES => TuiSection::CONTENT,
            TuiSection::CONTENT => TuiSection::CONTENT,
        };
        self.selected_chunk = new_chunk
    }

    pub fn select_prev_chunk(&mut self) {
        let new_chunk = match self.selected_chunk {
            TuiSection::BOOKS => TuiSection::BOOKS,
            TuiSection::PAGES => TuiSection::BOOKS,
            TuiSection::CONTENT => TuiSection::PAGES,
        };
        self.selected_chunk = new_chunk
    }
}
