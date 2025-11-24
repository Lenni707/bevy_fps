use bevy::prelude::*;

// todo:
// -> procedual terrain generation with Noise-Driven Heightmap Terrain

use crate::player::PlayerPlugin;
use crate::world::WorldPlugin;
use crate::snowflake::SnowflakePlugin;
use crate::world_gen::WorldGenPlugin;
use crate::hud::HudPlugin;

mod player;
mod world;
mod snowflake;
mod world_gen;
mod chunks;
mod hud;
mod noise;

fn main() {
    App::new()
        .insert_resource(noise::NoiseGenerators::new(6767))
        .add_plugins((DefaultPlugins, PlayerPlugin, WorldPlugin, SnowflakePlugin, WorldGenPlugin, HudPlugin))
        .run();
}
