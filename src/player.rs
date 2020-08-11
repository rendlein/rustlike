use specs::prelude::*;
use specs_derive::*;

use crate::map::Position;

#[derive(Component, Debug)]
pub struct Player {}

impl Player {
    pub fn move_player(ecs: &mut World, dx: i32, dy: i32) {
        println!("Moving");
        let mut position: WriteStorage<Position> = ecs.write_storage::<Position>();
        let mut player: WriteStorage<Player> = ecs.write_storage();

        for (_player, pos) in (&mut player, &mut position).join() {
            pos.x += dx;
            pos.y += dy;

            println!("NEW X: {}\nNEW Y: {}", pos.x, pos.y);
        }
    }
}