# 🎒 Luna — Context Pack Triggers

This document defines two triggers for selecting the right context when interacting with AI assistants.  
It helps keep sessions lightweight when coding, and complete when making strategic decisions.

---

## ⚡ Quick Trigger
/context quick
👉 Use this for **short sessions** (coding, testing, completing an ADR).  
Loaded automatically:
- Quick Context (10 lines — see `docs/WORKFLOW.md`)  
- Current ADR (draft/validated)  
- Related Decision Note  
- Tech Checklist (`docs/PROCESS/tech-checklist.md`)  
- `.drone.yml` (CI pipeline)

➡️ Ideal for Continue (VS Code) or when moving fast.

**Example prompt**:  
```
/context quick
ADR in progress: docs/DECISIONS/ADR-0002.md
Task: help me complete unit tests while respecting Drone CI.
```

---

## 📚 Full Trigger
/context full
👉 Use this for **full sessions** (new decisions, governance, sequence migration, onboarding).  
Loaded automatically:
- `README.md` (vision + AI sequences)  
- `docs/AGENT.md` (global agent)  
- `docs/AI-PM.md` (process guardian)  
- `docs/AI-GOVERNANCE.md` (framework & roles)  
- `docs/WORKFLOW.md` (detailed flow)  
- ADR draft + Decision Note  
- Domain Checklists (Legal, Marketing, Tech, Finance)  
- `docs/CONTRIBUTING.md` + `docs/DEVELOPER.md`  
- `.drone.yml`

➡️ Ideal for structural decisions and team sessions.

**Example prompt**:  
```
/context full
New decision: encrypted local storage mechanism.
Goal: produce a Decision Note with multi-domain analysis → ADR.
```

---

## ⚖️ Rule of Thumb
- **Quick = execution** (code, tests, completing a scoped decision).  
- **Full = governance** (new ADRs, migration, organisation, onboarding).
