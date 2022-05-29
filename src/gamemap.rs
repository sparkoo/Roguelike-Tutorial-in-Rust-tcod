use tcod::{BackgroundFlag, Color, Console};
use crate::Game;

pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 45;

const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_DARK_GROUND: Color = Color { r: 50, g: 50, b: 150 };

pub type GameMap = Vec<Vec<Tile>>;

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub blocked: bool,
    pub block_sight: bool,
}

pub fn make_map() -> GameMap {
    let mut game_map = vec![vec![Tile::empty(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    game_map[30][22] = Tile::wall();
    game_map[50][22] = Tile::wall();

    game_map
}

pub fn draw_map(game: &Game, con: &mut dyn Console) {
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            if game.map[x as usize][y as usize].block_sight {
                con.set_char_background(x, y, COLOR_DARK_WALL, BackgroundFlag::Set)
            } else {
                con.set_char_background(x, y, COLOR_DARK_GROUND, BackgroundFlag::Set);
            }
        }
    }
}

impl Tile {
    pub fn empty() -> Self {
        Self { blocked: false, block_sight: false }
    }

    pub fn wall() -> Self {
        Self { blocked: true, block_sight: true }
    }
}
