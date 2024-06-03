#import bevy_pbr::mesh_view_bindings::view;
#import bevy_pbr::mesh_functions;
#import bevy_pbr::view_transformations;

// #import "shaders/wgsl/common_const.wgsl"::CommonConstants

struct CommonConstants {
    tex_size: vec2<u32>,
    height_scale: f32,
    height_offset: f32,
};
@group(2) @binding(0) var<uniform> common_const: CommonConstants;

struct Constants {
    chosen_map_type: u32,
    terrain_colour: u32,
    igore_0: u32,
    igore_1: u32,
}
@group(2) @binding(1) var<uniform> constants: Constants;

@group(2) @binding(2) var height_map: texture_2d<f32>;
@group(2) @binding(3) var source_map: texture_2d<f32>;
@group(2) @binding(4) var colour_source_map: texture_2d<f32>;


@group(2) @binding(5) var height_map_sampler: sampler;
@group(2) @binding(6) var colour_source_map_sampler: sampler;


const MAPTYPE_SINGLE_HEIGHT:u32 = 0;
const MAPTYPE_SINGLE_FLOW_DEPTH:u32 = 1;

const COLOUR_TYPE_COLOUR_TEX:u32 = 0;
const COLOUR_TYPE_COLOUR_WATER:u32 = 1;

struct VertexInput {
    @builtin(instance_index) instance_id: u32,
    @builtin(vertex_index) vertex_index: u32,
    @location(0)position: vec3<f32>,//ignored
    @location(1)normal: vec3<f32>, //ignored
    @location(2)uv: vec2<f32>,
    @location(5)colour: vec4<f32>, //ignored
};
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(2) uv: vec2<f32>,
    @location(5) @interpolate(flat) color: vec4<f32>,
};

@vertex
fn vertex(input: VertexInput) -> VertexOutput {
    let tex_coords: vec2<i32> = vec2<i32>(input.uv * vec2<f32>(f32(common_const.tex_size.x)));
    let value  = textureLoad(source_map, tex_coords, 0).r*common_const.height_scale + common_const.height_offset;
    var position: vec3<f32> = vec3<f32>(input.position.x, value, input.position.z);

    if constants.chosen_map_type != MAPTYPE_SINGLE_HEIGHT {
        if value < 1.0 {
            position.y = -1.0;
        } else {
            position.y = 0.01;
            position.y += textureLoad(height_map, tex_coords, 0).r * common_const.height_scale + common_const.height_offset;
        }
    }

    if constants.chosen_map_type == MAPTYPE_SINGLE_FLOW_DEPTH {
        if value < 0.0 {
            position = vec3<f32>(0.0, 0.0, 0.0);
        }
    }

    var model = mesh_functions::get_model_matrix(0u);

    var output: VertexOutput;
    output.world_position = mesh_functions::mesh_position_local_to_world(model, vec4<f32>(position, 1.0));
    output.position = mesh_functions::mesh_position_local_to_clip(model, vec4<f32>(position, 1.0));
    output.uv = input.uv;
    output.color = input.colour;
    return output;
}


@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    var colour =  textureSample(height_map, height_map_sampler, input.uv).rrrr;

    if constants.terrain_colour == COLOUR_TYPE_COLOUR_TEX {
        colour = textureSample(colour_source_map, colour_source_map_sampler, input.uv);
    } else if constants.terrain_colour == COLOUR_TYPE_COLOUR_WATER {
        colour = vec4<f32>(0.01, 0.6, input.world_position.y / 10.0, 0.9);
    }

    if constants.chosen_map_type == MAPTYPE_SINGLE_FLOW_DEPTH {
        if input.world_position.y < 0.0 { // could check against the height map
            // colour = vec4<f32>(0.01, 0.6, 0.6, 0.9);
            // clip
            discard;
        }
    }

    return colour;
}
