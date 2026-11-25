use crate::commands::CommandOutcome;

/// A successful state transformation
/// recorded at a specific simulation tick.
///
/// Construction and access: O(1).
/// Storage per event: O(1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Event {
    tick: u64,
    outcome: CommandOutcome,
}

impl Event {
    pub fn from_outcome(tick: u64, outcome: CommandOutcome) -> Self {
        Self { tick, outcome }
    }

    pub fn tick(&self) -> u64 {
        self.tick
    }

    pub fn outcome(&self) -> CommandOutcome {
        self.outcome
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::{Command, execute};
    use crate::world::World;

    #[test]
    fn spawn_event_records_entity() {
        let mut world = World::new();

        let outcome = execute(&mut world, Command::Spawn).unwrap();

        let event = Event::from_outcome(0, outcome);

        assert_eq!(event.outcome(), outcome);

        match event.outcome() {
            CommandOutcome::Spawned(id) => {
                assert!(world.contains(id));
            }
            _ => panic!("Expected spawn event"),
        }
    }

    #[test]
    fn despawn_event_records_entity() {
        let mut world = World::new();
        let id = world.spawn().unwrap();

        let outcome = execute(&mut world, Command::Despawn(id)).unwrap();

        let event = Event::from_outcome(1, outcome);

        assert_eq!(event.outcome(), CommandOutcome::Despawned(id));

        assert!(!world.contains(id));
    }

    #[test]
    fn event_preserves_timestamp() {
        let mut world = World::new();

        let outcome = execute(&mut world, Command::Spawn).unwrap();

        let event = Event::from_outcome(42, outcome);

        assert_eq!(event.tick(), 42);
    }

    #[test]
    fn rejected_command_produces_no_event() {
        let mut world = World::new();

        let id = world.spawn().unwrap();
        world.despawn(id);

        let event = execute(&mut world, Command::Despawn(id))
            .map(|outcome| Event::from_outcome(0, outcome));

        assert!(event.is_err());
        assert_eq!(world.entity_count(), 0);
    }
}
