use std::io;

use crossterm::event::{self, KeyEventKind};
use ratatui::{crossterm, text::{self, Text}, widgets::Widget, DefaultTerminal, Frame};

use crate::{main_menu::MainMenu, menu::Menu};

pub struct App {
    should_exit: bool,
    menus: Vec<Box<dyn Menu<Output = ()>>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            should_exit: false,
            menus: vec![Box::new(MainMenu::new())],
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_input()?;
            if let Some(result) = self.menus.last_mut().unwrap().is_done() {
                match result {
                    crate::menu::MenuOptions::GoBack => {
                        self.menus.pop();
                    }
                    crate::menu::MenuOptions::BackToFirst => {
                        while self.menus.len() > 1 {
                            self.menus.pop();
                        }
                    }
                    crate::menu::MenuOptions::Continue(menu) => {
                        self.menus.push(menu);
                    },
                    crate::menu::MenuOptions::Exit(_) => self.should_exit = true,
                }
            }
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_input(&mut self) -> io::Result<()> {
        match event::read()? {
            event::Event::Key(key) if key.kind == KeyEventKind::Press => {
                self.menus.last_mut().unwrap().handle_input(key);
            }
            _ => {}
        };
        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        self.menus.last().unwrap().as_ref().render_ref(area, buf);
    }
}
