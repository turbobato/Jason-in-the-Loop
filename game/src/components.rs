use bevy::prelude::*;

// region : general components

#[derive(Component)]
pub enum Movement {
    Idle,
    Running,
}

#[derive(Component)]
pub struct AffectedByGravity;

#[derive(Component, Default)]
pub struct Velocity {
    pub vx: f32,
    pub vy: f32,
}
#[derive(Component, Default)]
pub struct Acceleration {
    pub ax: f32,
    pub ay: f32,
}
#[derive(Component)]
pub struct SpriteSize(pub Vec2);

impl From<(f32, f32)> for SpriteSize {
    fn from(val: (f32, f32)) -> Self {
        SpriteSize(Vec2::new(val.0, val.1))
    }
}

#[derive(Component)]
pub struct Platform {
    pub ground_level: f32,
    pub left_bound: f32,
    pub right_bound: f32,
}

#[derive(Component)]
pub struct Grounded(pub bool);

#[derive(Component)]
pub struct RepeatAnimation(pub bool);

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

//endregion : general components

//region : player components

#[derive(Component)]
pub struct Player;

//endregion : player components

//region : enemy components
#[derive(Component)]
pub struct Enemy;

//endregion : enemy components
