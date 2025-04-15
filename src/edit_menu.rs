use std::{fs, ops::Add, path::Path};

use ratatui::{
    buffer::Buffer,
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Layout, Margin, Rect},
    widgets::{Block, StatefulWidget, Widget, WidgetRef},
};
use ron::ser::PrettyConfig;

use crate::{
    board::{Board, Pos},
    menu::Menu,
    tools::{MenuLayer, MenuValue},
};
use anyhow::Result;

pub struct EditMenu {
    cursor: Pos,
    buffer: Board,
    path: Option<Box<Path>>,
    current_tool: Vec<usize>,
}

impl Default for EditMenu {
    fn default() -> Self {
        Self {
            cursor: Pos(0, 0),
            buffer: Board::new(20, 20),
            path: Default::default(),
            current_tool: vec![0],
        }
    }
}

impl EditMenu {
    pub fn from_path(path: Box<Path>) -> Result<Self> {
        Ok(Self {
            buffer: ron::from_str(&fs::read_to_string(&path)?)?,
            path: Some(path),
            ..Default::default()
        })
    }

    fn save(&self) -> bool {
        if let Some(path) = &self.path {
            if let Ok(content) = ron::ser::to_string_pretty(&self.buffer, PrettyConfig::default()) {
                return fs::write(path, content).is_ok();
            }
        }
        false
    }

    pub fn path(&self) -> Option<&Box<Path>> {
        self.path.as_ref()
    }
}

impl Menu for EditMenu {
    type Output = ();

    fn handle_input(&mut self, input: ratatui::crossterm::event::KeyEvent) {
        match input.code {
            KeyCode::Char(x) if x.eq_ignore_ascii_case(&'w') => {
                self.cursor.1 = self.cursor.1.saturating_sub(1)
            }
            KeyCode::Char(x) if x.eq_ignore_ascii_case(&'a') => {
                self.cursor.0 = self.cursor.0.saturating_sub(1)
            }
            KeyCode::Char(x) if x.eq_ignore_ascii_case(&'d') => {
                self.cursor.0 = self.cursor.0.add(1).min(self.buffer.width() - 1)
            }
            KeyCode::Char(x) if x.eq_ignore_ascii_case(&'s') => {
                self.cursor.1 = self.cursor.1.add(1).min(self.buffer.height() - 1)
            }
            KeyCode::Down => {
                if let Some(MenuValue::Nested(layer)) =
                    MenuLayer::STARTLAYER.get_value(&self.current_tool)
                {
                    let mut last_tool = self.current_tool.pop().unwrap();
                    last_tool +=1;
                    last_tool %= layer.sub_menu.len();
                    self.current_tool.push(last_tool);
                }
            }
            KeyCode::Up => {
                if let Some(MenuValue::Nested(layer)) =
                    MenuLayer::STARTLAYER.get_value(&self.current_tool)
                {
                    let mut last_tool:isize = self.current_tool.pop().unwrap() as isize;
                    last_tool -=1;
                    last_tool %= layer.sub_menu.len() as isize;
                    self.current_tool.push(last_tool as usize);
                }
            }
            KeyCode::Right => {
                if let Some(MenuValue::Nested(_))= MenuLayer::STARTLAYER.get_value(&self.current_tool){
                    self.current_tool.push(0);
                }
            }
            KeyCode::Left =>{
                if self.current_tool.len()>2{
                    self.current_tool.pop();
                }
            }
            _ => {}
        }
    }

    fn is_done(&mut self) -> Option<crate::menu::MenuOptions<Self::Output>> {
        None
    }
}

impl WidgetRef for EditMenu {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let lay = Layout::horizontal([
            Constraint::Max(self.buffer.width() as u16 + 2),
            Constraint::Fill(1),
            Constraint::Length(10),
        ])
        .split(area);
        {
            //Actual Map
            let buf_area = Layout::vertical([
                Constraint::Max(self.buffer.height() as u16 + 2),
                Constraint::Fill(1),
            ])
            .split(lay[0])[0];
            let b = Block::bordered()
                .title(
                    self.path
                        .as_ref()
                        .and_then(|path| path.file_stem())
                        .and_then(|stem| stem.to_str())
                        .unwrap_or("*Unsaved"),
                )
                .title_bottom(format!("{}; {}", self.cursor.0, self.cursor.1));
            let mut offset = Pos(0, 0);
            self.buffer.render(b.inner(buf_area), buf, &mut offset);
            if let Some(cell) = buf.cell_mut(
                self.cursor - offset
                    + buf_area
                        .inner(Margin::new(1, 1))
                        .positions()
                        .next()
                        .unwrap()
                        .into(),
            ) {
                cell.set_bg(ratatui::style::Color::Blue);
            }
            b.render(buf_area, buf);
        }
        {
            //Tool area
            if let MenuValue::Nested(layer) = MenuLayer::STARTLAYER {
                layer.render(lay[1], buf, &mut (&self.current_tool, 0))
            };
        }
    }
}
