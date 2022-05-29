use tcod::{BackgroundFlag, Color, Console};
use crate::Game;
use crate::gamemap::is_blocked;

#[derive(Debug)]
pub struct Object {
    x: i32,
    y: i32,
    char: char,
    color: Color,
    name: String,
    pub blocks: bool,
    pub alive: bool,
}

impl Object {
    pub fn new(x: i32, y: i32, char: char, name: &str, color: Color, blocks: bool) -> Self {
        Self {
            x,
            y,
            char,
            color,
            name: name.into(),
            blocks,
            alive: false,
        }
    }

    pub fn move_to(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn draw(&self, con: &mut dyn Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

pub fn move_by(id: usize, dx: i32, dy: i32, game: &Game, objects: &mut [Object]) {
    let (x, y) = objects[id].position();
    if !is_blocked(x + dx, y + dy, &game.map, objects) {
        objects[id].move_to(x + dx, y + dy);
    }
}
