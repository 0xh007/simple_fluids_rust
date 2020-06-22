use amethyst::ecs::{
    Component,
    DenseVecStorage,
};

const FLUID_SIZE: usize = 256;
const ITER: i32 = 10;

fn advect(
    b: i32,
    mut density: [f32; FLUID_SIZE],
    density_prev: [f32; FLUID_SIZE],
    velocity_x: [f32; FLUID_SIZE],
    velocity_y: [f32; FLUID_SIZE],
    dt: f32) {

    let fluid_size: f32 = FLUID_SIZE as f32;
    let dtx: f32 = dt * (fluid_size - 2.0);
    let dty: f32 = dt * (fluid_size - 2.0);

    for j in (1..FLUID_SIZE) {
        for i in (1..FLUID_SIZE) {
            let i_idx: i32 = i as i32;
            let j_idx: i32 = j as i32;

            let velocity_x_idx: usize = get_index(i_idx, j_idx);
            let velocity_y_idx: usize = get_index(i_idx, j_idx);

            let temp_1: f32 = dtx * velocity_x[velocity_x_idx];
            let temp_2: f32 = dtx * velocity_y[velocity_y_idx];

            let mut x: f32 = (i_idx as f32) - temp_1;
            let mut y: f32 = (j_idx as f32) - temp_2;

            if x < 0.5 {
                x = 0.5;
            }

            if x > FLUID_SIZE as f32 + 0.5 {
                x = FLUID_SIZE as f32 + 0.5;
            }

            let i_flt: f32 = i_idx as f32;
            let j_flt: f32 = j_idx as f32;

            let i_prev: f32 = x.floor();
            let i_prev_int: i32 = i_prev as i32;

            let i_next: f32 = i_prev + 1.0;
            let i_next_int: i32 = i_next as i32;

            if (y < 0.5) {
                y = 0.5;
            }

            if y > FLUID_SIZE as f32 + 0.5 {
                y = FLUID_SIZE as f32 + 0.5;
            }

            let j_prev: f32 = y.floor();
            let j_prev_int: i32 = j_prev as i32;

            let j_next: f32 = j_prev + 1.0;
            let j_next_int: i32 = j_next as i32;

            let s_next: f32 = x - i_prev;
            let s_prev: f32 = 1.0 - s_next;

            let t_next: f32 = y - j_prev;
            let t_prev: f32 = 1.0 - t_next;

            let new_density: f32 = 
                s_prev * (t_prev * density_prev[get_index(i_prev_int, j_prev_int)] + t_next * density_prev[get_index(i_prev_int, j_next_int)])
                + s_next * (t_prev * density_prev[get_index(i_next_int, j_prev_int)] + t_next * density_prev[get_index(i_next_int, j_next_int)]);

            density[get_index(i_idx, j_idx)] = new_density;
        }
    }

    set_boundary(b, density);
}

fn get_index(x: i32, y: i32) -> usize {
    let size = FLUID_SIZE as i32;
    let index = (x * y * size) as usize;
    return index;
}

fn diffuse(b: i32, x: [f32; FLUID_SIZE], x_0: [f32; FLUID_SIZE], diff: f32, dt: f32) {
    let size = FLUID_SIZE as f32;
    let a: f32 = dt * diff * (size - 2.0) * (size - 2.0);

    linear_solve(b, x, x_0, a, 1.0 + 6.0 * a);
}

fn linear_solve(b: i32, mut x: [f32; FLUID_SIZE], x_0: [f32; FLUID_SIZE], a: f32, c: f32) {
    let c_recip: f32 = 1.0 / c;

    for k in 0..ITER {
        for j in 1..FLUID_SIZE {
            for i in 1..FLUID_SIZE {
                let i_idx: i32 = i as i32;
                let j_idx: i32 = j as i32;

                x[get_index(i_idx, j_idx)] =
                    (x_0[get_index(i_idx, j_idx)]
                     + a
                     * (x[get_index(i_idx + 1, j_idx)] 
                         + x[get_index(i_idx - 1, j_idx)]
                         + x[get_index(i_idx, j_idx + 1)]
                         + x[get_index(i_idx, j_idx - 1)]
                       )
                    )
                    * c_recip;
            }
        }

        set_boundary(b, x);
    }
}

