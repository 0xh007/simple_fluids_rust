use amethyst::core::{Transform, SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
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
        WriteStorage<'s, FluidWorld>,
        WriteStorage<'s, Particle>,
        WriteStorage<'s, Tint>,
    );

    fn run(&mut self, (mut fluid_worlds, mut particles, mut tints): Self::SystemData) {
        for (fluid_world, particle, tint) in (&mut fluid_worlds, &mut particles, &mut tints).join() {
            println!("PARTICLE SYSTEM");
        }
    }
}
