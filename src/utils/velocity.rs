use bevy::prelude::*;

const GRAVITY: f64 = 0.98;

#[derive(Debug, Component)]
pub struct Gravity;

#[derive(Component)]
pub struct Velocity {
    linear: Vec3,
    angular: Vec3,
}

impl Default for Velocity {
    fn default() -> Velocity {
        Velocity {
            linear: Vec3::ZERO,
            angular: Vec3::ZERO,
        }
    }
}

fn apply_velocity(mut translation_query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut translation_query {
        transform.translation += velocity.linear;

        transform.rotate_x(velocity.angular.x);
        transform.rotate_y(velocity.angular.y);
        transform.rotate_z(velocity.angular.z);
    }
}

// fn apply_gravity(mut )
