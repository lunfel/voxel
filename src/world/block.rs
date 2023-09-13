#[derive(Default, Copy, Clone, PartialEq)]
pub enum GameBlockType {
    #[default]
    Empty,
    Ground
}

#[derive(Default, Copy, Clone)]
pub struct GameBlock {
    pub block_type: GameBlockType
}
