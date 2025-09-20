# 👩‍💻 Developer Guide — Luna Core

This document provides setup instructions, conventions, and the development workflow for contributors to the **Luna Assistant Core** project.  
The goal is to ensure a consistent developer experience and maintain alignment with the AI-native governance model.

---

## 🔧 Prerequisites

- **Rust** ≥ 1.81  
  Install via [rustup](https://rustup.rs).  
  ```bash
  rustup default stable
  rustc --version
  ```
- **Node.js** (optional, for local OAuth backend testing).  
  Recommended: LTS version via [nvm](https://github.com/nvm-sh/nvm).
- **Drone CLI** (optional, for running CI jobs locally).  
  Install via [Drone CLI docs](https://docs.drone.io/cli/).
  ```bash
drone --version
```
- **Python 3.11+** (optional, for label sync script).

---

## ⚙️ Build & Test

Run locally before committing:

```bash
cargo build
cargo test
cargo clippy -- -D warnings
```

Run security scan (requires [Trivy](https://aquasecurity.github.io/trivy/)):

```bash
trivy fs --severity HIGH,CRITICAL .
```

---

## 🚦 Continuous Integration

CI is defined in `.drone.yml`. Pipelines run on push/PR and include:
- Build & Test (`cargo build`, `cargo test`)
- Lint (`cargo clippy`)
- Security Scan (`trivy`)

🚫 PRs will be blocked if CI fails or if no ADR is linked.

---

## 📜 Conventions

- **Commits** follow [Conventional Commits](https://www.conventionalcommits.org/):
  - `feat: ...` → new feature
  - `fix: ...` → bug fix
  - `docs: ...` → documentation changes
  - `chore: ...` → tooling/infra changes
  - `refactor: ...` → code structure changes without behavioural change
- **Branches**:
  - `feat/<slug>`
  - `fix/<slug>`
  - `chore/<slug>`
- **Pull Requests** must use the PR template, including:
  - ADR reference (if structural)
  - Multi-domain checklists
  - Green CI
  - Correct labels applied

---

## 📝 Decision Workflow

1. **RFC** → create an issue using the RFC template.  
2. **Research** → capture analysis in Research issues.  
3. **Decision Note** → draft using `docs/PROCESS/decision-note-template.md`.  
4. **ADR** → record in `docs/DECISIONS/`, based on `ADR-0000-template.md`.  
5. **Index** → update `docs/DECISIONS/index.md`.  
6. **PR** → implement, link ADR, pass CI, apply labels.

---

## 📋 Checklists

Before merging a structural change, ensure all domain checklists are reviewed:
- `docs/PROCESS/legal-checklist.md`
- `docs/PROCESS/marketing-checklist.md`
- `docs/PROCESS/tech-checklist.md`
- `docs/PROCESS/finance-checklist.md`

---

## 🔒 Governance Safeguards

- No feature is decided by AI alone.  
- All structural changes require a validated ADR.  
- Transparency: ADRs are tracked in `docs/DECISIONS/index.md`.  
- Escalation: AI-PM alerts when sequence thresholds are exceeded.

---

## 🏁 Quick Start (Developer Workflow)

1. Clone and enter the repo:
   ```bash
   git clone https://github.com/luna-assistant-ai/luna-assistant-ai-core.git
   cd luna-assistant-ai-core
   ```
2. Build & test:
   ```bash
  cargo build && cargo test
   ```
3. Run lint & security scan:
   ```bash
   cargo clippy -- -D warnings
   trivy fs --severity HIGH,CRITICAL .
   ```
4. Create RFC/Decision issue if needed.  
5. Implement changes on a feature branch.  
6. Open PR using the template → link ADR → pass CI → merge.

---

With this guide, you have a clear entry point for contributing while staying aligned with the AI-native governance model.
