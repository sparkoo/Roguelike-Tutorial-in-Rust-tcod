use tcod::colors::{GREEN, RED};
use crate::{Game, Object};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Item {
    Heal,
}

pub fn pick_item_up(object_id: usize, game: &mut Game, objects: &mut Vec<Object>) {
    if game.inventory.len() >= 26 {
        game.messages.add(format!("Your inventory is full, cannot pick up {}", objects[object_id].name), RED);
    } else {
        let item = objects.swap_remove(object_id);
        game.messages.add(format!("You picked up a {}!", item.name), GREEN);
        game.inventory.push(item);
    }
}
