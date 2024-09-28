use color_eyre::eyre::Result;
use crossterm::event::{Event, KeyCode};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{Block, Borders, Clear, Padding, Paragraph},
};
use tokio::sync::mpsc::UnboundedSender;

use super::{Component, Frame};
use crate::{
    action::Action,
    config::Config,
    state::{InputMode, State},
};
use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;

#[derive(Default)]
pub enum PopupType {
    #[default]
    Normal,
    NewBook,
    RenameBook,
}

#[derive(Default)]
pub struct Popup {
    title: String,
    input: Input,
    input_label: String,
    note: Option<String>,
    popup_type: PopupType,
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
}

impl Popup {
    pub fn new(
        title: String,
        input_label: String,
        initial_value: String,
        note: Option<String>,
        popup_type: PopupType,
    ) -> Self {
        let input = Input::new(initial_value.clone());
        Self {
            title,
            input,
            input_label,
            note,
            popup_type,
            ..Default::default()
        }
    }

    fn send_action(&self, action: Action) -> Result<()> {
        if let Some(tx) = &self.command_tx {
            tx.send(action.clone())?;
        }
        Ok(())
    }
}

impl Component for Popup {
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
                KeyCode::Enter => match self.popup_type {
                    PopupType::NewBook => {
                        let book_name = self.input.value().to_string();
                        let cmd = String::from("dnote");
                        let cmd_args = vec!["add".into(), book_name];
                        let action = Action::ExecuteCommand(cmd, cmd_args);
                        self.send_action(action)?;
                        self.send_action(Action::ClosePopup)?;
                        self.send_action(Action::LoadBooks)?;
                        Ok(None)
                    }
                    PopupType::RenameBook => {
                        if let Some(book) = state.get_active_book() {
                            let old_name = book.name.clone();
                            let new_name = self.input.value().to_string();
                            let cmd = String::from("dnote");
                            let cmd_args = vec!["edit".into(), old_name, "-n".into(), new_name];
                            self.send_action(Action::ExecuteCommand(cmd, cmd_args))?;
                            self.send_action(Action::ClosePopup)?;
                            self.send_action(Action::LoadBooks)?;
                        }
                        Ok(None)
                    }
                    _ => Ok(None),
                },
                KeyCode::Esc => Ok(Some(Action::ClosePopup)),
                _ => {
                    self.input.handle_event(&Event::Key(key));
                    Ok(None)
                }
            },
        }
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
            .style(Style::default().blue());

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
                InputMode::Insert => Style::default().yellow(),
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
            .style(Style::default().dark_gray())
            .alignment(Alignment::Center);

        f.render_widget(bottom_text, chunks[2]);

        if let Some(note) = &self.note {
            let note_paragraph = Paragraph::new(note.as_str())
                .style(Style::default().yellow())
                .alignment(Alignment::Center);
            f.render_widget(note_paragraph, chunks[3]);
        }

        match state.input_mode {
            InputMode::Normal => {
                // Hide the cursor
            }
            InputMode::Insert => {
                // Make the cursor visible and put it at the specified coordinates after rendering
                f.set_cursor_position(Position {
                    x: chunks[1].x + ((self.input.visual_cursor()).max(scroll) - scroll) as u16 + 1,
                    y: chunks[1].y + 1,
                });
            }
        }

        Ok(())
    }
}
