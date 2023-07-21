use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub handle: usize,
}

#[derive(Component, Reflect, Default)]
pub struct BulletReady(pub bool);

#[derive(Component, Reflect, Default)]
pub struct Bullet;

#[derive(Component, Reflect, Default)]
pub struct RotationSpeed(pub f32);

#[derive(Component, Reflect, Default)]
pub struct MoveSpeed(pub f32);
