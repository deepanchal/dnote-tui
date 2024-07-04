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
    dnote::{Dnote, DnoteBook},
    state::State,
};

#[derive(Default)]
pub struct ContentPane {
    dnote: Dnote,
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
}

impl ContentPane {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for ContentPane {
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
            Action::Tick => {}
            Action::Render => {}
            Action::FocusNext => {}
            Action::FocusPrev => {}
            Action::LoadActivePageContent => {
                if let Some(page) = state.get_active_page() {
                    let page_info = self.dnote.get_page_content(page.id)?;
                    state.page_content = Some(page_info.content);
                }
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, state: &mut State) -> Result<()> {
        let title = Title::from(" Content ".cyan().bold());
        let title_padding = Line::from("");
        // let block = Block::default().borders(Borders::ALL).title(title);
        let block = Block::default()
            .borders(Borders::ALL)
            // .padding(Padding::proportional(1))
            .border_set(border::ROUNDED)
            .title(title_padding.clone().left_aligned())
            .title(title);
        let mut paragraph = Paragraph::new("".to_string()).block(block.clone());
        if let Some(note) = &state.page_content {
            paragraph = Paragraph::new(note.to_string()).block(block.clone());
        }
        f.render_widget(paragraph, area);
        Ok(())
    }
}
