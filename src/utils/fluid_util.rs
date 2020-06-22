use amethyst::{
    assets::Handle,
    core::Transform,
    ecs::Entity,
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};

use crate::components::fluid_world::FluidWorld;

pub fn init_fluid_world(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 0.1);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(FluidWorld::new(0.2, 0.0, 0.000001))
        .with(transform)
        .named("fluid world")
        .build()
}
