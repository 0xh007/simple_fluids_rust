use nalgebra::{Isometry2, Point2, Point3, Vector2};

use ncollide2d::shape::{Cuboid, ShapeHandle};

use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::object::{
    BodyPartHandle, ColliderDesc, DefaultBodySet, DefaultColliderSet, Ground, RigidBodyDesc,
};
use nphysics2d::world::{DefaultGeometricalWorld, DefaultMechanicalWorld};

use salva2d::object::{Boundary, Fluid};
use salva2d::solver::{ArtificialViscosity, IISPHSolver};
use salva2d::LiquidWorld;

pub fn create_liquid_world() {
    let particle_rad = 0.1;
    let solver = IISPHSolver::<f32>::new();
    let mut liquid_world = LiquidWorld::new(solver, particle_rad, 2.0);
    // TODO: Add physics coupling set

    // Liquid
    let mut points1 = Vec::new();
    let mut points2 = Vec::new();
    let ni = 25;
    let nj = 15;

    let shift2 = (nj as f32) * particle_rad * 2.0;

    for i in 0..ni {
        for j in 0..nj {
            let x = (i as f32) * particle_rad * 2.0 - ni as f32 * particle_rad;
            let y = (j as f32 + 1.0) * particle_rad * 2.0;
            points1.push(Point2::new(x, y));
            points2.push(Point2::new(x, y + shift2));
        }
    }

    let viscosity = ArtificialViscosity::new(0.5, 0.0);
    let mut fluid = Fluid::new(points1, particle_rad, 1.0);
    fluid.nonpressure_forces.push(Box::new(viscosity.clone()));
    let fluid_handle = liquid_world.add_fluid(fluid);
}
