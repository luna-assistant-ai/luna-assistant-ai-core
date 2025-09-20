# ADR-0002 — Asynchronous Event Bus

## Context
ADR-0001 introduced a synchronous `EventBus` and `Plugin` trait. While adequate for the initial skeleton, synchronous dispatch blocks the caller until all plugins finish, which is unsuitable for real-time voice interactions and I/O-heavy skills (SOS calls, email fetching, OCR, etc.). We need an asynchronous architecture that preserves responsiveness and isolates slow or failing plugins.

## Decision
Adopt a hybrid asynchronous plugin model:
- Use `tokio` and `async_trait` so every plugin exposes an `async fn handle_async(&self, event: &Event) -> EventResult` with a default wrapper around a synchronous `handle_sync` method.
- Dispatch events concurrently via `tokio::spawn`, wrapping the shared event in `Arc<Event>` to avoid unnecessary cloning.
- Enforce per-plugin timeouts (default 5 s) and capture task panics, returning structured errors.
- Update `EventBus::dispatch` to return aggregated `Vec<EventResult>` and document the error aggregation strategy (collect all results, including timeouts/panics).
- Convert existing examples/tests (e.g. `EchoPlugin`) to the async trait.

## Status
Accepted

## Consequences
- **Pros**
  - Voice interactions remain responsive (< 200 ms target) because plugin work runs concurrently.
  - CPU‑bound plugins can still rely on `handle_sync` and `tokio::task::spawn_blocking`.
  - Timeouts prevent a misbehaving plugin from blocking others.
  - The architecture aligns with the modern Rust async ecosystem and prepares for future I/O integrations (SOS, Gmail, Spotify).
- **Cons**
  - Breaking change: all existing and future plugins must support async.
  - Async introduces additional complexity (runtime setup, async testing, stack traces).
  - Need to benchmark overhead versus the synchronous baseline to ensure acceptable regression (< 10 %).

## Alternatives Considered
- **Thread-based dispatch**: spawn a thread per plugin and aggregate via channels. Rejected due to heavyweight threads, complex error handling, and divergence from async Rust practices.
- **Purely synchronous bus with worker threads**: keeps trait sync but does not solve latency/isolation. Rejected because blocking operations still block their worker threads and reintroduce complexity.

## Risk Assessment & Mitigations
- **Breaking change cascade**: supply migration guide/templates for plugin authors; keep `handle_sync` default implementation.
- **Runtime complexity (panics, timeouts)**: centralize runtime creation, wrap spawn results, log structured errors.
- **Testing difficulty**: use `#[tokio::test]` helpers, timeouts in tests, and benchmarking harness to compare sync/async performance.

## Implementation Plan
1. Add `tokio` and `async-trait` dependencies to the workspace.
2. Update the `Plugin` trait with `handle_sync` + `handle_async` default wrapper.
3. Modify `EventBus::dispatch` to spawn tasks, use `Arc<Event>`, apply `tokio::time::timeout`, and aggregate results.
4. Convert `EchoPlugin` and existing tests to async (`#[tokio::test]`).
5. Add benchmarks comparing sync vs async overhead (goal: < 10 % regression for lightweight plugins).

## Success Metrics
- Latency for typical voice interactions under 200 ms (SOS critical path < 100 ms when implemented).
- Multiple plugins processing events concurrently (validated via logs/metrics).
- CI remains green with async tests.
- Benchmark shows async overhead within acceptable threshold.
- Timeout handling confirmed via automated tests.

## References
- Decision Note 0002 — Async Event Bus Implementation
- Rust Async Book — https://rust-lang.github.io/async-book/
- Tokio Documentation — https://tokio.rs/tokio/tutorial
*** End Patch
