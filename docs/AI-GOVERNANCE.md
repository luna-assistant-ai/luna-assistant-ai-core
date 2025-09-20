# 🧭 AI Governance Framework

This document describes the governance model used in the Luna project.  
It ensures that AI agents and human contributors work together with clarity, transparency, and safeguards.

---

## 🔄 Governance Sequences

We progress **by sequence**, not by calendar.  
Each sequence represents a level of decision throughput and automation.

| Sequence   | Decision Load | Mode                  | Thresholds | Expected Deliverables |
|------------|---------------|-----------------------|------------|-----------------------|
| **Lite**   | ≤ 5 / cycle   | Manual + AI briefs    | Synthesis < 30 min | AI briefs + 1-page ADR |
| **Transition** | 5–15 / cycle | AI-assisted syntheses | Synthesis > 1h | Comparative syntheses + 3–5 page ADR |
| **Full**   | > 15 / cycle  | CrewAI + AutoGen debates | Synthesis > 2h, infra Python ready | CrewAI reports + debate transcripts + validated ADR |

👉 Rule of thumb:  
- Stay in Lite while manageable.  
- Move to Transition once synthesis becomes heavy.  
- Move to Full when orchestration adds more value than it costs.

---

## 🤖 AI Roles

- **Global Agent (`docs/AGENT.md`)**  
  Orchestrates governance, ensures ADR process, maintains coherence.

- **AI Project Manager (`docs/AI-PM.md`)**  
  Guardian of process: ADR linkage, Drone CI checks, labels, board updates.

- **Domain Checklists (`docs/PROCESS/`)**  
  - Legal → compliance, privacy, liability  
  - Marketing → accessibility positioning, community, roadmap  
  - Tech → CI green, reproducibility, ADR linkage  
  - Finance → costs, budgets, sustainability

---

## 📑 Decision Flow

1. **Decision Note** (`docs/PROCESS/decision-note-template.md`)  
   Draft prepared with inputs from domain agents (Claude, ChatGPT, Continue).  

2. **ADR** (`docs/DECISIONS/`)  
   Formalized decision, validated by human-in-the-loop.  
   Template: `ADR-0000-template.md`.

3. **Index** (`docs/DECISIONS/index.md`)  
   Central list of all ADRs, their status (Draft/Accepted/Superseded).

---

## ⚙️ Tools & Infrastructure

- **Drone CI** (`.drone.yml`)  
  Build, test, lint, security scans.  
  PRs blocked if ADR missing or CI fails.

- **GitHub Projects**  
  Board *Governance Flow* with columns: Incoming → AI Briefs → Synthesis → ADR Draft → ADR Validated → Implemented.

- **GitHub Issues & PR Templates**  
  RFCs, Research, Decisions, Accessibility feedback → Issue templates.  
  PR template requires ADR link + checklists.

- **Labels**  
  - `sequence:1-lite`, `sequence:2-transition`, `sequence:3-full`  
  - `domain:legal`, `domain:marketing`, `domain:tech`, `domain:finance`  
  - `adr:required`, `adr:linked`, `adr:done`

---

## 🛡️ Human-in-the-loop Safeguards

- **No feature decision is made solely by AI**.  
- **All structural changes** require a validated ADR.  
- **Transparency**: every decision is recorded in `DECISIONS/index.md`.  
- **Escalation**: AI-PM alerts when thresholds indicate sequence migration.

---

## 📌 Summary

This governance framework ensures:  
- Clear **sequence thresholds**,  
- Explicit **roles for AI and humans**,  
- A robust **Decision Note → ADR → Index** flow,  
- Integrated **tools** (CI, Issues, Projects, Labels),  
- And **human oversight** at every step.
