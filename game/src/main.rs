mod camera;
mod collisions;
mod components;
mod enemy;
mod platforms;
mod player;
mod player_loop;

use bevy::{
    log::LogSettings,
    math::{const_vec3, Vec3Swizzles},
    prelude::*,
    render::texture::ImageSettings,
    sprite::collide_aabb::{collide, Collision},
};

use camera::CameraPlugin;
use collisions::CollisionsPlugin;
use components::*;
use enemy::EnemyPlugin;
use platforms::PlatformsPlugin;
use player::PlayerPlugin;
use player_loop::PlayerLoopPlugin;

pub const GROUND_LEVEL: f32 = 0.;
pub const PLATFORM_MARGIN: f32 = 4.; // this is the thickness of the platforms

const BACKGROUND_1: &str = "textures/oak_woods_v1.0/background/background_game/background_1.png";
const BACKGROUND_2: &str = "textures/oak_woods_v1.0/background/background_game/background_2.png";
const BACKGROUND_3: &str = "textures/oak_woods_v1.0/background/background_game/background_3.png";
const BACKGROUND_4: &str = "textures/oak_woods_v1.0/background/background_game/background_4.png";
const BACKGROUND_5: &str =
    "textures/oak_woods_v1.0/background/background_game/background_layer_5.png";
const BACKGROUND_6: &str = "textures/oak_woods_v1.0/background/background_game/background_6.png";

const SHOP_SPRITE: &str = "textures/oak_woods_v1.0/decorations/shop_anim.png";
const BACKGROUND_DIM: (f32, f32) = (960., 540.);
const SPRITE_SCALE: f32 = 3.;

struct WinSize {
    win_h: f32,
    win_w: f32,
}

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest()) //prevent blurry sprites
        .insert_resource(WindowDescriptor {
            title: "ProjetX".to_string(),
            width: 960.0,
            height: 540.0,
            resizable: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(CollisionsPlugin)
        .add_plugin(PlatformsPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(PlayerLoopPlugin)
        .add_startup_system(setup)
        .add_system(animate_sprite)
        .add_system(movement)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands
        .spawn_bundle(Camera2dBundle::default());

    let background_1: Handle<Image> = asset_server.load(BACKGROUND_1);
    let background_2: Handle<Image> = asset_server.load(BACKGROUND_2);
    let background_3: Handle<Image> = asset_server.load(BACKGROUND_3);
    let background_4: Handle<Image> = asset_server.load(BACKGROUND_4);
    let background_5: Handle<Image> = asset_server.load(BACKGROUND_5);
    let background_6: Handle<Image> = asset_server.load(BACKGROUND_6);

    let shop_sprite: Handle<Image> = asset_server.load(SHOP_SPRITE);
    let texture_atlas_shop = TextureAtlas::from_grid(shop_sprite, Vec2::new(118., 128.), 6, 1);
    let texture_atlas_handle_shop = texture_atlases.add(texture_atlas_shop);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle_shop,
            transform: Transform {
                translation: Vec3::new(2. * BACKGROUND_DIM.0 - 395., -203. + 192., 1.),
                scale: Vec3::splat(3.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)));

    let window = windows.get_primary().unwrap();
    let (win_h, win_w) = (window.height(), window.width());

    commands.insert_resource(WinSize { win_h, win_w });

    commands.spawn_bundle(SpriteBundle {
        texture: background_1,
        transform: Transform {
            translation: Vec3::new(0., 0., 0.),
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn_bundle(SpriteBundle {
        texture: background_2,
        transform: Transform {
            translation: Vec3::new(BACKGROUND_DIM.0, 0., 0.),
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn_bundle(SpriteBundle {
        texture: background_3,
        transform: Transform {
            translation: Vec3::new(2. * BACKGROUND_DIM.0, 0., 0.),
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn_bundle(SpriteBundle {
        texture: background_4,
        transform: Transform {
            translation: Vec3::new(3. * BACKGROUND_DIM.0, 0., 0.),
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn_bundle(SpriteBundle {
        texture: background_5,
        transform: Transform {
            translation: Vec3::new(4. * BACKGROUND_DIM.0, 0., 0.),
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn_bundle(SpriteBundle {
        texture: background_6,
        transform: Transform {
            translation: Vec3::new(5. * BACKGROUND_DIM.0, 0., 0.),
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

fn movement(
    time: Res<Time>,
    mut query: Query<
        (&Grounded, &mut Velocity, &mut Acceleration, &mut Transform),
        Or<(With<Player>, With<Skeleton>, With<TemporalGhost>)>,
    >,
) {
    let delta = time.delta_seconds();
    for (grounded, mut velocity, mut acceleration, mut transform) in query.iter_mut() {
        transform.translation.x += velocity.vx * delta;
        transform.translation.y += velocity.vy * delta;
        velocity.vx += acceleration.ax * delta;
        velocity.vy += acceleration.ay * delta;
        if grounded.0 {
            acceleration.ay = 0.;
        } else {
            acceleration.ay = -400.;
        }
    }
}
