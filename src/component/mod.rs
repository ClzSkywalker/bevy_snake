use bevy::{math::Vec2, prelude::Component};

use crate::resources::ControlNextKeyRes;

pub mod body;
pub mod counter;
pub mod food;

#[derive(Debug, Clone, Component)]
pub struct MoveComp {
    pub dire: Vec2,
    pub speed: f32,
}

impl Default for MoveComp {
    fn default() -> Self {
        Self {
            dire: Vec2::new(1., 0.),
            speed: 1.,
        }
    }
}

impl MoveComp {
    pub fn get_direction(&self) -> Vec2 {
        self.dire * self.speed
    }

    pub fn upd_dirc(&mut self, dirc: ControlNextKeyRes) {
        match dirc {
            ControlNextKeyRes::Up => {
                if self.dire.y == 0. {
                    self.dire = Vec2::new(0., 1.);
                }
            }
            ControlNextKeyRes::Down => {
                if self.dire.y == 0. {
                    self.dire = Vec2::new(0., -1.);
                }
            }
            ControlNextKeyRes::Left => {
                if self.dire.x == 0. {
                    self.dire = Vec2::new(-1., 0.);
                }
            }
            ControlNextKeyRes::Right => {
                if self.dire.x == 0. {
                    self.dire = Vec2::new(1., 0.);
                }
            }
        }
    }
}

#[derive(Debug, Default, Clone, Component)]
pub struct CounterComp;
