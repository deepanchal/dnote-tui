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
    config::{Config, KeyBindings},
    dnote::DnoteBook,
    state::State,
};

#[derive(Default)]
pub struct BooksPane {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
}

impl BooksPane {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for BooksPane {
    fn init(&mut self, area: Rect) -> Result<()> {
        if let Some(tx) = &self.command_tx {
            tx.send(Action::LoadBooks)?;
        }
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

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Tick => {}
            Action::Render => {}
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
        let block = Block::default()
            .borders(Borders::ALL)
            .border_set(border::ROUNDED)
            .title(title_padding.clone().left_aligned())
            .title(title)
            .title_bottom(title_bottom.blue().bold())
            .title_bottom(title_padding.clone().right_aligned());
        let highlight_style = Style::default().on_black().blue().bold();
        let list = List::new(items)
            .block(block)
            .highlight_style(highlight_style);
        f.render_stateful_widget(list, area, &mut state.books.state);
        Ok(())
    }
}
