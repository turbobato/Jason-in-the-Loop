use bevy::prelude::*;

mod components;
mod player;

use crate::{player::*};
//region: ---ASSET CONSTANT

//region: ---Resources
pub struct WinSize{
		pub w: f32,
		pub h: f32,
}
struct GameTextures{
	player: Handle<Image>
}

//endregion: ---Resources

const PLAYER_SPRITE: &str = "_Crouch.png";
const PLAYER_SIZE: (f32,f32) = (120., 80.);
const SPRITE_SCALE: f32 = 2.;

// endregion: ---ASSET CONSTANT
fn main() {
	App::new()
		.insert_resource(ClearColor(Color::rgb(0.,0.,0.5)))
		.insert_resource(WindowDescriptor{
			title: "Projet X".to_string(),
			width: 600.0,
			height: 700.0,
			..Default::default()
		})
		.add_plugins(DefaultPlugins)
		.add_plugin(PlayerPlugin)
		.add_startup_system(setup_system)
		.run()
	}

	fn setup_system(mut commands: Commands, 
		asset_serveur: Res<AssetServer>, 
		mut windows: ResMut<Windows>,
	) {
		//camera
		commands.spawn_bundle(OrthographicCameraBundle::new_2d());

		//window size 
		let window = windows.get_primary_mut().unwrap();
		let (win_w, win_h) = (window.width(), window.height());
		
		//position of window
		window.set_position(IVec2::new(1100, 0));
		let win_size = WinSize {w: win_w, h: win_h};
		commands.insert_resource(win_size);	

		//game texture resource
		let game_texture = GameTextures{
			player: asset_serveur.load(PLAYER_SPRITE)
		};
		commands.insert_resource(game_texture);
	}
	