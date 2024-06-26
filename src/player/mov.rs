use bevy::prelude::*;
use super::main_p::{MainPlayer, Speed};

pub fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_q: Query<(&mut Transform, &Speed), With<MainPlayer>>,
    cam_q: Query<&Transform, (With<Camera3d>, Without<MainPlayer>)>
) {
    for (mut player_transform, player_speed) in player_q.iter_mut() {
        let cam_transform = match cam_q.get_single() {
            Ok(cam) => cam,
            Err(e) => {
                // Log the error or handle it appropriately
                println!("Error on camera: {}", e);
                continue;  // Skip this iteration if the camera is not found
            }
        };

        let mut direction = Vec3::ZERO;  // Initialize direction as zero vector

        if keys.pressed(KeyCode::KeyW) {
            direction += *cam_transform.forward();
        }
        if keys.pressed(KeyCode::KeyS) {
            direction += *-cam_transform.forward();
        }
        if keys.pressed(KeyCode::KeyD) {
            direction += *cam_transform.right();
        }
        if keys.pressed(KeyCode::KeyA) {
            direction += *-cam_transform.right();
        }

        direction.y = 0.0;

        let movement = direction.normalize_or_zero() * player_speed.value * time.delta_seconds();
        player_transform.translation += movement;
    }
}