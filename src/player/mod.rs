pub mod control;
mod selection;

use std::f32::consts::PI;

use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;

use crate::{
    player::control::{
        cursor_grab, follow_player_look_left_right, initial_grab_cursor, player_look, player_move,
        setup_player, InputState, JumpTimer, KeyBindings, MovementSettings,
    }
};
use crate::player::control::{follow_player_look_up_down, follow_player_position};
use crate::world::world_generation::generate_world;

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
            .add_systems(Update, make_the_sun_move_around)
            .add_systems(Update, player_move)
            .add_systems(Update, player_look)
            .add_systems(Update, follow_player_look_left_right)
            .add_systems(Update, follow_player_look_up_down)
            .add_systems(Update, follow_player_position)
            .add_systems(Update, cursor_grab);

        info!("PlayerPlugin loaded");
    }
}

fn setup(
    mut commands: Commands,
    mut meshes:  ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    info!("Inserting light in the world");
    commands.insert_resource(AmbientLight {
        brightness: 80.0,
        ..default()
    });

    // directional 'sun' light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            ..default()
        }.into(),
        transform: Transform {
            translation: Vec3::new(0.0, 250.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        ..default()
    });
    // .insert(PbrBundle {
    //     mesh: meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0))),
    //     material: materials.add(Color::rgb(0.9, 0.3, 0.6)),
    //     transform: Transform {
    //         translation: Vec3::new(0.0, 250.0, 0.0),
    //         rotation: Quat::from_rotation_x(-PI / 4.),
    //         ..default()
    //     },
    //     ..default()
    // });
}

fn make_the_sun_move_around(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>
) {
    for mut transform in query.iter_mut() {
        let trans = Transform::from_xyz(0.0, transform.translation.y, 0.0)
            .looking_at(Vec3::new(
                (time.elapsed_secs() / 100.0).cos() * 100.0,
                0.0,
                (time.elapsed_secs() / 100.0).sin() * 100.0,
            ), Vec3::Y);

        transform.translation = trans.translation;
        transform.rotation = trans.rotation;
    }
}