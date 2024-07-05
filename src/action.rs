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
    DeleteActivePage,
}

impl Action {
    pub fn label(&self) -> &'static str {
        match self {
            Action::Tick => "Handle periodic tick",
            Action::Render => "Render the UI",
            Action::Resize(_, _) => "Handle UI resize",
            Action::Suspend => "Suspend",
            Action::Resume => "Resume",
            Action::Quit => "Quit",
            Action::Refresh => "Refresh",
            Action::Error(_) => "Handle an error",
            Action::Help => "Show Help",
            Action::FocusNext => "Next Pane",
            Action::FocusPrev => "Prev Pane",
            Action::LoadBooks => "Load all books",
            Action::LoadActiveBookPages => "Load pages for the active book",
            Action::UpdateActiveBookPages => "Update pages for the active book",
            Action::LoadActivePageContent => "Load content for the active page",
            Action::SelectNextBook => "Down",
            Action::SelectPrevBook => "Up",
            Action::SelectNextPage => "Down",
            Action::SelectPrevPage => "Up",
            Action::AddPageToActiveBook => "Add",
            Action::EditActivePage => "Edit",
            Action::DeleteActivePage => "Delete",
        }
    }

    pub fn order(&self) -> usize {
        match self {
            Action::Quit => 80,
            Action::Help => 80,
            Action::FocusNext => 30,
            Action::FocusPrev => 30,
            Action::SelectNextBook => 10,
            Action::SelectPrevBook => 20,
            Action::SelectNextPage => 10,
            Action::SelectPrevPage => 20,
            Action::AddPageToActiveBook => 40,
            Action::EditActivePage => 50,
            Action::DeleteActivePage => 60,
            _ => 100,
        }
    }
}
