use amethyst::{
    core::Transform,
    ecs::Entity,
    prelude::*,
};

use crate::components::fluid_world::FluidWorld;

pub fn init_fluid_world(world: &mut World) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 0.1);

    world
        .create_entity()
        .with(FluidWorld::new(0.2, 0.0, 0.000001))
        .with(transform)
        .named("fluid world")
        .build()
}
