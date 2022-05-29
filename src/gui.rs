use tcod::{BackgroundFlag, Console, RootConsole, TextAlignment};
use crate::{PLAYER_ID, SCREEN_HEIGHT};
use crate::object::Object;

pub fn draw_gui(root: &mut RootConsole, objects: &[Object]) {
    if let Some(fighter) = objects[PLAYER_ID].fighter {
        root.print_ex(1, SCREEN_HEIGHT - 2,
                      BackgroundFlag::None,
                      TextAlignment::Left,
                      format!("HP: {}/{}", fighter.hp, fighter.max_hp))
    }
}