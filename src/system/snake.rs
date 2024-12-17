use bevy::prelude::*;

use crate::{
    common::BLOCK_SIZE,
    component::{
        body::{BodyComp, HeadComp, SnakeComp},
        food::FoodComp,
        MoveComp,
    },
    resources::{BodySegmentRes, ControlNextKeyRes, CountdownRes, GameInterval, TailPosRes},
};

use super::counter::CountEvent;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BodySegmentRes::default())
            .insert_resource(TailPosRes::default())
            .add_systems(Startup, snake_init)
            .add_systems(Update, (move_system, eat_food, control_system));
    }
}

fn snake_init(
    mut command: Commands,
    mut seg: ResMut<BodySegmentRes>,
    mut tail_pos: ResMut<TailPosRes>,
) {
    command.spawn(Camera2d);
    let head = SnakeComp::new_head();
    tail_pos.set(Vec2::default(), MoveComp::default());

    let id = command.spawn(head).id();
    seg.items.push(id);
}

#[allow(clippy::type_complexity)]
pub fn move_system(
    time: Res<CountdownRes<GameInterval>>,
    key_input: Option<Res<ControlNextKeyRes>>,
    seg: Res<BodySegmentRes>,
    mut tail_pos: ResMut<TailPosRes>,
    mut head_comp: Single<(&mut Transform, &mut MoveComp), (With<HeadComp>, Without<BodyComp>)>,
    mut body_comp: Query<(&mut Transform, &mut MoveComp), (With<BodyComp>, Without<HeadComp>)>,
) {
    if !time.is_complete() {
        return;
    }

    // 记录最后一个位置的数据
    let mut last_pos = None;
    let mut last_move = None;
    // 记录前一个的方向
    let mut pre_dirc = head_comp.1.dire;
    for (index, entity) in seg.items.iter().enumerate() {
        let mut res = match body_comp.get_mut(*entity) {
            Ok(r) => r,
            Err(_) => continue,
        };
        let tmp = pre_dirc;
        pre_dirc = res.1.dire;
        res.1.dire.clone_from(&tmp);
        if index == seg.items.len() - 1 {
            last_pos = Some(res.0.translation.truncate());
            last_move = Some(res.1.clone())
        }
        let dire = (res.1.get_direction() * BLOCK_SIZE).extend(0.0);
        res.0.translation += dire;
    }

    if last_pos.is_none() {
        last_pos = Some(head_comp.0.translation.truncate());
        last_move = Some(head_comp.1.clone());
    }
    tail_pos.set(last_pos.unwrap(), last_move.unwrap());

    if let Some(r) = key_input {
        head_comp.1.upd_dirc(r.clone());
    }
    let dire = (head_comp.1.get_direction() * BLOCK_SIZE).extend(0.0);
    head_comp.0.translation += dire;
}

fn control_system(mut cmd: Commands, keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        cmd.insert_resource(ControlNextKeyRes::Up);
        return;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        cmd.insert_resource(ControlNextKeyRes::Down);
        return;
    }
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        cmd.insert_resource(ControlNextKeyRes::Left);
        return;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        cmd.insert_resource(ControlNextKeyRes::Right);
    }
}

#[allow(clippy::type_complexity)]
fn eat_food(
    mut cmd: Commands,
    tail_pos: Res<TailPosRes>,
    mut seg: ResMut<BodySegmentRes>,
    mut counter_event: EventWriter<CountEvent>,
    head_comp: Single<&mut Transform, With<HeadComp>>,
    food_comp: Option<Single<(&mut Transform, Entity), (With<FoodComp>, Without<HeadComp>)>>,
) {
    if food_comp.is_none() {
        return;
    }
    let food_pos = food_comp.unwrap();
    if head_comp.translation != food_pos.0.translation {
        return;
    }
    let entity = cmd
        .spawn(SnakeComp::new_body(
            tail_pos.pos,
            tail_pos.move_comp.clone(),
        ))
        .id();
    seg.items.push(entity);
    cmd.entity(food_pos.1).despawn();
    counter_event.send(CountEvent::new(1));
}
