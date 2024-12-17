use crate::{
    common::{rand_pos, BLOCK_SIZE, WINDOW_BORDER_END, WINDOW_BORDER_START},
    component::{body::SnakeComp, food::FoodComp},
    resources::{CountdownRes, CountdownState, FoodCreateInterval},
};
use bevy::{app::Plugin, prelude::*};

pub struct FoodPlugin;

// whether you can create fooed
#[derive(Debug, Clone, PartialEq, Eq, Hash, States)]
enum FoodState {
    Waiting,
    Creating,
}

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_state(FoodState::Waiting)
            .add_systems(Update, food_count_down)
            .add_systems(OnEnter(FoodState::Creating), food_create);
    }
}

fn food_count_down(
    mut state: ResMut<NextState<FoodState>>,
    mut food_internal: ResMut<CountdownRes<FoodCreateInterval>>,
    food_comp: Option<Single<&FoodComp>>,
) {
    match food_comp {
        Some(_) => {
            if food_internal.state == CountdownState::Pause {
                return;
            }
            food_internal.state = CountdownState::Pause;
            food_internal.time = 0.;
            state.set(FoodState::Waiting);
        }
        None => {
            if food_internal.state == CountdownState::Pause {
                food_internal.state = CountdownState::Process;
            }
            if !food_internal.is_complete() {
                return;
            }
            state.set(FoodState::Creating);
        }
    }
}

fn food_create(
    mut cmd: Commands,
    mut state: ResMut<NextState<FoodState>>,
    body_comp: Query<&Transform, With<SnakeComp>>,
) {
    let pos_list: Vec<Vec2> = body_comp
        .iter()
        .map(|item| item.translation.truncate())
        .collect();
    let pos = rand_pos(
        [WINDOW_BORDER_START.x as i32, WINDOW_BORDER_END.x as i32],
        [WINDOW_BORDER_START.y as i32, WINDOW_BORDER_END.y as i32],
        pos_list,
    );
    cmd.spawn(FoodComp::new(pos * BLOCK_SIZE));
    state.set(FoodState::Waiting);
}
