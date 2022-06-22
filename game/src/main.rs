mod components;
<<<<<<< HEAD
=======
mod player;

>>>>>>> f599957c2da12101e6302bfb3470e924b10835f0
use components::*;
use bevy::{prelude::*, render::texture::ImageSettings};

const BACKGROUND : &str = "textures/forest/Free Pixel Art Forest/Preview/Background.png";
const CROUCH_SPRITE : &str = "textures/knight/Colour1/NoOutline/120x80_PNGSheets/_Crouch.png";

struct WinSize{
    win_h : f32,
    win_w : f32,
}

struct GroundLevel(f32);

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "ProjetX".to_string(),
            width: 928.0,
            height: 793.0,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}


<<<<<<< HEAD
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("textures/knight/Colour2/Outline/120x80_PNGSheets/_AttackComboNoMovement.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(120.0, 80.0), 10, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
=======
fn setup(mut commands : Commands, asset_server : Res<AssetServer>, windows : Res<Windows>){
>>>>>>> f599957c2da12101e6302bfb3470e924b10835f0
    commands.spawn_bundle(Camera2dBundle::default());
    let background_image: Handle<Image> = asset_server.load(BACKGROUND);
    commands.spawn_bundle(SpriteBundle {
        texture : background_image,
        transform : Transform {
            translation : Vec3::new(0.,0.,0.),
            ..Default::default()
        },
        ..Default::default()
    });
    if let Some(window) = windows.get_primary() {
        let (win_h, win_w) = (window.height(),window.width());
        let ground_lvl :f32 = -win_h/2. + 103.;
        commands.insert_resource(WinSize {win_h, win_w,});
        commands.insert_resource(GroundLevel(ground_lvl));
        let player_sprite = asset_server.load(CROUCH_SPRITE);
        commands.spawn_bundle(SpriteBundle{
            texture : player_sprite,
            transform : Transform {
                translation : Vec3::new(0.,ground_lvl,1.),
                //scale : Vec3::new(2.,2.,1.),
                ..Default::default()
            },
            ..Default::default()
        });
    }
    
    
}
