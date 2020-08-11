mod config;
mod map;
mod player;
use player::Player;

#[macro_use]
extern crate serde_derive;

use tcod::colors::*;
use tcod::console::*;
use tcod::input::*;
use std::collections::HashMap;
use specs::prelude::*;
use specs_derive::Component;
use std::convert::TryInto;

struct Tcod {
    root: Root,
}

#[derive(Component, Debug)]
struct Renderable {
    glyph: char,
    fg: Color,
    bg: Color,
}

fn main() {
    let mut entities = World::new();

    let configuration = crate::config::Configuration::new();
    // Set up the console.
    let root = Root::initializer()
        .font(configuration.font, FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(configuration.width, configuration.height)
        .title("Rustlike")
        .init();

    let mut tcod = Tcod { root };

    // Add components to ECS
    entities.register::<map::Position>();
    entities.register::<Renderable>();
    entities.register::<player::Player>();

    // Create the Player Entity
    entities
        .create_entity()
        .with(map::Position { x: configuration.width / 2, y: configuration.height / 2 })
        .with(Renderable {
            glyph: '@',
            fg: YELLOW,
            bg: BLACK,
        })
        .with(Player {})
        .build();

    // Draw screen
    while !tcod.root.window_closed() {
        tcod.root.set_default_foreground(WHITE);
        tcod.root.clear();

        {
            println!("SHOULD BE DRAWING @");
            // Draw all the entities
            let mut positions = &entities.read_storage::<map::Position>();
            let mut renderables = &entities.read_storage::<Renderable>();

            for (pos, render) in (positions, renderables).join() {
                println!("{:?} -> {:?}", pos, render);
                tcod.root.put_char_ex(pos.x, pos.y, render.glyph, render.fg, render.bg);
            }
        }
        let x = handle_keys(&mut tcod, &mut entities);
        if x {
            break;
        }

        tcod.root.flush();
        entities.maintain();
    }
}

fn handle_keys(tcod: &mut Tcod, ecs: &mut World) -> bool {
    println!("Waiting for keypress");
    let key = tcod.root.wait_for_keypress(true);

    match key.code {
        KeyCode::Up => {
            println!("UP");
            Player::move_player(ecs, 0, -1)
        },
        KeyCode::Down => {
            println!("Down");
            Player::move_player(ecs, 0, 1)
        },
        KeyCode::Left => {
            println!("Left");
            Player::move_player(ecs, -1, 0)
        },
        KeyCode::Right => {
            println!("Right");
            Player::move_player(ecs, 1, 0)
        },
        KeyCode::Escape => {
            println!("Exiting");
            return true
        },

        _ => {}
    }

    false

}