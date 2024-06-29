pub mod player_control;

use std::f32::consts::PI;

use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;

use crate::systems::{
    player::player_control::{
        cursor_grab, initial_grab_cursor, player_look, player_move, setup_player, InputState,
        JumpTimer, KeyBindings, MovementSettings,
    },
    world_generation::generate_world,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        info!("PlayerPlugin initializing");
        app.init_resource::<InputState>()
            .init_resource::<MovementSettings>()
            .init_resource::<KeyBindings>()
            .init_resource::<JumpTimer>()
            .add_systems(Startup, setup.after(generate_world))
            .add_systems(Startup, setup_player.after(generate_world))
            .add_systems(Startup, initial_grab_cursor.after(generate_world))
            .add_systems(Update, player_move)
            .add_systems(Update, player_look)
            .add_systems(Update, cursor_grab);

        info!("PlayerPlugin loaded");
    }
}

fn setup(mut commands: Commands) {
    info!("Inserting light in the world");
    commands.insert_resource(AmbientLight {
        brightness: 0.15,
        ..default()
    });

    // directional 'sun' light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 8000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 6.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .into(),
        ..default()
    });
}
