use bevy::prelude::Resource;

#[derive(Resource, Default)]
pub struct WorldGenerationState {
    pub finished_generating: bool
}