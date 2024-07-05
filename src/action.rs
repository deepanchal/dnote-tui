use std::{fmt, string::ToString};

use serde::{
    de::{self, Deserializer, Visitor},
    Deserialize, Serialize,
};
use strum::Display;

use crate::dnote::DnoteBook;

type DnoteBookName = String;
type DnoteBookPageId = u32;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Display, Deserialize)]
pub enum Action {
    Tick,
    Render,
    Resize(u16, u16),
    Suspend,
    Resume,
    Quit,
    Refresh,
    Error(String),
    Help,
    FocusNext,
    FocusPrev,
    LoadBooks,
    LoadActiveBookPages,
    UpdateActiveBookPages,
    LoadActivePageContent,
    SelectNextBook,
    SelectPrevBook,
    SelectNextPage,
    SelectPrevPage,
    AddPageToActiveBook,
    EditActivePage,
}
