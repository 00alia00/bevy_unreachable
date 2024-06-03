// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_game::GamePlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "test".to_string(),
                        ..default()
                    }),
                    ..default()
                })
        )
        .add_plugins(WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::F1)))
        .add_plugins(GamePlugin)
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {

    //  doesn't crash
    // commands.spawn( Camera2dBundle::default() );

    //Unomment this to crash
    commands.spawn( Camera3dBundle::default() );
}
