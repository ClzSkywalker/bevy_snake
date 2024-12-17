use bevy::{color::palettes::css, prelude::*};

use crate::common;

#[derive(Debug, Clone, Component)]
#[require(Sprite)]
pub struct FoodComp;

impl FoodComp {
    pub fn new(pos: Vec2) -> (Self, Sprite, Transform) {
        (
            FoodComp,
            Sprite::from_color(css::WHITE, common::BLOCK_SIZE),
            Transform::from_xyz(pos.x, pos.y, 0.),
        )
    }
}
