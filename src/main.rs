use tcod::colors::*;
use tcod::console::*;
use tcod::input::{self, Event, Key, Mouse};
use tcod::map::{FovAlgorithm, Map as FovMap};
use roguelike::{Game, gamemap, PLAYER_ID, SCREEN_HEIGHT, SCREEN_WIDTH};
use roguelike::ai::ai_take_turn;
use roguelike::gamemap::{draw_map, MAP_HEIGHT, MAP_WIDTH};
use roguelike::gui::{draw_gui, Messages, PANEL_HEIGHT, PANEL_Y};
use roguelike::inventory::{pick_item_up, use_item};
use roguelike::menu::inventory_menu;
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
    key: Key,
    mouse: Mouse,
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
        key: Default::default(),
        mouse: Default::default(),
    };

    tcod::system::set_fps(LIMIT_FPS);

    let mut player = Object::new(25, 23, '@', "Franta", WHITE, true);
    player.alive = true;
    player.fighter = Some(Fighter { max_hp: 30, hp: 30, defense: 2, power: 5, on_death: Player });

    let mut objects = vec![player];
    let mut game = Game {
        map: gamemap::make_map(&mut objects),
        messages: Messages::new(),
        inventory: vec![],
    };
    game.messages.add("Welcome stranger!", RED);

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            tcod.fov.set(x, y, !game.map[x as usize][y as usize].block_sight, !game.map[x as usize][y as usize].blocked);
        }
    }

    let mut previous_player_position = (-1, -1);
    while !tcod.root.window_closed() {
        match input::check_for_event(input::MOUSE | input::KEY_PRESS) {
            Some((_, Event::Mouse(m))) => tcod.mouse = m,
            Some((_, Event::Key(k))) => tcod.key = k,
            _ => tcod.key = Default::default(),
        }

        render(&mut tcod, &mut game, &objects, previous_player_position != objects[PLAYER_ID].position());
        tcod.root.flush();

        previous_player_position = objects[PLAYER_ID].position();
        let player_action = handle_keys(&mut tcod, &mut objects, &mut game);

        if objects[PLAYER_ID].alive && player_action == TookTurn {
            for id in 0..objects.len() {
                if objects[id].ai.is_some() {
                    ai_take_turn(id, &tcod.fov, &mut game, &mut objects);
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

    draw_gui(&mut tcod.gui, &objects, &game.messages, &tcod.mouse, &tcod.fov);
    blit(&mut tcod.gui, (0, 0), (SCREEN_WIDTH, PANEL_HEIGHT), &mut tcod.root, (0, PANEL_Y), 1.0, 1.0);
}

fn handle_keys(tcod: &mut Tcod, objects: &mut Vec<Object>, game: &mut Game) -> PlayerAction {
    use tcod::input::KeyCode::*;
    match (tcod.key, tcod.key.text(), objects[PLAYER_ID].alive) {
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

        (Key { code: Text, .. }, "g", true) => {
            let item_id = objects.iter().position(|o| o.position() == objects[PLAYER_ID].position() && o.item.is_some());
            if let Some(item_id) = item_id {
                pick_item_up(item_id, game, objects);
            }
            DidntTakeTurn
        }

        (Key { code: Text, .. }, "i", true) => {
            let inventory_index = inventory_menu(&game.inventory, "Press the key to an item to use it, or any other to cancel.\n", &mut tcod.root);
            if let Some(inventory_index) = inventory_index {
                use_item(inventory_index, game, objects);
            }
            DidntTakeTurn
        }

        (Key { code: Enter, alt: true, .. }, _, _) => {
            tcod.root.set_fullscreen(!tcod.root.is_fullscreen());
            DidntTakeTurn
        }
        (Key { code: Escape, .. }, _, _) => Exit,
        _ => DidntTakeTurn
    }
}
