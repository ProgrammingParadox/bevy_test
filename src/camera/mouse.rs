use bevy::{
    input::mouse::AccumulatedMouseMotion,
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};

use std::f32::consts::FRAC_PI_2;

#[derive(Component)]
pub struct CameraController {
    pub yaw: f32,
    pub pitch: f32,
    pub sensitivity: Vec2,
}

impl Default for CameraController {
    fn default() -> Self {
        CameraController {
            yaw: 0.0,
            pitch: 0.0,
            sensitivity: Vec2::new(0.003, 0.002),
        }
    }
}

pub fn get_camera_input(
    mut camera_controller_query: Query<&mut CameraController>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    cursor_options: Single<&CursorOptions>,
) {
    // if cursor_options.grab_mode == CursorGrabMode::None {
    //     return;
    // }

    let Ok(mut camera_controller) = camera_controller_query.single_mut() else {
        println!("no camera controller!");

        return;
    };

    let delta = accumulated_mouse_motion.delta;

    // println!("get_camera_input {:}", delta);

    if delta != Vec2::ZERO {
        // Note that we are not multiplying by delta_time here.
        // The reason is that for mouse movement, we already get the full movement that happened since the last frame.
        // This means that if we multiply by delta_time, we will get a smaller rotation than intended by the user.
        // This situation is reversed when reading e.g. analog input from a gamepad however, where the same rules
        // as for keyboard input apply. Such an input should be multiplied by delta_time to get the intended rotation
        // independent of the framerate.
        let delta_yaw = -delta.x * camera_controller.sensitivity.x;
        let delta_pitch = -delta.y * camera_controller.sensitivity.y;

        let yaw = camera_controller.yaw + delta_yaw;

        // If the pitch was ±¹⁄₂ π, the camera would look straight up or down.
        // When the user wants to move the camera back to the horizon, which way should the camera face?
        // The camera has no way of knowing what direction was "forward" before landing in that extreme position,
        // so the direction picked will for all intents and purposes be arbitrary.
        // Another issue is that for mathematical reasons, the yaw will effectively be flipped when the pitch is at the extremes.
        // To not run into these issues, we clamp the pitch to a safe range.
        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let pitch = (camera_controller.pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        camera_controller.pitch = pitch;
        camera_controller.yaw = yaw;
    }
}

pub fn apply_camera_input(mut controller_query: Query<(&mut Transform, &CameraController)>) {
    let Ok((mut transform, camera_controller)) = controller_query.single_mut() else {
        return;
    };

    // println!("move camera {:}", camera_controller.yaw);

    transform.rotation = Quat::from_euler(
        EulerRot::YXZ,
        camera_controller.yaw,
        camera_controller.pitch,
        0.0,
    );
}
