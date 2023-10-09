use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology, VertexAttributeValues};
use bevy::utils::HashMap;
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::rapier::prelude::{ColliderBuilder, InteractionGroups};
use crate::settings::{CHUNK_SIZE, GameParameters};
use crate::systems::player::player_control::PlayerControl;
use crate::systems::world_generation::BlockMaterialMap;
use crate::world::block::{BlockCoord, GameBlockType};
use crate::world::chunk::{ChunkCoord, GameChunk, VertexBuffer};

#[derive(Resource, Deref, DerefMut)]
pub struct DebugColliderTimer(pub Timer);

#[derive(Ord, PartialOrd, PartialEq, Eq, Hash)]
pub enum InteractionGroupName {
    Floor,
    Player
}

#[derive(Resource, Deref, DerefMut)]
pub struct CollisionInteractionGroups(pub HashMap<InteractionGroupName, InteractionGroups>);

impl Default for CollisionInteractionGroups {
    fn default() -> Self {
        let mut map = HashMap::new();

        map.insert(InteractionGroupName::Floor, InteractionGroups::new(0b0011.into(), 0b0001.into()));
        map.insert(InteractionGroupName::Player, InteractionGroups::new(0b0001.into(), 0b0011.into()));

        Self(map)
    }
}

pub fn debug_collider_counts(
    mut timer: ResMut<DebugColliderTimer>,
    time: Res<Time>,
    active_colliders: Query<&Collider, Without<ColliderDisabled>>,
    disabled_colliders: Query<&Collider, With<ColliderDisabled>>
) {
    if timer.tick(time.delta()).just_finished() {
        info!("Colliders: Active = {} Inactive = {}", active_colliders.iter().len(), disabled_colliders.iter().len());
    }
}

pub fn enable_close_colliders(
    mut chunk_query: Query<(Entity, &mut GameChunk, &Transform), (With<Collider>, With<ColliderDisabled>)>,
    mut player_query: Query<&Transform, With<PlayerControl>>,
    mut commands: Commands
) {
    for (chunk_entity, chunk, chunk_transform) in chunk_query.iter() {
        for player_transform in player_query.iter() {
            if chunk_transform.translation.distance(player_transform.translation) < (CHUNK_SIZE as f32 * 1.5) {
                commands.entity(chunk_entity).remove::<ColliderDisabled>();

                info!("Enabled chunk collider at {}", chunk_transform.translation);
            }
        }
    }
}

pub fn disable_far_colliders(
    mut chunk_query: Query<(Entity, &mut GameChunk, &Transform), (With<Collider>, Without<ColliderDisabled>)>,
    mut player_query: Query<&Transform, With<PlayerControl>>,
    mut commands: Commands
) {
    for (chunk_entity, chunk, chunk_transform) in chunk_query.iter() {
        for player_transform in player_query.iter() {
            if chunk_transform.translation.distance(player_transform.translation) >= (CHUNK_SIZE as f32 * 1.5) {
                commands.entity(chunk_entity).insert(ColliderDisabled::default());

                info!("Disabled chunk collider at {}", chunk_transform.translation);
            }
        }
    }
}

pub fn render_dirty_chunk(
    mut query: Query<(Entity, &mut GameChunk, &ChunkCoord, &mut Collider, &Handle<Mesh>)>,
    mut player_query: Query<(Entity, &PlayerControl, &Transform)>,
    game_parameters: Res<GameParameters>,
    mut mesh_manager: ResMut<Assets<Mesh>>,
    block_material_mapping: Res<BlockMaterialMap>,
    mut commands: Commands
) {
    for (entity, chunk, coord, collider, mesh_handle) in query.iter() {
        // Remove dirty components
        let mut entity_commands = commands.entity(entity);

        entity_commands.remove::<Collider>();
        entity_commands.remove::<Handle<Mesh>>();
        mesh_manager.remove(mesh_handle);

        let mesh = render_chunk(&game_parameters, chunk);

        let mesh_handle = mesh_manager.add(mesh);

        entity_commands.insert((
            mesh_handle,
        ));
    }
}

pub fn render_chunk(game_parameters: &GameParameters, chunk: &GameChunk) -> Mesh {
    let mut indices: Vec<u32> = vec![];
    let mut total_nb_faces: u32 = 0;
    let mut vertices: VertexBuffer = vec![];

    for x in 0..game_parameters.chunk_size {
        for y in 0..game_parameters.chunk_size {
            for z in 0..game_parameters.chunk_size {
                let block_coord: BlockCoord = (x, y, z).into();
                if let Some(block) = chunk.get_block(&block_coord) {
                    match block.block_type {
                        GameBlockType::Empty => (),
                        _ => render_chunk_block(chunk, &block_coord, &mut indices, &mut total_nb_faces, &mut vertices)
                    }
                }
            }
        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let positions: Vec<_> = vertices.iter().map(|(p, _, _)| *p).collect();
    let normals: Vec<_> = vertices.iter().map(|(_, n, _)| *n).collect();
    let uvs: Vec<_> = vertices.iter().map(|(_, _, uv)| *uv).collect();

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    let indices = Indices::U32(indices);

    mesh.set_indices(Some(indices));

    // let collider_base = Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::TriMesh)
    //     .unwrap();
    //
    // let collider = ColliderBuilder::new(collider_base.raw)
    //     .collision_groups(InteractionGroups::new(0b0011.into(), 0b0001.into()))
    //     .build();

    mesh
}

fn render_chunk_block(chunk: &GameChunk, coord: &BlockCoord, indices: &mut Vec<u32>, total_nb_faces: &mut u32, vertices: &mut VertexBuffer) {
    // suppose Y-up right hand, and camera look from +z to -z
    let sp = shape::Box::new(1.0, 1.0, 1.0);

    let indices_template = [0, 1, 2, 2, 3, 0];

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

    let block_offset: [f32; 3] = [
        coord.x as f32,
        coord.y as f32,
        coord.z as f32
    ];

    for (coord, attributes) in faces.iter() {
        let mut should_render_face = true;
        if let Some(coord) = coord {
            if let Some(cmp_block) = chunk.get_block(coord) {
                if cmp_block.block_type != GameBlockType::Empty {
                    should_render_face = false;
                }
            }
        }

        if should_render_face {
            attributes.iter()
                .for_each(|(position, normals, uv)| {
                    let v: [f32; 3] = position.iter()
                        .zip(block_offset)
                        .into_iter()
                        .map(|(v, offset)| v + offset)
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap();

                    vertices.push((
                        v,
                        *normals,
                        *uv
                    ))
                });

            indices_template.iter()
                .map(|i| i + (*total_nb_faces) * 4)
                .for_each(|i| indices.push(i));

            *total_nb_faces += 1;
        }
    }
}