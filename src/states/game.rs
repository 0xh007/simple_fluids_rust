use amethyst::{
    core::{
        math::Vector3,
        Named,
        Transform,
    },
    input::{
        is_close_requested,
        is_key_down,
    },
    prelude::*,
    renderer::camera::Camera,
    window::ScreenDimensions,
    winit,
};

use crate::{
    components::player::Player,
    utils::camera_util::*,
    utils::debug_util::*,
    utils::player_util::*,
    utils::sprite_util::*,
};

pub struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        let world = _data.world;
        world.register::<Named>();
        world.register::<Player>();

        let (width, height) = {
            let dim = world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        };

        let player = init_player(world);

        let _camera = init_camera(
            world,
            player,
            Transform::from(Vector3::new(0.0, 0.0, 1.1)),
            Camera::standard_2d(width, height),
        );

        // Creating a test debug lines entity
        let _ = create_debug_lines(world);
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        let StateData { .. } = data;
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, winit::VirtualKeyCode::Escape) {
                Trans::Quit
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }
}
