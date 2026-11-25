use std::hint::black_box;
use std::time::{Duration, Instant};

use abm_engine::commands::Command;
use abm_engine::simulation::Simulation;

const TICKS: u64 = 1000;
const REPEATS: usize = 5;

fn benchmark(population: usize, replacements: usize) -> Duration {
    let capacity = population.max(replacements * 2);
    let mut sim = Simulation::new(capacity);

    // Initialize the population outside the timed region.
    for _ in 0..population {
        sim.schedule(0, Command::Spawn).unwrap();
    }

    sim.step().unwrap();

    assert_eq!(sim.world().entity_count(), population);

    let start = Instant::now();

    for _ in 0..TICKS {
        let tick = sim.tick();

        if replacements > 0 {
            let ids: Vec<_> = sim.world().entities().take(replacements).collect();

            for id in ids {
                sim.schedule(tick, Command::Despawn(id)).unwrap();

                sim.schedule(tick, Command::Spawn).unwrap();
            }
        }

        let report = black_box(sim.step().unwrap());

        assert_eq!(report.records.len(), replacements * 2);

        assert!(report.records.iter().all(|record| record.result.is_ok()));
    }

    let elapsed = start.elapsed();

    assert_eq!(sim.world().entity_count(), population);

    assert_eq!(sim.pending_count(), 0);
    assert_eq!(sim.tick(), TICKS + 1);

    elapsed
}

fn main() {
    println!("ABM ENGINE - KERNEL BENCHMARK");
    println!("Ticks per run: {TICKS}");
    println!("Repetitions: {REPEATS}");
    println!();

    println!(
        "{:>10} {:>10} {:>12} {:>12} {:>12} {:>12}",
        "Entities", "Cmd/tick", "Min ms", "Median ms", "Max ms", "us/tick"
    );

    for population in [100, 1000, 10_000] {
        for replacements in [0, 1, 100] {
            // Warm-up run.
            black_box(benchmark(population, replacements));

            let mut samples = Vec::new();

            for _ in 0..REPEATS {
                samples.push(benchmark(population, replacements));
            }

            samples.sort_unstable();

            let minimum = samples[0].as_secs_f64();
            let median = samples[REPEATS / 2].as_secs_f64();
            let maximum = samples[REPEATS - 1].as_secs_f64();

            println!(
                "{:>10} {:>10} {:>12.4} {:>12.4} {:>12.4} {:>12.4}",
                population,
                replacements * 2,
                minimum * 1000.0,
                median * 1000.0,
                maximum * 1000.0,
                median * 1_000_000.0 / TICKS as f64
            );
        }
    }
}
