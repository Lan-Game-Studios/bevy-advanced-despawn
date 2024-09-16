use std::marker::PhantomData;
use bevy_ecs::{entity::Entity, event::Event, prelude::Component, system::{Commands, Query, Res}};
use bevy_time::prelude::Timer;
use bevy::time::prelude::Time;

#[derive(Component)]
pub struct DespawnScheduleFirst;

#[derive(Component)]
pub struct DespawnSchedulePreUpdate;

#[derive(Component)]
pub struct DespawnScheduleStateTransition;

#[derive(Component)]
pub struct DespawnScheduleRunFixedMainLoop;

/// despawn the entity in a certain schedule
/// TODO make the system to process this
/// TODO make test
/// TODO make example
#[derive(Component)]
pub struct DespawnScheduleUpdate;

#[derive(Component)]
pub struct DespawnSchedulePostUpdate;

#[derive(Component)]
pub struct DespawnScheduleLast;

/// despawn when the timer runs out
/// ```
/// use bevy_ecs::{prelude::Component, prelude::Commands};
/// use bevy_time::{Timer, TimerMode};
/// use bevy_advanced_despawn::DespawnAfterTimer;
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

fn despawn_after_timer(
    time: Res<Time>,
    mut commands: Commands,
    mut entity_query: Query<(Entity, &mut DespawnAfterTimer)>,
) {
    for (entity, mut timer) in entity_query.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.finished() {
            commands.entity(entity).despawn();
        }
    }
}

/// despawn after x frames
/// TODO make the system to process this
/// TODO make test
/// TODO make example
#[derive(Component)]
pub struct DespawnAfterFrames(pub usize);

/// despawn the entity when another entity gets removed
/// TODO make the system to process this
/// TODO make test
/// TODO make example
pub struct DespawnWith(pub Entity);

/// despawn the entity when an event E is written
/// TODO make the system to process this
/// TODO make test
/// TODO make example
pub struct DespawnWithEvent<E: Event>(pub Entity, PhantomData<E>);

/// TODO check if this is usefull in example
impl<E: Event> From<Entity> for DespawnWithEvent<E> {
    fn from(value: Entity) -> Self {
        Self(value, PhantomData::<E>)
    }
}

/// despawn the entity when an event E is written and
/// the event impl Into<Entity> (custom pub trait better?)
/// TODO make the system to process this
/// TODO make test
/// TODO make example
pub struct DespawnByEvent<E: Event + Into<Entity>>(PhantomData<E>);

/// TODO check in example if this usefull
impl<E: Event + Into<Entity>> From<E> for DespawnByEvent<E> {
    fn from(_value: E) -> Self {
        Self(PhantomData::<E>)
    }
}

/// TODO make examples combine some of them

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_time::{Timer, TimerMode};
    use std::thread::sleep;
    use bevy::prelude::*;
    use std::time::Duration;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_despawn_after_timer() {
        #[derive(Component)]
        struct A;

        let mut app = App::new();
        app
            .add_plugins(MinimalPlugins)
            .add_systems(Update, despawn_after_timer);

        let entity = app.world_mut().spawn((
            A,
            DespawnAfterTimer(Timer::from_seconds(0.1, TimerMode::Once)),
        )).id();
        app.update();

        assert!(app.world().get_entity(entity).is_some());

        sleep(Duration::from_secs_f32(0.1));

        app.update();

        assert!(app.world().get_entity(entity).is_none());
    }
}
