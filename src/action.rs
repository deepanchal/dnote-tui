use std::{fmt, string::ToString};

use serde::{
    de::{self, Deserializer, Visitor},
    Deserialize, Serialize,
};
use strum::Display;

use crate::dnote::DnoteBook;

type DnoteBookName = String;
type DnoteBookPageId = u32;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Display, Deserialize)]
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
    StatusLine(String),
    FocusNext,
    FocusPrev,
    LoadBooks,
    SelectNextBook,
    SelectPrevBook,
    LoadPages(DnoteBookName),
    SelectNextPage,
    SelectPrevPage,
    LoadContent(DnoteBookPageId),
    AddPageToActiveBook,
    EditPage,
}
