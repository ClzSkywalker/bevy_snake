pub mod snake;

use bevy::{color::palettes::css::BLUE, prelude::*};

use crate::{
    common::BLOCK_SIZE,
    component::{food::FoodComp, CounterComp},
};

#[derive(Bundle)]
pub struct BorderBundle {
    pub sprite: Sprite,
    pub transform: Transform,
}

impl BorderBundle {
    pub fn new(pos: Vec2) -> Self {
        BorderBundle {
            sprite: Sprite::from_color(Color::WHITE, Vec2::new(20., 20.)),
            transform: Transform::from_xyz(pos.x, pos.y, 0.),
        }
    }
}

#[derive(Bundle)]
pub struct FoodBoundle {
    pub food_comp: FoodComp,
    pub sprite: Sprite,
    pub transform: Transform,
}

impl FoodBoundle {
    pub fn new(pos: Vec2) -> Self {
        FoodBoundle {
            food_comp: FoodComp,
            sprite: Sprite::from_color(BLUE, BLOCK_SIZE),
            transform: Transform::from_xyz(pos.x, pos.y, 0.),
        }
    }
}

#[derive(Bundle)]
pub struct CounterBundle {
    pub comp: CounterComp,
    pub text: Text,
    pub text_font: TextFont,
    pub text_layout: TextLayout,
    pub node: Node,
}

impl CounterBundle {
    pub fn new(text: String) -> Self {
        Self {
            comp: CounterComp,
            text: Text::new(text),
            text_font: TextFont {
                font_size: 30.,
                ..default()
            },
            text_layout: TextLayout::new_with_justify(JustifyText::Center),
            node: Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(5.),
                right: Val::Px(5.),
                ..default()
            },
        }
    }
}
