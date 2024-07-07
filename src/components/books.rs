use color_eyre::eyre::Result;
use crossterm::{
    event::{KeyCode, KeyEvent},
    style::Color,
};
use ratatui::{
    prelude::*,
    style::Styled,
    symbols::border,
    widgets::{block::Title, *},
};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;

use super::{Component, Frame};
use crate::{
    action::Action,
    config::{build_status_line, Config, KeyBindings},
    dnote::{Dnote, DnoteBook},
    state::{Mode, State, StatefulList},
};

#[derive(Default)]
pub struct BooksPane {
    dnote: Dnote,
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
}

impl BooksPane {
    pub fn new() -> Self {
        Self::default()
    }

    fn mode(&self) -> Mode {
        Mode::Book
    }

    fn is_focused(&self, state: &State) -> bool {
        state.mode == self.mode()
    }

    fn get_status_line(&self) -> String {
        build_status_line(&self.config, &self.mode())
    }

    fn send_action(&self, action: Action) -> Result<()> {
        if let Some(tx) = &self.command_tx {
            tx.send(action.clone())?;
        }
        Ok(())
    }
}

impl Component for BooksPane {
    fn init(&mut self, area: Rect) -> Result<()> {
        self.send_action(Action::LoadBooks)?;
        Ok(())
    }

    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, action: Action, state: &mut State) -> Result<Option<Action>> {
        match action {
            Action::Tick => {
                if self.is_focused(state) {
                    let status_line = self.get_status_line();
                    self.send_action(Action::StatusLine(status_line))?;
                }
            }
            Action::Render => {}
            Action::FocusNext => {
                // Change to page pane
                if let Some(book_index) = state.books.state.selected() {
                    state.mode = Mode::Page;
                    self.send_action(Action::SelectNextPage)?;
                }
            }
            Action::FocusPrev => {}
            Action::LoadBooks => {
                let books = self.dnote.get_books()?;
                state.books = StatefulList::with_items(books);
            }
            Action::SelectNextBook => {
                state.select_next_book();
                self.send_action(Action::LoadActiveBookPages)?;
            }
            Action::SelectPrevBook => {
                state.select_prev_book();
                self.send_action(Action::LoadActiveBookPages)?;
            }
            Action::AddPageToActiveBook => {
                if let Some(book) = state.get_active_book() {
                    let cmd = String::from("dnote");
                    let cmd_args = vec!["add".into(), book.name.clone()];
                    self.send_action(Action::ExecuteCommand(cmd, cmd_args))?;
                    self.send_action(Action::UpdateActiveBookPages)?;
                    self.send_action(Action::LoadActivePageContent)?;
                } else {
                    log::error!("No active book to add page to");
                }
            }
            Action::DeleteActiveBook => {
                if let Some(book) = state.get_active_book() {
                    let cmd = String::from("dnote");
                    let cmd_args = vec!["remove".into(), book.name.clone()];
                    self.send_action(Action::ExecuteCommand(cmd, cmd_args))?;
                    self.send_action(Action::LoadBooks)?;
                } else {
                    log::error!("No active book to delete");
                }
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, state: &mut State) -> Result<()> {
        let items: Vec<ListItem> = state
            .books
            .items
            .iter()
            .map(|i| ListItem::new(i.name.clone()))
            .collect();
        let total_items = items.len();
        let current_item_index = match state.books.state.selected() {
            Some(v) => v + 1,
            None => 0,
        };
        let title = Title::from(" Books ".blue().bold());
        let title_bottom =
            Line::from(format!(" {} of {} ", current_item_index, total_items)).right_aligned();
        let title_padding = Line::from("");
        let border_style = match self.is_focused(state) {
            true => Style::default().blue(),
            false => Style::default(),
        };
        let block = Block::default()
            .borders(Borders::ALL)
            .border_set(border::ROUNDED)
            .style(border_style)
            .title(title_padding.clone().left_aligned())
            .title(title)
            .title_bottom(title_bottom.blue().bold())
            .title_bottom(title_padding.clone().right_aligned());
        let highlight_style = Style::default().on_black().bold();
        let list = List::new(items)
            .block(block)
            .style(Style::default().white())
            // .highlight_symbol("→ ")
            .highlight_style(highlight_style);
        f.render_stateful_widget(list, area, &mut state.books.state);
        Ok(())
    }
}
