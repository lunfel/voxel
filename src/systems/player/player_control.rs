use std::ops::{Deref, DerefMut};

use bevy::{
    app::AppExit,
    ecs::event::ManualEventReader,
    input::mouse::MouseMotion,
    prelude::*,
    time::Stopwatch,
    window::{CursorGrabMode, PrimaryWindow},
};
use bevy_rapier3d::{
    control::CharacterAutostep,
    prelude::{
        CharacterLength, CoefficientCombineRule, Collider, Friction, KinematicCharacterController,
        KinematicCharacterControllerOutput, RigidBody,
    },
};

use crate::settings::CHUNK_SIZE;

#[derive(Resource, Default)]
pub struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
}

#[derive(Resource)]
pub struct MovementSettings {
    pub sensitivity: f32,
    pub speed: f32,
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
            speed: 5.0, // Used to be 5
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
    pub jump: KeyCode,
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
            move_downward: KeyCode::KeyQ,
            move_upward: KeyCode::KeyE,
            jump: KeyCode::Space,
        }
    }
}

#[derive(Component)]
pub struct PlayerControl;

#[derive(Component)]
pub struct PlayerEyes;

#[derive(Component)]
pub struct FollowsPlayerPosition;

#[derive(Component)]
pub struct FollowsPlayerLookLeftRight;

#[derive(Component)]
pub struct FollowsPlayerLookUpDown;

#[derive(Default, Debug, PartialEq)]
pub enum PlayerGroundedEnum {
    Grounded,
    #[default]
    NonGrounded,
}

#[derive(Component, Default, Debug)]
pub struct PlayerState {
    pub grounded_state: PlayerGroundedEnum,
    pub time_grounded_changed: Stopwatch,
    pub last_velocity: Vec3,
    pub is_jumping: bool,
}

