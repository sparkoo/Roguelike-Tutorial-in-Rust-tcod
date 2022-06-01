use std::cmp;
use rand::Rng;
use tcod::{BackgroundFlag, Color, colors, Console, Map};
use tcod::colors::VIOLET;
use DeathCallback::Monster;
use crate::{Game, PLAYER_ID};
use crate::ai::Ai;
use crate::inventory::Item;
use crate::object::{DeathCallback, Fighter, Object};

pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 43;

const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_LIGHT_WALL: Color = Color { r: 130, g: 110, b: 50 };

const COLOR_DARK_GROUND: Color = Color { r: 50, g: 50, b: 150 };
const COLOR_LIGHT_GROUND: Color = Color { r: 200, g: 180, b: 50 };


const ROOM_MAX_SIZE: i32 = 10;
const ROOM_MIN_SIZE: i32 = 6;
const MAX_ROOMS: i32 = 30;
const MAX_ROOM_MONSTERS: i32 = 3;
const MAX_ROOM_ITEMS: i32 = 2;

pub type GameMap = Vec<Vec<Tile>>;

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub blocked: bool,
    pub block_sight: bool,
    pub explored: bool,
}

pub fn make_map(objects: &mut Vec<Object>) -> GameMap {
    let mut game_map = vec![vec![Tile::wall(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    let mut rooms = vec![];
    for _ in 0..MAX_ROOMS {
        let w = rand::thread_rng().gen_range(ROOM_MIN_SIZE..(ROOM_MAX_SIZE + 1));
        let h = rand::thread_rng().gen_range(ROOM_MIN_SIZE..(ROOM_MAX_SIZE + 1));

        let x = rand::thread_rng().gen_range(0..(MAP_WIDTH - w));
        let y = rand::thread_rng().gen_range(0..(MAP_HEIGHT - h));

        let new_room = RectRoom::new(x, y, w, h);
        let failed = rooms.iter()
            .any(|other_room| new_room.intersects_with(other_room));
        if !failed {
            create_room(new_room, &mut game_map);
            place_objects(new_room, &game_map, objects);

            let (new_x, new_y) = new_room.center();
            if rooms.is_empty() {
                objects[PLAYER_ID].move_to(new_x, new_y)
            } else {
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();
                if rand::random() {
                    create_h_tunnel(prev_x, new_x, prev_y, &mut game_map);
                    create_v_tunnel(prev_y, new_y, new_x, &mut game_map);
                } else {
                    create_v_tunnel(prev_y, new_y, prev_x, &mut game_map);
                    create_h_tunnel(prev_x, new_x, new_y, &mut game_map);
                }
            }
            rooms.push(new_room)
        }
    }

    game_map
}

fn create_room(room: RectRoom, map: &mut GameMap) {
    for x in (room.x1 + 1)..room.x2 {
        for y in (room.y1 + 1)..room.y2 {
            map[x as usize][y as usize] = Tile::empty();
        }
    }
}

fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut GameMap) {
    for x in cmp::min(x1, x2)..(cmp::max(x1, x2) + 1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

fn create_v_tunnel(y1: i32, y2: i32, x: i32, map: &mut GameMap) {
    for y in cmp::min(y1, y2)..(cmp::max(y1, y2) + 1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

fn place_objects(room: RectRoom, map: &GameMap, objects: &mut Vec<Object>) {
    let num_monsters = rand::thread_rng().gen_range(0..MAX_ROOM_MONSTERS + 1);

    for _ in 0..num_monsters {
        let x = rand::thread_rng().gen_range(room.x1 + 1..room.x2);
        let y = rand::thread_rng().gen_range(room.y1 + 1..room.y2);
        if !is_blocked(x, y, map, objects) {
            let mut monster = if rand::thread_rng().gen_ratio(4, 5) {
                let mut ork = Object::new(x, y, 'o', "ork", colors::DESATURATED_GREEN, true);
                ork.fighter = Some(Fighter { max_hp: 10, hp: 10, defense: 0, power: 3, on_death: Monster });
                ork.ai = Some(Ai::Basic);
                ork
            } else {
                let mut troll = Object::new(x, y, 'T', "troll", colors::DARKER_GREEN, true);
                troll.fighter = Some(Fighter { max_hp: 16, hp: 16, defense: 1, power: 4, on_death: Monster });
                troll.ai = Some(Ai::Basic);
                troll
            };
            monster.alive = true;
            objects.push(monster)
        }
    }

    let num_items = rand::thread_rng().gen_range(0..MAX_ROOM_ITEMS + 1);

    for _ in 0..num_items {
        let x = rand::thread_rng().gen_range(room.x1 + 1..room.x2);
        let y = rand::thread_rng().gen_range(room.y1 + 1..room.y2);

        if !is_blocked(x, y, map, objects) {
            let mut object = Object::new(x, y, '!', "healing potion", VIOLET, false);
            object.item = Some(Item::Heal);
            objects.push(object);
        }
    }
}

pub fn draw_map(game: &mut Game, con: &mut dyn Console, fov_map: &Map) {
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let visible = fov_map.is_in_fov(x, y);
            let wall = game.map[x as usize][y as usize].block_sight;
            let color = match (visible, wall) {
                (false, true) => COLOR_DARK_WALL,
                (false, false) => COLOR_DARK_GROUND,
                (true, true) => COLOR_LIGHT_WALL,
                (true, false) => COLOR_LIGHT_GROUND,
            };

            let explored = &mut game.map[x as usize][y as usize].explored;
            if visible {
                *explored = true;
            }

            if *explored {
                con.set_char_background(x, y, color, BackgroundFlag::Set);
            }
        }
    }
}

pub fn is_blocked(x: i32, y: i32, map: &GameMap, objects: &[Object]) -> bool {
    if map[x as usize][y as usize].blocked {
        return true;
    }

    objects.iter().any(|o| o.blocks && o.position() == (x, y))
}

impl Tile {
    pub fn empty() -> Self {
        Self { blocked: false, block_sight: false, explored: false }
    }

    pub fn wall() -> Self {
        Self { blocked: true, block_sight: true, explored: false }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct RectRoom {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl RectRoom {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self { x1: x, y1: y, x2: x + w, y2: y + h }
    }

    pub fn center(&self) -> (i32, i32) {
        ((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }

    pub fn intersects_with(&self, other: &RectRoom) -> bool {
        (self.x1 <= other.x2) &&
            (self.x2 >= other.x1) &&
            (self.y1 <= other.y2) &&
            (self.y2 >= other.y1)
    }
}
