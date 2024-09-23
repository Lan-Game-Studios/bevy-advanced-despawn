use bevy_advanced_despawn::{
    despawn_after_timer,
    despawn_with_event::{despawn_with_event, DespawnWithEvent},
    TimerPlugin,
};
use bevy_app::{App, Startup, Update};
use bevy_ecs::event::Event;
use bevy_ecs::{prelude::Component, system::Commands};
use bevy_time::{Timer, TimerMode};

#[derive(Component)]
struct A;

#[derive(Event)]
struct TestEvent;

fn main() {
    App::new()
        .add_plugins(TimerPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, despawn_with_event::<TestEvent>)
        .run();
}

fn setup(mut commands: Commands) {
    let timer = Timer::from_seconds(1.0, TimerMode::Once);
    commands.spawn((A, despawn_after_timer::DespawnAfterTimer(timer)));

    commands.spawn((A, DespawnWithEvent::<TestEvent>::default()));
}
