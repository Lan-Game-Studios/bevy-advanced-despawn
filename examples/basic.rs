use bevy_advanced_despawn::prelude::*;
use bevy_app::{App, Startup};
use bevy_ecs::{prelude::Component, system::Commands};
use bevy_time::{Timer, TimerMode};

#[derive(Component)]
struct A;

fn main() {
    App::new()
        .add_plugins(DespawnPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    let timer = Timer::from_seconds(1.0, TimerMode::Once);
    commands.spawn((A, despawn_after_timer::DespawnAfterTimer(timer)));

    commands.spawn((A, despawn_recursive::DespawnRecursive));
}