pub fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("Setup player");

    // Player's eyes
    commands.spawn((FollowsPlayerPosition, PlayerEyes, Transform::from_xyz(8.0, 20.0, 8.0).looking_at(
        Vec3 {
            z: CHUNK_SIZE as f32 / 2.0,
            x: CHUNK_SIZE as f32 / 2.0,
            ..default()
        },
        Vec3::Y,
    )));

    // Camera
    commands.spawn((
        FollowsPlayerLookLeftRight,
        FollowsPlayerLookUpDown,
        FollowsPlayerPosition,
        Camera3dBundle {
            transform: Transform::from_xyz(5.0, 13.0, 5.0).looking_at(
                Vec3 {
                    z: CHUNK_SIZE as f32 / 2.0,
                    x: CHUNK_SIZE as f32 / 2.0,
                    ..default()
                },
                Vec3::Y,
            ),
            ..default()
        },
        FogSettings {
            color: Color::rgba(0.5, 0.5, 0.5, 0.7),
            falloff: FogFalloff::Linear {
                start: 100.0,
                end: 125.0,
            },
            ..default()
        },
    ));

    // The player itself
    commands.spawn((
        PlayerControl,
        FollowsPlayerLookLeftRight,
        PlayerState::default(),
        // PbrBundle {
        //     mesh: meshes.add(Mesh::from(shape::Cylinder {
        //         height: 1.65,
        //         radius: 0.5,
        //         ..Default::default()
        //     })),
        //     material: materials.add(Color::rgb(79.0 / 255.0, 87.0 / 255.0, 99.0 / 255.0).into()),
        //     transform: Transform::from_xyz(8.0, 20.0, 8.0),
        //     ..Default::default()
        // },
        GlobalTransform::default(),
        Transform::from_xyz(8.0, 100.0, 8.0),
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

pub fn player_move(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    settings: Res<MovementSettings>,
    key_bindings: Res<KeyBindings>,
    mut query: Query<
        (
            &Transform,
            &mut KinematicCharacterController,
            &KinematicCharacterControllerOutput,
            &mut PlayerState,
        ),
        With<PlayerControl>,
    >,
) {
    if let Ok(window) = primary_window.get_single() {
        for (transform, mut character_controller, character_output, mut player_state) in
            query.iter_mut()
        {
            let mut move_velocity = Vec3::ZERO;
            let local_z = transform.local_z();
            let local_y = transform.local_y();
            let forward = Vec3::new(-local_z.x, 0.0, -local_z.z);
            let right = Vec3::new(local_z.z, 0.0, -local_z.x);
            let upward = Vec3::new(0.0, local_y.y, 0.0);
            // let jump = Vec3::new(0.0, 2.0, 0.0);
            let jump_vel = 4.5;
            let mut just_started_jumping = false;
            // Approximativement 53m/s en chute libre dans les airs

            match (character_output.grounded, &mut player_state.grounded_state) {
                (true, PlayerGroundedEnum::NonGrounded) => {
                    player_state.grounded_state = PlayerGroundedEnum::Grounded;
                    player_state.time_grounded_changed.reset();
                }
                (false, PlayerGroundedEnum::Grounded) => {
                    player_state.grounded_state = PlayerGroundedEnum::NonGrounded;
                    player_state.time_grounded_changed.reset();
                }
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
                        } else if key == key_bindings.move_downward {
                            move_velocity -= upward;
                        } else if key == key_bindings.move_upward {
                            move_velocity += upward;
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

            let v0_y = player_state.last_velocity * Vec3::Y;

            let final_vel = move_velocity
                + if just_started_jumping {
                    // Vec3::new(0.0, jump_vel, 0.0) * time.delta_seconds()
                    Vec3::new(0.0, jump_vel, 0.0)
                } else {
                    let grav = Vec3::new(0.0, -9.81, 0.0);
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
    mut query: Query<&mut Transform, With<PlayerEyes>>,
) {
    if let Ok(window) = primary_window.get_single() {
        for mut transform in query.iter_mut() {
            for ev in state.reader_motion.read(&motion) {
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

                transform.rotation =
                    Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
            }
        }
    } else {
        warn!("Primary window not found for `player_look`");
    }
}

pub fn follow_player_look_left_right(
    mut query: Query<&mut Transform, (With<FollowsPlayerLookLeftRight>, Without<PlayerEyes>)>,
    query_source: Query<&Transform, With<PlayerEyes>>,
) {
    if let Ok(source_transform) = query_source.get_single() {
        for mut transform in query.iter_mut() {
            let (_, pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
            let (yaw, _, _) = source_transform.rotation.to_euler(EulerRot::YXZ);

            transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
        }
    } else {
        warn!("No player eyes found!");
    }
}

pub fn follow_player_look_up_down(
    mut query: Query<&mut Transform, (With<FollowsPlayerLookUpDown>, Without<PlayerEyes>)>,
    query_source: Query<&Transform, With<PlayerEyes>>,
) {
    if let Ok(source_transform) = query_source.get_single() {
        for mut transform in query.iter_mut() {
            let (yaw, _, _) = transform.rotation.to_euler(EulerRot::YXZ);
            let (_, pitch, _) = source_transform.rotation.to_euler(EulerRot::YXZ);

            transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
        }
    } else {
        warn!("No player eyes found!");
    }
}

pub fn follow_player_position(
    mut query: Query<&mut Transform, With<FollowsPlayerPosition>>,
    query_source: Query<&Transform, (With<PlayerControl>, Without<FollowsPlayerPosition>)>
) {
    if let Ok(source_transform) = query_source.get_single() {
        for mut transform in query.iter_mut() {
            transform.translation = source_transform.translation + Vec3::new(0.0, 0.825 - 0.1, 0.0)
        }
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
        }
        _ => {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        }
    }
}

pub fn cursor_grab(
    keys: Res<ButtonInput<KeyCode>>,
    key_bindings: Res<KeyBindings>,
    mut app_exit_events: ResMut<Events<AppExit>>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(mut window) = primary_window.get_single_mut() {
        if keys.just_pressed(key_bindings.toggle_grab_cursor) {
            toggle_grab_cursor(&mut window);
            // app_exit_events.send(AppExit);
        }
    } else {
        warn!("Primary window not found for `cursor_grab`");
    }
}
