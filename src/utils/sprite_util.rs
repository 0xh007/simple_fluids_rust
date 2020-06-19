use amethyst::{
    assets::{
        AssetStorage,
        Loader,
    },
    core::Transform,
    ecs::Entity,
    prelude::*,
    renderer::{
        formats::texture::ImageFormat,
        sprite::{
            SpriteRender,
            SpriteSheet,
            SpriteSheetFormat,
            SpriteSheetHandle,
        },
        Texture,
        Transparent,
    },
};

pub fn load_sprite_sheet(world: &mut World, png_path: &str, ron_path: &str) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(png_path, ImageFormat::default(), (), &texture_storage)
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();

    loader.load(
        ron_path,
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

pub fn init_reference_sprite(world: &mut World, sprite_sheet: &SpriteSheetHandle) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 0.1);

    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(transform)
        .with(sprite)
        .with(Transparent)
        .named("screen_reference")
        .build()
}
