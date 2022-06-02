use std::cmp;
use crate::gamemap::GameMap;
use crate::gui::Messages;
use crate::object::Object;

pub mod object;
pub mod gamemap;
pub mod ai;
pub mod gui;
pub mod inventory;
pub mod menu;

pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 50;

pub const PLAYER_ID: usize = 0;

pub struct Game {
    pub map: GameMap,
    pub messages: Messages,
    pub inventory: Vec<Object>,
}

pub fn mut_two<T>(first_index: usize, second_index: usize, items: &mut [T]) -> (&mut T, &mut T) {
    assert_ne!(first_index, second_index);

    let split_at_index = cmp::max(first_index, second_index);
    let (first_slice, second_slice) = items.split_at_mut(split_at_index);
    if first_index < second_index {
        (&mut first_slice[first_index], &mut second_slice[0])
    } else {
        (&mut second_slice[0], &mut first_slice[second_index])
    }
}