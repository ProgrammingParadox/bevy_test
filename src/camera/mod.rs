use bevy::prelude::*;

mod follow;
pub mod mouse;

use crate::player::{PlayerEntity, PlayerSpawnSet};

fn setup_camera(mut commands: Commands, player_entity: Res<PlayerEntity>) {
    commands.spawn((
        Camera3d::default(),
        Transform::default(),
        mouse::CameraController::default(),
        follow::FollowTarget {
            entity: player_entity.0,
            offset: Vec3::new(0., 0., 0.),
        },
    ));
}

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera.after(PlayerSpawnSet))
            .add_systems(Update, follow::follow_camera)
            .add_systems(Update, (mouse::get_camera_input, mouse::apply_camera_input));
    }
}
