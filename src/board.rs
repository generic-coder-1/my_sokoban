use std::ops::{Add, Sub};

use ratatui::{buffer::Buffer, layout::{Position, Rect}, widgets::StatefulWidget};
use serde_derive::{Deserialize, Serialize};

use crate::tiles::Tile;

#[derive(Clone, Serialize, Deserialize)]
pub struct Board{
    width: usize,
    tiles: Box<[Tile]>,
    player_pos: Pos,
    goal: Pos,
}


#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct Pos(pub usize, pub usize);

impl Board{
    pub fn new(width: usize, height: usize) -> Self{
        Self{
            width,
            tiles: (0..).take(width*height).map(|_|{Tile::default()}).collect(),
            player_pos: Pos(0, 0),
            goal: Pos(width-1, height-1),
        }
    }

    pub fn height(&self) -> usize{
        self.tiles.len()/self.width
    }

    pub fn width(&self) -> usize{
        self.width
    }

    fn pos_to_i(&self, pos: Pos) -> usize{
        pos.1 * self.width + pos.0
    }

    pub fn get_tile(&self, pos: Pos) -> Option<&Tile>{
        self.tiles.get(self.pos_to_i(pos))
    }

    pub fn get_tile_mut(&mut self, pos: Pos) -> Option<&mut Tile>{
        let index = self.pos_to_i(pos);
        self.tiles.get_mut(index)
    }

    pub fn set_tile(&mut self, pos: Pos, tile: Tile) {
        let index = self.pos_to_i(pos);
        self.tiles.get_mut(index).map(|tile_| *tile_ = tile);
    }

    pub fn player_pos(&self) -> &Pos {
        &self.player_pos
    }

    pub fn player_pos_mut(&mut self) -> &mut Pos {
        &mut self.player_pos
    }

    pub fn set_player_pos(&mut self, player_pos: Pos) {
        self.player_pos = player_pos;
    }

    pub fn goal(&self) -> &Pos {
        &self.goal
    }

    pub fn goal_mut(&mut self) -> &mut Pos {
        &mut self.goal
    }

    pub fn set_goal(&mut self, goal: Pos) {
        self.goal = goal;
    }
}

impl StatefulWidget for &Board{
    type State = Pos;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let offset = *state;
        let pos = Pos(area.x as usize, area.y as usize);
        (offset.0..self.width()).for_each(|x|{
            (offset.1..self.height()).for_each(|y|{
                if let Some(cell) = buf.cell_mut(Pos(x, y) - offset + pos){
                    cell.set_bg(ratatui::style::Color::DarkGray);
                    cell.set_symbol(self.get_tile(Pos(x, y)).expect("looping through the indices of tile").into());
                }
            });
        });
        if let Some(cell) = buf.cell_mut(self.player_pos - offset + pos){
            const PLAYER_CHAR: &str = "@";
            cell.set_symbol(PLAYER_CHAR);
        }
        if let Some(cell) = buf.cell_mut(self.goal - offset + pos){
            cell.set_bg(ratatui::style::Color::Green);
        }
    }
}


impl From<Pos> for Position{
    fn from(value: Pos) -> Self {
        Position { x: value.0 as u16, y: value.1 as u16 }
    }
}

impl From<Position> for Pos{
    fn from(value: Position) -> Self {
        Pos(value.x as usize, value.y as usize)
    }
}

impl Add<Pos> for Pos{
    type Output = Pos;

    fn add(self, rhs: Pos) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub<Pos> for Pos{
    type Output = Pos;

    fn sub(self, rhs: Pos) -> Self::Output {
        Pos(self.0 - rhs.0, self.1 - rhs.1)
    }
}
