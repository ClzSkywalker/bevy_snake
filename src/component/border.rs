use bevy::prelude::*;

#[derive(Debug, Component)]
#[require(Sprite)]
pub struct BorderComp;

impl BorderComp {
    pub fn new(pos: Vec2) -> (Self, Sprite, Transform) {
        (
            BorderComp,
            Sprite::from_color(Color::WHITE, Vec2::new(20., 20.)),
            Transform::from_xyz(pos.x, pos.y, 0.),
        )
    }
}
