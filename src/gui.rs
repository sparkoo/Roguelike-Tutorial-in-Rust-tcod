use tcod::{BackgroundFlag, Color, Console, TextAlignment};
use tcod::colors::{BLACK, DARKER_RED, LIGHT_GREY, LIGHT_RED, WHITE};
use tcod::console::Offscreen;
use tcod::input::Mouse;
use tcod::map::Map as FovMap;
use crate::{PLAYER_ID, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::object::Object;

const BAR_WIDTH: i32 = 20;
pub const PANEL_HEIGHT: i32 = 7;
pub const PANEL_Y: i32 = SCREEN_HEIGHT - PANEL_HEIGHT;

const MSG_X: i32 = BAR_WIDTH + 2;
const MSG_WIDTH: i32 = SCREEN_WIDTH - BAR_WIDTH - 2;
const MSG_HEIGHT: usize = PANEL_HEIGHT as usize - 1;

pub struct Messages {
    messages: Vec<(String, Color)>,
}

impl Messages {
    pub fn new() -> Self {
        Self { messages: vec![] }
    }

    pub fn add<T: Into<String>>(&mut self, message: T, color: Color) {
        self.messages.push((message.into(), color))
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item=&(String, Color)> {
        self.messages.iter()
    }
}

pub fn draw_gui(panel: &mut Offscreen, objects: &[Object], messages: &Messages, mouse: &Mouse, fov: &FovMap) {
    panel.set_default_background(BLACK);
    panel.clear();

    let hp = objects[PLAYER_ID].fighter.map_or(0, |f| f.hp);
    let max_hp = objects[PLAYER_ID].fighter.map_or(0, |f| f.max_hp);

    render_bar(panel, 1, 1, BAR_WIDTH, "HP", hp, max_hp, LIGHT_RED, DARKER_RED);

    render_messages(panel, messages);

    render_mouse(panel, mouse, objects, fov);
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

fn render_messages(panel: &mut Offscreen, messages: &Messages) {
    let mut y = MSG_HEIGHT as i32;
    for &(ref msg, color) in messages.iter().rev() {
        let msg_height = panel.get_height_rect(MSG_X, y, MSG_WIDTH, 0, msg);
        y -= msg_height;
        if y < 0 { break; }
        panel.set_default_foreground(color);
        panel.print_rect(MSG_X, y, MSG_WIDTH, 0, msg);
    }
}

fn render_mouse(panel: &mut Offscreen, mouse: &Mouse, objects: &[Object], fov: &FovMap) {
    panel.set_default_foreground(LIGHT_GREY);
    panel.print_ex(1, 0, BackgroundFlag::None, TextAlignment::Left, get_names_under_mouse(mouse, objects, fov));
}

fn get_names_under_mouse(mouse: &Mouse, objects: &[Object], fov: &FovMap) -> String {
    let (x, y) = (mouse.cx as i32, mouse.cy as i32);

    let names = objects.iter()
        .filter(|o| o.position() == (x, y) && fov.is_in_fov(o.position().0, o.position().1))
        .map(|o| o.name.clone())
        .collect::<Vec<_>>();

    names.join(", ")
}
