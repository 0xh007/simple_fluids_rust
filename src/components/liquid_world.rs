use amethyst::ecs::{
    Component,
    NullStorage,
};

#[derive(Default)]
pub struct LiquidWorld;

impl Component for LiquidWorld {
    type Storage = NullStorage<Self>;
}
