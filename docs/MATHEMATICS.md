# Mathematical specification

Status: initial formal model; not an empirical calibration or a proof of implementation correctness.

## A. State and time
Let t ∈ ℕ, V_t be the finite set of live entity IDs, and S_t be the full simulation state (including pending commands, interaction states, clock and random-generator state as needed for replay).

## B. Observation and policy
For each eligible agent i, o_i = O_i(S_t), with O_i exposing only authorized information. A policy π_i maps observation and policy memory to a distribution over candidate commands. Draw a_i ~ π_i using a controlled random source. Player commands share the same domain validation.

## C. Transition
Let Q_t be proposed commands. Resolve(S_t,Q_t) defines a deterministic ordering and rejection policy. Apply accepted commands against current committed state, rechecking feasibility; each accepted command produces an updated state and committed events. Rejected commands do not alter domain state. Advance the clock according to documented scheduling semantics. Fixed seed + initial state + configuration + ordered external commands + compatible engine version ⇒ identical replay within the defined determinism scope.

## D. Invariants
- Entity references resolve or are explicitly historical/tombstoned.
- Clock is monotone; identifiers do not silently wrap or alias.
- State changes preserve declared module invariants.
- Commands cannot transfer more resources than a source owns; integer operations are checked.
- For a pure transfer of q ≥ 0 from i to j, r_i' = r_i − q, r_j' = r_j + q, and r_i' + r_j' = r_i + r_j. Production, consumption and destruction are separate explicit flows.

For any invariant P: if P(S_0) and each valid transition preserves P, then induction gives P(S_t) for all t. Tests are evidence about code, not a substitute for a proof.

## E. Networks and informal credit (later milestone)
Represent directed trust as sparse G_t=(V_t,E_t,w_t), w_ij ∈ [0,1] (or a documented fixed-point representation). Obligations are separate records with stable IDs, creditor, debtor, terms, outstanding amount or service, status, and optional due time. A missing edge is not automatically distrust. Generalized reciprocity and unconditional aid do not necessarily create enforceable debt. Explicit IOUs require accepted terms. A food transfer and creation of an agreed obligation must succeed or fail together.

## F. Strategic composition
First implement a typed interaction protocol: request → evaluation → accepted transfer/obligation → later settlement or default → trust update. Each stage declares input/output types; asynchronous stages persist state. Sequential composition requires compatible interfaces; parallel composition requires explicit semantics for shared resources. This is inspired by open games, not a full implementation.

A formal open game G:(X,S)→(Y,R) additionally has strategy space Σ, play P:Σ×X→Y, coplay C:Σ×X×R→S, and context-dependent best-response relation B:X×(Y→R)→Rel(Σ). Introduce these only when we have a concrete analytical use case. Runtime agents use local policies; global equilibrium analysis is separate.

Reference: Ghani, Hedges, Winschel and Zahn, *Compositional Game Theory*, arXiv:1603.04641. The proposed stateful protocol layer is a project design, not a theorem inherited from that paper.
