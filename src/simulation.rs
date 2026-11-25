use crate::clock::SimulationClock;
use crate::commands::{Command, CommandError, execute};
use crate::events::Event;
use crate::scheduler::{ScheduleError, Scheduler};
use crate::world::World;

/// Errors that prevent an entire tick from executing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationError {
    ClockExhausted,
}

/// Ordered record of an attempted command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandRecord {
    pub scheduled_tick: u64,
    pub command: Command,
    pub result: Result<Event, CommandError>,
}

/// Results produced during one simulation tick.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TickReport {
    pub tick: u64,
    pub records: Vec<CommandRecord>,
}

/// Authoritative simulation runtime.
#[derive(Debug)]
pub struct Simulation {
    world: World,
    clock: SimulationClock,
    scheduler: Scheduler,
}

impl Simulation {
    pub fn new(queue_capacity: usize) -> Self {
        Self {
            world: World::new(),
            clock: SimulationClock::new(),
            scheduler: Scheduler::new(queue_capacity),
        }
    }

    pub fn tick(&self) -> u64 {
        self.clock.tick()
    }

    /// Read-only access to authoritative world state.
    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn pending_count(&self) -> usize {
        self.scheduler.pending_count()
    }

    /// Submit a command for future execution.
    pub fn schedule(&mut self, execution_tick: u64, command: Command) -> Result<(), ScheduleError> {
        self.scheduler
            .schedule(self.clock.tick(), execution_tick, command)
    }

    /// Execute all due commands and advance one tick.
    pub fn step(&mut self) -> Result<TickReport, SimulationError> {
        let tick = self.clock.tick();

        // Check clock overflow before changing world state.
        let mut next_clock = self.clock;
        next_clock
            .advance()
            .ok_or(SimulationError::ClockExhausted)?;

        let mut records = Vec::new();

        while let Some((scheduled_tick, command)) = self.scheduler.pop_due(tick) {
            let result =
                execute(&mut self.world, command).map(|outcome| Event::from_outcome(tick, outcome));

            records.push(CommandRecord {
                scheduled_tick,
                command,
                result,
            });
        }

        self.clock = next_clock;

        Ok(TickReport { tick, records })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::CommandOutcome;

    #[test]
    fn empty_simulation_advances() {
        let mut sim = Simulation::new(100);

        let report = sim.step().unwrap();

        assert_eq!(report.tick, 0);
        assert_eq!(sim.tick(), 1);
        assert!(report.records.is_empty());
    }

    #[test]
    fn scheduled_commands_execute_at_correct_tick() {
        let mut sim = Simulation::new(100);

        sim.schedule(2, Command::Spawn).unwrap();

        assert!(sim.step().unwrap().records.is_empty());
        assert!(sim.step().unwrap().records.is_empty());

        let report = sim.step().unwrap();

        assert_eq!(report.tick, 2);
        assert_eq!(report.records.len(), 1);
        assert_eq!(sim.world().entity_count(), 1);

        let event = report.records[0].result.unwrap();
        assert_eq!(event.tick(), 2);
    }

    #[test]
    fn conflicting_commands_are_resolved_in_order() {
        let mut sim = Simulation::new(100);

        sim.schedule(0, Command::Spawn).unwrap();
        sim.step().unwrap();

        let id = sim.world().entities().next().unwrap();

        sim.schedule(1, Command::Despawn(id)).unwrap();
        sim.schedule(1, Command::Despawn(id)).unwrap();
        sim.schedule(1, Command::Spawn).unwrap();

        let report = sim.step().unwrap();

        assert_eq!(report.records.len(), 3);

        assert_eq!(
            report.records[0].result.unwrap().outcome(),
            CommandOutcome::Despawned(id)
        );

        assert_eq!(
            report.records[1].result,
            Err(CommandError::EntityNotFound(id))
        );

        assert!(report.records[2].result.is_ok());
        assert_eq!(sim.world().entity_count(), 1);
        assert!(!sim.world().contains(id));
        assert_eq!(sim.pending_count(), 0);
    }

    #[test]
    fn scheduling_constraints_are_preserved() {
        let mut sim = Simulation::new(1);

        sim.step().unwrap();

        assert_eq!(
            sim.schedule(0, Command::Spawn),
            Err(ScheduleError::PastTick)
        );

        sim.schedule(2, Command::Spawn).unwrap();

        assert_eq!(
            sim.schedule(3, Command::Spawn),
            Err(ScheduleError::CapacityReached)
        );

        assert_eq!(sim.pending_count(), 1);
    }

    #[test]
    fn identical_simulations_produce_identical_results() {
        let mut first = Simulation::new(100);
        let mut second = Simulation::new(100);

        for tick in 0..10 {
            first.schedule(tick, Command::Spawn).unwrap();
            second.schedule(tick, Command::Spawn).unwrap();
        }

        for _ in 0..10 {
            assert_eq!(first.step().unwrap(), second.step().unwrap());
        }

        assert_eq!(
            first.world().entities().collect::<Vec<_>>(),
            second.world().entities().collect::<Vec<_>>()
        );
    }
}
