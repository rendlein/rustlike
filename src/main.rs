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
use tcod::input::*;
use specs::prelude::*;


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
    while !tcod.render(&mut entities) {
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
            return false
        },

        _ => {}
    }

    true

}
