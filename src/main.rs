use bevy::{app::App, DefaultPlugins};
use system::{
    border::BorderPlugin, countdown::CountdownPlugin, counter::CounterPlugin, food::FoodPlugin,
    snake::SnakePlugin,
};

mod common;
mod component;
mod resources;
mod system;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BorderPlugin)
        .add_plugins(CountdownPlugin)
        .add_plugins(CounterPlugin)
        .add_plugins(FoodPlugin)
        .add_plugins(SnakePlugin)
        .run();
}
