/// Unique identifier within a simulation world.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EntityId(u64);

impl EntityId {
    pub fn value(self) -> u64 {
        self.0
    }
}

/// Allocates monotonically increasing entity identifiers.
///
/// Time complexity: O(1) per allocation.
/// Space complexity: O(1).
#[derive(Debug, Default)]
pub struct EntityAllocator {
    next: u64,
}

impl EntityAllocator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn allocate(&mut self) -> Option<EntityId> {
        let id = EntityId(self.next);
        self.next = self.next.checked_add(1)?;
        Some(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifiers_are_unique() {
        let mut allocator = EntityAllocator::new();

        let first = allocator.allocate().unwrap();
        let second = allocator.allocate().unwrap();

        assert_ne!(first, second);
    }

    #[test]
    fn identifiers_are_sequential() {
        let mut allocator = EntityAllocator::new();

        for expected in 0..1000 {
            let id = allocator.allocate().unwrap();
            assert_eq!(id.value(), expected);
        }
    }

    #[test]
    fn identifiers_cannot_overflow() {
        let mut allocator = EntityAllocator { next: u64::MAX };

        assert_eq!(allocator.allocate(), None);
    }
}
