use amethyst::ecs::{
    Component,
    DenseVecStorage,
    Entity,
};

pub struct Particle {}

impl Particle {
    pub fn new() -> Particle {
        Particle {}
    }
}

impl Component for Particle {
    type Storage = DenseVecStorage<Self>;
}
