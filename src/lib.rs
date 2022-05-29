use crate::gamemap::GameMap;

pub mod object;
pub mod gamemap;

pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 50;

pub const PLAYER_ID: usize = 0;

pub struct Game {
    pub map: GameMap,
}