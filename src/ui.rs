use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
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
                Constraint::Percentage(20),
                Constraint::Percentage(60),
            ]
            .as_ref(),
        )
        .split(frame.size());
    let books_chunk = chunks[0];
    let pages_chunk = chunks[1];
    let page_content_chunk = chunks[2];

    let books_block = Block::default()
        .title("Books")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);

    let items: Vec<ListItem> = app
        .get_books()
        .items
        .iter()
        .map(|i| ListItem::new(i.name.to_string()).style(Style::default().fg(Color::White)))
        .collect();

    // Create a List from all list items and highlight the currently selected one
    let items = List::new(items)
        .block(books_block)
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    frame.render_stateful_widget(items, books_chunk, &mut app.books.state);

    let pages_block = Block::default()
        .title("Pages")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);

    let items: Vec<ListItem> = app
        .get_pages()
        .items
        .iter()
        .map(|i| ListItem::new(i.id.to_string()).style(Style::default().fg(Color::White)))
        .collect();

    // Create a List from all list items and highlight the currently selected one
    let items = List::new(items)
        .block(pages_block)
        .highlight_style(
            Style::default()
                .bg(Color::LightBlue)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    frame.render_stateful_widget(items, pages_chunk, &mut app.pages.state);

    let content_block = Block::default()
        .title("Content")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);
    let page_content = app.get_page_content();
    let paragraph = Paragraph::new(page_content.content.clone())
        .style(Style::default().fg(Color::Gray))
        .block(content_block);
    frame.render_widget(paragraph, page_content_chunk);
}
