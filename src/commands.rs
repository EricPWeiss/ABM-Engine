use crate::entity::EntityId;
use crate::world::World;

/// A requested state transformation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Spawn,
    Despawn(EntityId),
}

/// The result of a successful command.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandOutcome {
    Spawned(EntityId),
    Despawned(EntityId),
}

/// Reasons why a command may be rejected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandError {
    EntityNotFound(EntityId),
    IdentifierExhausted,
}

/// Validates and executes a command.
///
/// Rejected commands leave the world unchanged.
/// Time complexity: O(log N).
/// Auxiliary space complexity: O(1).
pub fn execute(world: &mut World, command: Command) -> Result<CommandOutcome, CommandError> {
    match command {
        Command::Spawn => world
            .spawn()
            .map(CommandOutcome::Spawned)
            .ok_or(CommandError::IdentifierExhausted),

        Command::Despawn(id) => {
            if world.despawn(id) {
                Ok(CommandOutcome::Despawned(id))
            } else {
                Err(CommandError::EntityNotFound(id))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spawn_command_creates_entity() {
        let mut world = World::new();

        let result = execute(&mut world, Command::Spawn);

        match result {
            Ok(CommandOutcome::Spawned(id)) => {
                assert!(world.contains(id));
                assert_eq!(world.entity_count(), 1);
            }
            _ => panic!("Expected successful spawn"),
        }
    }

    #[test]
    fn despawn_command_removes_entity() {
        let mut world = World::new();
        let id = world.spawn().unwrap();

        let result = execute(&mut world, Command::Despawn(id));

        assert_eq!(result, Ok(CommandOutcome::Despawned(id)));

        assert!(!world.contains(id));
    }

    #[test]
    fn invalid_command_is_rejected() {
        let mut world = World::new();

        let id = world.spawn().unwrap();
        world.despawn(id);

        let result = execute(&mut world, Command::Despawn(id));

        assert_eq!(result, Err(CommandError::EntityNotFound(id)));
    }

    #[test]
    fn rejected_command_preserves_world() {
        let mut world = World::new();

        let removed = world.spawn().unwrap();
        world.despawn(removed);

        world.spawn().unwrap();
        world.spawn().unwrap();

        let before: Vec<_> = world.entities().collect();

        let result = execute(&mut world, Command::Despawn(removed));

        let after: Vec<_> = world.entities().collect();

        assert!(result.is_err());
        assert_eq!(before, after);

        let next = world.spawn().unwrap();
        assert_eq!(next.value(), 3);
    }

    #[test]
    fn command_execution_is_deterministic() {
        let mut first = World::new();
        let mut second = World::new();

        for _ in 0..100 {
            assert_eq!(
                execute(&mut first, Command::Spawn),
                execute(&mut second, Command::Spawn)
            );
        }

        assert_eq!(
            first.entities().collect::<Vec<_>>(),
            second.entities().collect::<Vec<_>>()
        );
    }
}
