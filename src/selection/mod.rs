use bevy::asset::RenderAssetUsages;
use crate::player::control::{FollowsPlayerLookLeftRight, FollowsPlayerLookUpDown, FollowsPlayerPosition, PlayerControl, PlayerEyes};
use crate::settings::CHUNK_SIZE;
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::render::view::RenderLayers;
use bevy_rapier3d::prelude::*;

// Marker component for the wireframe cube
#[derive(Component)]
pub struct WireframeCube;

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_selection)
            .add_plugins(MaterialPlugin::<OutlineMaterial>::default())
            .add_systems(Update, select_block);
    }
}

pub fn setup_selection(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<OutlineMaterial>>,
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
            // MeshMaterial3d(materials.add(StandardMaterial {
            //     base_color: Color::srgba(0.14509803, 0.5882353, 0.74509803, 0.3),
            //     unlit: true,
            //     alpha_mode: AlphaMode::Blend,
            //     ..default()
            // })),
            MeshMaterial3d(materials.add(OutlineMaterial {})),
            Mesh3d(meshes.add(create_cuboid_edges())),
            WireframeCube,
            RenderLayers::layer(1),
            // Wireframe,
            // WireframeColor {
            //     color: Color::srgba(0.14509803, 0.5882353, 0.74509803, 1.0)
            // }
        ));
}

fn create_cuboid_edges() -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::default());
    // Vertices representing the 8 corners of the cuboid
    let vertices = [
        [-0.5, -0.5, -0.5], // Bottom-front-left (0)
        [ 0.5, -0.5, -0.5], // Bottom-front-right (1)
        [ 0.5,  0.5, -0.5], // Top-front-right (2)
        [-0.5,  0.5, -0.5], // Top-front-left (3)
        [-0.5, -0.5,  0.5], // Bottom-back-left (4)
        [ 0.5, -0.5,  0.5], // Bottom-back-right (5)
        [ 0.5,  0.5,  0.5], // Top-back-right (6)
        [-0.5,  0.5,  0.5], // Top-back-left (7)
    ];

    // Edges defined as pairs of indices into the vertex array
    let edges = [
        (0, 1), // Bottom-front
        (1, 2), // Front-right
        (2, 3), // Top-front
        (3, 0), // Front-left

        (4, 5), // Bottom-back
        (5, 6), // Back-right
        (6, 7), // Top-back
        (7, 4), // Back-left

        (0, 4), // Left vertical
        (1, 5), // Right vertical
        (2, 6), // Top-front-to-back
        (3, 7), // Top-back-to-front
    ];

    // Convert edges into a flat vertex array for LineList
    let edge_vertices: Vec<[f32; 3]> = edges
        .iter()
        .flat_map(|&(start, end)| vec![vertices[start], vertices[end]])
        .collect();

    // Assign positions to the mesh
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, edge_vertices);

    mesh
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct OutlineMaterial {}

impl Material for OutlineMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/outline.wgsl".into()
    }
}

pub fn select_block(
    rapier_context: RapierContextAccess,
    commands: Commands,
    query: Query<&Transform, With<PlayerEyes>>,
    mut selection_block_query: Query<(&mut Transform, &mut Visibility), (With<WireframeCube>, Without<PlayerEyes>)>,
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

    let (mut selection_transform, mut selection_visibility) = selection_block_query.single_mut();
    
    if let Some((_, intersection)) = rapier_context.cast_ray_and_get_normal(ray_pos, ray_dir, max_toi, solid, filter) {
        let hit_point = ray_pos + ray_dir * intersection.time_of_impact;

        let transform: Vec3 = hit_point - (intersection.normal / 100.0);

        selection_transform.translation = transform.map(|c| c.round());
        
        *selection_visibility = Visibility::Visible;
    } else {
        *selection_visibility = Visibility::Hidden;
    }
}