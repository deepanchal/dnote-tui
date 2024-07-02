use color_eyre::eyre::Result;
use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::Rect,
};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::{
    action::Action,
    components::{books::BooksPane, content::ContentPane, pages::PagesPane, Component},
    config::Config,
    dnote::Dnote,
    mode::Mode,
    state::{State, StatefulList},
    tui,
};

pub struct App {
    pub config: Config,
    pub tick_rate: f64,
    pub frame_rate: f64,
    pub components: Vec<Box<dyn Component>>,
    // pub header: Box<dyn Component>,
    // pub footer: Box<dyn Component>,
    pub should_quit: bool,
    pub should_suspend: bool,
    pub mode: Mode,
    pub last_tick_key_events: Vec<KeyEvent>,
    pub dnote: Dnote,
    pub state: State,
}

impl App {
    pub fn new(tick_rate: f64, frame_rate: f64) -> Result<Self> {
        let state = State::new();
        let dnote = Dnote::new();
        let books = BooksPane::default();
        let pages = PagesPane::default();
        let content = ContentPane::default();
        let config = Config::new()?;
        let mode = Mode::Book;
        Ok(Self {
            tick_rate,
            frame_rate,
            components: vec![
                // Note: ordering is important as layout constraints draw components in order
                Box::new(books),
                Box::new(pages),
                Box::new(content),
            ],
            should_quit: false,
            should_suspend: false,
            config,
            mode,
            last_tick_key_events: Vec::new(),
            dnote,
            state,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        let (action_tx, mut action_rx) = mpsc::unbounded_channel();

        let mut tui = tui::Tui::new()?
            .tick_rate(self.tick_rate)
            .frame_rate(self.frame_rate);
        // tui.mouse(true);
        tui.enter()?;

        for component in self.components.iter_mut() {
            component.register_action_handler(action_tx.clone())?;
        }

        for component in self.components.iter_mut() {
            component.register_config_handler(self.config.clone())?;
        }

        for component in self.components.iter_mut() {
            component.init(tui.size()?)?;
        }

        loop {
            if let Some(e) = tui.next().await {
                match e {
                    tui::Event::Quit => action_tx.send(Action::Quit)?,
                    tui::Event::Tick => action_tx.send(Action::Tick)?,
                    tui::Event::Render => action_tx.send(Action::Render)?,
                    tui::Event::Resize(x, y) => action_tx.send(Action::Resize(x, y))?,
                    tui::Event::Key(key) => {
                        if let Some(keymap) = self.config.keybindings.get(&self.mode) {
                            if let Some(action) = keymap.get(&vec![key]) {
                                log::info!("Got action: {action:?}");
                                action_tx.send(action.clone())?;
                            } else {
                                // If the key was not handled as a single key action,
                                // then consider it for multi-key combinations.
                                self.last_tick_key_events.push(key);

                                // Check for multi-key combinations
                                if let Some(action) = keymap.get(&self.last_tick_key_events) {
                                    log::info!("Got action: {action:?}");
                                    action_tx.send(action.clone())?;
                                }
                            }
                        };
                    }
                    _ => {}
                }
                for component in self.components.iter_mut() {
                    if let Some(action) = component.handle_events(Some(e.clone()))? {
                        action_tx.send(action)?;
                    }
                }
            }

            while let Ok(action) = action_rx.try_recv() {
                if action != Action::Tick && action != Action::Render {
                    log::debug!("{action:?}");
                }
                match action {
                    Action::Tick => {
                        self.last_tick_key_events.drain(..);
                    }
                    Action::Quit => self.should_quit = true,
                    Action::Suspend => self.should_suspend = true,
                    Action::Resume => self.should_suspend = false,
                    Action::Refresh => tui.terminal.clear()?,
                    Action::Resize(w, h) => {
                        tui.resize(Rect::new(0, 0, w, h))?;
                        tui.draw(|f| {
                            self.draw(f).unwrap_or_else(|err| {
                                action_tx
                                    .send(Action::Error(format!("Failed to draw: {:?}", err)))
                                    .unwrap();
                            });
                        })?;
                    }
                    Action::Render => {
                        tui.draw(|f| {
                            self.draw(f).unwrap_or_else(|err| {
                                action_tx
                                    .send(Action::Error(format!("Failed to draw: {:?}", err)))
                                    .unwrap();
                            });
                        })?;
                    }
                    Action::FocusNext => match self.mode {
                        Mode::Book => {
                            self.mode = Mode::Page;
                            action_tx.send(Action::SelectNextPage)?;
                        }
                        Mode::Page => self.mode = Mode::Content,
                        _ => {}
                    },
                    Action::FocusPrev => match self.mode {
                        Mode::Content => self.mode = Mode::Page,
                        Mode::Page => self.mode = Mode::Book,
                        _ => {}
                    },
                    Action::LoadBooks => {
                        let books = self.dnote.get_books()?;
                        self.state.books = StatefulList::with_items(books);
                    }
                    Action::SelectNextBook => {
                        self.state.books.next();
                        if let Some(book_index) = self.state.books.state.selected() {
                            let selected_book = &self.state.books.items[book_index];
                            action_tx.send(Action::LoadPages(selected_book.name.to_string()))?;
                        }
                    }
                    Action::SelectPrevBook => {
                        self.state.books.previous();
                        if let Some(book_index) = self.state.books.state.selected() {
                            let selected_book = &self.state.books.items[book_index];
                            action_tx.send(Action::LoadPages(selected_book.name.to_string()))?;
                        }
                    }
                    Action::SelectNextPage => {
                        self.state.pages.next();
                        if let Some(page_index) = self.state.pages.state.selected() {
                            let selected_page = &self.state.pages.items[page_index];
                            action_tx.send(Action::LoadContent(selected_page.id))?;
                        }
                    }
                    Action::SelectPrevPage => {
                        self.state.pages.previous();
                        if let Some(page_index) = self.state.pages.state.selected() {
                            let selected_page = &self.state.pages.items[page_index];
                            action_tx.send(Action::LoadContent(selected_page.id))?;
                        }
                    }
                    Action::LoadPages(ref book_name) => {
                        let pages = self.dnote.get_pages(book_name)?;
                        self.state.pages = StatefulList::with_items(pages);
                    }
                    Action::LoadContent(page_id) => {
                        let page_info = self.dnote.get_page_content(page_id)?;
                        self.state.page_content = Some(page_info.content);
                    }
                    _ => {}
                }
                for component in self.components.iter_mut() {
                    if let Some(action) = component.update(action.clone())? {
                        action_tx.send(action)?
                    };
                }
            }
            if self.should_suspend {
                tui.suspend()?;
                action_tx.send(Action::Resume)?;
                action_tx.send(Action::Refresh)?;
                // tui.mouse(true);
                tui.enter()?;
            } else if self.should_quit {
                tui.stop()?;
                break;
            }
        }
        tui.exit()?;
        Ok(())
    }

    fn draw(&mut self, f: &mut tui::Frame<'_>) -> Result<()> {
        let vertical_layout = Layout::vertical(vec![
            Constraint::Max(1),
            Constraint::Fill(1),
            Constraint::Max(1),
        ])
        .split(f.size());
        let header_chunk = vertical_layout[0];
        let main_chunk = vertical_layout[1];
        let footer_chunk = vertical_layout[2];

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(50),
                ]
                .as_ref(),
            )
            .split(main_chunk);

        // self.header.draw(f, header_chunk, &self.state)?;

        for (index, component) in self.components.iter_mut().enumerate() {
            component.draw(f, chunks[index], &mut self.state)?;
        }

        // self.footer.draw(f, footer_chunk, &self.state)?;
        Ok(())
    }
}
