use tcod::colors::*;
use tcod::console::*;
use tcod::map::{FovAlgorithm, Map as FovMap};
use roguelike::{Game, gamemap, PLAYER_ID, SCREEN_HEIGHT, SCREEN_WIDTH};
use roguelike::gamemap::{draw_map, MAP_HEIGHT, MAP_WIDTH};
use roguelike::object::{move_by, Object};

const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;
const FOV_LIGHT_WALLS: bool = true;
const TORCH_RADIUS: i32 = 10;

const LIMIT_FPS: i32 = 20;

struct Tcod {
    root: Root,
    con: Offscreen,
    fov: FovMap,
}

fn main() {
    let root = Root::initializer()
        .font("terminal10x10_gs_tc.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Roguelike")
        .init();
    let mut tcod = Tcod {
        root,
        con: Offscreen::new(MAP_WIDTH, MAP_HEIGHT),
        fov: FovMap::new(MAP_WIDTH, MAP_HEIGHT),
    };

    tcod::system::set_fps(LIMIT_FPS);

    let mut player = Object::new(25, 23, '@', "Franta", WHITE, false);
    player.alive = true;

    let mut objects = vec![player];
    let mut game = Game {
        map: gamemap::make_map(&mut objects),
    };

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            tcod.fov.set(x, y, !game.map[x as usize][y as usize].block_sight, !game.map[x as usize][y as usize].blocked);
        }
    }

    let mut previous_player_position = (-1, -1);
    while !tcod.root.window_closed() {
        render(&mut tcod, &mut game, &objects, previous_player_position != objects[PLAYER_ID].position());
        tcod.root.flush();

        previous_player_position = objects[PLAYER_ID].position();
        let exit = handle_keys(&mut tcod, &mut objects, &game);

        if exit { break; }
    }
}

fn render(tcod: &mut Tcod, game: &mut Game, objects: &[Object], fov_recompute: bool) {
    tcod.con.clear();

    for o in objects {
        let (ox, oy) = o.position();
        if tcod.fov.is_in_fov(ox, oy) {
            o.draw(&mut tcod.con);
        }
    }

    draw_map(game, &mut tcod.con, &tcod.fov);

    if fov_recompute {
        let player = &objects[PLAYER_ID];
        let (px, py) = player.position();
        tcod.fov.compute_fov(px, py, TORCH_RADIUS, FOV_LIGHT_WALLS, FOV_ALGO);
    }

    blit(&tcod.con, (0, 0), (MAP_WIDTH, MAP_HEIGHT), &mut tcod.root, (0, 0), 1.0, 1.0);
}

fn handle_keys(tcod: &mut Tcod, objects: &mut [Object], game: &Game) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;
    let key = tcod.root.wait_for_keypress(true);
    match key {
        Key { code: Up, .. } => move_by(PLAYER_ID, 0, -1, game, objects),
        Key { code: Down, .. } => move_by(PLAYER_ID,0, 1, game, objects),
        Key { code: Left, .. } => move_by(PLAYER_ID,-1, 0, game, objects),
        Key { code: Right, .. } => move_by(PLAYER_ID,1, 0, game, objects),
        Key { code: Enter, alt: true, .. } => tcod.root.set_fullscreen(!tcod.root.is_fullscreen()),
        Key { code: Escape, .. } => return true,
        _ => {}
    }

    false
}
