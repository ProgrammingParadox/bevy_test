use bevy::prelude::*;
use bevy_rapier3d::{geometry::*, prelude::*};

use crate::camera::mouse::CameraController;

const PLAYER_SPEED: f32 = 5.0;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlayerSpawnSet;

#[derive(Debug, Component)]
pub struct Player;

#[derive(Debug, Resource)]
pub struct PlayerEntity(pub Entity);

#[derive(Debug, Component, Default)]
struct PlayerInput {
    direction: Vec3,
}

fn read_player_input(
    input: Res<ButtonInput<KeyCode>>,
    mut player_input_query: Query<&mut PlayerInput, With<Player>>,
) {
    let Ok(mut player_input) = player_input_query.single_mut() else {
        panic!("read_player_input ran without a Player with PlayerInput component!");
    };

    player_input.direction = Vec3::ZERO;

    if input.pressed(KeyCode::KeyD) {
        player_input.direction.x += 1.;
    }

    if input.pressed(KeyCode::KeyA) {
        player_input.direction.x -= 1.;
    }

    if input.pressed(KeyCode::KeyS) {
        player_input.direction.z += 1.;
    }

    if input.pressed(KeyCode::KeyW) {
        player_input.direction.z -= 1.;
    }
}

fn move_player(
    time: Res<Time>,
    mut player_input_query: Query<&mut PlayerInput, With<Player>>,
    mut player_controller_query: Query<&mut KinematicCharacterController, With<Player>>,
    camera_query: Query<&Transform, With<CameraController>>,
) {
    let Ok(player_input) = player_input_query.single_mut() else {
        panic!("move_player ran without a Player with Transform and PlayerInput components!");
    };

    let Ok(mut player_controller) = player_controller_query.single_mut() else {
        unreachable!(
            "move_player ran without a Player with Transform and PlayerInput components! And worse-- the first check didn't panic. Oops!"
        );
    };

    let Ok(camera_transform) = camera_query.single() else {
        panic!("no camera with a direction to move the player! (move_player in player/mod.rs)");
    };

    let mut forward = camera_transform.forward().as_vec3();
    forward.y = 0.0;
    forward = forward.normalize_or_zero();

    let right = Vec3::Y.cross(forward);

    let player_direction = -((player_input.direction.z * forward)
        + (player_input.direction.x * right))
        .normalize_or_zero();

    player_controller.translation = Some(player_direction * PLAYER_SPEED * time.delta_secs());
}

fn setup_player(mut commands: Commands) {
    let player_id = commands
        .spawn((
            Player,
            RigidBody::Dynamic,
            Collider::capsule_y(0.5, 1.0),
            KinematicCharacterController::default(),
            LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z,
            //Transform::default(),
            Transform::from_xyz(0., 20., 0.),
            PlayerInput::default(),
        ))
        .id();

    commands.insert_resource(PlayerEntity(player_id));
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player.in_set(PlayerSpawnSet))
            .add_systems(Update, read_player_input)
            .add_systems(Update, move_player);
    }
}
