#![allow(warnings)]
use bevy::prelude::*;

struct InfantryStats {
    hp: f32,
    damage: f32,
    speed: f32,
}

// In order to use .init_resource, we need to specify a default starting value
// by implementing the Default trait
impl Default for InfantryStats {
    fn default() -> Self {
        InfantryStats {
            hp: 10.0,
            damage: 2.0,
            speed: 1.0,
        }
    }
}

struct PlayerResources {
    gold: usize,
    wood: usize,
}

struct InitialPosition(i32, i32);

enum PlayerColor {
    Red,
    Blue,
    Pink,
}

fn main() {
    App::build()
        .init_resource::<InfantryStats>()
        // Ordinary structs, tuple structs and enums can all be used as resources
        .add_resource(PlayerResources {
            gold: 1000,
            wood: 500,
        })
        .add_resource(InitialPosition(0, 0))
        .add_resource(PlayerColor::Pink)
        .run();
}
