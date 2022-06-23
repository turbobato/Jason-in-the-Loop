use bevy::prelude::*;


// region : general components

#[derive(Component)]
pub enum Movement{
    Idle,
    Running,
}

#[derive(Component, Default)]
pub struct Velocity {
	pub vx : f32,
	pub vy : f32,
}

#[derive(Component, Default)]
pub struct Acceleration {
    pub ax : f32,
    pub ay : f32,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct Platform{
    pub ground_level : f32,
    pub left_bound : f32,
    pub right_bound : f32,
}
//endregion : general components 

//region : player components

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Grounded(pub bool);

//endregion : player components

//region : enemy components
#[derive(Component)]
pub struct Enemy;

//endregion : enemy components 