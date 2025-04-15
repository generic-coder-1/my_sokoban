use ratatui::{
    crossterm::cursor::position,
    layout::{Constraint, Layout},
    text::{Line, Text},
    widgets::{StatefulWidget, Widget},
};

use crate::tiles::Tile;

pub enum Tool {
    Tile(Tile),
    AreaTool(AreaTool),
    Save,
}

#[derive(Debug)]
pub enum AreaTool {
    Move,
    Delete,
    Fill,
}

pub enum MenuValue {
    Terminal(Tool),
    Nested(MenuLayer),
}

pub struct MenuLayer {
    name: &'static str,
    pub sub_menu: &'static [MenuValue],
}

impl MenuLayer {
    pub const STARTLAYER: MenuValue = MenuValue::Nested(MenuLayer {
        name: "Root",
        sub_menu: &[
            MenuValue::Nested(MenuLayer {
                name: "Tiles",
                sub_menu: &[
                    MenuValue::Terminal(Tool::Tile(Tile::Empty)),
                    MenuValue::Terminal(Tool::Tile(Tile::Wall)),
                    MenuValue::Terminal(Tool::Tile(Tile::Box)),
                ],
            }),
            MenuValue::Nested(MenuLayer {
                name: "Area Select",
                sub_menu: &[
                    MenuValue::Terminal(Tool::AreaTool(AreaTool::Move)),
                    MenuValue::Terminal(Tool::AreaTool(AreaTool::Delete)),
                    MenuValue::Terminal(Tool::AreaTool(AreaTool::Fill)),
                ],
            }),
            MenuValue::Terminal(Tool::Save),
        ],
    });

    pub fn get_value(&self, position: &Vec<usize>) -> Option<&MenuValue> {
        position
            .iter()
            .skip(1)
            .try_fold(
                self.sub_menu.get(*position.first()?)?,
                |val, pos| match val {
                    MenuValue::Terminal(_) => Some(val),
                    MenuValue::Nested(layer) => layer.sub_menu.get(*pos),
                },
            )
    }
}

impl MenuValue {
    pub fn name(&self) -> &str {
        match self {
            MenuValue::Terminal(tool) => tool.into(),
            MenuValue::Nested(menu_layer) => menu_layer.name,
        }
    }

    pub fn get_value(&self, position: &Vec<usize>) -> Option<&MenuValue> {
        position
            .iter()
            .skip(1)
            .try_fold(
                self,
                |val, pos| match val {
                    MenuValue::Terminal(_) => Some(val),
                    MenuValue::Nested(layer) => layer.sub_menu.get(*pos),
                },
            )
    }
}

impl<'a> StatefulWidget for &'a MenuLayer {
    type State = (&'a Vec<usize>, usize);

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut (&Vec<usize>, usize),
    ) where
        Self: Sized,
    {
        let max_len: usize = self
            .sub_menu
            .iter()
            .map(|val| val.name().len())
            .max()
            .unwrap_or(0)
            + 1;
        let lay = Layout::horizontal([Constraint::Fill(1), Constraint::Length(max_len as u16)])
            .split(area);
        let text = Text::from_iter(self.sub_menu.iter().enumerate().map(|(i, val)| {
            Line::from_iter([if state.0[state.1] == i { ">" } else { " " }, val.name()])
        }));
        text.render(lay[1], buf);
        if let MenuValue::Nested(layer) = &self.sub_menu[state.0[state.1]] {
            if state.0.len() > state.1 + 1 {
                state.1 += 1;
                layer.render(lay[0], buf, state);
            }
        }
    }
}

impl From<&Tool> for &str {
    fn from(value: &Tool) -> Self {
        match value {
            Tool::Tile(tile) => match tile {
                Tile::Empty => "Empty",
                Tile::Wall => "Wall",
                Tile::Box => "Box",
            },
            Tool::AreaTool(area_tool) => match area_tool {
                AreaTool::Move => "Move",
                AreaTool::Delete => "Delete",
                AreaTool::Fill => "Fill",
            },
            Tool::Save => "Save",
        }
    }
}
