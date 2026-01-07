use bevy::prelude::*;
use bevy::window::WindowResolution;

use crate::{ExtendedUiPlugin, ExtendedUiConfiguration};

pub fn make_app(title: impl Into<String>) -> App {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: title.into(),
            resolution: WindowResolution::new(1270, 720),
            ..default()
        }),
        ..default()
    }))
        .add_plugins(ExtendedUiPlugin)
        .add_systems(Startup, setup);

    app
}

fn setup(mut commands: Commands, mut configuration: ResMut<ExtendedUiConfiguration>) {
    commands.spawn(Camera2d);
    configuration.enable_default_camera = false;
}
