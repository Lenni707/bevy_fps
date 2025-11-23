use bevy::prelude::*;

// todo:
// -> procedual terrain generation with Noise-Driven Heightmap Terrain

use crate::player::PlayerPlugin;
use crate::world::WorldPlugin;
use crate::snowflake::SnowflakePlugin;

mod player;
mod world;
mod snowflake;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlayerPlugin, WorldPlugin, SnowflakePlugin))
        .run();
}
