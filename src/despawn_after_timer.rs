use bevy_ecs::{
    entity::Entity,
    prelude::Component,
    system::{Commands, Query, Res},
};
use bevy_time::prelude::{Time, Timer};

/// despawn when the timer runs out
/// ```
/// use bevy_ecs::{prelude::Component, prelude::Commands};
/// use bevy_time::{Timer, TimerMode};
/// use bevy_advanced_despawn::despawn_after_timer::DespawnAfterTimer;
///
/// #[derive(Component)]
/// struct A;
///
/// fn setup(mut commands: Commands) {
///   let timer = Timer::from_seconds(1.0, TimerMode::Once);
///   commands.spawn((A, DespawnAfterTimer(timer)));
/// }
/// ```
#[derive(Component)]
pub struct DespawnAfterTimer(pub Timer);

impl From<Timer> for DespawnAfterTimer {
    fn from(value: Timer) -> Self {
        Self(value)
    }
}

pub fn despawn_after_timer(
    time: Res<Time>,
    mut commands: Commands,
    mut entity_query: Query<(Entity, &mut DespawnAfterTimer)>,
) {
    for (entity, mut timer) in entity_query.iter_mut() {
        timer.0.tick(time.delta());
        println!("{:?}", timer.0.elapsed_secs());
        if timer.0.finished() {
            commands.entity(entity).despawn();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_app::{App, Update};
    use bevy_ecs::prelude::Component;
    use bevy_time::{TimePlugin, Timer, TimerMode};
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_despawn_after_timer() {
        #[derive(Component)]
        struct A;

        let mut app = App::new();

        app.add_plugins(TimePlugin)
            .add_systems(Update, despawn_after_timer);

        let entity = app
            .world_mut()
            .spawn((
                A,
                DespawnAfterTimer(Timer::from_seconds(0.1, TimerMode::Once)),
            ))
            .id();
        app.update();

        sleep(Duration::from_secs_f32(0.1));
        app.update();

        assert!(app.world().get_entity(entity).is_none());
    }
}
