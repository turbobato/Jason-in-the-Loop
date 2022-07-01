use crate::components::*;
use bevy::prelude::*;

const PLATFORM_JSON: &str = include_str!("../platform.json");

pub struct PlatformsPlugin;

impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(platform_setup)
            .add_system(platform_movement);
    }
}

fn platform_setup(mut commands: Commands) {
    let platforms: Vec<Platform> = serde_json::from_str(PLATFORM_JSON).unwrap();
    for platform in platforms {
        commands.spawn().insert(platform);
    }
}

fn platform_movement(mut query: Query<(&mut Platform, &Transform), With<MovingPlatform>>) {
    for (mut platform, transform) in query.iter_mut() {
        platform.position.x = transform.translation.x;
        platform.position.y = transform.translation.y;
    }
}
