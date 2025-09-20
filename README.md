# 🌙 Luna – AI-First & Inclusive Voice Assistant

**Mission**  
Deliver autonomy and simplicity for people with low vision (and broader accessibility needs) with an **AI-native** approach:  
→ Strategic decisions are produced by AI squads (Legal, Marketing, Tech, Finance) and ratified through Architecture Decision Records (ADRs). The implementation is handled solo to preserve coherence, with human oversight applied at every step through ADR validation.

---

## 🔄 AI Governance Sequences

We progress **by sequences**, not by calendar. Each sequence reflects a maturity level for decision throughput and automation.

| Sequence | Decision Load | Mode | Indicators | Expected Deliverables |
|----------|---------------|------|------------|-----------------------|
| **1. Lite** | ≤ 5 decisions / cycle | Manual | Synthesis < 30 min | AI briefs + 1-page ADR |
| **2. Transition** | 5–15 decisions / cycle | AI-assisted | Synthesis > 1 h | Comparative syntheses + 3–5 page ADR |
| **3. Full** | > 15 decisions / cycle | CrewAI + AutoGen orchestration | Synthesis > 2 h, automated debates ready, Python infra available | CrewAI reports + debate transcript + validated ADR |

👉 **Rule of thumb**  
- Stay in Lite while it remains manageable.  
- Switch to Transition when synthesis becomes heavy.  
- Shift to Full when AI automation adds more value than it costs.

---

## 🏗️ High-Level Architecture

- **Core (Rust)**: event-driven engine, intent bus, neutral API surface  
- **Clients**: iOS (SwiftUI), Android (Jetpack Compose), Raspberry Pi hub (stub)  
- **Cloud**: lightweight backends (ephemeral keys, OAuth callbacks)  
- **Governance**: decisions are tracked via ADRs (Architecture Decision Records)

---

## 🌍 What Luna Delivers for Users

- Greater day-to-day autonomy for people with visual impairments  
- Reassurance in emergency scenarios through voice-triggered safety flows  
- Voice-first simplicity that minimizes friction in critical tasks  
- Transparent, ethical governance that keeps human oversight in the loop  

---

## 🗂️ Repository Layout

```
luna-assistant-ai-core/
├─ core/            # compilable skeleton (cargo build/test)
├─ clients/         # iOS / Android / Pi stubs
├─ docs/
│  ├─ AGENT.md
│  ├─ AI-PM.md
│  ├─ AI-GOVERNANCE.md
│  ├─ SECURITY.md
│  ├─ PRIVACY.md
│  ├─ DEVELOPER.md
│  ├─ CONTRIBUTING.md
│  ├─ WORKFLOW.md
│  ├─ CONTEXT.md
│  ├─ DECISIONS/
│  └─ PROCESS/
├─ .drone.yml       # Drone pipeline definition (build/test/lint/security)
└─ .github/
   ├─ PULL_REQUEST_TEMPLATE.md
   └─ ISSUE_TEMPLATE/
```

---

## 🚀 Getting Started

1. Clone the repository  
   ```bash
   git clone https://github.com/luna-assistant-ai/luna-assistant-ai-core.git
   cd luna-assistant-ai-core
   ```
2. Verify the toolchain  
   ```bash
   cd core
   cargo build
   cargo test
   ```
   ✅ The core already ships a neutral skeleton (minimal event bus, “hello world” tests).
3. Explore AI governance in `docs/`
   - 📐 **AI Workflow**  
   - Domain briefs (Legal / Marketing / Tech / Finance)

   Flow overview:
   ```text
   AI Domain Briefs → Synthesis (Claude + GPT-4) → Decision Note (template)
   → ADR (human validation) → Implementation (PR + Drone CI)
   ```
   > Note: documentation inside `docs/` ships in sync with AI sequences; some files may appear as skeletons before full delivery.

---

## 🤖 AI Development Stack

- **CrewAI** – Multi-agent coordination and reporting  
- **AutoGen** – Multi-party debates for complex decisions  
- **Claude (Anthropic)** – Strategic analysis and domain expertise  
- **GPT-4 (OpenAI)** – Cross-validation and technical analysis  
- **Continue (VS Code)** – In-editor development assistance  

---

## 🛡️ Security & Privacy

- 🔑 Ephemeral keys only (PKCE, short TTL)  
- 🔒 Local data encrypted (SQLCipher + system keychain/keystore)  
- 🚫 No voice recordings by default (explicit opt-in)  
- 📖 Transparent policy documented in `docs/PRIVACY.md`

---

## 🤝 Contributions

- ✅ Welcome: RFCs, documentation, tests, accessibility feedback  
- 🚫 Not accepted: core engine code without a validated ADR  
- 📬 GitHub tooling: issue templates for [RFCs](./.github/ISSUE_TEMPLATE/rfc.md), [decisions](./.github/ISSUE_TEMPLATE/decision.md), research, and feedback  
- 👩‍⚖️ Decisions are public via ADRs (`docs/DECISIONS/`)

---

## 📊 CI & Quality

- Drone pipeline: build / test / lint / security checks  
- Policy: PRs blocked if the related ADR is missing or CI fails  

---

## 📜 License

- Core & clients: MIT  
- Premium modules (AI copilots, advanced integrations): proprietary license (BUSL or equivalent)
