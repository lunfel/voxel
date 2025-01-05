use bevy::prelude::*;
use bevy_rapier3d::na::Point2;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::asset::RenderAssetUsages;
use crate::{settings::CHUNK_SIZE, utils::point::Point3D};
use crate::settings::{CoordSystemIntegerSize, CHUNK_HEIGHT};
use crate::world::block::{BlockCoord, GameBlockType};

use super::block::GameBlock;

/// ChunkCoord is the coordinate of the chunk in using the
/// value 1 for each chunk. Multiply ChunkCoord by CHUNK_SIZE
/// to get offset in real world
#[derive(Deref, DerefMut, Clone, PartialEq, Eq, Hash, Component, Debug, Default, Copy)]
pub struct ChunkCoord(Point2<CoordSystemIntegerSize>);

impl ChunkCoord {
    pub fn new(x: CoordSystemIntegerSize, y: CoordSystemIntegerSize) -> Self {
        Self(Point2::new(x, y))
    }
}

impl From<Point3D<CoordSystemIntegerSize>> for ChunkCoord {
    fn from(value: Point3D<CoordSystemIntegerSize>) -> Self {
        Self(Point2::new(value.x as CoordSystemIntegerSize, value.z as CoordSystemIntegerSize))
    }
}

impl From<(CoordSystemIntegerSize, CoordSystemIntegerSize, CoordSystemIntegerSize)> for ChunkCoord {
    fn from(value: (CoordSystemIntegerSize, CoordSystemIntegerSize, CoordSystemIntegerSize)) -> Self {
        Self(Point2::new(value.0, value.2))
    }
}

#[derive(Deref, DerefMut, Debug, Clone)]
pub struct ChunkBlocks([[[GameBlock; CHUNK_SIZE as usize]; CHUNK_HEIGHT as usize]; CHUNK_SIZE as usize]);

impl Default for ChunkBlocks {
    fn default() -> Self {
        ChunkBlocks([[[GameBlock::default(); CHUNK_SIZE as usize]; CHUNK_HEIGHT as usize]; CHUNK_SIZE as usize])
    }
}

impl ChunkBlocks {
    pub fn blocks_with_coord(&self) -> Vec<(Point3D<i8>, &GameBlock)> {
        let mut pairs: Vec<(Point3D<i8>, &GameBlock)> = Vec::with_capacity((CHUNK_SIZE*CHUNK_HEIGHT*CHUNK_SIZE) as usize);

        for x in 0.. {
            for y in 0..(CHUNK_HEIGHT as usize) {
                for z in 0..(CHUNK_SIZE as usize) {
                    pairs.push(((x as i8, y as i8, z as i8).into(), &self.0[x][y][z]))
                }
            }
        }

        pairs
    }
}

#[derive(Component, Debug, Clone)]
pub struct GameChunk {
    pub blocks: ChunkBlocks
}

pub type Vertex = [f32; 3];
type Normal = [f32; 3];
pub type UV = [f32; 2];
pub type VertexBuffer = Vec<(Vertex, Normal, UV)>;

impl GameChunk {
    pub fn new() -> Self {
        Self {
            blocks: default()
        }
    }

    pub fn update_block<P, F>(&mut self, into_coord: &P, update: F)
        where P: Into<BlockCoord> + Clone,
              F: Fn(&mut GameBlock)
    {
        if let Some(block) = self.get_block_mut(into_coord) {
            update(block);
        }
    }

    pub fn get_block<P>(&self, into_coord: &P) -> Option<&GameBlock>
    where P: Into<BlockCoord> + Clone
    {
        let coord: BlockCoord = (*into_coord).clone().into();

        self.blocks.get(coord.x as usize)
            .and_then(|blocks_y| blocks_y.get(coord.y as usize)
                .and_then(|blocks_z| blocks_z.get(coord.z as usize))
            )
    }

    pub fn get_block_mut<P>(&mut self, into_coord: &P) -> Option<&mut GameBlock>
        where P: Into<BlockCoord> + Clone
    {
        let coord: BlockCoord = (*into_coord).clone().into();

        self.blocks.get_mut(coord.x as usize)
            .and_then(|blocks_y| blocks_y.get_mut(coord.y as usize)
                .and_then(|blocks_z| blocks_z.get_mut(coord.z as usize))
            )
    }
}

pub fn world_transform_to_chunk_coordinates(transform: &Transform) -> ChunkCoord
{
    ChunkCoord(Point2::new(
        (transform.translation.x / CHUNK_SIZE as f32).floor() as CoordSystemIntegerSize,
        (transform.translation.z / CHUNK_SIZE as f32).floor() as CoordSystemIntegerSize
    ))
}

pub fn chunk_coordinates_to_world_transform(coords: &Point2<CoordSystemIntegerSize>) -> Transform {
    Transform::from_xyz(
        (coords.x * CHUNK_SIZE) as f32,
        0.0,
        (coords.y * CHUNK_SIZE) as f32
    )
}

pub fn render_indices_and_vertices(chunk: &GameChunk) -> (Indices, VertexBuffer) {
    let mut indices: Vec<u32> = vec![];
    let mut total_nb_faces: u32 = 0;
    let mut vertices: VertexBuffer = vec![];

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_HEIGHT {
            for z in 0..CHUNK_SIZE {
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

    let indices = Indices::U32(indices);

    (indices, vertices)
}

pub fn render_mesh(indices: &Indices, vertices: &VertexBuffer) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD);

    let positions: Vec<_> = vertices.iter().map(|(p, _, _)| *p).collect();
    let normals: Vec<_> = vertices.iter().map(|(_, n, _)| *n).collect();
    let uvs: Vec<_> = vertices.iter().map(|(_, _, uv)| *uv).collect();

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    mesh.insert_indices(indices.clone());

    mesh
}

