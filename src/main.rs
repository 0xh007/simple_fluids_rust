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
mod utils;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let resources = application_root_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()
        + "/resources";

    let display_config_path = resources.clone() + "/config/display.ron";
    let key_bindings_path = resources.clone() + "/config/input.ron";

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new()
                .with_bindings_from_file(key_bindings_path)?
        )?
        .with_bundle(RenderingBundle::<DefaultBackend>::new()
            .with_plugin(
                RenderToWindow::from_config_path(display_config_path)?
                    .with_clear([0.34, 0.36, 0.52, 1.0]),
            )
            .with_plugin(RenderDebugLines::default())
            .with_plugin(RenderFlat2D::default())
        )?;

    let mut game = Application::build(resources, states::GameState)?.build(game_data)?;
    game.run();

    Ok(())
}
