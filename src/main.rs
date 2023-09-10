mod components;
mod systems;
mod resources;

use std::{f32::consts::PI, cmp::min};

use bevy::{prelude::{*, shape::Box}, diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin}, pbr::CascadeShadowConfigBuilder, render::{mesh::Indices, render_resource::PrimitiveTopology}};
use resources::world::{GameWorld, CHUNK_SIZE, GameBlockType, GameChunk};
use systems::{player::player_control::PlayerPlugin, world_generation::generate_single_chunk};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .init_resource::<GameWorld>()
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, generate_world)
        .add_systems(Startup, setup.after(generate_world))
        .add_systems(Startup, setup_physics)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        // .add_plugins(WorldInspectorPlugin::new())
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

pub type Point3D = (i8, i8, i8);

fn generate_world(
    mut game_world: ResMut<GameWorld>
) {
    info!("Generate world chunks");
    for x in 0..11 {
        for z in 0..11 {
            let point = (x, 0, z);
            let chunk = generate_single_chunk(&point);

            game_world.chunks.insert(point, chunk);
        }
    }

}

fn create_custom_cube(coord: Point3D, chunk: &GameChunk) -> Option<Mesh> {
    // suppose Y-up right hand, and camera look from +z to -z  
    let sp = Box::new(1.0, 1.0, 1.0);

    let mut vertices: Vec<([f32; 3], [f32; 3], [f32; 2])> = vec![];
    let mut indices: Vec<u32> = vec![];

    let indices_template = [0, 1, 2, 2, 3, 0];
    let mut nb_faces = 0;

    if let Some(cmp_block) = chunk.get_block(&(coord.0, coord.1, coord.2 + 1)) {
        if cmp_block.block_type == GameBlockType::Empty {
            // Front
            vertices.push(([sp.min_x, sp.min_y, sp.max_z], [0., 0., 1.0], [0., 0.]));
            vertices.push(([sp.max_x, sp.min_y, sp.max_z], [0., 0., 1.0], [1.0, 0.]));
            vertices.push(([sp.max_x, sp.max_y, sp.max_z], [0., 0., 1.0], [1.0, 1.0]));
            vertices.push(([sp.min_x, sp.max_y, sp.max_z], [0., 0., 1.0], [0., 1.0]));

            indices_template.iter()
                .map(|i| i + nb_faces * 4)
                .for_each(|i| indices.push(i));

            nb_faces += 1;
        }
    }

    if let Some(cmp_block) = chunk.get_block(&(coord.0, coord.1, coord.2 - 1)) {
        if cmp_block.block_type == GameBlockType::Empty {
            // Back
            vertices.push(([sp.min_x, sp.max_y, sp.min_z], [0., 0., -1.0], [1.0, 0.]));
            vertices.push(([sp.max_x, sp.max_y, sp.min_z], [0., 0., -1.0], [0., 0.]));
            vertices.push(([sp.max_x, sp.min_y, sp.min_z], [0., 0., -1.0], [0., 1.0]));
            vertices.push(([sp.min_x, sp.min_y, sp.min_z], [0., 0., -1.0], [1.0, 1.0]));

            indices_template.iter()
                .map(|i| i + nb_faces * 4)
                .for_each(|i| indices.push(i));

            nb_faces += 1;
        }
    }

    if let Some(cmp_block) = chunk.get_block(&(coord.0 + 1, coord.1, coord.2)) {
        if cmp_block.block_type == GameBlockType::Empty {
            // Right
            vertices.push(([sp.max_x, sp.min_y, sp.min_z], [1.0, 0., 0.], [0., 0.]));
            vertices.push(([sp.max_x, sp.max_y, sp.min_z], [1.0, 0., 0.], [1.0, 0.]));
            vertices.push(([sp.max_x, sp.max_y, sp.max_z], [1.0, 0., 0.], [1.0, 1.0]));
            vertices.push(([sp.max_x, sp.min_y, sp.max_z], [1.0, 0., 0.], [0., 1.0]));

            indices_template.iter()
                .map(|i| i + nb_faces * 4)
                .for_each(|i| indices.push(i));

            nb_faces += 1;
        }
    }

    if let Some(cmp_block) = chunk.get_block(&(coord.0 - 1, coord.1, coord.2)) {
        if cmp_block.block_type == GameBlockType::Empty {
            // Left
            vertices.push(([sp.min_x, sp.min_y, sp.max_z], [-1.0, 0., 0.], [1.0, 0.]));
            vertices.push(([sp.min_x, sp.max_y, sp.max_z], [-1.0, 0., 0.], [0., 0.]));
            vertices.push(([sp.min_x, sp.max_y, sp.min_z], [-1.0, 0., 0.], [0., 1.0]));
            vertices.push(([sp.min_x, sp.min_y, sp.min_z], [-1.0, 0., 0.], [1.0, 1.0]));

            indices_template.iter()
                .map(|i| i + nb_faces * 4)
                .for_each(|i| indices.push(i));

            nb_faces += 1;
        }
    }

    if let Some(cmp_block) = chunk.get_block(&(coord.0, coord.1 + 1, coord.2)) {
        if cmp_block.block_type == GameBlockType::Empty {
            // Top
            vertices.push(([sp.max_x, sp.max_y, sp.min_z], [0., 1.0, 0.], [1.0, 0.]));
            vertices.push(([sp.min_x, sp.max_y, sp.min_z], [0., 1.0, 0.], [0., 0.]));
            vertices.push(([sp.min_x, sp.max_y, sp.max_z], [0., 1.0, 0.], [0., 1.0]));
            vertices.push(([sp.max_x, sp.max_y, sp.max_z], [0., 1.0, 0.], [1.0, 1.0]));

            indices_template.iter()
                .map(|i| i + nb_faces * 4)
                .for_each(|i| indices.push(i));

            nb_faces += 1;
        }
    }

    if let Some(cmp_block) = chunk.get_block(&(coord.0, coord.1 - 1, coord.2)) {
        if cmp_block.block_type == GameBlockType::Empty {
            // Top
            vertices.push(([sp.max_x, sp.min_y, sp.max_z], [0., -1.0, 0.], [0., 0.]));
            vertices.push(([sp.min_x, sp.min_y, sp.max_z], [0., -1.0, 0.], [1.0, 0.]));
            vertices.push(([sp.min_x, sp.min_y, sp.min_z], [0., -1.0, 0.], [1.0, 1.0]));
            vertices.push(([sp.max_x, sp.min_y, sp.min_z], [0., -1.0, 0.], [0., 1.0]));

            indices_template.iter()
                .map(|i| i + nb_faces * 4)
                .for_each(|i| indices.push(i));
        }
    }

    if nb_faces == 0 {
        return None
    }

    let positions: Vec<_> = vertices.iter().map(|(p, _, _)| *p).collect();
    let normals: Vec<_> = vertices.iter().map(|(_, n, _)| *n).collect();
    let uvs: Vec<_> = vertices.iter().map(|(_, _, uv)| *uv).collect();

    let indices = Indices::U32(indices);

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(indices));
    Some(mesh)
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game_world: Res<GameWorld>
) {
    // let mesh_handle = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let ground_material: Handle<StandardMaterial> = materials.add(Color::rgb(76.0 / 255.0, 153.0 / 255.0, 0.0 / 255.0).into());
    // let materials_handles: Vec<Handle<StandardMaterial>> = {
    //     (0..world_settings.unique_blocks).into_iter().map(|index| {
    //         let red = rng.gen_range(color_range.clone());
    //         let green = rng.gen_range(color_range.clone());
    //         let blue = rng.gen_range(color_range.clone());
    //
    //         materials.add(Color::rgb(red, green, blue).into())
    //     }).collect::<Vec<Handle<StandardMaterial>>>()
    // };

    info!("Inserting cubes in the world");
    for (coord, chunk) in game_world.chunks.iter() {
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    if let Some(block) = chunk.get_block(&(x as i8, y as i8, z as i8)) {
                        match block.block_type {
                            GameBlockType::Ground => {
                                if let Some(mesh) = create_custom_cube((x as i8, y as i8, z as i8), &chunk) {
                                    let mesh_handle = meshes.add(mesh);
                                    commands.spawn((PbrBundle {
                                        mesh: mesh_handle.clone(),
                                        material: ground_material.clone(),
                                        transform: Transform::from_xyz(
                                            x as f32 + coord.0 as f32 * CHUNK_SIZE as f32,
                                            y as f32 + coord.1 as f32 * CHUNK_SIZE as f32,
                                            z as f32 + coord.2 as f32 * CHUNK_SIZE as f32
                                        ),
                                        ..default()
                                    }));
                                        // Collider::cuboid(0.5, 0.5, 0.5),
                                        // Friction {
                                        //     coefficient: 0.0,
                                        //     combine_rule: CoefficientCombineRule::Min
                                        // }));
                                }
                            },
                            _ => ()
                        } 
                    }
                }
            }
        }
    }

    // for x in 0..=chunk_size {
    //     for y in 0..=chunk_size {
    //         for z in 0..=chunk_size {
    //             if y == 3 || y == 0 {
    //                 // let height = rng.gen_range(0..3);
    //                 let material_index = rng.gen_range(0..world_settings.unique_blocks);
    //
    //                 commands.spawn((PbrBundle {
    //                     mesh: mesh_handle.clone(),
    //                     material: materials_handles.get(material_index).expect("Material does not exist").clone(),
    //                     transform: Transform::from_xyz(x as f32, y as f32, z as f32),
    //                     ..default()
    //                 },
    //                     Collider::cuboid(0.5, 0.5, 0.5),
    //                     Friction {
    //                         coefficient: 0.0,
    //                         combine_rule: CoefficientCombineRule::Min
    //                     }));
    //             }
    //         }
    //     }
    // }

    info!("Inserting light in the world");
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
