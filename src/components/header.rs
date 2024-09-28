use color_eyre::eyre::Result;
use ratatui::{prelude::*, widgets::Paragraph};

use crate::{
    components::Component,
    config::{PROJECT_NAME, PROJECT_VERSION},
    state::State,
};

#[derive(Default)]
pub struct HeaderPane {}

impl HeaderPane {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for HeaderPane {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, _state: &mut State) -> Result<()> {
        let app_name = PROJECT_NAME.to_string();
        let app_version = PROJECT_VERSION.to_string();
        let line = Line::from(vec![
            Span::styled(
                format!("[ {} {} ", app_name, symbols::DOT),
                Style::default().fg(Color::Blue),
            ),
            Span::styled(
                format!("v{} ", app_version),
                Style::default().fg(Color::LightCyan),
            ),
            Span::styled("]", Style::default().fg(Color::Blue)),
        ]);
        let text = Text::from(vec![
            Line::from("").centered(),
            line.clone(),
            Line::from("").centered(),
        ])
        .centered();
        let paragraph = Paragraph::new(text).centered();
        f.render_widget(paragraph, area);

        Ok(())
    }
}
