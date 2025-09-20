# ADR-0001 — Establish Minimal Rust Core Skeleton

## Context
We need an executable baseline for the Luna Assistant Core that proves the chosen stack (Rust + plugin architecture) and exercises the governance workflow (Decision Note → ADR → implementation → CI).

## Options Considered
- **Option A — Implement a Rust workspace with an EventBus and echo plugin (chosen)**  
  Provides a concrete foundation, validates CI, and keeps future ADRs grounded in working code.
- **Option B — Postpone implementation until more research**  
  Keeps the repository purely documentary but leaves CI and governance untested.
- **Option C — Prototype in another language first**  
  Could speed up brainstorming but diverges from the intended Rust architecture, creating translation overhead later.

## Decision
Adopt Option A. Create a `core` crate inside a Cargo workspace with:
- An `Event` struct and `EventBus` capable of registering plugins and dispatching events.
- A `Plugin` trait for skills/components.  
- A simple echo example proving dispatch behaviour.  
- Unit tests covering registration and event dispatch.

## Consequences
- ✅ Rust toolchain, fmt, clippy, and trivy stages are exercised automatically in CI.  
- ✅ Provides a canonical example for future contributors / ADRs.  
- ⚠️ We must evolve the API via future ADRs as requirements expand (intents, async handling, etc.).

## Metrics for Success
- CI pipeline remains green (`cargo fmt`, `cargo build/test`, `cargo clippy`, `trivy`).  
- A feature branch implementing the skeleton can be merged without additional scaffolding.  
- Subsequent ADRs can reference this baseline instead of drafting from scratch.

## References
- Decision Note: `docs/DECISIONS/decision-note-0001-base-rust-core.md`  
- Issue/PR: *(link to be added when PR is opened)*
