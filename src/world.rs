use std::collections::BTreeSet;

use crate::entity::{EntityAllocator, EntityId};

/// Authoritative registry of active entities.
/// Removed identifiers are never reused.
#[derive(Debug, Default)]
pub struct World {
    allocator: EntityAllocator,
    entities: BTreeSet<EntityId>,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    /// O(log N)
    pub fn spawn(&mut self) -> Option<EntityId> {
        let id = self.allocator.allocate()?;
        self.entities.insert(id);
        Some(id)
    }

    /// O(log N)
    pub fn despawn(&mut self, id: EntityId) -> bool {
        self.entities.remove(&id)
    }

    /// O(log N)
    pub fn contains(&self, id: EntityId) -> bool {
        self.entities.contains(&id)
    }

    /// O(1)
    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }

    /// O(N) full traversal in deterministic order.
    pub fn entities(&self) -> impl Iterator<Item = EntityId> + '_ {
        self.entities.iter().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn world_starts_empty() {
        let world = World::new();
        assert_eq!(world.entity_count(), 0);
    }

    #[test]
    fn spawned_entities_exist() {
        let mut world = World::new();

        let first = world.spawn().unwrap();
        let second = world.spawn().unwrap();

        assert_ne!(first, second);
        assert!(world.contains(first));
        assert!(world.contains(second));
        assert_eq!(world.entity_count(), 2);
    }

    #[test]
    fn despawn_removes_entity() {
        let mut world = World::new();
        let id = world.spawn().unwrap();

        assert!(world.despawn(id));
        assert!(!world.contains(id));
        assert!(!world.despawn(id));
        assert_eq!(world.entity_count(), 0);
    }

    #[test]
    fn removed_identifiers_are_not_reused() {
        let mut world = World::new();

        let first = world.spawn().unwrap();
        world.despawn(first);

        let second = world.spawn().unwrap();

        assert_ne!(first, second);
        assert!(second > first);
    }

    #[test]
    fn iteration_is_deterministic() {
        let mut world = World::new();

        let first = world.spawn().unwrap();
        let second = world.spawn().unwrap();
        let third = world.spawn().unwrap();

        world.despawn(second);

        let entities: Vec<_> = world.entities().collect();

        assert_eq!(entities, vec![first, third]);
    }
}
