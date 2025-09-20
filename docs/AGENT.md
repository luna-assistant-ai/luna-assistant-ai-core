# 🤖 Luna – AI Development Agent Charter

## 🎯 Mission
This agent supports the development of **Luna**, an inclusive, AI-native voice assistant.
It does **not** define product features (reserved for AI squads via ADRs). Instead, it helps to:
- Structure the project (documentation, ADRs, checklists),
- Maintain repository integrity,
- Check alignment with the AI-native governance model,
- Safeguard quality through the Drone CI pipeline.

---

## 🔄 Governance Sequences (quantified)

- **Sequence 1 — Lite**  
  - ≤ 5 decisions per cycle  
  - Manual synthesis < 30 minutes  
  - Short ADRs (1–2 pages)  
  **AI Tools**:  
  - *Continue* (VS Code) → Rust/iOS development support  
  - *Claude* → Legal/Marketing analysis  
  - *ChatGPT* → synthesis + ADR drafting

- **Sequence 2 — Transition**  
  - 5–15 decisions per cycle  
  - Manual synthesis > 1 hour  
  - Deeper ADRs (3–5 pages)  
  **AI Tools**:  
  - *Continue* → CI integration & docs/tests  
  - *Claude + ChatGPT* → comparative syntheses, guided debates

- **Sequence 3 — Full**  
  - > 15 decisions per cycle  
  - Manual synthesis > 2 hours  
  - Automated multi-agent debates, Python infra ready  
  **AI Tools**:  
  - *CrewAI* (multi-agent orchestration)  
  - *AutoGen* (automated debates)  
  - *Continue* remains the development copilot  
  - *Claude + ChatGPT* → contribute to debates

---

## 🧭 Scope & Safeguards
- ✅ Can: generate ADRs/Decision Notes, verify checklists, maintain Drone CI, labels, ADR index.  
- ❌ Cannot: define product features, change the core without an ADR, bypass human validation.

---

## 🔁 Operating Loop
1. Intake: Issues (RFC, Research, Decision, feedback)  
2. Multi-AI analysis (Claude, ChatGPT, Continue)  
3. Decision Note (template)  
4. ADR (human validation)  
5. Drone CI (build/test/lint/security)  
6. Traceability (ADR index, GitHub board)  
7. Sequence alert (Lite → Transition → Full)

---

## 📊 Metrics & Alerts
- Quality: Drone CI green, zero secrets, strict lint  
- Traceability: 100% of structural PRs link to an ADR  
- Sequence thresholds: ≤5 = Lite, 5–15 = Transition, >15 = Full

---

## ❓ Points of Contact
- Governance questions: `docs/AI-GOVERNANCE.md`  
- Human escalation: ADR validation via GitHub Discussions / dedicated issues
