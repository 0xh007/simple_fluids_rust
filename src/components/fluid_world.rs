use amethyst::ecs::{
    Component,
    DenseVecStorage,
};

use crate::utils::fluid_util::*;

pub struct FluidWorld {
    pub dt: f32,
    pub diffusion: f32,
    pub viscosity: f32,

    pub started: bool,

    pub s: [f32; FLUID_SIZE],
    pub density: [f32; FLUID_SIZE],

    pub velocity_x: [f32; FLUID_SIZE],
    pub velocity_y: [f32; FLUID_SIZE],

    pub velocity_x_prev: [f32; FLUID_SIZE],
    pub velocity_y_prev: [f32; FLUID_SIZE],
}

impl FluidWorld {
    pub fn new(dt: f32, diffusion: f32, viscosity: f32) -> FluidWorld {
        FluidWorld {
            dt,
            diffusion,
            viscosity,
            s: [0.0; FLUID_SIZE],
            density: [0.0; FLUID_SIZE],
            velocity_x: [0.0; FLUID_SIZE],
            velocity_y: [0.0; FLUID_SIZE],
            velocity_x_prev: [0.0; FLUID_SIZE],
            velocity_y_prev: [0.0; FLUID_SIZE],
            started: false,
        }
    }

    pub fn add_density(mut self, x: i32, y: i32, amount: f32) {
        let index: usize = get_index(x, y);
        self.density[index] += amount;
    }

    pub fn add_velocity(mut self, x: i32, y: i32, amount_x: f32, amount_y: f32) {
        let index: usize = get_index(x, y);
        self.velocity_x[index] += amount_x;
        self.velocity_y[index] += amount_y;
    }
}

impl Component for FluidWorld {
    type Storage = DenseVecStorage<Self>;
}
