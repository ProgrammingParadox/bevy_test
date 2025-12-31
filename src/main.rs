use bevy::{camera::visibility::RenderLayers, color::palettes::tailwind, prelude::*};
use bevy_rapier3d::prelude::*;

mod cursor_grab;

mod camera;

mod player;

use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        // EGUI inspector
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        // Physics
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        // Cursor stuff
        .add_plugins(cursor_grab::CursorPlugin)
        // player
        .add_plugins(player::PlayerPlugin)
        // camera
        .add_plugins(camera::CameraPlugin)
        // World, lights, text
        .add_systems(Startup, (spawn_world_model, spawn_lights, spawn_text));
    // Update stuff
    // .add_systems(Update, change_fov);

    app.run();
}

/// Used implicitly by all entities without a `RenderLayers` component.
/// Our world model camera and all objects other than the player are on this layer.
/// The light source belongs to both layers.
const DEFAULT_RENDER_LAYER: usize = 0;

/// Used by the view model camera and the player's arm.
/// The light source belongs to both layers.
const VIEW_MODEL_RENDER_LAYER: usize = 1;

fn spawn_world_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(10.0)));
    let material = materials.add(Color::WHITE);
    let cube = meshes.add(Cuboid::new(2.0, 0.5, 1.0));

    // The world model camera will render the floor and the cubes spawned in this system.
    // Assigning no `RenderLayers` component defaults to layer 0.

    commands.spawn((Mesh3d(floor), MeshMaterial3d(material.clone())));

    commands.spawn((
        Mesh3d(cube.clone()),
        MeshMaterial3d(material.clone()),
        Transform::from_xyz(0.0, 0.25, -3.0),
    ));

    commands.spawn((
        Mesh3d(cube),
        MeshMaterial3d(material),
        Transform::from_xyz(0.75, 1.75, 0.0),
    ));

    commands
        // Spawn the ground plane
        .spawn((
            // bevy
            Mesh3d(meshes.add(Mesh::from(Cuboid {
                half_size: Vec3::new(128., 0.5, 128.),
            }))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(1.0, 0.0, 0.0), // Bright red for testing
                ..default()
            })),
            Transform::from_xyz(0.0, -10., 0.0),
            // physics
            RigidBody::Fixed,
            Collider::cuboid(128., 0.5, 128.),
        ));
}

fn spawn_lights(mut commands: Commands) {
    commands.spawn((
        PointLight {
            color: Color::from(tailwind::ROSE_300),
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(-2.0, 4.0, -0.75),
        // The light source illuminates both the world model and the view model.
        RenderLayers::from_layers(&[DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER]),
    ));
}

fn spawn_text(mut commands: Commands) {
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            bottom: px(12),
            left: px(12),
            ..default()
        })
        .with_child(Text::new(concat!(
            "I'm going to be completely honest\nand say that I have absolutely no clue what I'm doing"
        )));
}

// fn change_fov(
//     input: Res<ButtonInput<KeyCode>>,
//     mut world_model_projection: Single<&mut Projection, With<WorldModelCamera>>,
// ) {
//     let Projection::Perspective(perspective) = world_model_projection.as_mut() else {
//         unreachable!(
//             "The `Projection` component was explicitly built with `Projection::Perspective`"
//         );
//     };

//     if input.pressed(KeyCode::ArrowUp) {
//         perspective.fov -= 1.0_f32.to_radians();
//         perspective.fov = perspective.fov.max(20.0_f32.to_radians());
//     }
//     if input.pressed(KeyCode::ArrowDown) {
//         perspective.fov += 1.0_f32.to_radians();
//         perspective.fov = perspective.fov.min(160.0_f32.to_radians());
//     }
// }
