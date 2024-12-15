use bevy::{
    app::{App, Startup, Update},
    prelude::*,
};

use crate::{
    common::TIME_INTERVAL,
    resources::{CountdownRes, CountdownState, FoodCreateInterval, GameInterval},
};

pub struct CountdownPlugin;

impl Plugin for CountdownPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, countdown_init)
            .add_systems(Update, countdown_system);
    }
}

fn countdown_init(mut command: Commands) {
    command.insert_resource(CountdownRes::<GameInterval>::new(TIME_INTERVAL));
    command.insert_resource(CountdownRes::<FoodCreateInterval>::new(TIME_INTERVAL));
}

fn countdown_system(
    time: Res<Time>,
    mut game_internal: ResMut<CountdownRes<GameInterval>>,
    mut food_internal: ResMut<CountdownRes<FoodCreateInterval>>,
) {
    if game_internal.state == CountdownState::Process {
        game_internal.add_delta(time.delta_secs());
    }
    if food_internal.state == CountdownState::Process {
        food_internal.add_delta(time.delta_secs());
    }
}
