use abm_engine::commands::{Command, CommandError, CommandOutcome};
use abm_engine::scheduler::ScheduleError;
use abm_engine::simulation::Simulation;

#[test]
fn complete_entity_lifecycle() {
    let mut sim = Simulation::new(100);

    sim.schedule(0, Command::Spawn).unwrap();
    sim.schedule(0, Command::Spawn).unwrap();

    let report = sim.step().unwrap();

    assert_eq!(report.records.len(), 2);
    assert_eq!(sim.world().entity_count(), 2);

    let id = sim.world().entities().next().unwrap();

    sim.schedule(2, Command::Despawn(id)).unwrap();

    sim.step().unwrap();

    assert!(sim.world().contains(id));

    let report = sim.step().unwrap();

    assert_eq!(
        report.records[0].result.unwrap().outcome(),
        CommandOutcome::Despawned(id)
    );

    assert!(!sim.world().contains(id));
    assert_eq!(sim.world().entity_count(), 1);

    sim.schedule(3, Command::Spawn).unwrap();
    sim.step().unwrap();

    let ids: Vec<_> = sim.world().entities().map(|id| id.value()).collect();

    assert_eq!(ids, vec![1, 2]);
}

#[test]
fn queue_capacity_preserves_pending_commands() {
    let mut sim = Simulation::new(2);

    sim.schedule(0, Command::Spawn).unwrap();
    sim.schedule(10, Command::Spawn).unwrap();

    assert_eq!(
        sim.schedule(1, Command::Spawn),
        Err(ScheduleError::CapacityReached)
    );

    assert_eq!(sim.pending_count(), 2);

    sim.step().unwrap();

    assert_eq!(sim.pending_count(), 1);

    sim.schedule(1, Command::Spawn).unwrap();
    sim.step().unwrap();

    assert_eq!(sim.world().entity_count(), 2);
    assert_eq!(sim.pending_count(), 1);

    while sim.tick() <= 10 {
        sim.step().unwrap();
    }

    assert_eq!(sim.world().entity_count(), 3);
    assert_eq!(sim.pending_count(), 0);
}

#[test]
fn repeated_execution_matches_expected_state() {
    let mut sim = Simulation::new(100);

    for tick in 0..100u64 {
        sim.schedule(tick, Command::Spawn).unwrap();

        let report = sim.step().unwrap();

        assert_eq!(report.tick, tick);
        assert_eq!(sim.tick(), tick + 1);
        assert_eq!(report.records.len(), 1);

        let ids: Vec<_> = sim.world().entities().map(|id| id.value()).collect();

        let expected: Vec<_> = (0..=tick).collect();

        assert_eq!(ids, expected);

        let event = report.records[0].result.unwrap();

        assert_eq!(event.tick(), tick);
    }
}

#[test]
fn rejected_commands_do_not_corrupt_future_state() {
    let mut sim = Simulation::new(100);

    sim.schedule(0, Command::Spawn).unwrap();
    sim.step().unwrap();

    let id = sim.world().entities().next().unwrap();

    sim.schedule(1, Command::Despawn(id)).unwrap();
    sim.schedule(1, Command::Despawn(id)).unwrap();
    sim.schedule(2, Command::Spawn).unwrap();

    let report = sim.step().unwrap();

    assert_eq!(
        report.records[1].result,
        Err(CommandError::EntityNotFound(id))
    );

    assert_eq!(sim.world().entity_count(), 0);
    assert_eq!(sim.pending_count(), 1);

    sim.step().unwrap();

    let new_id = sim.world().entities().next().unwrap();

    assert_eq!(new_id.value(), 1);
    assert_eq!(sim.world().entity_count(), 1);
    assert_eq!(sim.tick(), 3);
}
