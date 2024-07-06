use std::{collections::HashMap, time::Duration};

use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::*,
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
    mode::Mode,
    state::{State, StatefulList},
};

#[derive(Default)]
pub struct PagesPane {
    dnote: Dnote,
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
}

impl PagesPane {
    pub fn new() -> Self {
        Self::default()
    }

    fn mode(&self) -> Mode {
        Mode::Page
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

impl Component for PagesPane {
    fn init(&mut self, area: Rect) -> Result<()> {
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
            Action::FocusNext => {}
            Action::FocusPrev => {
                // Change to book pane
                state.mode = Mode::Book;
                state.page_content = None;
            }
            Action::LoadActiveBookPages => {
                if let Some(book) = state.get_active_book() {
                    let pages = self.dnote.get_pages(&book.name)?;
                    state.pages = StatefulList::with_items(pages);
                }
            }
            Action::UpdateActiveBookPages => {
                if let Some(book) = state.get_active_book() {
                    let new_pages = self.dnote.get_pages(&book.name)?;
                    state.update_pages(new_pages);
                }
            }
            Action::SelectNextPage => {
                state.select_next_page();
                self.send_action(Action::LoadActivePageContent)?;
            }
            Action::SelectPrevPage => {
                state.select_prev_page();
                self.send_action(Action::LoadActivePageContent)?;
            }
            Action::EditActivePage => {
                if let Some(page) = state.get_active_page() {
                    let cmd = String::from("dnote");
                    let cmd_args = vec!["edit".into(), page.id.to_string()];
                    self.send_action(Action::ExecuteCommand(cmd, cmd_args))?;
                    self.send_action(Action::UpdateActiveBookPages)?;
                    self.send_action(Action::LoadActivePageContent)?;
                } else {
                    log::error!("No active page to edit");
                }
            }
            Action::DeleteActivePage => {
                if let Some(page) = state.get_active_page() {
                    let cmd = String::from("dnote");
                    let cmd_args = vec!["remove".into(), page.id.to_string()];
                    self.send_action(Action::ExecuteCommand(cmd, cmd_args))?;
                    self.send_action(Action::FocusPrev)?;
                    self.send_action(Action::LoadActiveBookPages)?;
                } else {
                    log::error!("No active page to delete");
                }
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, state: &mut State) -> Result<()> {
        let items: Vec<ListItem> = state
            .pages
            .items
            .iter()
            .map(|i| {
                let _id = i.id.to_string();
                let _summary = i.summary.to_string();
                ListItem::new(Line::from(vec![
                    Span::styled("[", Style::default().fg(Color::White)),
                    Span::styled(_id, Style::default().fg(Color::Green)),
                    Span::styled("]", Style::default().fg(Color::White)),
                    Span::styled(format!(" {}", _summary), Style::default().fg(Color::White)),
                ]))
            })
            .collect();
        let total_items = items.len();
        let current_item_index = match state.pages.state.selected() {
            Some(v) => v + 1,
            None => 0,
        };
        let title = Title::from(" Pages ".green().bold());
        let title_bottom =
            Line::from(format!(" {current_item_index} of {total_items} ")).right_aligned();
        let title_right = match state.get_active_book() {
            Some(book) => Line::styled(
                format!("[{}]", book.name),
                Style::default().add_modifier(Modifier::ITALIC),
            ),
            None => Line::default(),
        }
        .right_aligned();
        let title_padding = Line::from("");
        let border_style = match self.is_focused(state) {
            true => Style::default().green(),
            false => Style::default(),
        };
        let block = Block::default()
            .borders(Borders::ALL)
            .border_set(border::ROUNDED)
            .style(border_style)
            .title(title_padding.clone().left_aligned())
            .title(title)
            .title(title_right)
            .title_bottom(title_bottom.green().bold())
            .title_bottom(title_padding.clone().right_aligned());
        let highlight_style = Style::default().on_black().bold();
        let list = List::new(items)
            .block(block)
            .style(Style::default().white())
            // .highlight_symbol("→ ")
            .highlight_style(highlight_style);
        f.render_stateful_widget(list, area, &mut state.pages.state);
        Ok(())
    }
}
