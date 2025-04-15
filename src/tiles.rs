use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Default, Debug)]
pub enum Tile{
    #[default]
    Empty,
    Wall,
    Box,
}

impl From<&Tile> for &str{
    fn from(value: &Tile) -> Self {
        match value{
            Tile::Wall => "#",
            Tile::Box => "*",
            Tile::Empty => " ",
        }
    }
}

pub struct TileDescriptor{
    solid: bool,
    pushable: bool,
}

impl From<&Tile> for &'static TileDescriptor{
    fn from(value: &Tile) -> Self {
        match value{
            Tile::Wall => &TileDescriptor{
                solid: true,
                pushable: false,
            },
            Tile::Box => &TileDescriptor{
                solid: true,
                pushable: true,
            },
            Tile::Empty => &TileDescriptor{
                solid: false,
                pushable: true, 
            },
        }
    }
}
