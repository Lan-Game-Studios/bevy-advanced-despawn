pub mod despawn_after_timer;
pub mod despawn_recursive;
pub mod despawn_with_event;
pub mod prelude;

use bevy_app::{App, Last, Plugin, Update};
use bevy_ecs::{entity::Entity, event::Event, prelude::Component};
use std::marker::PhantomData;

pub struct DespawnPlugin;
impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn_after_timer::despawn_after_timer)
            .add_systems(Last, despawn_recursive::despawn_marked_entities);
    }
}

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

// TODO make examples combine some of them
