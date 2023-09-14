#[derive(Default, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GameBlockType {
    #[default]
    Empty,
    Rock,
    Ground
}

#[derive(Default, Copy, Clone)]
pub struct GameBlock {
    pub block_type: GameBlockType
}
