use amethyst::{
    ecs::Entity,
    prelude::*,
    renderer::debug_drawing::DebugLinesComponent,
};

pub fn create_debug_lines(world: &mut World) -> Entity {
    world
        .create_entity()
        .with(DebugLinesComponent::with_capacity(1))
        .build()
}
