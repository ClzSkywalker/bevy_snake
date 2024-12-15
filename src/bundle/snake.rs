use bevy::{
    color::palettes::css::{GREEN, RED},
    prelude::*,
    sprite::Sprite,
};

use crate::{
    common::BLOCK_SIZE,
    component::{
        body::{BodyComp, HeadComp, SnakeComp},
        MoveComp,
    },
};

#[derive(Clone, Bundle)]
pub struct SnakeBodyBundle {
    pub snake: SnakeComp,
    pub move_comp: MoveComp,
    pub sprite: Sprite,
    pub transform: Transform,
}

impl SnakeBodyBundle {
    pub fn new_head() -> (Self, HeadComp) {
        let head = SnakeBodyBundle {
            snake: SnakeComp,
            move_comp: MoveComp::default(),
            sprite: Sprite::from_color(RED, BLOCK_SIZE),
            transform: Transform::default(),
        };
        (head, HeadComp)
    }

    pub fn new_body(pos: Vec2, move_comp: MoveComp) -> (Self, BodyComp) {
        let body = SnakeBodyBundle {
            sprite: Sprite::from_color(GREEN, BLOCK_SIZE),
            transform: Transform::from_xyz(pos.x, pos.y, 0.),
            snake: SnakeComp,
            move_comp,
        };
        (body, BodyComp)
    }
}
