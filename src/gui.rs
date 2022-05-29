use std::fmt::format;
use tcod::{BackgroundFlag, Color, Console, RootConsole, TextAlignment};
use tcod::colors::{BLACK, DARKER_RED, LIGHT_RED, WHITE};
use tcod::console::{blit, Offscreen};
use crate::{PLAYER_ID, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::object::Object;

const BAR_WIDTH: i32 = 20;
pub const PANEL_HEIGHT: i32 = 7;
pub const PANEL_Y: i32 = SCREEN_HEIGHT - PANEL_HEIGHT;

pub fn draw_gui(panel: &mut Offscreen, objects: &[Object]) {
    panel.set_default_background(BLACK);
    panel.clear();

    let hp = objects[PLAYER_ID].fighter.map_or(0, |f| f.hp);
    let max_hp = objects[PLAYER_ID].fighter.map_or(0, |f| f.max_hp);

    render_bar(panel, 1, 1, BAR_WIDTH, "HP", hp, max_hp, LIGHT_RED, DARKER_RED);
}

fn render_bar(panel: &mut Offscreen, x: i32, y: i32, total_width: i32, name: &str, value: i32, max: i32, bar_color: Color, back_color: Color) {
    let bar_width = (value as f32 / max as f32 * total_width as f32) as i32;

    panel.set_default_background(back_color);
    panel.rect(x, y, total_width, 1, false, BackgroundFlag::Screen);

    panel.set_default_background(bar_color);
    if bar_width > 0 {
        panel.rect(x, y, bar_width, 1, false, BackgroundFlag::Screen);
    }

    panel.set_default_foreground(WHITE);
    panel.print_ex(x + total_width / 2, y, BackgroundFlag::None, TextAlignment::Center, &format!("{}: {}/{}", name, value, max));
}
