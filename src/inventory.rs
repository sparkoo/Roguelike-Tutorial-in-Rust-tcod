use tcod::colors::{GREEN, LIGHT_VIOLET, RED, WHITE};
use crate::{Game, Object, PLAYER_ID};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Item {
    Heal,
}

enum UseResult {
    UsedUp,
    Cancelled,
}

const HEAL_AMOUNT: i32 = 4;

pub fn pick_item_up(object_id: usize, game: &mut Game, objects: &mut Vec<Object>) {
    if game.inventory.len() >= 26 {
        game.messages.add(format!("Your inventory is full, cannot pick up {}", objects[object_id].name), RED);
    } else {
        let item = objects.swap_remove(object_id);
        game.messages.add(format!("You picked up a {}!", item.name), GREEN);
        game.inventory.push(item);
    }
}

pub fn use_item(inventory_id: usize, game: &mut Game, objects: &mut [Object]) {
    use Item::*;

    if let Some(item) = game.inventory[inventory_id].item {
        let on_use = match item {
            Heal => cast_heal,
        };
        match on_use(inventory_id, game, objects) {
            UseResult::UsedUp => {
                game.inventory.remove(inventory_id);
            }
            UseResult::Cancelled => {
                game.messages.add("Cancelled", WHITE);
            }
        }
    } else {
        game.messages.add(format!("The {} cannot be used.", game.inventory[inventory_id].name), WHITE);
    }
}

fn cast_heal(_inventory_id: usize, game: &mut Game, objects: &mut [Object]) -> UseResult {
    if let Some(fighter) = objects[PLAYER_ID].fighter {
        if fighter.hp == fighter.max_hp {
            game.messages.add("You are already at full hp.", RED);
            return UseResult::Cancelled;
        }
        game.messages.add("Healted.", LIGHT_VIOLET);
        objects[PLAYER_ID].heal(HEAL_AMOUNT);
        return UseResult::UsedUp;
    }
    UseResult::Cancelled
}
