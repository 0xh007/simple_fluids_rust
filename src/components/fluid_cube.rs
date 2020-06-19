use amethyst::ecs::{
    Component,
    NullStorage,
};

#[derive(Default)]
pub struct FluidCube;

impl Component for FluidCube {
    type Storage = NullStorage<Self>;
}
