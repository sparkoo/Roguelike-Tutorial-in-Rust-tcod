use tcod::colors::*;
use tcod::console::*;
use roguelike::player::Object;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const LIMIT_FPS: i32 = 20;

struct Tcod {
    root: Root,
    con: Offscreen,
}

fn main() {
    let root = Root::initializer()
        .font("terminal10x10_gs_tc.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Roguelike")
        .init();
    let con = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut tcod = Tcod { root, con };

    tcod::system::set_fps(LIMIT_FPS);

    let player = Object::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', WHITE);
    let npc = Object::new(3, 3, '#', YELLOW);

    let mut objects = [player, npc];
    while !tcod.root.window_closed() {
        tcod.con.clear();

        for o in &objects {
            o.draw(&mut tcod.con);
        }

        blit(&tcod.con, (0, 0), (SCREEN_WIDTH, SCREEN_HEIGHT), &mut tcod.root, (0, 0), 1.0, 1.0);
        tcod.root.flush();

        let player = &mut objects[0];
        let exit = handle_keys(&mut tcod, player);

        if exit { break; }
    }
}

fn handle_keys(tcod: &mut Tcod, player: &mut Object) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;
    let key = tcod.root.wait_for_keypress(true);
    match key {
        Key { code: Up, .. } => player.move_by(0, -1),
        Key { code: Down, .. } => player.move_by(0, 1),
        Key { code: Left, .. } => player.move_by(-1, 0),
        Key { code: Right, .. } => player.move_by(1, 0),
        Key { code: Enter, alt: true, .. } => tcod.root.set_fullscreen(!tcod.root.is_fullscreen()),
        Key { code: Escape, .. } => return true,
        _ => {}
    }

    false
}
