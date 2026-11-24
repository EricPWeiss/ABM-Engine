# Agent instructions

## Mission
Build a reusable, headless, deterministic Rust agent-based simulation (ABM) runtime. The first scenario is an informal credit economy; the eventual product is a zombie-apocalypse settlement game with autonomous survivors, emergent institutions, character control and god mode. Simulation precedes rendering.

## Required workflow
1. Read `docs/ARCHITECTURE.md`, `docs/MATHEMATICS.md`, `docs/COMPLEXITY.md`, `docs/ROADMAP.md`, and `docs/DECISIONS.md` before changing core behavior.
2. Select one roadmap milestone. State scope, affected files, contracts, invariants, algorithms, and complexity assumptions. Do not implement later milestones without approval.
3. Implement the smallest working change. Use safe Rust by default; justify new dependencies. Avoid speculative abstractions, unrelated refactors and duplicated state.
4. Add regression tests and relevant invariant tests in the same change. Test failure paths, not just success.
5. Run `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo test --all-targets`. Report actual results; never claim an unrun check passed. Run relevant benchmarks when available.
6. Update specifications and the decision log when contracts or assumptions change. Report changed files, limitations, and next recommended step. Leave committing to the user unless asked to commit.

## Hard constraints
- No renderer dependencies in the core library. Scenario rules depend on core, not vice versa.
- Commands request changes; authoritative handlers validate and commit; events describe committed facts. Observers cannot silently rewrite authoritative state.
- Explicit simulation time, stable identifiers, deterministic ordering and controlled random sources. Do not use wall-clock time, unordered iteration or global randomness to decide outcomes.
- No unrestricted all-pairs scans or dense relationship matrices in routine updates without explicit justification and measurements.
- Separate content/configuration from algorithms. Validate external content at boundaries. No unbounded event recursion or queues without limits or monitoring.
- Prefer explicit bounded policies over global equilibrium computation in real-time simulation. Do not call a protocol a full open game unless its strategic semantics are specified.
- Never weaken tests, remove safeguards or introduce unsafe code just to pass a build.
- Player actions must eventually use the same validated command interface as autonomous agents.
