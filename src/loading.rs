use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .load_collection::<TextureAssets>()
                .continue_to_state(GameState::Menu),
        );
    }
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "HeightMap_24.png")]
    pub heightmap0: Handle<Image>,
    #[asset(path = "heightmap.png")]
    pub heightmap1: Handle<Image>,
    #[asset(path = "terrain.png")]
    pub heightmap2: Handle<Image>,
}
