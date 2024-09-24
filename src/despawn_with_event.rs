use bevy_ecs::{
    entity::Entity,
    event::{Event, EventReader},
    prelude::Component,
    query::With,
    system::{Commands, Query},
};
use bevy_hierarchy::DespawnRecursiveExt;
use std::marker::PhantomData;

/// despawn the entity when an event E is written
/// ```
/// use bevy_ecs::{prelude::Component, prelude::Commands, event::{Event}};
/// use bevy_advanced_despawn::despawn_with_event::DespawnWithEvent;
///
/// #[derive(Component)]
/// struct A;
///
/// #[derive(Event)]
/// struct TestEvent;
///
/// fn setup(mut commands: Commands) {
///   commands.spawn((A, DespawnWithEvent::<TestEvent>::default()));
/// }
/// ```
#[derive(Component)]
pub struct DespawnWithEvent<E: Event>(PhantomData<E>);

impl<E: Event> Default for DespawnWithEvent<E> {
    fn default() -> Self {
        Self(PhantomData::<E>)
    }
}

pub fn despawn_with_event<E: Event>(
    mut commands: Commands,
    mut event_reader: EventReader<E>,
    query: Query<Entity, With<DespawnWithEvent<E>>>,
) {
    for _event in event_reader.read() {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_app::{App, Update};
    use bevy_ecs::event::Events;
    use bevy_ecs::prelude::Component;

    #[test]
    fn test_despawn_with_event() {
        #[derive(Component)]
        struct A;

        #[derive(Event)]
        struct TestEvent;

        let mut app = App::new();

        app.add_event::<TestEvent>()
            .add_systems(Update, despawn_with_event::<TestEvent>);

        let entity = app
            .world_mut()
            .spawn((A, DespawnWithEvent::<TestEvent>::default()))
            .id();
        app.update();
        assert!(app.world().get_entity(entity).is_some());

        let mut event_writer = app.world_mut().resource_mut::<Events<TestEvent>>();
        event_writer.send(TestEvent);

        app.update();

        assert!(app.world().get_entity(entity).is_none());
    }
}
