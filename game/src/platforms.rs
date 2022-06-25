use crate::{components::*, WinSize};

use bevy::{prelude::*, transform};

pub struct PlatformsPlugin;

impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, platform_setup)
            .add_system(platform_setup);
    }
}

fn platform_setup(mut commands: Commands, windows: Res<WinSize>) {
    let (win_h, win_w) = (windows.win_h, windows.win_w);
    let ground_lvl: f32 = -win_h / 2. + 67.;
    commands.insert_resource(Platform {
        ground_level: ground_lvl,
        left_bound: -win_w / 2.,
        right_bound: win_w / 2.,
    });
}
