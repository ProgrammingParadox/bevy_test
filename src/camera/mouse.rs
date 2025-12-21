use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

#[derive(Component)]
pub struct CameraController {
    yaw: f32,
    pitch: f32,
    sensitivity: f32,
}

pub fn get_camera_input(mut camera_controller_query: Query<&mut CameraController>) {}
