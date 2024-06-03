use bevy::{
    asset::Asset,
    log,
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, ShaderType,
            SpecializedMeshPipelineError,
        },
    },
};
use bevy_inspector_egui::InspectorOptions;

use crate::{loading::TextureAssets, util::Plane, GameState};

pub struct RenderHeightMapPlugin;

impl Plugin for RenderHeightMapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(MaterialPlugin::<HeightMapMaterial>::default())
            .add_systems(OnEnter(GameState::Menu), setup);
    }
}

#[derive(Component)]
struct Terrain;

// TODO: Crash internal to bevy/naga doesnt matter if i use my material or not
fn setup(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    mut custom_materials: ResMut<Assets<HeightMapMaterial>>,
    textures: Res<TextureAssets>,
) {
    log::info!("Setting up heightmap");
    // let custom_texture_handle: Handle<Image> = asset_server.load("textures/terrain.png");

    const SIZE: u32 = 1024;
    let mesh = Mesh::from(Plane {
        size: 200,
        num_divs: SIZE,
    });

    let material = custom_materials.add(HeightMapMaterial {
        common_const: CommonConstants {
            tex_size: UVec2::new(SIZE, SIZE),
            height_scale: 50.0,
            height_offset: 0.0,
        },
        constants: Constants {
            chosen_map_type: 0,
            terrain_colour: 0,
            _padding0: 0,
            _padding1: 0,
        },
        height_map: textures.heightmap1.clone(),
        source_map: textures.heightmap2.clone(),
        colour_source_map: textures.heightmap0.clone(),
    });
    commands.spawn((
        MaterialMeshBundle::<HeightMapMaterial> {
            mesh: meshes.add(mesh),
            material,
            ..default()
        },
        Terrain,
        Name::new("heightmap"),
    ));

    //    let material = materials.add(StandardMaterial {
    //     base_color_texture: Some(custom_texture_handle),
    //     ..default()
    // });
    //     commands.spawn((
    //     PbrBundle
    //      {
    //         mesh: meshes.add(mesh),
    //         material,
    //         ..default()
    //     },
    //     Terrain,
    //     Name::new("heightmap"),
    // ));

    log::info!("Heightmap setup complete");
}

#[derive(Debug, Clone, Default, ShaderType, Reflect)]
struct CommonConstants {
    tex_size: UVec2,
    height_scale: f32,
    height_offset: f32,
}

#[derive(Debug, Clone, Default, ShaderType, Reflect)]
struct Constants {
    chosen_map_type: u32,
    terrain_colour: u32,
    _padding0: u32,
    _padding1: u32,
}

#[derive(Asset, Debug, Clone, AsBindGroup, Default, Resource, Reflect, InspectorOptions)]
struct HeightMapMaterial {
    #[uniform(0)]
    common_const: CommonConstants,
    #[uniform(1)]
    constants: Constants,

    #[texture(2)]
    #[sampler(5)]
    height_map: Handle<Image>, // height map
    #[texture(3)]
    source_map: Handle<Image>, // water or height map
    #[texture(4)]
    #[sampler(6)]
    colour_source_map: Handle<Image>,
}

impl Material for HeightMapMaterial {
    fn vertex_shader() -> ShaderRef {
        "render_heightmap_vs.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "render_heightmap_vs.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>, descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout, _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = None;
        if let Some(label) = &mut descriptor.label {
            *label = format!("render_heightmap_{}", *label).into();
        }
        Ok(())
    }
}
