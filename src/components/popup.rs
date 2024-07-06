use color_eyre::eyre::Result;
use crossterm::event::{Event, KeyCode, KeyEvent};
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
    state::{InputMode, Mode, State, StatefulList},
};
use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;

#[derive(Default)]
pub struct Popup {
    title: String,
    input: Input,
    input_label: String,
    note: Option<String>,
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    on_submit: Option<Box<dyn Fn(String) -> Result<()>>>,
    on_cancel: Option<Box<dyn Fn() -> Result<()>>>,
}

impl Popup {
    pub fn new(title: String, input_label: String, note: Option<String>) -> Self {
        Self {
            title,
            input_label,
            note,
            ..Default::default()
        }
    }

    pub fn on_submit(&mut self, handler: impl Fn(String) -> Result<()> + 'static) {
        self.on_submit = Some(Box::new(handler));
    }

    pub fn on_cancel(&mut self, handler: impl Fn() -> Result<()> + 'static) {
        self.on_cancel = Some(Box::new(handler));
    }

    fn send_action(&self, action: Action) -> Result<()> {
        if let Some(tx) = &self.command_tx {
            tx.send(action.clone())?;
        }
        Ok(())
    }
}

impl Component for Popup {
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

    fn handle_key_events(
        &mut self,
        key: crossterm::event::KeyEvent,
        state: &mut State,
    ) -> Result<Option<Action>> {
        match state.input_mode {
            InputMode::Normal => Ok(None),
            InputMode::Insert => match key.code {
                KeyCode::Enter => {
                    if let Some(handler) = &self.on_submit {
                        handler(self.input.value().to_string())?;
                    }
                    Ok(Some(Action::SubmitPopup))
                }
                KeyCode::Esc => {
                    if let Some(handler) = &self.on_cancel {
                        handler()?;
                    }
                    Ok(Some(Action::ClosePopup))
                }
                _ => {
                    self.input.handle_event(&Event::Key(key));
                    Ok(None)
                }
            },
        }
    }

    fn update(&mut self, action: Action, state: &mut State) -> Result<Option<Action>> {
        match action {
            Action::Tick => {}
            Action::Render => {}
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, state: &mut State) -> Result<()> {
        // Clear the background of the popup area
        f.render_widget(Clear, area);

        // Outer block with borders
        let outer_block = Block::default()
            .borders(Borders::ALL)
            .border_set(border::ROUNDED)
            .title_top(format!("> {} <", self.title.clone()))
            .title_alignment(Alignment::Center)
            .padding(Padding::horizontal(4))
            .style(Style::default());

        f.render_widget(outer_block.clone(), area);

        // Inner layout for the contents within the border
        let inner_area = outer_block.inner(area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(1), // Title
                    Constraint::Length(3), // Input field
                    Constraint::Length(2), // Bottom text
                    self.note
                        .as_ref()
                        .map(|_| Constraint::Length(2))
                        .unwrap_or(Constraint::Min(1)), // Note if available
                ]
                .as_ref(),
            )
            .split(inner_area);

        let width = chunks[1].width.max(3) - 3; // keep 2 for borders and 1 for cursor
        let scroll = self.input.visual_scroll(width as usize);

        let input = Paragraph::new(self.input.value())
            .style(match state.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Insert => Style::default().fg(Color::Yellow),
            })
            .scroll((0, scroll as u16))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_set(border::ROUNDED)
                    .title(self.input_label.clone()),
            );

        f.render_widget(input, chunks[1]);

        let bottom_text = Paragraph::new("Press Esc to cancel, Enter to submit")
            .style(Style::default().fg(Color::DarkGray))
            .alignment(Alignment::Center);

        f.render_widget(bottom_text, chunks[2]);

        if let Some(note) = &self.note {
            let note_paragraph = Paragraph::new(note.as_str())
                .style(Style::default().fg(Color::DarkGray))
                .alignment(Alignment::Center);
            f.render_widget(note_paragraph, chunks[3]);
        }

        match state.input_mode {
            InputMode::Normal => {
                // Hide the cursor
            }
            InputMode::Insert => {
                // Make the cursor visible and put it at the specified coordinates after rendering
                f.set_cursor(
                    chunks[1].x + ((self.input.visual_cursor()).max(scroll) - scroll) as u16 + 1,
                    chunks[1].y + 1,
                );
            }
        }

        Ok(())
    }
}
