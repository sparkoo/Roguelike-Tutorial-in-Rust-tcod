use tcod::{BackgroundFlag, Color, Console};
use crate::{Game, mut_two, PLAYER_ID};
use crate::ai::Ai;
use crate::gamemap::is_blocked;

#[derive(Debug)]
pub struct Object {
    x: i32,
    y: i32,
    char: char,
    color: Color,
    pub name: String,
    pub blocks: bool,
    pub alive: bool,
    pub fighter: Option<Fighter>,
    pub ai: Option<Ai>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Fighter {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
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
            fighter: None,
            ai: None,
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

    pub fn distance_to(&self, other: &Object) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        ((dx.pow(2) + dy.pow(2)) as f32).sqrt()
    }

    pub fn take_damage(&mut self, damage: i32) {
        println!("take damage {}", damage);
        if let Some(fighter) = self.fighter.as_mut() {
            if damage > 0 {
                fighter.hp -= damage;
            }
        }
    }

    pub fn attack(&mut self, target: &mut Object) {
        let damage = self.fighter.map_or(0, |f| f.power) - target.fighter.map_or(0, |f| f.defense);

        if damage > 0 {
            println!("{} attacks {} for {} hit points.", self.name, target.name, damage);
            target.take_damage(damage);
        } else {
            println!("{} attacks {}, but it has no effect!", self.name, target.name);
        }
    }
}

pub fn move_by(id: usize, dx: i32, dy: i32, game: &Game, objects: &mut [Object]) {
    let (x, y) = objects[id].position();
    if !is_blocked(x + dx, y + dy, &game.map, objects) {
        objects[id].move_to(x + dx, y + dy);
    }
}

pub fn player_move_or_attack(dx: i32, dy: i32, game: &Game, objects: &mut [Object]) {
    let x = objects[PLAYER_ID].x + dx;
    let y = objects[PLAYER_ID].y + dy;

    let target_id = objects.iter().position(|o| o.position() == (x, y));

    match target_id {
        Some(target_id) => {
            let (player, target) = mut_two(PLAYER_ID, target_id, objects);
            player.attack(target);
        }
        None => {
            move_by(PLAYER_ID, dx, dy, game, objects);
        }
    }
}
