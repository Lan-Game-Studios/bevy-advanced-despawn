use std::marker::PhantomData;

use bevy_ecs::{entity::Entity, event::Event, prelude::Component};
use bevy_time::prelude::Timer;

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
/// TODO make the system to process this
/// TODO make test
/// TODO make example
#[derive(Component)]
pub struct DespawnAfterTimer(pub Timer);

impl From<Timer> for DespawnAfterTimer {
    fn from(value: Timer) -> Self {
        Self(value)
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
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
