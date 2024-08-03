mod terrain;
mod terrain_modification;

pub use terrain::{spawn_terrain, Terrain, TerrainBundle, TerrainCollisionData};
pub use terrain_modification::{
    TerrainModification, TerrainModificationExtensions, TerrainModificationOp,
    TerrainModificationShape,
};

use bevy::prelude::*;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {}
}
