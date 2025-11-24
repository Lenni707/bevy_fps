use std::collections::HashMap; // for saving chunks
use bevy::prelude::*;
use noise::{Perlin, Seedable, NoiseFn}; // "random" noise generator
use bevy::render::mesh::{Mesh, Indices, PrimitiveTopology}; // for rendering specifc meshes with indices
use bevy::render::render_asset::RenderAssetUsages;

pub const CHUNK_SIZE: usize = 64;
pub const VERTEX_SPACING: f32 = 1.0; // wie viele verticies in einem chunk sind
pub const RENDER_DISTANCE: i32 = 2;
pub const SEED: u32 = 67;
pub const NOISE_FREQ: f64 = 0.05; // wie hart die übergänge sind
pub const NOISE_AMP: f32 = 20.0; // wie steil alles ist, also berge und so



#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ChunkCoord {
    pub x: i32,
    pub z: i32,
}

#[derive(Resource, Default)]
pub struct LoadedChunks {
    pub chunks: HashMap<ChunkCoord, Entity>,
}

pub struct WorldGenPlugin;

impl Plugin for WorldGenPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<LoadedChunks>();
            .add_systems(Update, chunk_system);
    }
}

pub fn chunk_system(
    mut commands: Commands,
    mut loaded: ResMut<LoadedChunks>,
    player_query: Query<&GlobalTransform, With<Camera3d>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Ok(player_transform) = player_query.single() else { return };
    let player_pos = player_transform.translation();

    let cx = (player_pos.x / (CHUNK_SIZE as f32 * VERTEX_SPACING)).floor() as i32; // chunk coords 
    let cz = (player_pos.z / (CHUNK_SIZE as f32 * VERTEX_SPACING)).floor() as i32;

    let mut wanted_chunk = Vec::new();
    for dx in -RENDER_DISTANCE..=RENDER_DISTANCE { // check which chunks should be loaded
        for dz in -RENDER_DISTANCE..=RENDER_DISTANCE {
            wanted_chunk.push(ChunkCoord { x: cx + dx, z: cz + dz });
        }
    }

    // todo:
    // actually add loading and unloading chumks
}

pub fn calc_to_generate_chunk(coord: ChunkCoord) -> Mesh {
    let perlin = Perlin::new().set_seed(SEED);

    let mut positions = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    for z in 0..CHUNK_SIZE { // calc vertices
        for x in 0..CHUNK_SIZE {
            let world_x = (coord.x * CHUNK_SIZE as i32 + x as i32) as f64;
            let world_z = (coord.z * CHUNK_SIZE as i32 + z as i32) as f64;

            let height = (perlin.get([world_x * NOISE_FREQ, world_z * NOISE_FREQ]) as f32) * NOISE_AMP; // calc height

            positions.push([x as f32 * VERTEX_SPACING, height, z as f32 * VERTEX_SPACING]);
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

    let mut mesh = Mesh::new( // create mesh
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions); // edit mesh based on vertices and indices
    mesh.insert_indices(Indices::U32(indices));
    mesh.compute_normals();

    mesh
}