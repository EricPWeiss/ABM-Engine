# Roadmap

Only milestone 0.0.1 is currently authorized for implementation once the user requests it. All later milestones are proposed scope.

| Milestone | Deliverable | Acceptance gate |
|---|---|---|
| 0.0.1 | Primitive runtime: stable IDs, clock, deterministic command execution, events, headless demo | Tests for ID uniqueness, tick order, rejection without domain mutation, deterministic repeated execution; CLI works |
| 0.0.2 | Resources and atomic transfers | Conservation, nonnegativity, overflow and conflicting transfer tests |
| 0.0.3 | Observation and local decision policies; seeded RNG | Same seed/input repeats; policies cannot bypass validation |
| 0.0.4 | Sparse trust network | Directed updates, valid references, complexity tests |
| 0.0.5 | Typed interaction protocols | Compatible stages, persisted protocol state, rejected invalid transitions |
| 0.0.6 | Informal IOU economy | Voluntary aid, agreed obligations, repayment, renegotiation/default; 20-agent scenario |
| 0.0.7 | Persistence, replay and stress benchmarks | Round-trip state, deterministic continuation, documented scaling |
| 0.0.8 | Spatial grid, local queries, movement, procedural town | Navigability, occupancy validation, bounded-locality tests |
| 0.1.0 | Zombie scenario | Autonomous survival, infection and threats; headless simulation |
| 0.2.0 | Pixel client | Observe/possess/release survivor, same command path, pause/step/speed controls |
| Later | Emergent settlements, institutions and factions; complex survival; 3D | Individually specified and tested, no rewrite of core without justified decision |

Tests and complexity reasoning begin at 0.0.1. Full formal open-game semantics, equilibrium solvers, multiplayer, and advanced 3D are explicitly deferred. No barter-first economy: reciprocal aid and informal credit are the initial economic mechanisms.

## 0.0.1 scope
Keep the initial implementation dependency-free unless a concrete requirement justifies a crate. Create a library and a tiny binary demonstration. A neutral test component/command is sufficient to exercise validation and events. No zombies, economy, graphics, networking, async runtime, generalized plugin ABI, or ECS rewrite. Do not claim full reproducibility across versions or platforms until tested. Document the initial scheduling and error semantics.
