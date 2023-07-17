use crate::app::{App, AppResult, TuiSection};
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
            app.pages.unselect();
            app.select_prev_chunk();
            match app.selected_chunk {
                TuiSection::BOOKS => {}
                TuiSection::PAGES => app.books.previous(),
                TuiSection::CONTENT => app.pages.previous(),
            }
        }
        KeyCode::Right | KeyCode::Char('l') => {
            match app.selected_chunk {
                TuiSection::BOOKS => app.pages.next(),
                TuiSection::PAGES => {}
                _ => {}
            }
            app.select_next_chunk();
        }
        KeyCode::Up | KeyCode::Char('k') => match app.selected_chunk {
            TuiSection::BOOKS => app.books.previous(),
            TuiSection::PAGES => app.pages.previous(),
            _ => {}
        },
        KeyCode::Down | KeyCode::Char('j') => match app.selected_chunk {
            TuiSection::BOOKS => app.books.next(),
            TuiSection::PAGES => app.pages.next(),
            _ => {}
        },
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
