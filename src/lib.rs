#![allow(clippy::type_complexity)]
#![allow(dead_code)]

mod loading;
mod render_heightmap;
mod util;

use crate::loading::LoadingPlugin;
use crate::render_heightmap::RenderHeightMapPlugin;

use bevy::app::App;
use bevy::prelude::*;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    #[default]
    Loading,
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_plugins((
            LoadingPlugin,
            RenderHeightMapPlugin
        ));
    }
}
