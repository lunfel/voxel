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
}