fn project(mut velocity_x: [f32; FLUID_SIZE], mut velocity_y: [f32; FLUID_SIZE], mut p: [f32; FLUID_SIZE], mut div: [f32; FLUID_SIZE]) {
    let fluid_size_flt: f32 = FLUID_SIZE as f32;
    for j in 1..FLUID_SIZE {
        for i in 1..FLUID_SIZE {
            let i_idx: i32 = i as i32;
            let j_idx: i32 = j as i32;

            let velocity_x_next_index: usize = get_index(i_idx + 1, j_idx);
            let velocity_x_prev_index: usize = get_index(i_idx - 1, j_idx);

            let velocity_y_next_index: usize = get_index(i_idx, j_idx + 1);
            let velocity_y_prev_index: usize = get_index(i_idx, j_idx - 1);

            let new_velocity_x: f32 = velocity_x[velocity_x_next_index] - velocity_x[velocity_x_prev_index];
            let new_velocity_y: f32 = velocity_y[velocity_y_next_index] - velocity_y[velocity_y_prev_index];

            let new_div: f32 = -0.5 * (new_velocity_x - new_velocity_y) / fluid_size_flt;

            div[get_index(i_idx, j_idx)] = new_div;

            p[get_index(i_idx, j_idx)] = 0.0;
        }
    }

    set_boundary(0, div);
    set_boundary(0, p);
    linear_solve(0, p, div, 1.0, 6.0);

    for j in 1..FLUID_SIZE {
        for i in 1..FLUID_SIZE {
            let i_idx: i32 = i as i32;
            let j_idx: i32 = j as i32;

            let new_velocity_x: f32 = 0.5 * (p[get_index(i_idx + 1, j_idx)] - p[get_index(i_idx - 1, j_idx)]) * fluid_size_flt;
            let new_velocity_y: f32 = 0.5 * (p[get_index(i_idx, j_idx + 1)] - p[get_index(i_idx, j_idx - 1)]) * fluid_size_flt;

            velocity_x[get_index(i_idx, j_idx)] = new_velocity_x;
            velocity_y[get_index(i_idx, j_idx)] = new_velocity_y;
        }
    }

    set_boundary(1, velocity_x);
    set_boundary(2, velocity_y);
}

fn set_boundary(b: i32, mut x: [f32; FLUID_SIZE]) {
    let fluid_size_int: i32 = FLUID_SIZE as i32;

    for i in 1..FLUID_SIZE {
        let i_idx: i32 = i as i32;

        let new_next_x: f32;
        let new_prev_x: f32;

        let new_next_y: f32;
        let new_prev_y: f32;

        if b == 2 {
            new_next_x = -x[get_index(i_idx, 1)];
            new_prev_x = -x[get_index(i_idx, fluid_size_int - 2)];
        } else {
            new_next_x = x[get_index(i_idx, 1)];
            new_prev_x = x[get_index(i_idx, fluid_size_int - 2)];
        }

        if b == 1 {
            new_next_y = -x[get_index(1, i_idx)];
            new_prev_y = -x[get_index(fluid_size_int - 2, i_idx)];
        } else {
            new_next_y = x[get_index(1, i_idx)];
            new_prev_y = x[get_index(fluid_size_int - 2, i_idx)];
        }

        x[get_index(i_idx, 0)] = new_next_x;
        x[get_index(i_idx, fluid_size_int - 1)] = new_prev_x;

        x[get_index(0, i_idx)] = new_next_y;
        x[get_index(fluid_size_int - 1, i_idx)] = new_prev_y;
    }

    x[get_index(0, 0)] =
        0.5 
        * (x[get_index(1, 0)] 
        + x[get_index(0, 1)]);

    x[get_index(0, fluid_size_int - 1)] =
        0.5 * (x[get_index(1, fluid_size_int - 1)]
        + x[get_index(0, fluid_size_int - 2)]);

    x[get_index(fluid_size_int - 1, 0)] =
        0.5
        * (x[get_index(fluid_size_int - 2, 0)]
        + x[get_index(fluid_size_int - 1, 1)]);

    x[get_index(fluid_size_int - 1, fluid_size_int - 1)] =
        0.5
        * (x[get_index(fluid_size_int - 2, fluid_size_int - 1)]
        + x[get_index(fluid_size_int - 1, fluid_size_int - 2)]);
}

pub struct FluidWorld {
    dt: f32,
    diffusion: f32,
    viscosity: f32,

    s: [f32; FLUID_SIZE],
    density: [f32; FLUID_SIZE],

    velocity_x: [f32; FLUID_SIZE],
    velocity_y: [f32; FLUID_SIZE],

    velocity_x_prev: [f32; FLUID_SIZE],
    velocity_y_prev: [f32; FLUID_SIZE],
}

impl FluidWorld {
    pub fn new(dt: f32, diffusion: f32, viscosity: f32) -> FluidWorld {
        println!("Creating fluid world");
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

    pub fn step(mut self) {
        diffuse(1, self.velocity_x_prev, self.velocity_x, self.viscosity, self.dt);
        diffuse(2, self.velocity_y_prev, self.velocity_y, self.viscosity, self.dt);

        project(self.velocity_x_prev, self.velocity_y_prev, self.velocity_x, self.velocity_y);

        advect(1, self.velocity_x, self.velocity_x_prev, self.velocity_x_prev, self.velocity_y_prev, self.dt);
        advect(2, self.velocity_y, self.velocity_y_prev, self.velocity_x_prev, self.velocity_y_prev, self.dt);

        project(self.velocity_x, self.velocity_y, self.velocity_x_prev, self.velocity_y_prev);

        diffuse(0, self.s, self.density, self.diffusion, self.dt);
    }
}


impl Component for FluidWorld {
    type Storage = DenseVecStorage<Self>;
}
