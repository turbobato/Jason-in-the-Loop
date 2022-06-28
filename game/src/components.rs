use bevy::prelude::*;

// region : general components

#[derive(Component)]
pub enum Movement {
    Idle,
    Running,
}

#[derive(Component)]
pub struct SpriteSize(pub Vec2);

impl From<(f32, f32)> for SpriteSize {
    fn from(val: (f32, f32)) -> Self {
        SpriteSize(Vec2::new(val.0, val.1))
    }
}

#[derive(Component, Default)]
pub struct Velocity {
    pub vx: f32,
    pub vy: f32,
}

impl From<(f32, f32)> for Velocity {
    fn from(val: (f32, f32)) -> Self {
        Velocity {
            vx: val.0,
            vy: val.1,
        }
    }
}

#[derive(Component, Default)]
pub struct Acceleration {
    pub ax: f32,
    pub ay: f32,
}

impl From<(f32, f32)> for Acceleration {
    fn from(val: (f32, f32)) -> Self {
        Acceleration {
            ax: val.0,
            ay: val.1,
        }
    }
}

#[derive(Component)]
pub struct Grounded(pub bool);

#[derive(Component)]
pub struct RepeatAnimation(pub bool);

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct Platform {
    pub size: Vec2,
    pub position: Vec3,
}
//endregion : general components

//region : player components

#[derive(Component)]
pub struct Player;

//endregion : player components

//region : enemy components
#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Skeleton;

#[derive(Component)]
pub struct Eye;

#[derive(Component)]
pub struct Projectile;

#[derive(Component)]
pub struct FromEnemy;
//endregion : enemy components
