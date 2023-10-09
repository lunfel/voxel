use bevy::prelude::Resource;
use bevy::app::App;
use bevy::prelude::*;

use crate::utils::point::Point3D;
use crate::world::systems::chunk::{debug_collider_counts, DebugColliderTimer, enable_close_colliders};

pub mod block;
pub mod chunk;
pub mod systems;

pub struct GameCoord(Point3D<i32>);

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        info!("WorldPlugin initializing");
        app
            .insert_resource(DebugColliderTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Update, debug_collider_counts);
            // .add_systems(Update, render_dirty_chunk)
            // .add_systems(Update, enable_close_colliders)
            // .add_systems(Update, disable_far_colliders);
        info!("WorldPlugin loaded");
    }
}