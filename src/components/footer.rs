use color_eyre::eyre::Result;
use ratatui::prelude::*;
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    action::Action,
    components::Component,
    config::Config,
    state::{InputMode, State},
};

#[derive(Default)]
pub struct FooterPane {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    status_line: String,
}

impl FooterPane {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for FooterPane {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, action: Action, _state: &mut State) -> Result<Option<Action>> {
        match action {
            Action::Tick => {}
            Action::StatusLine(ref s) => self.status_line.clone_from(s),
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, state: &mut State) -> Result<()> {
        let status = format!("  {}  ", self.status_line);
        let footer_line = Line::from(vec![Span::styled(status, Style::default())]).dark_gray();
        f.render_widget(footer_line, area);
        f.render_widget(
            Line::from(vec![match state.input_mode {
                InputMode::Normal => Span::from("  [Normal]  "),
                InputMode::Insert => Span::from("  [Insert]  "),
            }])
            .right_aligned(),
            area,
        );
        Ok(())
    }
}
