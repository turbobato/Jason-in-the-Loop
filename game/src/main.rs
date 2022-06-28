mod collisions;
mod components;
mod enemy;
mod platforms;
mod player;

use bevy::{
    log::LogSettings,
    math::{const_vec3, Vec3Swizzles},
    prelude::*,
    render::texture::ImageSettings,
    sprite::collide_aabb::{collide, Collision},
};

use collisions::CollisionsPlugin;
use components::*;
use enemy::EnemyPlugin;
use platforms::PlatformsPlugin;
use player::PlayerPlugin;

pub const GROUND_LEVEL: f32 = 0.;
pub const PLATFORM_MARGIN: f32 = 4.; // this is the thickness of the platforms

const BACKGROUND_1: &str = "textures/oak_woods_v1.0/background/background_game/background_1.png";
const BACKGROUND_2: &str = "textures/oak_woods_v1.0/background/background_game/background_2.png";
/*
const BACKGROUND_LAYER1: &str = "textures/oak_woods_v1.0/background/background_layer_1.png";
const BACKGROUND_LAYER2: &str = "textures/oak_woods_v1.0/background/background_layer_2.png";
const BACKGROUND_LAYER3: &str = "textures/oak_woods_v1.0/background/background_layer_3.png";
*/
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
        .add_startup_system(setup)
        .add_system(animate_sprite)
        .add_system(movement)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, windows: Res<Windows>) {
    commands.spawn_bundle(Camera2dBundle::default());

    let background_1: Handle<Image> = asset_server.load(BACKGROUND_1);
    let background_2: Handle<Image> = asset_server.load(BACKGROUND_2);
    /*
    let background_layer1: Handle<Image> = asset_server.load(BACKGROUND_LAYER1);
    let background_layer2: Handle<Image> = asset_server.load(BACKGROUND_LAYER2);
    let background_layer3: Handle<Image> = asset_server.load(BACKGROUND_LAYER3);
    */
    // capture window size
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
    /*
    commands.spawn().insert(Platform {
        size: Vec2::new(win_w, PLATFORM_MARGIN),
        position: Vec3::new(0., GROUND_LEVEL, 0.),
    });*/

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::AQUAMARINE,
                custom_size: Some(Vec2::new(250., PLATFORM_MARGIN)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(-356., -145., 3.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Platform {
            position: Vec3::new(-356., -145., 1.),
            size: Vec2::new(250., PLATFORM_MARGIN),
        });

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::AQUAMARINE,
                custom_size: Some(Vec2::new(283., PLATFORM_MARGIN)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(-248., -205., 3.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Platform {
            position: Vec3::new(-248., -205., 1.),
            size: Vec2::new(283., PLATFORM_MARGIN),
        });

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::AQUAMARINE,
                custom_size: Some(Vec2::new(283., PLATFORM_MARGIN)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(150., -205., 3.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Platform {
            position: Vec3::new(150., -205., 1.),
            size: Vec2::new(283., PLATFORM_MARGIN),
        });

    /*
    commands.insert_resource(WinSize { win_h, win_w });
    commands.spawn_bundle(SpriteBundle {
        texture: background_layer1,
        transform: Transform {
            translation: Vec3::new(0., 0., 0.),
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: background_layer2,
        transform: Transform {
            translation: Vec3::new(0., 0., 1.),
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: background_layer3,
        transform: Transform {
            translation: Vec3::new(0., 0., 1.),
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
            ..Default::default()
        },
        ..Default::default()
    });*/
    /*
    commands.spawn().insert(Platform {
        size: Vec2::new(win_w, PLATFORM_MARGIN),
        position: Vec3::new(0., GROUND_LEVEL, 0.),
    });*/
    /*
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::AQUAMARINE,
                custom_size: Some(Vec2::new(70., PLATFORM_MARGIN)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0., 40., 3.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Platform {
            position: Vec3::new(0., 40., 1.),
            size: Vec2::new(70., PLATFORM_MARGIN),
        });
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::AQUAMARINE,
                custom_size: Some(Vec2::new(70., PLATFORM_MARGIN)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0., 120., 3.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Platform {
            position: Vec3::new(0., 120., 1.),
            size: Vec2::new(70., PLATFORM_MARGIN),
        });
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::AQUAMARINE,
                custom_size: Some(Vec2::new(70., PLATFORM_MARGIN)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0., 80., 3.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Platform {
            position: Vec3::new(0., 80., 1.),
            size: Vec2::new(70., PLATFORM_MARGIN),
        });*/
}

/*          .insert(Platform {
    ground_level: ground_lvl + 50.,
    left_bound: 100. / 2.,
    right_bound: 150.,
})*/
/* let player_sprite = asset_server.load(CROUCH_SPRITE);
commands.spawn_bundle(SpriteBundle{
    texture : player_sprite,
    transform : Transform {
        translation : Vec3::new(0.,ground_lvl,1.),
        //scale : Vec3::new(2.,2.,1.),
        ..Default::default()
    },
    ..Default::default()
}); */

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
        } else {
            acceleration.ay = -100.;
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
