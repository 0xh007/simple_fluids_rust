use amethyst::{
    core::TransformBundle,
    input::{
        InputBundle,
        StringBindings,
    },
    prelude::*,
    renderer::{
        RenderingBundle,
        RenderDebugLines,
        RenderFlat2D,
        RenderToWindow,
        types::DefaultBackend,
    },
    utils::application_root_dir,
    window::DisplayConfig,
};

mod components;
mod states;
mod systems;
mod utils;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let assets = app_root.join("assets");
    let config = app_root.join("config");

    let display_config = config.join("display.ron");
    let key_bindings_config = config.join("input.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new()
                .with_bindings_from_file(key_bindings_config)?
        )?
        .with_bundle(RenderingBundle::<DefaultBackend>::new()
            .with_plugin(
                RenderToWindow::from_config_path(display_config)?
                    .with_clear([0.0, 0.0, 0.0, 1.0]),
            )
            .with_plugin(RenderDebugLines::default())
            .with_plugin(RenderFlat2D::default())
        )?
        .with(systems::fluid_system::FluidSystem, "fluid_system", &[]);

    let mut game = Application::build(assets, states::GameState)?.build(game_data)?;
    game.run();

    Ok(())
}
