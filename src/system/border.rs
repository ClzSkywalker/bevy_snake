use bevy::prelude::*;

use crate::{
    common::{BLOCK_SIZE, WINDOW_BORDER_END, WINDOW_BORDER_START},
    component::border::BorderComp,
};

pub struct BorderPlugin;

impl Plugin for BorderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, border_init);
    }
}

// draw border
fn border_init(mut command: Commands) {
    let start = WINDOW_BORDER_START;
    let end = WINDOW_BORDER_END;
    let border = BLOCK_SIZE;

    // 区间：左闭右开，所以需要+1
    for pos in start.x as i32..end.x as i32 + 1 {
        command.spawn(BorderComp::new(Vec2::new(
            pos as f32 * border.x,
            start.y * border.y,
        )));
        command.spawn(BorderComp::new(Vec2::new(
            pos as f32 * border.x,
            end.y * border.y,
        )));
    }

    // 上下边缘已渲染，所以从start.y+BLOCK_SIZE.y开始
    for pos in start.y as i32 + 1..end.y as i32 {
        command.spawn(BorderComp::new(Vec2::new(
            start.x * border.x,
            pos as f32 * border.y,
        )));
        command.spawn(BorderComp::new(Vec2::new(
            end.x * border.x,
            pos as f32 * border.y,
        )));
    }
}
