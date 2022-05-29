use tcod::colors::*;
use tcod::console::*;
use roguelike::{Game, gamemap, SCREEN_HEIGHT, SCREEN_WIDTH};
use roguelike::gamemap::{draw_map, MAP_HEIGHT, MAP_WIDTH};
use roguelike::object::Object;


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
    let con = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);
    let mut tcod = Tcod { root, con };

    tcod::system::set_fps(LIMIT_FPS);

    let mut player = Object::new(25, 23, '@', WHITE);
    let game = Game {
        map: gamemap::make_map(&mut player),
    };

    let npc = Object::new(3, 3, '#', YELLOW);

    let mut objects = [player, npc];
    while !tcod.root.window_closed() {

        render(&mut tcod, &game, &objects);
        tcod.root.flush();

        let player = &mut objects[0];
        let exit = handle_keys(&mut tcod, player, &game);

        if exit { break; }
    }
}

fn render(tcod: &mut Tcod, game: &Game, objects: &[Object]) {
    tcod.con.clear();

    for o in objects {
        o.draw(&mut tcod.con);
    }

    draw_map(&game, &mut tcod.con);

    blit(&tcod.con, (0, 0), (MAP_WIDTH, MAP_HEIGHT), &mut tcod.root, (0, 0), 1.0, 1.0);
}

fn handle_keys(tcod: &mut Tcod, player: &mut Object, game: &Game) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;
    let key = tcod.root.wait_for_keypress(true);
    match key {
        Key { code: Up, .. } => player.move_by(0, -1, game),
        Key { code: Down, .. } => player.move_by(0, 1, game),
        Key { code: Left, .. } => player.move_by(-1, 0, game),
        Key { code: Right, .. } => player.move_by(1, 0, game),
        Key { code: Enter, alt: true, .. } => tcod.root.set_fullscreen(!tcod.root.is_fullscreen()),
        Key { code: Escape, .. } => return true,
        _ => {}
    }

    false
}
