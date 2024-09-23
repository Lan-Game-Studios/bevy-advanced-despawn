use bevy_ecs::{
    entity::Entity,
    prelude::Component,
    query::With,
    system::{Commands, Query},
};
use bevy_hierarchy::DespawnRecursiveExt;

/// despawn the entity recursively
/// ```
/// use bevy_ecs::{prelude::Component, prelude::Commands};
/// use bevy_time::{Timer, TimerMode};
/// use bevy_advanced_despawn::despawn_recursive::DespawnRecursive;
///
/// #[derive(Component)]
/// struct A;
///
/// fn setup(mut commands: Commands) {
///   commands.spawn((A, DespawnRecursive));
/// }
/// ```
#[derive(Component)]
pub struct DespawnRecursive;

pub fn despawn_marked_entities(
    mut commands: Commands,
    query: Query<Entity, With<DespawnRecursive>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_app::{App, Last};
    use bevy_ecs::prelude::Component;

    #[test]
    fn test_despawn_after_timer() {
        #[derive(Component)]
        struct A;

        let mut app = App::new();

        app.add_systems(Last, despawn_marked_entities);

        let entity = app.world_mut().spawn((A, DespawnRecursive)).id();

        assert!(app.world().get_entity(entity).is_some());

        app.update();

        assert!(app.world().get_entity(entity).is_none());
    }
}
