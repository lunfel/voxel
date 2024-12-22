use bevy::math::Vec3;
use bevy::pbr::{CascadeShadowConfigBuilder, DirectionalLight};
use bevy::prelude::*;
use std::f32::consts::PI;
use crate::world::world_generation::generate_world;

pub struct SunPlugin;

impl Plugin for SunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, make_the_sun_move_around)
            .add_systems(Startup, setup.after(generate_world));
    }
}

fn setup(
    mut commands: Commands
) {
    info!("Inserting light in the world");
    commands.insert_resource(AmbientLight {
        brightness: 80.0,
        ..default()
    });

    // directional 'sun' light
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            ..default()
        }.build(),
        Transform {
            translation: Vec3::new(0.0, 250.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        }
    ));
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