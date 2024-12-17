use bevy::prelude::*;

use crate::component::counter::CounterComp;

pub struct CounterPlugin;

impl Plugin for CounterPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CounterRes(0))
            .add_event::<CountEvent>()
            .add_systems(Startup, count_init)
            .add_systems(Update, counter_update);
    }
}

#[derive(Debug, Event)]
pub struct CountEvent(i32);

impl CountEvent {
    pub fn new(count: i32) -> Self {
        CountEvent(count)
    }
    pub fn get(&self) -> i32 {
        self.0
    }
}

#[derive(Debug, Resource)]
pub struct CounterRes(i32);
impl CounterRes {
    pub fn get(&self) -> i32 {
        self.0
    }

    pub fn add(&mut self, count: i32) {
        self.0 += count;
    }
}

fn count_init(mut cmd: Commands, count: Res<CounterRes>) {
    cmd.spawn(CounterComp::new(count.get().to_string()));
}

fn counter_update(
    mut count: ResMut<CounterRes>,
    mut event: EventReader<CountEvent>,
    mut counter_comp: Single<&mut Text, With<CounterComp>>,
) {
    for event in event.read() {
        count.add(event.get());
        counter_comp.0 = count.get().to_string();
    }
}
