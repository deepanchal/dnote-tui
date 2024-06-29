use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    prelude::Rect,
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/tui-rs-revival/ratatui/tree/master/examples
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(20),
                Constraint::Percentage(70),
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
    let items = List::new(items).block(books_block).highlight_style(
        Style::default()
            .bg(Color::LightGreen)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    frame.render_stateful_widget(items, books_chunk, &mut app.books.state);

    let pages_block = Block::default()
        .title("Pages")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);

    let items: Vec<ListItem> = app
        .get_pages()
        .items
        .iter()
        .map(|i| {
            let _id = i.id.to_string();
            let _summary = i.summary.to_string();
            ListItem::new(format!("({}) {}...", _id, _summary))
                .style(Style::default().fg(Color::White))
        })
        .collect();

    // Create a List from all list items and highlight the currently selected one
    let items = List::new(items).block(pages_block).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    frame.render_stateful_widget(items, pages_chunk, &mut app.pages.state);

    let content_block = Block::default()
        .title("Content")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);
    let page_content = app.get_page_content();
    let paragraph = Paragraph::new(page_content.content)
        .style(Style::default().fg(Color::Gray))
        .block(content_block);
    frame.render_widget(paragraph, page_content_chunk);

    if app.show_popup {
        let input = Paragraph::new(Text::from(app.popup_content.as_str()))
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::ALL).title("Rename Book"));
        let area = centered_rect(60, 20, frame.size());
        frame.render_widget(Clear, area); // Clear the background
        frame.render_widget(input, area);
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
