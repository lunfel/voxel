mod control;
mod cursor;

use crate::game_state::GameState;
use crate::player::control::{player_look, player_move, InputState, MovementSettings};
use crate::player::cursor::{
    cursor_grab, initial_grab_cursor, initial_grab_cursor_delayed, DelayedSystemTimer,
};
use bevy::prelude::*;
use bevy_rapier3d::control::{
    CharacterAutostep, CharacterLength, KinematicCharacterController,
    KinematicCharacterControllerOutput,
};
use bevy_rapier3d::dynamics::{CoefficientCombineRule, RigidBody};
use bevy_rapier3d::geometry::{Collider, Friction};

pub use crate::player::control::KeyBindings;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            // cursor
            .init_resource::<MovementSettings>()
            .init_resource::<KeyBindings>()
            .init_resource::<DelayedSystemTimer>()
            .add_systems(Startup, initial_grab_cursor)
            .add_systems(Update, (initial_grab_cursor_delayed, cursor_grab))
            // control
            .init_resource::<InputState>()
            .add_systems(Update, (player_move, player_look));
    }
}

#[derive(Component, Default, Debug)]
pub struct ThePlayer;

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-10.0, 45.0, -10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ThePlayer,
        Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        },
        RigidBody::KinematicPositionBased,
        Collider::cylinder(0.825, 0.45),
        KinematicCharacterController {
            snap_to_ground: None,
            autostep: Some(CharacterAutostep {
                max_height: CharacterLength::Absolute(0.1),
                ..default()
            }),
            ..default()
        },
        KinematicCharacterControllerOutput::default(),
    ));
}
