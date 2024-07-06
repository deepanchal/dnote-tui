use std::{borrow::BorrowMut, process::Command};

use color_eyre::eyre::Result;
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use crossterm::{event::KeyEvent, ExecutableCommand};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::Rect,
};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use crate::{
    action::Action,
    components::{
        books::BooksPane,
        content::ContentPane,
        footer::FooterPane,
        header::HeaderPane,
        pages::PagesPane,
        popup::{Popup, PopupType},
        Component,
    },
    config::Config,
    dnote::Dnote,
    state::{InputMode, Mode, State, StatefulList},
    tui,
};

pub struct App {
    pub config: Config,
    pub tui: tui::Tui,
    pub action_tx: UnboundedSender<Action>,
    pub action_rx: UnboundedReceiver<Action>,
    pub tick_rate: f64,
    pub frame_rate: f64,
    pub components: Vec<Box<dyn Component>>,
    pub header: Box<dyn Component>,
    pub footer: Box<dyn Component>,
    pub popup: Option<Box<dyn Component>>,
    pub should_quit: bool,
    pub should_suspend: bool,
    pub last_tick_key_events: Vec<KeyEvent>,
    pub dnote: Dnote,
    pub state: State,
}

impl App {
    pub fn new(tick_rate: f64, frame_rate: f64) -> Result<Self> {
        let tui = tui::Tui::new()?.tick_rate(tick_rate).frame_rate(frame_rate);
        let (action_tx, action_rx) = mpsc::unbounded_channel();
        let mut state = State::new();
        state.mode = Mode::Book;
        let dnote = Dnote::new();
        let header = HeaderPane::default();
        let footer = FooterPane::default();
        let books = BooksPane::default();
        let pages = PagesPane::default();
        let content = ContentPane::default();
        let config = Config::new()?;
        let app = Self {
            tui,
            action_tx,
            action_rx,
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
            popup: None,
            should_quit: false,
            should_suspend: false,
            config,
            last_tick_key_events: Vec::new(),
            dnote,
            state,
        };
        Ok(app)
    }

    fn wait_for_enter_to_return(&self) -> Result<()> {
        std::io::stdout()
            .execute(SetForegroundColor(Color::Green))?
            .execute(Print("\nPress enter to return to dnote-tui"))?
            .execute(ResetColor)?;
        use std::io::{self, BufRead};
        let stdin = io::stdin();
        stdin.lock().lines().next();
        Ok(())
    }

    fn log_command(&self, command: &str, args: &[&str]) -> Result<()> {
        std::io::stdout()
            .execute(SetForegroundColor(Color::Blue))?
            .execute(Print(format!("\n=> {} {}\n\n", command, args.join(" "))))?
            .execute(ResetColor)?;
        Ok(())
    }

    fn spawn_process(&self, command: &str, args: &[&str]) -> Result<()> {
        self.log_command(command, args)?;
        let status = Command::new(command).args(args).status()?;
        if !status.success() {
            eprintln!("\nCommand failed with status: {}", status);
        }
        self.wait_for_enter_to_return()?;
        Ok(())
    }

    pub fn resume(&mut self) -> Result<()> {
        self.tui.enter()?;
        Ok(())
    }

    pub fn pause(&mut self) -> Result<()> {
        self.tui.exit()?;
        Ok(())
    }

    pub fn close_popup(&mut self) -> Result<()> {
        self.popup.take();
        Ok(())
    }

