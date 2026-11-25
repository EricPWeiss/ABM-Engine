# Complexity and performance policy

Status: targets and assumptions, not benchmark results.

Parameters: N live entities; M social edges; L outstanding obligations; K pending events; W materialized tiles; A candidate actions per agent; k average spatial candidates; C commands in a batch.

| Operation | Target | Assumption |
|---|---|---|
| Entity lookup | expected O(1) or worst-case O(1) indexed | chosen storage documented |
| Full entity traversal | O(N) | excludes domain work |
| Sparse network storage | O(N+M) | bounded-size edge records |
| Outstanding obligations storage | O(N+L) | indexed by participants; bounded-size records |
| Local proximity sweep | O(N+Nk) expected | efficient spatial index; bounded density for linearity |
| Heap event insertion/removal | O(log K) | if a binary heap is chosen |
| Batch command ordering | O(C log C) | comparison sort if needed |
| Policy evaluation | O(A) per agent | constant-cost candidate evaluation, inputs already available |

Worst-case spatial crowding can still yield quadratic work. Do not claim that sparse graphs, composition or Rust automatically make problems tractable. Analyze time, memory, amortized/expected vs worst-case behavior, allocations and constants. For concurrent work use total work and critical path; concurrency does not eliminate synchronization or resource limits.

## Verification plan
- Start with 100, 1,000 and 10,000 entities; expand to 100,000 only after profiling. These are benchmark tiers, not population promises.
- Record seed, workload, release build, toolchain, hardware, tick count, wall time, throughput, and memory measurement method. Separate simulation from rendering and I/O. Use repeat runs and report variance.
- Vary density and active commands independently of population. Include a crowded adversarial workload and long-running simulations for queue growth.
- Add Criterion when there is a meaningful workload. Do not claim measured complexity from a small timing sample alone.
- Keep determinism and invariants intact when optimizing. Optimize measured bottlenecks; document intentional fidelity tradeoffs.

## Initial entity registry

The primitive world uses BTreeSet for deterministic
entity ordering. Lookup, insertion and removal are
O(log N), with O(N) storage and full traversal.

This is a provisional implementation. Evaluate
indexed or ECS-based storage after benchmarking.

## Command execution

Commands are fixed-size values with O(1) construction
and result storage.

Spawn and despawn inherit the O(log N) time complexity
of the current BTreeSet entity registry.

Rejected commands preserve authoritative world state.

A sequential batch of C commands costs
O(C log(N_max + 1)), where N_max is the maximum
active population during execution.

## Simulation events

Events consist of a simulation timestamp and a
successful command outcome.

Event construction, copying and field access
have O(1) time and space complexity.

Retaining E fixed-size events requires O(E) storage.

Persistent event history, dispatch and subscriptions
are deferred. Future implementations must prevent
unbounded event accumulation and recursive dispatch.

## Deterministic scheduler

The scheduler uses a BTreeMap of simulation ticks,
with a VecDeque of commands at each tick.

Let K be pending commands and D distinct scheduled ticks.

Scheduling: amortized O(log(D + 1)).
Retrieval: O(log(D + 1)).
Pending count and capacity checks: O(1).
Logical storage: O(K + D).

Commands execute chronologically, with FIFO ordering
within the same tick.

The queue has an explicit capacity limit.
Scheduling in the past is rejected.

## Integrated simulation kernel

Simulation owns the world, clock and scheduler.

Let C be the number of commands executed in a tick,
N the maximum active population, and D the maximum
number of distinct scheduled ticks during that tick.

Amortized tick execution bound:
O(1 + log(D+1) + C*(1+log(D+1)+log(N+1))).

Tick report storage: O(C).

The runtime does not retain historical tick reports.
The caller controls their retention.

Clock exhaustion is checked before world mutation.
No population-wide scan is required by the kernel.

## Primitive kernel benchmark

Location: examples/kernel_benchmark.rs

Population sizes: 100, 1000, 10000.
Commands per tick: 0, 2, 200.
Ticks per repetition: 1000.
Repetitions: 5 after one warm-up.
Randomness: none.

Population is held constant through paired
despawn and spawn commands.

Measurements include entity selection,
command submission, scheduling, execution,
tick report construction and validation.

Initialization and console output are
excluded from the timed region.

Run using:
cargo run --release --example kernel_benchmark

Memory profiling and heterogeneous future-tick
queue workloads require separate benchmarks.