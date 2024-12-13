use crate::player::control::{FollowsPlayerLookLeftRight, FollowsPlayerLookUpDown, FollowsPlayerPosition, PlayerControl, PlayerEyes};
use crate::settings::CHUNK_SIZE;
use bevy::math::Vec3;
use bevy::pbr::wireframe::{Wireframe, WireframeColor};
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy_rapier3d::prelude::*;

// Marker component for the wireframe cube
#[derive(Component)]
pub struct WireframeCube;

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_selection)
            .add_systems(Update, select_block);
    }
}

pub fn setup_selection(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
)
{
    // Cube selection camera
    commands.spawn((
        FollowsPlayerLookLeftRight,
        FollowsPlayerLookUpDown,
        FollowsPlayerPosition,
        Transform::from_xyz(5.0, 13.0, 5.0).looking_at(
            Vec3 {
                z: CHUNK_SIZE as f32 / 2.0,
                x: CHUNK_SIZE as f32 / 2.0,
                ..default()
            },
            Vec3::Y,
        ),
        Camera3d::default(),
        Camera {
            order: 1,
            ..default()
        },
        RenderLayers::layer(1)
    ));

    // Spawn a wireframe cube (hidden initially)
    commands
        .spawn((
            Transform::default(),
            Visibility::default(),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgba(0.14509803, 0.5882353, 0.74509803, 0.3),
                unlit: true,
                alpha_mode: AlphaMode::Blend,
                ..default()
            })),
            Mesh3d(meshes.add(Cuboid::new(1.01, 1.01, 1.01))),
            WireframeCube,
            RenderLayers::layer(1),
            Wireframe,
            WireframeColor {
                color: Color::srgba(0.14509803, 0.5882353, 0.74509803, 1.0)
            }
        ));
}

pub fn select_block(
    rapier_context: RapierContextAccess,
    mut commands: Commands,
    query: Query<&Transform, With<PlayerEyes>>,
    mut selection_block_query: Query<&mut Transform, (With<WireframeCube>, Without<PlayerEyes>)>,
    exclusion_query: Query<Entity, With<PlayerControl>>) {

    let transform = query.single();
    let ray_pos = transform.translation;
    // Je ne sais pas pourquoi c'est -Z, mais ça semble marcher jusqu'à présent
    let ray_dir = transform.rotation * -Vec3::Z;
    let max_toi: f32 = 6.0;
    // let max_toi = Real::default();
    let solid = false;

    let player_entity = exclusion_query.single();

    let filter = QueryFilter::exclude_dynamic()
        .exclude_sensors()
        .exclude_rigid_body(player_entity);

    if let Some((_, intersection)) = rapier_context.cast_ray_and_get_normal(ray_pos, ray_dir, max_toi, solid, filter) {
        let hit_point = ray_pos + ray_dir * intersection.time_of_impact;

        let transform: Vec3 = hit_point - (intersection.normal / 100.0);

        let mut selection_transform = selection_block_query.single_mut();

        selection_transform.translation = transform.map(|c| c.round());

        // entity_command.log_components();
        info!("Pointing at a chunk right now! hit point {}, normal {}", hit_point, intersection.normal);
    }
}