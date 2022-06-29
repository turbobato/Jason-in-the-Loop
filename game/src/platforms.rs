use crate::components::*;
use bevy::prelude::*;

const PLATFORM_JSON: &str = include_str!("../platform.json");

pub struct PlatformsPlugin;

impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(platform_setup);
    }
}

fn platform_setup(mut commands: Commands) {
    let platforms: Vec<Platform> = serde_json::from_str(PLATFORM_JSON).unwrap();
    for platform in platforms {
        commands.spawn().insert(platform);
    }
}
