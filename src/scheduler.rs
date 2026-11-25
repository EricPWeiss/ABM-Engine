use std::collections::{BTreeMap, VecDeque};

use crate::commands::Command;

/// Reasons why scheduling can fail.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScheduleError {
    PastTick,
    CapacityReached,
}

/// Deterministic, bounded command scheduler.
///
/// Commands execute in chronological order.
/// Commands at the same tick execute in FIFO order.
#[derive(Debug)]
pub struct Scheduler {
    queue: BTreeMap<u64, VecDeque<Command>>,
    pending: usize,
    capacity: usize,
}

impl Scheduler {
    pub fn new(capacity: usize) -> Self {
        Self {
            queue: BTreeMap::new(),
            pending: 0,
            capacity,
        }
    }

    pub fn pending_count(&self) -> usize {
        self.pending
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn is_empty(&self) -> bool {
        self.pending == 0
    }

    /// Schedules a command for a current or future tick.
    pub fn schedule(
        &mut self,
        current_tick: u64,
        execution_tick: u64,
        command: Command,
    ) -> Result<(), ScheduleError> {
        if execution_tick < current_tick {
            return Err(ScheduleError::PastTick);
        }

        if self.pending >= self.capacity {
            return Err(ScheduleError::CapacityReached);
        }

        self.queue
            .entry(execution_tick)
            .or_default()
            .push_back(command);

        self.pending += 1;

        Ok(())
    }

    /// Retrieves the earliest due command.
    ///
    /// Returns its scheduled tick and command.
    pub fn pop_due(&mut self, current_tick: u64) -> Option<(u64, Command)> {
        let mut entry = self.queue.first_entry()?;

        let tick = *entry.key();

        if tick > current_tick {
            return None;
        }

        let command = entry.get_mut().pop_front()?;

        if entry.get().is_empty() {
            entry.remove();
        }

        self.pending -= 1;

        Some((tick, command))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::EntityAllocator;

    #[test]
    fn scheduler_starts_empty() {
        let mut scheduler = Scheduler::new(100);

        assert!(scheduler.is_empty());
        assert_eq!(scheduler.pending_count(), 0);
        assert_eq!(scheduler.pop_due(0), None);
    }

    #[test]
    fn future_commands_do_not_execute_early() {
        let mut scheduler = Scheduler::new(100);

        scheduler.schedule(0, 10, Command::Spawn).unwrap();

        assert_eq!(scheduler.pop_due(9), None);
        assert_eq!(scheduler.pending_count(), 1);

        assert_eq!(scheduler.pop_due(10), Some((10, Command::Spawn)));
    }

    #[test]
    fn commands_execute_chronologically() {
        let mut scheduler = Scheduler::new(100);

        scheduler.schedule(0, 20, Command::Spawn).unwrap();
        scheduler.schedule(0, 5, Command::Spawn).unwrap();
        scheduler.schedule(0, 10, Command::Spawn).unwrap();

        assert_eq!(scheduler.pop_due(20), Some((5, Command::Spawn)));

        assert_eq!(scheduler.pop_due(20), Some((10, Command::Spawn)));

        assert_eq!(scheduler.pop_due(20), Some((20, Command::Spawn)));

        assert!(scheduler.is_empty());
    }

    #[test]
    fn same_tick_commands_preserve_fifo_order() {
        let mut scheduler = Scheduler::new(100);
        let mut allocator = EntityAllocator::new();

        let id = allocator.allocate().unwrap();

        scheduler.schedule(0, 5, Command::Spawn).unwrap();

        scheduler.schedule(0, 5, Command::Despawn(id)).unwrap();

        assert_eq!(scheduler.pop_due(5), Some((5, Command::Spawn)));

        assert_eq!(scheduler.pop_due(5), Some((5, Command::Despawn(id))));
    }

    #[test]
    fn scheduling_in_the_past_is_rejected() {
        let mut scheduler = Scheduler::new(100);

        assert_eq!(
            scheduler.schedule(10, 5, Command::Spawn),
            Err(ScheduleError::PastTick)
        );

        assert!(scheduler.is_empty());
    }

    #[test]
    fn capacity_is_enforced() {
        let mut scheduler = Scheduler::new(1);

        scheduler.schedule(0, 1, Command::Spawn).unwrap();

        assert_eq!(
            scheduler.schedule(0, 2, Command::Spawn),
            Err(ScheduleError::CapacityReached)
        );

        assert_eq!(scheduler.pending_count(), 1);
    }

    #[test]
    fn executing_commands_releases_capacity() {
        let mut scheduler = Scheduler::new(1);

        scheduler.schedule(0, 1, Command::Spawn).unwrap();
        scheduler.pop_due(1).unwrap();

        assert!(scheduler.is_empty());

        scheduler.schedule(1, 2, Command::Spawn).unwrap();

        assert_eq!(scheduler.pending_count(), 1);
    }
}
