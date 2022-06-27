mod components;
mod player;
mod collisions;

use bevy::{
    log::LogSettings,
    prelude::*,
    render::texture::ImageSettings,
    sprite::collide_aabb::{collide, Collision},
};
use collisions::CollisionsPlugin;
use components::*;
use player::PlayerPlugin;

const BACKGROUND: &str = "textures/forest/Free Pixel Art Forest/Preview/Background.png";
pub const GROUND_LEVEL: f32 = -330.5;
pub const PLATFORM_MARGIN: f32 = 1.; // this is the thickness of the platforms

// TODO : refactor interactions with ground (add sprite sizes constants, add ground components and change ground level)

struct WinSize {
    win_h: f32,
    win_w: f32,
}

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest()) //prevent blurry sprites
        .insert_resource(WindowDescriptor {
            title: "ProjetX".to_string(),
            width: 928.0,
            height: 793.0,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(CollisionsPlugin)
        .add_startup_system(setup)
        .add_system(animate_sprite)
        .add_system(movement)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, windows: Res<Windows>) {
    commands.spawn_bundle(Camera2dBundle::default());
    let background_image: Handle<Image> = asset_server.load(BACKGROUND);
    let window = windows.get_primary().unwrap();
    let (win_h, win_w) = (window.height(), window.width());
    commands.insert_resource(WinSize { win_h, win_w });
    commands
        .spawn_bundle(SpriteBundle {
            texture: background_image,
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Platform {
            size: Vec2::new(win_w, PLATFORM_MARGIN),
            position: Vec3::new(0., GROUND_LEVEL, 0.),
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
    mut query: Query<(&Grounded, &mut Velocity, &mut Acceleration, &mut Transform), With<Player>>,
) {
    let delta = time.delta_seconds();
    for (grounded, mut velocity, mut acceleration, mut transform) in query.iter_mut() {
        transform.translation.x += velocity.vx * delta;
        transform.translation.y += velocity.vy * delta;
        velocity.vx += acceleration.ax * delta;
        velocity.vy += acceleration.ay * delta;
        if grounded.0 {
            acceleration.ay = 0.;
            velocity.vy = 0.;
        } else {
            acceleration.ay = -100.
        }
        /* let sprite_height = texture_atlases.get(texture_atlas).unwrap().size.y /2.;
        for platform in query_platforms.iter(){
            let ground_level = platform.ground_level;
            let left_bound = platform.left_bound;
            let right_bound = platform.right_bound;
            //println!("ground_level {ground_level}, left_bound {left_bound}, right_bound {right_bound}, y_level : {}", transform.translation.y);
            if grounded.0 == false {
                if ground_level + sprite_height + MARGIN >= transform.translation.y
                && ground_level + sprite_height - MARGIN <= transform.translation.y
                && transform.translation.x >= left_bound
                && transform.translation.x <= right_bound
                {
                    acceleration.ay = 0.;
                    velocity.vy = 0.;
                    grounded.0 = true;
                }
                else {
                    acceleration.ay = -100.
                }
            }
        } */
    }
}

