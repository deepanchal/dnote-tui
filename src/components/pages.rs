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
pub struct PagesPane {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
}

impl PagesPane {
    pub fn new() -> Self {
        Self::default()
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

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Tick => {}
            Action::Render => {}
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, state: &mut State) -> Result<()> {
        let title = Title::from(" Pages ".green().bold());
        // let block = Block::default().borders(Borders::ALL).title(title);
        let block = Block::default()
            .borders(Borders::ALL)
            // .padding(Padding::proportional(1))
            .border_set(border::ROUNDED)
            .title(title);
        let items: Vec<ListItem> = state
            .pages
            .items
            .iter()
            .map(|i| {
                let _id = i.id.to_string();
                let _summary = i.summary.to_string();
                ListItem::new(format!("({}) {}...", _id, _summary)).white()
            })
            .collect();
        let highlight_style = Style::default().on_black().green().bold();
        let list = List::new(items)
            .block(block)
            .highlight_style(highlight_style);
        f.render_stateful_widget(list, area, &mut state.pages.state);
        Ok(())
    }
}
