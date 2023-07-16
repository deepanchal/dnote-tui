use crate::app::{App, AppResult, TuiChunk};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        // Counter handlers
        KeyCode::Left | KeyCode::Char('h') => {
            app.select_prev_chunk();
            match app.selected_chunk {
                TuiChunk::BOOKS => {},
                TuiChunk::PAGES => app.books.previous(),
                TuiChunk::CONTENT => app.pages.previous(),
            }
        }
        KeyCode::Right | KeyCode::Char('l') => {
            app.select_next_chunk();
            match app.selected_chunk {
                TuiChunk::BOOKS => app.pages.next(),
                TuiChunk::PAGES => {},
                _ => {}
            }
        }
        KeyCode::Up | KeyCode::Char('k') => match app.selected_chunk {
            TuiChunk::BOOKS => app.books.previous(),
            TuiChunk::PAGES => app.pages.previous(),
            _ => {}
        },
        KeyCode::Down | KeyCode::Char('j') => match app.selected_chunk {
            TuiChunk::BOOKS => app.books.next(),
            TuiChunk::PAGES => app.pages.next(),
            _ => {}
        },
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
