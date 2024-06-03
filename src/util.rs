
use bevy::{
    prelude::*,
    render::{mesh::Indices, render_asset::RenderAssetUsages, render_resource::*},
};

#[derive(Debug, Copy, Clone)]
pub struct Plane {
    pub size: u32,
    pub num_divs: u32,
}

impl From<Plane> for Mesh {
    fn from(terrain: Plane) -> Self {
        let size = terrain.size as f32;
        let num_divs = terrain.num_divs as usize;
        let step = size / terrain.num_divs as f32;

        // Generate vertices and normals
        let vertices: Vec<_> = (0..=num_divs)
            .flat_map(|x| {
                (0..=num_divs).map(move |y| {
                    (
                        // position
                        [
                            x as f32 * step - 0.5 * size,
                            0.0,
                            y as f32 * step - 0.5 * size,
                        ],
                        // normal
                        [0.0, 1.0, 0.0],
                        // uv
                        [x as f32 / num_divs as f32, y as f32 / num_divs as f32],
                        // colour
                        [0.0, 0.0, 0.0, 1.0],
                    )
                })
            })
            .collect();

        let positions: Vec<_> = vertices.iter().map(|(p, _, _, _)| *p).collect();
        let normals: Vec<_> = vertices.iter().map(|(_, n, _, _)| *n).collect();
        let uvs: Vec<_> = vertices.iter().map(|(_, _, uv, _)| *uv).collect();
        let colours: Vec<_> = vertices.iter().map(|(_, _, _, c)| *c).collect();

        // Generate indices
        let mut indices = Vec::new();
        for x in 0..num_divs {
            for y in 0..num_divs {
                let top_left = x * (num_divs + 1) + y;
                let top_right = top_left + 1;
                let bottom_left = top_left + num_divs + 1;
                let bottom_right = bottom_left + 1;

                indices.push(top_left as u32);
                indices.push(bottom_left as u32);
                indices.push(top_right as u32);

                indices.push(top_right as u32);
                indices.push(bottom_left as u32);
                indices.push(bottom_right as u32);
            }
        }

        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        );

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colours);

        mesh.insert_indices(Indices::U32(indices));

        mesh
    }
}