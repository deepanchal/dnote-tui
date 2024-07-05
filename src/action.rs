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
        }
    }

    pub fn order(&self) -> usize {
        match self {
            Action::Tick => 100,
            Action::Render => 100,
            Action::Resize(_, _) => 100,
            Action::Suspend => 100,
            Action::Resume => 100,
            Action::Refresh => 100,
            Action::Error(_) => 100,
            Action::LoadBooks => 100,
            Action::LoadActiveBookPages => 100,
            Action::UpdateActiveBookPages => 100,
            Action::LoadActivePageContent => 100,
            Action::Quit => 80,
            Action::Help => 80,
            Action::FocusNext => 20,
            Action::FocusPrev => 20,
            Action::SelectNextBook => 10,
            Action::SelectPrevBook => 10,
            Action::SelectNextPage => 10,
            Action::SelectPrevPage => 10,
            Action::AddPageToActiveBook => 40,
            Action::EditActivePage => 50,
        }
    }
}
