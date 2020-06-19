use amethyst::{
    core::{
        Parent,
        Transform,
    },
    ecs::Entity,
    prelude::*,
    renderer::camera::Camera,
};

pub fn init_camera(world: &mut World, parent: Entity, transform: Transform, camera: Camera) -> Entity {
    world
        .create_entity()
        .with(transform)
        .with(Parent { entity: parent })
        .with(camera)
        .named("camera")
        .build()
}
