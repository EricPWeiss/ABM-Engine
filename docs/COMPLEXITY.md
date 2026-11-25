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