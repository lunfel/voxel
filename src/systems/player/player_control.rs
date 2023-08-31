use bevy::{prelude::*, window::{CursorGrabMode, PrimaryWindow}, input::mouse::MouseMotion, ecs::event::ManualEventReader};
use bevy_rapier3d::{prelude::{RigidBody, Collider, KinematicCharacterController, Velocity, KinematicCharacterControllerOutput}, na::clamp};

use crate::WorldSettings;

#[derive(Resource, Default)]
pub struct InputState {
    reader_motion: ManualEventReader<MouseMotion>
}

#[derive(Resource)]
pub struct MovementSettings {
    pub sensitivity: f32,
    pub speed: f32
}

#[derive(Resource)]
pub struct PlayerState {
    pub is_jumping: bool
}

impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.00012,
            speed: 6.0
        }
    }
}

#[derive(Resource)]
pub struct KeyBindings {
    pub toggle_grab_cursor: KeyCode,
    pub move_forward: KeyCode,
    pub move_backward: KeyCode,
    pub move_left: KeyCode,
    pub move_right: KeyCode,
    pub jump: KeyCode,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            toggle_grab_cursor: KeyCode::Escape,
            move_forward: KeyCode::W,
            move_backward: KeyCode::S,
            move_left: KeyCode::A,
            move_right: KeyCode::D,
            jump: KeyCode::Space
        }
    }
}

#[derive(Component)]
pub struct PlayerControl;

pub fn setup_player(
    mut commands: Commands,
    world_settings: Res<WorldSettings>
) {
    commands.spawn((
        PlayerControl,
        Camera3dBundle {
            transform: Transform::from_xyz(5.0, 15.0, 5.0).looking_at(Vec3 {
                z: world_settings.chunk_size as f32 / 2.0,
                x: world_settings.chunk_size as f32 / 2.0,
                ..default()
            }, Vec3::Y),
            ..default()
        },
        RigidBody::KinematicPositionBased,
        Collider::cuboid(0.5, 1.65, 0.5),
        KinematicCharacterController::default(),
        KinematicCharacterControllerOutput::default(),
        Velocity::default()
    ));
}

pub fn player_move(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    settings: Res<MovementSettings>,
    key_bindings: Res<KeyBindings>,
    mut player_state: ResMut<PlayerState>,
    mut query: Query<(&mut Transform, &mut KinematicCharacterController, &KinematicCharacterControllerOutput, &mut Velocity), With<PlayerControl>>
) {
    if let Ok(window) = primary_window.get_single() {
        for (mut transform, mut character_controller, character_output, mut falling_velocity) in query.iter_mut() {
            let mut velocity = Vec3::ZERO;
            let local_z = transform.local_z();
            let forward = Vec3::new(-local_z.x, 0.0, -local_z.z);
            let right = Vec3::new(local_z.z, 0.0, -local_z.x);
            let up = Vec3::new(0.0, 100.0, 0.0);

            for key in keys.get_pressed() {
               match window.cursor.grab_mode {
                    CursorGrabMode::None => (),
                    _ => {
                        let key = *key;

                        if key == key_bindings.move_forward {
                            velocity += forward;
                        } else if key == key_bindings.move_backward {
                            velocity -= forward;
                        } else if key == key_bindings.move_left {
                            velocity -= right;
                        } else if key == key_bindings.move_right {
                            velocity += right;
                        } else if key == key_bindings.jump {
                            velocity += up;
                            player_state.is_jumping = true;
                        } 
                    }
                } 
            }

            velocity = velocity.normalize_or_zero();

            let gravity_force = -0.981;

            if character_output.grounded {
                falling_velocity.linvel = Vec3::ZERO;
            } else {
                falling_velocity.linvel = Vec3 {
                    y: clamp(falling_velocity.linvel.y + gravity_force * time.delta_seconds(), -15.0, 0.0),
                    ..default()
                };
            }

            transform.translation += velocity * time.delta_seconds() * settings.speed;
            character_controller.translation = Some(falling_velocity.linvel * time.delta_seconds());
        }
    }
}

pub fn player_look(
    settings: Res<MovementSettings>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    mut query: Query<&mut Transform, With<PlayerControl>>
) {
    if let Ok(window) = primary_window.get_single() {
        for mut transform in query.iter_mut() {
            for ev in state.reader_motion.iter(&motion) {
                let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
                match window.cursor.grab_mode {
                    CursorGrabMode::None => (),
                    _ => {
                        let window_scale = window.height().min(window.width());
                        pitch -= (settings.sensitivity * ev.delta.y * window_scale).to_radians();
                        yaw -= (settings.sensitivity * ev.delta.x * window_scale).to_radians();
                    }
                }

                pitch = pitch.clamp(-1.54, 1.54);

                transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
            }
        }
    } else {
        warn!("Primary window not found for `player_look`");
    }
}

pub fn initial_grab_cursor(mut primary_window: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = primary_window.get_single_mut() {
        toggle_grab_cursor(&mut window);
    } else {
        warn!("Primary window not found for `initial_grab_cursor`");
    }
}

pub fn toggle_grab_cursor(window: &mut Window) {
    match window.cursor.grab_mode {
        CursorGrabMode::None => {
            window.cursor.grab_mode = CursorGrabMode::Confined;
            window.cursor.visible = false;
        },
        _ => {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        }
    }
}

fn cursor_grab(
    keys: Res<Input<KeyCode>>,
    key_bindings: Res<KeyBindings>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>
) {
    if let Ok(mut window) = primary_window.get_single_mut() {
        if keys.just_pressed(key_bindings.toggle_grab_cursor) {
            toggle_grab_cursor(&mut window);
        }
    } else {
        warn!("Primary window not found for `cursor_grab`");
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputState>()
            .init_resource::<MovementSettings>()
            .init_resource::<KeyBindings>()
            .add_systems(Startup, setup_player)
            .add_systems(Startup, initial_grab_cursor)
            .add_systems(Update, player_move)
            .add_systems(Update, player_look)
            .add_systems(Update, cursor_grab);
    }
}
