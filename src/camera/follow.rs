use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct FollowTarget {
    pub entity: Entity,
    pub offset: Vec3,
}

pub fn follow_camera(
    mut camera_query: Query<(&mut Transform, &FollowTarget), With<Camera>>,
    transform_query: Query<&Transform, Without<Camera>>,
) {
    for (mut camera_transform, follow_target) in &mut camera_query {
        let entity = follow_target.entity;
        if let Ok(entity_transform) = transform_query.get(entity) {
            camera_transform.translation = entity_transform.translation + follow_target.offset;

            camera_transform.look_at(entity_transform.translation, Vec3::Y);
        }
    }
}
