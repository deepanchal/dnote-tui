use crate::app::{App, AppResult, TuiSection};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    if app.show_popup {
        match key_event.code {
            KeyCode::Char(c) => {
                app.popup_content.push(c);
            }
            KeyCode::Backspace => {
                app.popup_content.pop();
            }
            KeyCode::Enter => {
                if app.show_popup {
                    let selected_index = app.books.state.selected().unwrap_or(0);
                    let old_name = &app.books.items[selected_index].name;
                    let new_name = &app.popup_content;
                    if let Err(e) = app.dnote_client.rename_book(old_name, new_name) {
                        println!("Error renaming book: {:?}", e);
                        // Handle error (e.g., show an error message to the user)
                    } else {
                        // Update the book's name in the UI
                        app.books.items[selected_index].name.clone_from(new_name);
                        app.show_popup = false;
                    }
                }
            }
            KeyCode::Esc => {
                app.show_popup = false;
            }
            _ => {}
        }
    } else {
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
                app.select_prev_section();
                match app.selected_section {
                    TuiSection::BOOKS => {}
                    TuiSection::PAGES => app.books.previous(),
                    TuiSection::CONTENT => app.pages.previous(),
                }
            }
            KeyCode::Right | KeyCode::Char('l') => {
                match app.selected_section {
                    TuiSection::BOOKS => app.pages.next(),
                    TuiSection::PAGES => {}
                    _ => {}
                }
                app.select_next_section();
            }
            KeyCode::Up | KeyCode::Char('k') => match app.selected_section {
                TuiSection::BOOKS => app.books.previous(),
                TuiSection::PAGES => app.pages.previous(),
                _ => {}
            },
            KeyCode::Down | KeyCode::Char('j') => match app.selected_section {
                TuiSection::BOOKS => app.books.next(),
                TuiSection::PAGES => app.pages.next(),
                _ => {}
            },
            KeyCode::Char('r') => {
                if app.selected_section == TuiSection::BOOKS {
                    app.show_popup = true;
                    if app.show_popup {
                        app.popup_content.clone_from(&app.books.items
                            [app.books.state.selected().unwrap_or(0)]
                        .name);
                    }
                }
            }
            // Other handlers you could add here.
            _ => {}
        }
    }
    Ok(())
}