fn apply_uv_offset(uv: UV, offset: f32) -> UV {
    let [u, v] = uv;

    [u / 4.0 + offset, v]
}

fn render_chunk_block(chunk: &GameChunk, coord: &BlockCoord, indices: &mut Vec<u32>, total_nb_faces: &mut u32, vertices: &mut VertexBuffer) {
    // suppose Y-up right hand, and camera look from +z to -z
    let sp = Cuboid::new(1.0, 1.0, 1.0);
    // let sp = shape::Box::new(1.0, 1.0, 1.0);

    let indices_template = [0, 1, 2, 2, 3, 0];

    let block = chunk.get_block(coord).expect("A block was expected here, but no block found");

    let uv_offset = match block.block_type {
        GameBlockType::Rock => 0.50,
        GameBlockType::Ground => 0.00,
        GameBlockType::Gem => 0.00,
        _ => 0.00
    };

    let faces: [_; 6] = [
        (
            coord.front_neighbor(),
            [
                ([-sp.half_size.x, -sp.half_size.y, sp.half_size.z], [0., 0., 1.0], apply_uv_offset([0., 0.], uv_offset)),
                ([sp.half_size.x, -sp.half_size.y, sp.half_size.z], [0., 0., 1.0], apply_uv_offset([1.0, 0.], uv_offset)),
                ([sp.half_size.x, sp.half_size.y, sp.half_size.z], [0., 0., 1.0], apply_uv_offset([1.0, 1.0], uv_offset)),
                ([-sp.half_size.x, sp.half_size.y, sp.half_size.z], [0., 0., 1.0], apply_uv_offset([0., 1.0], uv_offset))
            ]
        ),
        (
            coord.back_neighbor(),
            [
                ([-sp.half_size.x, sp.half_size.y, -sp.half_size.z], [0., 0., -1.0], apply_uv_offset([1.0, 0.], uv_offset)),
                ([sp.half_size.x, sp.half_size.y, -sp.half_size.z], [0., 0., -1.0], apply_uv_offset([0., 0.], uv_offset)),
                ([sp.half_size.x, -sp.half_size.y, -sp.half_size.z], [0., 0., -1.0], apply_uv_offset([0., 1.0], uv_offset)),
                ([-sp.half_size.x, -sp.half_size.y, -sp.half_size.z], [0., 0., -1.0], apply_uv_offset([1.0, 1.0], uv_offset))
            ]
        ),
        (
            coord.right_neighbor(),
            [
                ([sp.half_size.x, -sp.half_size.y, -sp.half_size.z], [1.0, 0., 0.], apply_uv_offset([0., 0.], uv_offset)),
                ([sp.half_size.x, sp.half_size.y, -sp.half_size.z], [1.0, 0., 0.], apply_uv_offset([1.0, 0.], uv_offset)),
                ([sp.half_size.x, sp.half_size.y, sp.half_size.z], [1.0, 0., 0.], apply_uv_offset([1.0, 1.0], uv_offset)),
                ([sp.half_size.x, -sp.half_size.y, sp.half_size.z], [1.0, 0., 0.], apply_uv_offset([0., 1.0], uv_offset))
            ]
        ),
        (
            coord.left_neighbor(),
            [
                ([-sp.half_size.x, -sp.half_size.y, sp.half_size.z], [-1.0, 0., 0.], apply_uv_offset([1.0, 0.], uv_offset)),
                ([-sp.half_size.x, sp.half_size.y, sp.half_size.z], [-1.0, 0., 0.], apply_uv_offset([0., 0.], uv_offset)),
                ([-sp.half_size.x, sp.half_size.y, -sp.half_size.z], [-1.0, 0., 0.], apply_uv_offset([0., 1.0], uv_offset)),
                ([-sp.half_size.x, -sp.half_size.y, -sp.half_size.z], [-1.0, 0., 0.], apply_uv_offset([1.0, 1.0], uv_offset))
            ]
        ),
        (
            coord.top_neighbor(),
            [
                ([sp.half_size.x, sp.half_size.y, -sp.half_size.z], [0., 1.0, 0.], apply_uv_offset([1.0, 0.], uv_offset)),
                ([-sp.half_size.x, sp.half_size.y, -sp.half_size.z], [0., 1.0, 0.], apply_uv_offset([0., 0.], uv_offset)),
                ([-sp.half_size.x, sp.half_size.y, sp.half_size.z], [0., 1.0, 0.], apply_uv_offset([0., 1.0], uv_offset)),
                ([sp.half_size.x, sp.half_size.y, sp.half_size.z], [0., 1.0, 0.], apply_uv_offset([1.0, 1.0], uv_offset))
            ]
        ),
        (
            coord.bottom_neighbor(),
            [
                ([sp.half_size.x, -sp.half_size.y, sp.half_size.z], [0., -1.0, 0.], apply_uv_offset([0., 0.], uv_offset)),
                ([-sp.half_size.x, -sp.half_size.y, sp.half_size.z], [0., -1.0, 0.], apply_uv_offset([1.0, 0.], uv_offset)),
                ([-sp.half_size.x, -sp.half_size.y, -sp.half_size.z], [0., -1.0, 0.], apply_uv_offset([1.0, 1.0], uv_offset)),
                ([sp.half_size.x, -sp.half_size.y, -sp.half_size.z], [0., -1.0, 0.], apply_uv_offset([0., 1.0], uv_offset))
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