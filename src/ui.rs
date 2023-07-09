use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render<B: Backend>(_app: &mut App, frame: &mut Frame<'_, B>) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/tui-rs-revival/ratatui/tree/master/examples
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(30),
                Constraint::Percentage(50),
            ]
            .as_ref(),
        )
        .split(frame.size());
    let books_block = Block::default()
        .title("Books")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);
    frame.render_widget(books_block, chunks[0]);

    let pages_block = Block::default()
        .title("Pages")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);
    frame.render_widget(pages_block, chunks[1]);

    let content_block = Block::default()
        .title("Content")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);
    frame.render_widget(content_block, chunks[2]);
}
