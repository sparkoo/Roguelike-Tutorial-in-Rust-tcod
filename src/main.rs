use tcod::colors::*;
use tcod::console::*;
use roguelike::player::Player;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const LIMIT_FPS: i32 = 20;

struct Tcod {
    root: Root,
}

fn main() {
    let root = Root::initializer()
        .font("terminal10x10_gs_tc.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Roguelike")
        .init();
    let mut tcod = Tcod { root };

    tcod::system::set_fps(LIMIT_FPS);

    let mut player = Player::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
    while !tcod.root.window_closed() {
        tcod.root.set_default_foreground(WHITE);
        tcod.root.clear();
        tcod.root.put_char(player.x, player.y, '@', BackgroundFlag::None);
        tcod.root.flush();
        let exit = handle_keys(&mut tcod, &mut player);
        if exit { break; }
    }
}

fn handle_keys(tcod: &mut Tcod, player: &mut Player) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;
    let key = tcod.root.wait_for_keypress(true);
    match key {
        Key { code: Up, .. } => player.y -= 1,
        Key { code: Down, .. } => player.y += 1,
        Key { code: Left, .. } => player.x -= 1,
        Key { code: Right, .. } => player.x += 1,
        Key { code: Enter, alt: true, .. } => tcod.root.set_fullscreen(!tcod.root.is_fullscreen()),
        Key { code: Escape, .. } => return true,
        _ => {}
    }

    false
}
