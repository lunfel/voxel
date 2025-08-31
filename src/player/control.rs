use crate::player::ThePlayer;
use bevy::ecs::event::EventCursor;
use bevy::input::mouse::MouseMotion;
use bevy::input::ButtonInput;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy_rapier3d::control::KinematicCharacterController;

#[derive(Resource)]
pub struct MovementSettings {
    pub sensitivity: f32,
    pub speed: f32,
}

impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.00012,
            speed: 20.0, // Used to be 5
        }
    }
}

#[derive(Resource)]
pub struct KeyBindings {
    pub toggle_fullscreen: KeyCode,
    pub toggle_grab_cursor: KeyCode,
    pub move_forward: KeyCode,
    pub move_backward: KeyCode,
    pub move_left: KeyCode,
    pub move_right: KeyCode,
    pub move_downward: KeyCode,
    pub move_upward: KeyCode,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            toggle_fullscreen: KeyCode::KeyF,
            toggle_grab_cursor: KeyCode::Escape,
            move_forward: KeyCode::KeyW,
            move_backward: KeyCode::KeyS,
            move_left: KeyCode::KeyA,
            move_right: KeyCode::KeyD,
            move_downward: KeyCode::KeyC,
            move_upward: KeyCode::Space,
        }
    }
}

#[derive(Resource, Default)]
pub struct InputState {
    reader_motion: EventCursor<MouseMotion>,
}

pub fn player_move(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    settings: Res<MovementSettings>,
    key_bindings: Res<KeyBindings>,
    mut query: Query<(&Transform, &mut KinematicCharacterController), With<ThePlayer>>,
) {
    let window = primary_window.single().expect("No primary window");

    for (transform, mut character_controller) in query.iter_mut() {
        let mut move_velocity = Vec3::ZERO;

        for key in keys.get_pressed() {
            match window.cursor_options.grab_mode {
                CursorGrabMode::None => (),
                _ => {
                    move_velocity += apply_movement(&key_bindings, transform, *key);
                }
            }
        }
        move_velocity = move_velocity.normalize_or_zero() * settings.speed;

        character_controller.translation = Some(move_velocity * time.delta_secs());
    }
}

pub fn player_look(
    settings: Res<MovementSettings>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    mut query: Query<&mut Transform, With<ThePlayer>>,
) {
    let window = primary_window.single().expect("No primary window");

    for mut transform in query.iter_mut() {
        for ev in state.reader_motion.read(&motion) {
            let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
            match window.cursor_options.grab_mode {
                CursorGrabMode::None => (),
                _ => {
                    let window_scale = window.height().min(window.width());
                    pitch -= (settings.sensitivity * ev.delta.y * window_scale).to_radians();
                    yaw -= (settings.sensitivity * ev.delta.x * window_scale).to_radians();
                }
            }

            pitch = pitch.clamp(-1.54, 1.54);

            transform.rotation =
                Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
        }
    }
}

fn apply_movement(key_bindings: &Res<KeyBindings>, transform: &Transform, key: KeyCode) -> Vec3 {
    let mut move_velocity = Vec3::ZERO;
    let local_z = transform.local_z();
    let local_y = transform.local_y();
    let forward = Vec3::new(-local_z.x, 0.0, -local_z.z);
    let right = Vec3::new(local_z.z, 0.0, -local_z.x);
    let upward = Vec3::new(0.0, local_y.y, 0.0);

    if key == key_bindings.move_forward {
        move_velocity += forward;
    } else if key == key_bindings.move_backward {
        move_velocity -= forward;
    } else if key == key_bindings.move_left {
        move_velocity -= right;
    } else if key == key_bindings.move_right {
        move_velocity += right;
    } else if key == key_bindings.move_downward {
        move_velocity -= upward;
    } else if key == key_bindings.move_upward {
        move_velocity += upward;
    }

    move_velocity
}
