use bevy::asset::Handle;
use bevy::pbr::StandardMaterial;
use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;

#[derive(Default, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum VoxelBlockType {
    #[default]
    Empty,
    Rock,
    Grass,
    Gem,
    Dirt
}

#[derive(Default, Copy, Clone, Debug)]
pub struct VoxelBlock {
    pub block_type: VoxelBlockType,
    pub is_fully_surrounded: bool
}

pub type BlockMaterialHashMap = HashMap<VoxelBlockType, Handle<StandardMaterial>>;

#[derive(Resource, Deref, DerefMut)]
pub struct BlockMaterialMap(BlockMaterialHashMap);

#[derive(Resource, Deref, DerefMut, Clone)]
pub struct BlockMaterial(pub Handle<StandardMaterial>);

impl FromWorld for BlockMaterial {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource_mut::<AssetServer>();
        let handle_image = asset_server.load("atlas.png");

        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();

        let handle_material = materials.add(handle_image);

        Self(handle_material)
    }
}

impl FromWorld for BlockMaterialMap {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();

        let mut material_map: BlockMaterialHashMap = HashMap::new();

        material_map.insert(VoxelBlockType::Rock, materials.add(Color::srgba(79.0 / 255.0, 87.0 / 255.0, 99.0 / 255.0, 1.0)));
        material_map.insert(VoxelBlockType::Grass, materials.add(Color::srgba(76.0 / 255.0, 153.0 / 255.0, 0.0 / 255.0, 1.0)));

        Self(material_map)
    }
}