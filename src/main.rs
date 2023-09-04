mod components;
mod systems;

use std::f32::consts::PI;

use bevy::{prelude::*, diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin}, pbr::CascadeShadowConfigBuilder};
use rand::Rng;
use systems::player::player_control::PlayerPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .init_resource::<WorldSettings>()
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_physics)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .run();
} 

fn setup_physics(mut commands: Commands) {
    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(0.5))
        .insert(Restitution::coefficient(0.1))
        .insert(TransformBundle::from(Transform::from_xyz(16.0, 20.0, 16.0)));
}

#[derive(Resource)]
pub struct WorldSettings {
    chunk_size: u32,
    unique_blocks: usize 
}

impl Default for WorldSettings {
    fn default() -> Self {
        Self {
            chunk_size: 16,
            unique_blocks: 4
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    world_settings: Res<WorldSettings>
) {
    let chunk_size = world_settings.chunk_size;
    let color_range = 0.0..1.0;
    let mut rng = rand::thread_rng();

    let mesh_handle = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let materials_handles: Vec<Handle<StandardMaterial>> = {
        (0..world_settings.unique_blocks).into_iter().map(|index| {
            let red = rng.gen_range(color_range.clone());
            let green = rng.gen_range(color_range.clone());
            let blue = rng.gen_range(color_range.clone());

            materials.add(Color::rgb(red, green, blue).into())
        }).collect::<Vec<Handle<StandardMaterial>>>()
    };

    for x in 0..=chunk_size {
        for y in 0..=chunk_size {
            for z in 0..=chunk_size {
                if y == 3 || y == 0 {
                    // let height = rng.gen_range(0..3);
                    let material_index = rng.gen_range(0..world_settings.unique_blocks);

                    commands.spawn((PbrBundle {
                        mesh: mesh_handle.clone(),
                        material: materials_handles.get(material_index).expect("Material does not exist").clone(),
                        transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                        ..default()
                    },
                        Collider::cuboid(0.5, 0.5, 0.5),
                        Friction {
                            coefficient: 0.0,
                            combine_rule: CoefficientCombineRule::Min
                        }));
                }
            }
        }
    }

    commands.insert_resource(AmbientLight {
        brightness: 0.15,
        ..default()
    });

    // directional 'sun' light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
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
