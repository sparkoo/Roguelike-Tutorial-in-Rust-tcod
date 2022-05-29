use tcod::colors::*;
use tcod::console::*;
use tcod::map::{FovAlgorithm, Map as FovMap};
use roguelike::{Game, gamemap, PLAYER_ID, SCREEN_HEIGHT, SCREEN_WIDTH};
use roguelike::ai::ai_take_turn;
use roguelike::gamemap::{draw_map, MAP_HEIGHT, MAP_WIDTH};
use roguelike::gui::{draw_gui, PANEL_HEIGHT, PANEL_Y};
use roguelike::object::{Fighter, Object, player_move_or_attack};
use roguelike::object::DeathCallback::Player;
use crate::PlayerAction::{DidntTakeTurn, Exit, TookTurn};

const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;
const FOV_LIGHT_WALLS: bool = true;
const TORCH_RADIUS: i32 = 10;

const LIMIT_FPS: i32 = 20;

struct Tcod {
    root: Root,
    con: Offscreen,
    gui: Offscreen,
    fov: FovMap,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum PlayerAction {
    TookTurn,
    DidntTakeTurn,
    Exit,
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
        gui: Offscreen::new(SCREEN_WIDTH, PANEL_HEIGHT),
        fov: FovMap::new(MAP_WIDTH, MAP_HEIGHT),
    };

    tcod::system::set_fps(LIMIT_FPS);

    let mut player = Object::new(25, 23, '@', "Franta", WHITE, true);
    player.alive = true;
    player.fighter = Some(Fighter { max_hp: 30, hp: 30, defense: 2, power: 5, on_death: Player });

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
        let player_action = handle_keys(&mut tcod, &mut objects, &game);

        if objects[PLAYER_ID].alive && player_action == TookTurn {
            for id in 0..objects.len() {
                if objects[id].ai.is_some() {
                    ai_take_turn(id, &tcod.fov, &game, &mut objects);
                }
            }
        }

        if player_action == Exit { break; }
    }
}

fn render(tcod: &mut Tcod, game: &mut Game, objects: &[Object], fov_recompute: bool) {
    tcod.con.clear();

    let mut to_draw: Vec<_> = objects.iter()
        .filter(|o| tcod.fov.is_in_fov(o.position().0, o.position().1))
        .collect();
    to_draw.sort_by(|o1, o2| { o1.blocks.cmp(&o2.blocks) });
    for o in &to_draw {
        o.draw(&mut tcod.con);
    }

    draw_map(game, &mut tcod.con, &tcod.fov);

    if fov_recompute {
        let player = &objects[PLAYER_ID];
        let (px, py) = player.position();
        tcod.fov.compute_fov(px, py, TORCH_RADIUS, FOV_LIGHT_WALLS, FOV_ALGO);
    }

    blit(&tcod.con, (0, 0), (MAP_WIDTH, MAP_HEIGHT), &mut tcod.root, (0, 0), 1.0, 1.0);

    draw_gui(&mut tcod.gui, &objects);
    blit(&mut tcod.gui, (0, 0), (SCREEN_WIDTH, PANEL_HEIGHT), &mut tcod.root, (0, PANEL_Y), 1.0, 1.0);
}

fn handle_keys(tcod: &mut Tcod, objects: &mut [Object], game: &Game) -> PlayerAction {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;
    let key = tcod.root.wait_for_keypress(true);
    match (key, key.text(), objects[PLAYER_ID].alive) {
        (Key { code: Up, .. }, _, true) => {
            player_move_or_attack(0, -1, game, objects);
            TookTurn
        }
        (Key { code: Down, .. }, _, true) => {
            player_move_or_attack(0, 1, game, objects);
            TookTurn
        }
        (Key { code: Left, .. }, _, true) => {
            player_move_or_attack(-1, 0, game, objects);
            TookTurn
        }
        (Key { code: Right, .. }, _, true) => {
            player_move_or_attack(1, 0, game, objects);
            TookTurn
        }

        (Key { code: Enter, alt: true, .. }, _, _) => {
            tcod.root.set_fullscreen(!tcod.root.is_fullscreen());
            DidntTakeTurn
        }
        (Key { code: Escape, .. }, _, _) => Exit,
        _ => DidntTakeTurn
    }
}
