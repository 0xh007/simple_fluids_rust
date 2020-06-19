use amethyst::{
    core::{
        math::Vector3,
        Transform,
    },
    ecs::Entity,
    prelude::*,
    renderer::sprite::SpriteSheetHandle,
};

use amethyst_tiles::{
    MortonEncoder,
    TileMap,
};

use crate::tiles::dirt_tile::*;

pub fn create_tile_map(sprite_sheet: SpriteSheetHandle) -> TileMap<DirtTile, MortonEncoder> {
    TileMap::<DirtTile, MortonEncoder>::new(
        Vector3::new(50, 50, 1),
        Vector3::new(50, 50, 1),
        Some(sprite_sheet),
    )
}

pub fn create_map_entity(world: &mut World, tile_map: TileMap<DirtTile, MortonEncoder>) -> Entity {
    world
        .create_entity()
        .with(tile_map)
        .with(Transform::default())
        .build()
}
