use abm_engine::commands::Command;
use abm_engine::simulation::Simulation;

fn main() {
    let mut sim = Simulation::new(1000);

    println!("ABM ENGINE");
    println!("Starting deterministic simulation");

    sim.schedule(0, Command::Spawn).unwrap();
    sim.schedule(0, Command::Spawn).unwrap();
    sim.schedule(2, Command::Spawn).unwrap();

    for _ in 0..5 {
        let report = sim.step().unwrap();

        println!("\nTick {}", report.tick);

        for record in report.records {
            println!("  {:?}", record.result);
        }

        println!("  Active entities: {}", sim.world().entity_count());
    }

    println!("\nSimulation complete.");
    println!("Final tick: {}", sim.tick());
    println!("Population: {}", sim.world().entity_count());
}
