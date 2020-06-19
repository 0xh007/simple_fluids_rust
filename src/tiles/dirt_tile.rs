use amethyst::{
    core::math::Point3,
    prelude::*,    
};
use amethyst_tiles::Tile;

#[derive(Default, Clone)]
pub struct DirtTile;
impl Tile for DirtTile {
    fn sprite(&self, _: Point3<u32>, _: &World) -> Option<usize> {
        Some(1)
    }
}
