use bevy::prelude::*;
use bevy_ggrs::ggrs;

const INPUT_UP: u8 = 1 << 0;
const INPUT_DOWN: u8 = 1 << 1;
const INPUT_LEFT: u8 = 1 << 2;
const INPUT_RIGHT: u8 = 1 << 3;
const INPUT_FIRE: u8 = 1 << 4;

pub fn input(_: In<ggrs::PlayerHandle>, keys: Res<Input<KeyCode>>) -> u8 {
    let mut input = 0u8;

    if keys.any_pressed([KeyCode::Up, KeyCode::W]) {
        input |= INPUT_UP;
    }
    if keys.any_pressed([KeyCode::Down, KeyCode::S]) {
        input |= INPUT_DOWN;
    }
    if keys.any_pressed([KeyCode::Left, KeyCode::A]) {
        input |= INPUT_LEFT
    }
    if keys.any_pressed([KeyCode::Right, KeyCode::D]) {
        input |= INPUT_RIGHT;
    }
    if keys.any_pressed([KeyCode::Space, KeyCode::Return]) {
        input |= INPUT_FIRE;
    }

    input
}

pub fn forward(input: u8) -> f32 {
    let mut result = 0.0;
    if input & INPUT_UP != 0 {
        result += 1.;
    }
    if input & INPUT_DOWN != 0 {
        result -= 1.;
    }
    result
}

pub fn rotation(input: u8) -> f32 {
    let mut result = 0.0;
    if input & INPUT_LEFT != 0 {
        result += 1.;
    }
    if input & INPUT_RIGHT != 0 {
        result -= 1.;
    }
    result
}

pub fn fire(input: u8) -> bool {
    input & INPUT_FIRE != 0
}
