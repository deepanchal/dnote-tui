use color_eyre::eyre::Result;
use serde::{Deserialize, Serialize};

use crate::dnote::{DnoteBook, DnotePage};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct State {
    active_book: Option<DnoteBook>,
    active_page: Option<DnotePage>,
}

impl State {
    pub fn new() -> Self {
        Self::default()
    }
}
