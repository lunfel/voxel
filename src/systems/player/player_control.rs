use std::ops::{DerefMut, Deref};

use bevy::{prelude::*, window::{CursorGrabMode, PrimaryWindow}, input::mouse::MouseMotion, ecs::event::ManualEventReader, time::Stopwatch, app::AppExit};
use bevy_rapier3d::prelude::{RigidBody, Collider, KinematicCharacterController, KinematicCharacterControllerOutput, CharacterLength};

use crate::settings::CHUNK_SIZE;

#[derive(Resource, Default)]
pub struct InputState {
    reader_motion: ManualEventReader<MouseMotion>
}

#[derive(Resource)]
pub struct MovementSettings {
    pub sensitivity: f32,
    pub speed: f32
}

#[derive(Resource, Default)]
pub struct JumpTimer(Option<Timer>);

impl Deref for JumpTimer {
    type Target = Option<Timer>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for JumpTimer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.00012,
            speed: 50.0 // Used to be 5
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

#[derive(Default, Debug)]
pub enum PlayerGroundedEnum {
    Grounded,
    #[default]
    NonGrounded
}

#[derive(Component, Default, Debug)]
pub struct PlayerState {
    pub grounded_state: PlayerGroundedEnum,
    pub time_grounded_changed: Stopwatch,
    pub last_velocity: Vec3,
    pub is_jumping: bool
}

pub fn setup_player(
    mut commands: Commands
) {
    info!("Setup player");
    commands.spawn((
        PlayerControl,
        PlayerState::default(),
        Camera3dBundle {
            transform: Transform::from_xyz(5.0, 15.0, 5.0).looking_at(Vec3 {
                z: CHUNK_SIZE as f32 / 2.0,
                x: CHUNK_SIZE as f32 / 2.0,
                ..default()
            }, Vec3::Y),
            ..default()
        },
        FogSettings {
            color: Color::rgba(0.5, 0.5, 0.5, 0.8),
            falloff: FogFalloff::Linear {
                start: 100.0,
                end: 125.0,
            },
            ..default()
        },
        RigidBody::KinematicPositionBased,
        Collider::cuboid(0.5, 1.65, 0.5),
        KinematicCharacterController {
            snap_to_ground: Some(CharacterLength::Relative(0.5)),
            ..default()
        },
        KinematicCharacterControllerOutput::default(),
    ));
}

pub fn player_move(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    settings: Res<MovementSettings>,
    key_bindings: Res<KeyBindings>,
    mut jump_timer: ResMut<JumpTimer>,
    mut query: Query<(&Transform, &mut KinematicCharacterController, &KinematicCharacterControllerOutput, &mut PlayerState), With<PlayerControl>>
) {
    if let Ok(window) = primary_window.get_single() {
        for (transform, mut character_controller, character_output, mut player_state) in query.iter_mut() {
            let mut move_velocity = Vec3::ZERO;
            let local_z = transform.local_z();
            let forward = Vec3::new(-local_z.x, 0.0, -local_z.z);
            let right = Vec3::new(local_z.z, 0.0, -local_z.x);
            // let jump = Vec3::new(0.0, 2.0, 0.0);
            let jump_vel = 3.0;
            let mut just_started_jumping = false;
            // Approximativement 53m/s en chute libre dans les airs

            match (character_output.grounded, &mut player_state.grounded_state) {
                (true, PlayerGroundedEnum::NonGrounded) => {
                    player_state.grounded_state = PlayerGroundedEnum::Grounded;
                    player_state.time_grounded_changed.reset();
                },
                (false, PlayerGroundedEnum::Grounded) => {
                    player_state.grounded_state = PlayerGroundedEnum::NonGrounded;
                    player_state.time_grounded_changed.reset();
                },
                _ => {
                    player_state.time_grounded_changed.tick(time.delta());
                }
            }

            for key in keys.get_pressed() {
               match window.cursor.grab_mode {
                    CursorGrabMode::None => (),
                    _ => {
                        let key = *key;

                        if key == key_bindings.move_forward {
                            move_velocity += forward;
                        } else if key == key_bindings.move_backward {
                            move_velocity -= forward;
                        } else if key == key_bindings.move_left {
                            move_velocity -= right;
                        } else if key == key_bindings.move_right {
                            move_velocity += right;
                        } 
                    }
                } 
            }

            move_velocity = move_velocity.normalize_or_zero() * settings.speed;

            // let mut jump_timer: &mut Option<_> = &mut jump_timer;
            for key in keys.get_just_pressed() {
                match window.cursor.grab_mode {
                    CursorGrabMode::None => (),
                    _ => {
                        let key = *key;

                        if key == key_bindings.jump && character_output.grounded {
                            // jump_timer.get_or_insert(Timer::from_seconds(0.5, TimerMode::Once));
                            player_state.is_jumping = true;
                            just_started_jumping = true;
                        }
                    }
                }
            }

            let mut v0_y = player_state.last_velocity * Vec3::Y;

            let final_vel = move_velocity + if just_started_jumping {
                // Vec3::new(0.0, jump_vel, 0.0) * time.delta_seconds()
                Vec3::new(0.0, jump_vel, 0.0)
            } else {
                let grav = Vec3::new(0.0, -0.0001, 0.0);
                let delta = time.delta_seconds();
                // info!("Y vel: {} + {} * {}", v0_y, grav, delta);
                v0_y + grav * delta
            };

            player_state.last_velocity = final_vel;

            character_controller.translation = Some(final_vel * time.delta_seconds());
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

pub fn cursor_grab(
    keys: Res<Input<KeyCode>>,
    key_bindings: Res<KeyBindings>,
    mut app_exit_events: ResMut<Events<AppExit>>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>
) {
    if let Ok(mut window) = primary_window.get_single_mut() {
        if keys.just_pressed(key_bindings.toggle_grab_cursor) {
            // toggle_grab_cursor(&mut window);
            app_exit_events.send(AppExit);
        }
    } else {
        warn!("Primary window not found for `cursor_grab`");
    }
}

