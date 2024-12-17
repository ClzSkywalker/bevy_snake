use bevy::{color::palettes::css, prelude::*};

use crate::common::BLOCK_SIZE;

use super::MoveComp;

#[derive(Debug, Clone, Default, Component)]
#[require(MoveComp)]
#[require(Sprite)]
pub struct SnakeComp;

impl SnakeComp {
    pub fn new_head() -> (Self, Sprite, HeadComp) {
        let sprite = Sprite::from_color(css::RED, BLOCK_SIZE);
        (Self, sprite, HeadComp)
    }

    pub fn new_body(
        pos: Vec2,
        move_comp: MoveComp,
    ) -> (Self, BodyComp, MoveComp, Sprite, Transform) {
        let sprite = Sprite::from_color(css::GREEN, BLOCK_SIZE);
        let transform = Transform::from_xyz(pos.x, pos.y, 0.);
        (Self, BodyComp, move_comp, sprite, transform)
    }
}

#[derive(Debug, Clone, Component)]
pub struct HeadComp;

#[derive(Debug, Clone, Component)]
pub struct BodyComp;
