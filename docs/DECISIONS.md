# Architectural decision log

These are proposed baseline decisions, subject to user approval and later evidence. Append dated entries for significant changes with alternatives, rationale, consequences and status.

| ID | Decision | Rationale / limitation |
|---|---|---|
| ADR-001 | Rust core, one Cargo package initially | Native execution and type safety; language alone does not guarantee efficiency. |
| ADR-002 | Headless first, separate renderer | Testability and future perspective changes; 3D still requires new rendering and physics work. |
| ADR-003 | Deterministic sequential execution first | Reference semantics for later parallelization. |
| ADR-004 | Commands and committed events | Explicit ownership and validation, avoid hidden coupled mutations. |
| ADR-005 | Sparse social/credit networks | Storage proportional to actual ties and obligations; density can still grow. |
| ADR-006 | Informal credit and reciprocity before barter | Allows aid without immediate exchange; no claim of a universal historical sequence. |
| ADR-007 | Typed protocols before full open games | Prove runtime viability before implementing strategic semantics and solvers. |
| ADR-008 | Evaluate Bevy ECS rather than requiring it on day one | Avoid unnecessary dependencies in the first minimal runtime; revisit when real storage needs arise. |
| ADR-009 | Complexity contracts and benchmarking | Compare algorithms, not just languages; performance targets remain unvalidated. |
| ADR-010 | Pixel graphics before 3D | Scope control; avoid tying world state to sprites or UI. |
| ADR-011 | BTreeSet entity registry | Deterministic traversal and simple implementation; O(log N) lookup. Revisit after benchmarking. |
| ADR-012 | Bounded BTreeMap scheduler with FIFO queues | Deterministic chronological execution, explicit capacity and efficient scheduling without scanning all pending commands. |