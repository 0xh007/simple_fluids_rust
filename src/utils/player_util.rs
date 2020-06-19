use amethyst::{
    core::Transform,
    ecs::Entity,
    prelude::*,
};

use crate::components::player::Player;

pub fn init_player(world: &mut World) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 0.1);

    world
        .create_entity()
        .with(transform)
        .with(Player)
        .named("player")
        .build()
}
