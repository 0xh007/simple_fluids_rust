use amethyst::core::{Transform, SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::SpriteRender;
use amethyst::renderer::resources::Tint;

use crate::components::{
    fluid_world::FluidWorld,
    particle::Particle,
};
use crate::utils::fluid_util::*;

#[derive(SystemDesc)]
pub struct ParticleSystem;

impl<'s> System<'s> for ParticleSystem {
    type SystemData = (
        WriteStorage<'s, Particle>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Tint>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (mut particles, mut sprites, mut tints, mut transforms): Self::SystemData) {
        for (particle, sprite, tint, transform) in (&mut particles, &mut sprites, &mut tints, &mut transforms).join() {
            println!("PARTICLE SYSTEM");
        }
    }
}
