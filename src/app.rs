use std::process::Command;

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
    components::{
        books::BooksPane, content::ContentPane, footer::FooterPane, header::HeaderPane,
        pages::PagesPane, Component,
    },
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
    pub header: Box<dyn Component>,
    pub footer: Box<dyn Component>,
    pub should_quit: bool,
    pub should_suspend: bool,
    pub last_tick_key_events: Vec<KeyEvent>,
    pub dnote: Dnote,
    pub state: State,
}

impl App {
    pub fn new(tick_rate: f64, frame_rate: f64) -> Result<Self> {
        let mut state = State::new();
        state.mode = Mode::Book;
        let dnote = Dnote::new();
        let header = HeaderPane::default();
        let footer = FooterPane::default();
        let books = BooksPane::default();
        let pages = PagesPane::default();
        let content = ContentPane::default();
        let config = Config::new()?;
        Ok(Self {
            tick_rate,
            frame_rate,
            components: vec![
                // Note: ordering is important as layout constraints draw components in order
                Box::new(books),
                Box::new(pages),
                Box::new(content),
            ],
            header: Box::new(header),
            footer: Box::new(footer),
            should_quit: false,
            should_suspend: false,
            config,
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
                        if let Some(keymap) = self.config.keybindings.get(&self.state.mode) {
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
                    if let Some(action) =
                        component.handle_events(Some(e.clone()), &mut self.state)?
                    {
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
                    Action::StatusLine(ref s) => self.state.status_line.clone_from(s),
                    Action::AddPageToActiveBook => {
                        if let Some(book) = self.state.get_active_book() {
                            tui.exit()?;
                            std::process::Command::new("dnote")
                                .arg("add")
                                .arg(&book.name)
                                .status()?;
                            tui.enter()?;
                            action_tx.send(Action::LoadActiveBookPages)?;
                        } else {
                            log::error!("No active book to add page to");
                        }
                    }
                    Action::EditActivePage => {
                        if let Some(page) = self.state.get_active_page() {
                            tui.exit()?;
                            std::process::Command::new("dnote")
                                .arg("edit")
                                .arg(page.id.to_string())
                                .status()?;
                            action_tx.send(Action::UpdateActiveBookPages)?;
                            action_tx.send(Action::LoadActivePageContent)?;
                            tui.enter()?;
                        } else {
                            log::error!("No active page to edit");
                        }
                    }
                    _ => {}
                }
                for component in self.components.iter_mut() {
                    if let Some(action) = component.update(action.clone(), &mut self.state)? {
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
            Constraint::Max(3),
            Constraint::Fill(1),
            Constraint::Max(1),
        ])
        .horizontal_margin(1)
        .split(f.size());
        let header_chunk = vertical_layout[0];
        let main_chunk = vertical_layout[1];
        let footer_chunk = vertical_layout[2];

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .horizontal_margin(1)
            .constraints(
                [
                    Constraint::Percentage(15),
                    Constraint::Percentage(35),
                    Constraint::Percentage(50),
                ]
                .as_ref(),
            )
            .split(main_chunk);

        self.header.draw(f, header_chunk, &mut self.state)?;

        for (index, component) in self.components.iter_mut().enumerate() {
            component.draw(f, chunks[index], &mut self.state)?;
        }

        self.footer.draw(f, footer_chunk, &mut self.state)?;
        Ok(())
    }
}
