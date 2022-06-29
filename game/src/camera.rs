use bevy::prelude::*;

use crate::{components::Player, WinSize};

pub struct CameraPlugin;

const CAMERA_LIMITS: (f32, f32) = (0., 5280.);

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(flying_camera_setup);
    }
}

fn flying_camera_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Res<WinSize>,
    query_player: Query<&Transform, (With<Player>, Without<Camera>)>,
    mut query_camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    //let window = windows.get_primary().unwrap();
    let (win_h, win_w) = (windows.win_h, windows.win_w);
    for (player_transform) in query_player.iter() {
        for (mut camera_transform) in query_camera.iter_mut() {
            if camera_transform.translation.x < CAMERA_LIMITS.0
                || camera_transform.translation.x > CAMERA_LIMITS.1
            {
                camera_transform.translation.x = 0.;
            } else if player_transform.translation.x >= 200. + camera_transform.translation.x
                && camera_transform.translation.x + 200. < CAMERA_LIMITS.1
            {
                camera_transform.translation.x = -200. + player_transform.translation.x;
            } else if player_transform.translation.x <= camera_transform.translation.x - 200.
                && camera_transform.translation.x - 200. > CAMERA_LIMITS.0
            {
                camera_transform.translation.x = 200. + player_transform.translation.x;
            }
        }
    }
}
