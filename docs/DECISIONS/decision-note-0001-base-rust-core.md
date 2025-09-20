# Decision Note 0001 — Base Rust Core Skeleton

## Context
- Establish the minimal code foundation for Luna Assistant Core.
- Enable the ADR → implementation workflow to be tested end-to-end.
- Provide an example for future contributors on how governance artefacts map to code.

## Options
- **Option A — Rust skeleton with event bus + echo plugin (recommended)**  
  Create a minimal Rust workspace with an `EventBus` and a sample plugin to prove dispatching works.  
  Pros: validates architecture, easy to test, aligns with long-term tech choice.  
  Cons: requires initial investment in Rust tooling.
- **Option B — Delay implementation until more research**  
  Keep documenting without code.  
  Pros: saves time in the short term.  
  Cons: governance remains theoretical, CI pipeline unused.
- **Option C — Prototype in a higher-level language first**  
  Use Python/TypeScript to sketch behaviour.  
  Pros: faster iteration if Rust skills lacking.  
  Cons: diverges from ADR direction, adds translation cost later.

## Multi-domain Analysis
- **Legal**: No user data is handled; no additional legal exposure.  
- **Marketing**: Demonstrates tangible progress to community; supports future announcements.  
- **Tech**: Validates Rust toolchain, CI, fmt/clippy/trivy stages, plugin API direction.  
- **Finance**: No extra cost beyond CI minutes; avoids future rework by aligning early with target stack.

## Risks & Mitigations
- **Risk**: Over-engineering the first skeleton.  
  Mitigation: keep the feature set to a simple echo plugin and unit tests.  
- **Risk**: Rust setup issues for contributors.  
  Mitigation: ensure docs/DEVELOPER.md remains accurate and references cargo commands.  
- **Risk**: Event bus API may evolve.  
  Mitigation: capture changes via future ADRs; this ADR documents the initial baseline.

## Recommendation (AI synthesis)
Proceed with Option A: implement a Rust workspace containing a minimal `EventBus`, registerable plugins, and an echo example with unit tests. Use this cycle to validate the ADR workflow and CI pipeline.

## References
- Related Issue: *(to be created when RFC is filed)*  
- ADR Template: `docs/DECISIONS/ADR-0000-template.md`