    pub async fn run(&mut self) -> Result<()> {
        // tui.mouse(true);
        self.tui.enter()?;

        for component in self.components.iter_mut() {
            component.register_action_handler(self.action_tx.clone())?;
        }

        for component in self.components.iter_mut() {
            component.register_config_handler(self.config.clone())?;
        }

        for component in self.components.iter_mut() {
            component.init(self.tui.size()?)?;
        }

        self.header
            .register_action_handler(self.action_tx.clone())?;
        self.footer
            .register_action_handler(self.action_tx.clone())?;

        self.header.register_config_handler(self.config.clone())?;
        self.footer.register_config_handler(self.config.clone())?;

        self.header.init(self.tui.size()?)?;
        self.footer.init(self.tui.size()?)?;

        if let Some(popup) = &mut self.popup {
            popup.register_action_handler(self.action_tx.clone())?;
            popup.register_config_handler(self.config.clone())?;
            popup.init(self.tui.size()?)?;
        }

        loop {
            if let Some(e) = self.tui.next().await {
                match e {
                    tui::Event::Quit if self.state.input_mode == InputMode::Normal => {
                        self.action_tx.send(Action::Quit)?
                    }
                    tui::Event::Tick => self.action_tx.send(Action::Tick)?,
                    tui::Event::Render => self.action_tx.send(Action::Render)?,
                    tui::Event::Resize(x, y) => self.action_tx.send(Action::Resize(x, y))?,
                    tui::Event::Key(key) => {
                        match self.state.input_mode {
                            InputMode::Normal => {
                                if let Some(keymap) = self.config.keybindings.get(&self.state.mode)
                                {
                                    if let Some(action) = keymap.get(&vec![key]) {
                                        log::info!("Got action: {action:?}");
                                        self.action_tx.send(action.clone())?;
                                    } else {
                                        // If the key was not handled as a single key action,
                                        // then consider it for multi-key combinations.
                                        self.last_tick_key_events.push(key);

                                        // Check for multi-key combinations
                                        if let Some(action) = keymap.get(&self.last_tick_key_events)
                                        {
                                            log::info!("Got action: {action:?}");
                                            self.action_tx.send(action.clone())?;
                                        }
                                    }
                                };
                            }
                            InputMode::Insert => {
                                log::debug!("Skipping keybinds from config in insert mode...");
                            }
                        }
                    }
                    _ => {}
                }
                for component in self.components.iter_mut() {
                    if let Some(action) =
                        component.handle_events(Some(e.clone()), &mut self.state)?
                    {
                        self.action_tx.send(action)?;
                    }
                }
                if let Some(popup) = &mut self.popup {
                    if let Some(action) = popup.handle_events(Some(e.clone()), &mut self.state)? {
                        self.action_tx.send(action)?
                    };
                }
                if let Some(action) = self
                    .header
                    .handle_events(Some(e.clone()), &mut self.state)?
                {
                    self.action_tx.send(action)?
                };
                if let Some(action) = self
                    .footer
                    .handle_events(Some(e.clone()), &mut self.state)?
                {
                    self.action_tx.send(action)?
                };
            }

            while let Ok(action) = self.action_rx.try_recv() {
                if action != Action::Tick && action != Action::Render {
                    log::info!("{action:?}");
                }
                match action {
                    Action::Tick => {
                        self.last_tick_key_events.drain(..);
                    }
                    Action::Quit if self.state.input_mode == InputMode::Normal => {
                        self.should_quit = true
                    }
                    Action::Suspend => self.should_suspend = true,
                    Action::Resume => self.should_suspend = false,
                    Action::Refresh => self.tui.terminal.clear()?,
                    Action::Resize(w, h) => {
                        self.tui.resize(Rect::new(0, 0, w, h))?;
                        self.draw()?;
                    }
                    Action::Render => {
                        self.draw()?;
                    }
                    Action::ExecuteCommand(ref command, ref args) => {
                        self.pause()?;
                        let cmd = command.to_string();
                        let cmd_args = args.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
                        self.spawn_process(&cmd, &cmd_args)?;
                        self.resume()?;
                    }
                    Action::AddBook => {
                        let input_popup = Popup::new(
                            "Add New Book".into(),
                            "Name".into(),
                            Some("Note: Book names cannot contain spaces!".into()),
                            PopupType::NewBook,
                        );
                        self.popup = Some(Box::new(input_popup));
                        if let Some(popup) = &mut self.popup {
                            popup.register_action_handler(self.action_tx.clone())?;
                            popup.register_config_handler(self.config.clone())?;
                            popup.init(self.tui.size()?)?;
                        }
                        self.state.input_mode = InputMode::Insert;
                    }
                    Action::SubmitPopup => {
                        self.popup.take(); // set popup to None
                        self.state.input_mode = InputMode::Normal;
                    }
                    Action::ClosePopup => {
                        self.popup.take(); // set popup to None
                        self.state.input_mode = InputMode::Normal;
                    }
                    _ => {}
                }
                for component in self.components.iter_mut() {
                    if let Some(action) = component.update(action.clone(), &mut self.state)? {
                        self.action_tx.send(action)?
                    };
                }
                if let Some(popup) = &mut self.popup {
                    if let Some(action) = popup.update(action.clone(), &mut self.state)? {
                        self.action_tx.send(action)?
                    };
                }
                if let Some(action) = self.header.update(action.clone(), &mut self.state)? {
                    self.action_tx.send(action)?
                };
                if let Some(action) = self.footer.update(action.clone(), &mut self.state)? {
                    self.action_tx.send(action)?
                };
            }
            if self.should_suspend {
                self.tui.suspend()?;
                self.action_tx.send(Action::Resume)?;
                self.action_tx.send(Action::Refresh)?;
                // tui.mouse(true);
                self.tui.enter()?;
            } else if self.should_quit {
                self.tui.stop()?;
                break;
            }
        }
        self.tui.exit()?;
        Ok(())
    }

    fn draw(&mut self) -> Result<()> {
        self.tui.draw(|f| {
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

            self.header
                .draw(f, header_chunk, &mut self.state)
                .unwrap_or_else(|err| {
                    self.action_tx
                        .send(Action::Error(format!("Failed to draw header: {:?}", err)))
                        .unwrap();
                });

            for (index, component) in self.components.iter_mut().enumerate() {
                component
                    .draw(f, chunks[index], &mut self.state)
                    .unwrap_or_else(|err| {
                        self.action_tx
                            .send(Action::Error(format!(
                                "Failed to draw component: {:?}",
                                err
                            )))
                            .unwrap();
                    });
            }

            if let Some(popup) = &mut self.popup {
                let popup_vertical_layout = Layout::vertical(vec![
                    Constraint::Min(1),
                    Constraint::Length(10),
                    Constraint::Min(1),
                ])
                .split(f.size());

                let popup_layout = Layout::horizontal(vec![
                    Constraint::Min(3),
                    Constraint::Length(50),
                    Constraint::Min(3),
                ])
                .split(popup_vertical_layout[1]);

                popup
                    .draw(f, popup_layout[1], &mut self.state)
                    .unwrap_or_else(|err| {
                        self.action_tx
                            .send(Action::Error(format!("Failed to draw popup: {:?}", err)))
                            .unwrap();
                    });
            }
            self.footer
                .draw(f, footer_chunk, &mut self.state)
                .unwrap_or_else(|err| {
                    self.action_tx
                        .send(Action::Error(format!("Failed to draw footer: {:?}", err)))
                        .unwrap();
                });
        })?;
        Ok(())
    }
}
