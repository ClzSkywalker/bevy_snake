// countdown to control item movement
use bevy::prelude::*;

use crate::component::MoveComp;

// countdown to control item movement
pub struct GameInterval;
// countdown to control food creation
pub struct FoodCreateInterval;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CountdownState {
    Process,
    Pause,
}

#[derive(Debug, Clone, Resource)]
pub struct CountdownRes<T> {
    pub state: CountdownState,
    pub interval: f32,
    pub time: f32,
    pub _data: std::marker::PhantomData<T>,
}

impl<T> CountdownRes<T> {
    pub fn new(interval: f32) -> CountdownRes<T> {
        CountdownRes {
            state: CountdownState::Process,
            interval,
            time: 0.,
            _data: std::marker::PhantomData,
        }
    }

    pub fn add_delta(&mut self, t: f32) {
        if self.time >= self.interval {
            self.time = 0.;
            return;
        }
        if self.time + t < self.interval {
            self.time += t;
            return;
        }
        self.time = self.interval;
    }

    pub fn is_complete(&self) -> bool {
        self.interval == self.time
    }
}

#[derive(Debug, Clone, Resource)]
pub enum ControlNextKeyRes {
    Up,
    Down,
    Left,
    Right,
}

// 蛇身躯
#[derive(Debug, Clone, Default, Resource)]
pub struct BodySegmentRes {
    pub items: Vec<Entity>,
}

// 最后一个位置的数据
#[derive(Debug, Clone, Default, Resource)]
pub struct TailPosRes {
    pub pos: Vec2,
    pub move_comp: MoveComp,
}

impl TailPosRes {
    pub fn set(&mut self, pos: Vec2, move_comp: MoveComp) {
        self.pos = pos;
        self.move_comp = move_comp;
    }
}
