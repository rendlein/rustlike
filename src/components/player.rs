/// Contains the components specific to the player's character

use specs::prelude::*;
use specs_derive::*;

use crate::components::Position;

#[derive(Component, Debug)]
pub struct Player {}

impl Player {
    pub fn move_player(ecs: &mut World, dx: i32, dy: i32) {
        let mut position: WriteStorage<Position> = ecs.write_storage::<Position>();
        let mut player: WriteStorage<Player> = ecs.write_storage();

        for (_player, pos) in (&mut player, &mut position).join() {
            pos.x += dx;
            pos.y += dy;
        }
    }
}
