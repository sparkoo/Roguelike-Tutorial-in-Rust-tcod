use tcod::Map;
use crate::{Game, PLAYER_ID};
use crate::object::{move_by, Object};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Ai {
    Basic,
}

fn move_towards(id: usize, target_x: i32, target_y: i32, game: &Game, objects: &mut [Object]) {
    let (x, y) = objects[id].position();
    let dx = target_x - x;
    let dy = target_y - y;
    let distance = ((dx.pow(2) + dy.pow(2)) as f32).sqrt();

    let dx = (dx as f32 / distance).round() as i32;
    let dy = (dy as f32 / distance).round() as i32;
    move_by(id, dx, dy, game, objects);
}

pub fn ai_take_turn(monster_id: usize, fov_map: &Map, game: &Game, objects: &mut [Object]) {
    let (monster_x, monster_y) = objects[monster_id].position();
    if fov_map.is_in_fov(monster_x, monster_y) {
        if objects[monster_id].distance_to(&objects[PLAYER_ID]) >= 2.0 {
            let (px, py) = objects[PLAYER_ID].position();
            move_towards(monster_id, px, py, game, objects);
        } else {
            let monster = &objects[monster_id];
            println!("The attack from {}", monster.name);
        }
    }
}
