use bevy::prelude::*;

use noise::NoiseFn; // neede for .get func on noise

use bevy::render::mesh::{Mesh, Indices, PrimitiveTopology}; // for rendering specifc meshes with indices
use bevy::render::render_asset::RenderAssetUsages;

use crate::world_gen::*; // link to world gen module
use crate::noise::NoiseGenerators;

// biome shit
pub const BIOME_FREQ: f64 = 0.0008;

pub const RIVER_FREQ: f64 = 0.0015;
pub const RIVER_WIDTH: f32 = 0.05;

pub const PLAINS_SCALE: f32 = 0.5;
pub const FOREST_SCALE: f32 = 1.0;
pub const MOUNTAIN_SCALE: f32 = 3.0;

pub const CLIFF_SLOPE: f32 = 0.30;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Biome {
    Plains,
    Forest,
    Mountains,
    River
}

pub fn calc_to_generate_chunk(coord: ChunkCoord, noise: &NoiseGenerators,) -> Mesh {
    let mut positions = Vec::new();
    let mut colors: Vec<[f32; 4]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    for z in 0..CHUNK_SIZE { // calc vertices
        for x in 0..CHUNK_SIZE {
            let world_x = (coord.x * CHUNK_SIZE as i32 + x as i32) as f64;
            let world_z = (coord.z * CHUNK_SIZE as i32 + z as i32) as f64;

            // calc for rivers chatty meinte mit .abs sehen die cooler aus
            let river_val = noise.river.get([world_x * RIVER_FREQ, world_z * RIVER_FREQ]).abs() as f32;
            let is_river = river_val < RIVER_WIDTH;

            // calc für biomes
            let biome_val = noise.biome.get([world_x * BIOME_FREQ, world_z * BIOME_FREQ]) as f32;

            // check which biome it is based on noise level
            let biome = if is_river {
                Biome::River
            } else if biome_val < -0.2 {
                Biome::Plains
            } else if biome_val < 0.3 {
                Biome::Forest
            } else {
                Biome::Mountains
            };

            // height pro biom
            let base_h = noise.height.get([world_x * NOISE_FREQ, world_z * NOISE_FREQ]) as f32;

            let height = match biome {
                Biome::Plains => base_h * NOISE_AMP * PLAINS_SCALE,
                Biome::Forest => base_h * NOISE_AMP * FOREST_SCALE,
                Biome::Mountains => base_h.abs() * NOISE_AMP * MOUNTAIN_SCALE,
                Biome::River => base_h * NOISE_AMP * PLAINS_SCALE,
            };

            positions.push([
                x as f32 * VERTEX_SPACING,
                height,
                z as f32 * VERTEX_SPACING,
            ]);

            // get steepness of slope
            let h_l = noise.height.get([(world_x - 1.0) * NOISE_FREQ, world_z * NOISE_FREQ]) as f32;
            let h_r = noise.height.get([(world_x + 1.0) * NOISE_FREQ, world_z * NOISE_FREQ]) as f32;
            let h_u = noise.height.get([world_x * NOISE_FREQ, (world_z + 1.0) * NOISE_FREQ]) as f32;
            let h_d = noise.height.get([world_x * NOISE_FREQ, (world_z - 1.0) * NOISE_FREQ]) as f32;

            let slope = ((h_l - h_r).abs() + (h_u - h_d).abs()) * 0.5;

            let color = if biome == Biome::River {
                [0.15, 0.55, 0.90, 1.0]
            } else if slope > CLIFF_SLOPE {
                [0.25, 0.25, 0.25, 1.0]
            } else {
                match biome {
                    Biome::Plains   => [0.70, 1.0, 0.70, 1.0],
                    Biome::Forest   => [0.10, 0.40, 0.10, 1.0],
                    Biome::Mountains => [1.0, 1.0, 1.0, 1.0],
                    Biome::River => unreachable!(),
                }
            };

            colors.push(color);
        }
    }

    for z in 0..CHUNK_SIZE - 1 { // calc indices (triangles connecting vertices)
        for x in 0..CHUNK_SIZE - 1 {
            let i = z * CHUNK_SIZE + x;

            indices.extend_from_slice(&[
                i as u32,
                (i + CHUNK_SIZE) as u32,
                (i + 1) as u32,
                (i + 1) as u32,
                (i + CHUNK_SIZE) as u32,
                (i + CHUNK_SIZE + 1) as u32,
            ]);
        }
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_indices(Indices::U32(indices));
    mesh.compute_normals();

    mesh
}
