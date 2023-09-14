pub mod player_control;

use std::f32::consts::PI;

use bevy::{prelude::{*, shape::Box}, render::{mesh::Indices, render_resource::PrimitiveTopology}, pbr::CascadeShadowConfigBuilder};

use crate::{systems::{player::player_control::{MovementSettings, KeyBindings, JumpTimer, setup_player, InputState, initial_grab_cursor, player_move, player_look, cursor_grab}, world_generation::generate_world}, utils::point::Point3D, world::{GameWorld, block::GameBlockType, chunk::GameChunk}, settings::CHUNK_SIZE};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        info!("Player Pluing");
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
    }
}

fn create_custom_cube<P>(into_coord: &P, chunk: &GameChunk) -> Option<Mesh>
where P: Into<Point3D<i8>> + Clone
{
    let coord: Point3D<i8> = (*into_coord).clone().into();
    // suppose Y-up right hand, and camera look from +z to -z  
    let sp = Box::new(1.0, 1.0, 1.0);

    let mut vertices: Vec<([f32; 3], [f32; 3], [f32; 2])> = vec![];
    let mut indices: Vec<u32> = vec![];

    let indices_template = [0, 1, 2, 2, 3, 0];
    let mut nb_faces = 0;

    let faces: [_; 6] = [
        (
            coord.front_neighbor(),
            [
                ([sp.min_x, sp.min_y, sp.max_z], [0., 0., 1.0], [0., 0.]),
                ([sp.max_x, sp.min_y, sp.max_z], [0., 0., 1.0], [1.0, 0.]),
                ([sp.max_x, sp.max_y, sp.max_z], [0., 0., 1.0], [1.0, 1.0]),
                ([sp.min_x, sp.max_y, sp.max_z], [0., 0., 1.0], [0., 1.0])
            ]
        ),
        (
            coord.back_neighbor(),
            [
                ([sp.min_x, sp.max_y, sp.min_z], [0., 0., -1.0], [1.0, 0.]),
                ([sp.max_x, sp.max_y, sp.min_z], [0., 0., -1.0], [0., 0.]),
                ([sp.max_x, sp.min_y, sp.min_z], [0., 0., -1.0], [0., 1.0]),
                ([sp.min_x, sp.min_y, sp.min_z], [0., 0., -1.0], [1.0, 1.0])
            ]
        ),
        (
            coord.right_neighbor(),
            [
                ([sp.max_x, sp.min_y, sp.min_z], [1.0, 0., 0.], [0., 0.]),
                ([sp.max_x, sp.max_y, sp.min_z], [1.0, 0., 0.], [1.0, 0.]),
                ([sp.max_x, sp.max_y, sp.max_z], [1.0, 0., 0.], [1.0, 1.0]),
                ([sp.max_x, sp.min_y, sp.max_z], [1.0, 0., 0.], [0., 1.0])
            ]
        ),
        (
            coord.left_neighbor(),
            [
                ([sp.min_x, sp.min_y, sp.max_z], [-1.0, 0., 0.], [1.0, 0.]),
                ([sp.min_x, sp.max_y, sp.max_z], [-1.0, 0., 0.], [0., 0.]),
                ([sp.min_x, sp.max_y, sp.min_z], [-1.0, 0., 0.], [0., 1.0]),
                ([sp.min_x, sp.min_y, sp.min_z], [-1.0, 0., 0.], [1.0, 1.0])
            ]
        ),
        (
            coord.top_neighbor(),
            [

                ([sp.max_x, sp.max_y, sp.min_z], [0., 1.0, 0.], [1.0, 0.]),
                ([sp.min_x, sp.max_y, sp.min_z], [0., 1.0, 0.], [0., 0.]),
                ([sp.min_x, sp.max_y, sp.max_z], [0., 1.0, 0.], [0., 1.0]),
                ([sp.max_x, sp.max_y, sp.max_z], [0., 1.0, 0.], [1.0, 1.0])
            ]
        ),
        (
            coord.bottom_neighbor(),
            [
                ([sp.max_x, sp.min_y, sp.max_z], [0., -1.0, 0.], [0., 0.]),
                ([sp.min_x, sp.min_y, sp.max_z], [0., -1.0, 0.], [1.0, 0.]),
                ([sp.min_x, sp.min_y, sp.min_z], [0., -1.0, 0.], [1.0, 1.0]),
                ([sp.max_x, sp.min_y, sp.min_z], [0., -1.0, 0.], [0., 1.0])
            ]
        )
    ];

    for (coord, attributes) in faces.iter() {
        if let Some(cmp_block) = chunk.get_block(coord) {
            if cmp_block.block_type == GameBlockType::Empty {
                attributes.iter()
                    .for_each(|attribute| {
                        vertices.push(*attribute)
                    });

                indices_template.iter()
                    .map(|i| i + nb_faces * 4)
                    .for_each(|i| indices.push(i));

                nb_faces += 1;
            }
        } else {
            attributes.iter()
                .for_each(|attribute| {
                    vertices.push(*attribute)
                });

            indices_template.iter()
                .map(|i| i + nb_faces * 4)
                .for_each(|i| indices.push(i));

            nb_faces += 1;
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
    let ground_material: Handle<StandardMaterial> = materials.add(Color::rgb(76.0 / 255.0, 153.0 / 255.0, 0.0 / 255.0).into());

    info!("Inserting cubes in the world");
    for (chunk_coord, chunk) in game_world.chunks.iter() {
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let block_coord: Point3D<usize> = (x, y, z).into();
                    if let Some(block) = chunk.get_block(&block_coord) {
                        match block.block_type {
                            GameBlockType::Ground => {
                                if let Some(mesh) = create_custom_cube(&(x as i8, y as i8, z as i8), &chunk) {
                                    let mesh_handle = meshes.add(mesh);
                                    commands.spawn(PbrBundle {
                                        mesh: mesh_handle.clone(),
                                        material: ground_material.clone(),
                                        transform: Transform::from_xyz(
                                            x as f32 + chunk_coord.x as f32 * CHUNK_SIZE as f32,
                                            y as f32 + chunk_coord.y as f32 * CHUNK_SIZE as f32,
                                            z as f32 + chunk_coord.z as f32 * CHUNK_SIZE as f32
                                        ),
                                        ..default()
                                    });
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

