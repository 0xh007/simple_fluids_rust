use amethyst::core::{Transform, SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::components::fluid_world::FluidWorld;
use crate::utils::fluid_util::*;

#[derive(SystemDesc)]
pub struct FluidSystem;

impl<'s> System<'s> for FluidSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, FluidWorld>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, mut fluid_worlds, input): Self::SystemData) {
        for (fluid_world, transform) in (&mut fluid_worlds, &mut transforms).join() {
            if !fluid_world.started {
                println!("Starting fluid sim");
                diffuse(1, fluid_world.velocity_x_prev, fluid_world.velocity_x, fluid_world.viscosity, fluid_world.dt);
                diffuse(2, fluid_world.velocity_y_prev, fluid_world.velocity_y, fluid_world.viscosity, fluid_world.dt);

                project(fluid_world.velocity_x_prev, fluid_world.velocity_y_prev, fluid_world.velocity_x, fluid_world.velocity_y);

                advect(
                    1,
                    fluid_world.velocity_x,
                    fluid_world.velocity_x_prev,
                    fluid_world.velocity_x_prev,
                    fluid_world.velocity_y_prev,
                    fluid_world.dt);

                advect(
                    2,
                    fluid_world.velocity_y,
                    fluid_world.velocity_y_prev,
                    fluid_world.velocity_x_prev,
                    fluid_world.velocity_y_prev,
                    fluid_world.dt);

                project(fluid_world.velocity_x, fluid_world.velocity_y, fluid_world.velocity_x_prev, fluid_world.velocity_y_prev);

                diffuse(0, fluid_world.s, fluid_world.density, fluid_world.diffusion, fluid_world.dt);

                fluid_world.started = true;
            }
            else {
                println!("Fluid sim ready");
            }
        }
    }
}
