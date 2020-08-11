mod ui;
use ui::Ui;
mod components;
use components::{Player, Position, Renderable};
mod config;
use crate::config::CONFIG;
mod map;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

use tcod::colors::*;
use tcod::console::*;
use tcod::input::*;
use std::collections::HashMap;
use specs::prelude::*;
use specs_derive::Component;
use std::convert::TryInto;


fn main() {
    let mut tcod = Ui::new();
    let mut entities = World::new();

    // Add components to ECS
    entities.register::<Position>();
    entities.register::<Renderable>();
    entities.register::<Player>();

    // Create the Player Entity
    entities
        .create_entity()
        .with(Position { x: CONFIG.width / 2, y: CONFIG.height / 2 })
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
            // Draw all the entities
            let mut positions = &entities.read_storage::<Position>();
            let mut renderables = &entities.read_storage::<Renderable>();

            for (pos, render) in (positions, renderables).join() {
                tcod.root.put_char_ex(pos.x, pos.y, render.glyph, render.fg, render.bg);
            }
        }
        tcod.root.flush();
        let x = handle_keys(&mut tcod, &mut entities);
        if x {
            break;
        }

        entities.maintain();
    }
}

fn handle_keys(tcod: &mut Ui, ecs: &mut World) -> bool {
    let key = tcod.root.wait_for_keypress(true);

    match key.code {
        KeyCode::Up => {
            Player::move_player(ecs, 0, -1)
        },
        KeyCode::Down => {
            Player::move_player(ecs, 0, 1)
        },
        KeyCode::Left => {
            Player::move_player(ecs, -1, 0)
        },
        KeyCode::Right => {
            Player::move_player(ecs, 1, 0)
        },
        KeyCode::Escape => {
            return true
        },

        _ => {}
    }

    false

}