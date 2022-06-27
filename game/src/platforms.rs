use crate::{components::*, WinSize, BACKGROUND_DIM};

use bevy::{prelude::*, text, transform};

const EARTHY_PLATFORM: &str = "textures/oak_woods_v1.0/platforms/earthy_platform(74x26).png";
const EARTHY_PLAT_DIM: (f32, f32) = (74., 26.);
const EARTHY_PLATFORM_SCALE: f32 = 2.;

const EARTHY_SPLATFORM: &str = "textures/oak_woods_v1.0/platforms/earthy_small_platform(50x26).png";
const EARTHY_SPLAT_DIM: (f32, f32) = (50., 26.);
const EARTHY_SPLATFORM_SCALE: f32 = 2.;

const HUGE_EARTHY_BLOCK: &str = "textures/oak_woods_v1.0/platforms/huge_block_earthy(97x97).png";
const EARTHY_HBLOCK_DIM: (f32, f32) = (97., 97.);
const EARTHY_HBLOCK_SCALE: f32 = 2.;

const HUGE_EARTHY_BLOCK_CUT: &str =
    "textures/oak_woods_v1.0/platforms/cut_earthy_hblock(82x98).png";
const EARTHY_HBLOCK_CUT_DIM: (f32, f32) = (82., 98.);
const EARTHY_HBLOCK_CUT_SCALE: f32 = 2.;

const TRANSITION_EARTH_DIRT: &str =
    "textures/oak_woods_v1.0/platforms/transition_platform_dirt (244x51).png";
const TRANSITION_EART_DIRT_DIM: (f32, f32) = (244., 51.);
const TRANSITION_EARTH_DIRT_SCALE: f32 = 2.4;

const TRASITION_PLAT_EARTH_ROCK: &str =
    "textures/oak_woods_v1.0/platforms/transition_platform(50x26).png";
const TRANSITION_EART_ROCK_DIM: (f32, f32) = (50., 26.);
const TRANSITION_EARTH_ROCK_SCALE: f32 = 2.;

const ROCK_HPLATFORM: &str = "textures/oak_woods_v1.0/platforms/rock_hplatform(97x25).png";
const ROCK_HPLAT_DIM: (f32, f32) = (97., 25.);
const ROCK_HPLATFORM_SCALE: f32 = 2.;
pub struct PlatformsPlugin;

impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, platform_setup);
    }
}

fn platform_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Res<WinSize>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    //commands.spawn_bundle(Camera2dBundle::default());

    let earthy_platform: Handle<Image> = asset_server.load(EARTHY_SPLATFORM);
    let earthy_small_platform: Handle<Image> = asset_server.load(EARTHY_PLATFORM);
    let earthy_hblock: Handle<Image> = asset_server.load(HUGE_EARTHY_BLOCK);
    let earthy_hblock_cut: Handle<Image> = asset_server.load(HUGE_EARTHY_BLOCK_CUT);
    let transition_plat_dirt: Handle<Image> = asset_server.load(TRANSITION_EARTH_DIRT);
    let transition_plat_earth_rock: Handle<Image> = asset_server.load(TRASITION_PLAT_EARTH_ROCK);
    let rock_hplatform: Handle<Image> = asset_server.load(ROCK_HPLATFORM);

    let (win_h, win_w) = (windows.win_h, windows.win_w);
    let ground_lvl: f32 = -win_h / 2. + 67.;

    commands.spawn_bundle(SpriteBundle {
        texture: earthy_hblock.clone(),
        transform: Transform {
            translation: Vec3::new(
                -BACKGROUND_DIM.0 / 2. + 1.5 * EARTHY_HBLOCK_DIM.0,
                -BACKGROUND_DIM.1 / 1.8 - EARTHY_HBLOCK_DIM.0 / 3.2,
                1.5,
            ),
            scale: Vec3::new(EARTHY_HBLOCK_SCALE, EARTHY_HBLOCK_SCALE, 1.),
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: earthy_hblock.clone(),
        transform: Transform {
            translation: Vec3::new(
                -BACKGROUND_DIM.0 / 2. + 0.5 * EARTHY_HBLOCK_DIM.0,
                -BACKGROUND_DIM.1 / 1.9,
                1.4,
            ),
            scale: Vec3::new(EARTHY_HBLOCK_SCALE, EARTHY_HBLOCK_SCALE, 1.),
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: transition_plat_dirt.clone(),
        transform: Transform {
            translation: Vec3::new(
                -BACKGROUND_DIM.0 / 2. - 0.89 * TRANSITION_EART_DIRT_DIM.0,
                -BACKGROUND_DIM.1 / 1.925,
                1.6,
            ),
            scale: Vec3::new(
                -TRANSITION_EARTH_DIRT_SCALE,
                TRANSITION_EARTH_DIRT_SCALE,
                1.,
            ),
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: earthy_hblock_cut.clone(),
        transform: Transform {
            translation: Vec3::new(
                EARTHY_HBLOCK_DIM.0,
                -BACKGROUND_DIM.1 / 2. - EARTHY_HBLOCK_DIM.1 / 1.9,
                1.5,
            ),
            scale: Vec3::new(EARTHY_HBLOCK_SCALE, EARTHY_HBLOCK_SCALE, 1.),
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: transition_plat_earth_rock.clone(),
        transform: Transform {
            translation: Vec3::new(
                4. * TRANSITION_EART_ROCK_DIM.0,
                -BACKGROUND_DIM.1 / 1.9 + 1.25 * TRANSITION_EART_ROCK_DIM.1,
                1.5,
            ),
            scale: Vec3::new(EARTHY_HBLOCK_SCALE, EARTHY_HBLOCK_SCALE, 1.),
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: rock_hplatform.clone(),
        transform: Transform {
            translation: Vec3::new(
                5. * TRANSITION_EART_ROCK_DIM.0,
                -BACKGROUND_DIM.1 / 1.9 + 1.25 * ROCK_HPLAT_DIM.1,
                1.4,
            ),
            scale: Vec3::new(ROCK_HPLATFORM_SCALE, ROCK_HPLATFORM_SCALE, 1.),
            ..Default::default()
        },
        ..Default::default()
    });
    /*
    commands.spawn_bundle(SpriteBundle{
        texture: earthy_small_platform.clone(),
        transform: Transform{
            translation: Vec3::new(-win_w/2. + 2. * EARTHY_SPLAT_DIM.0, - win_h/2.05 + EARTHY_HBLOCK_DIM.0 /2.2 ,1.6),
            scale: Vec3::new(EARTHY_HBLOCK_SCALE,EARTHY_HBLOCK_SCALE, 1.),
            ..Default::default()
        },
        ..Default::default()

    });*/
}
