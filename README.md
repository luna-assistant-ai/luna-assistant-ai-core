# 🌙 Luna – Inclusive & AI-first Voice Assistant

**Luna** is a voice assistant designed for people with visual impairments (and later adaptable to other disabilities).  
Its mission is to bring **autonomy, safety, and simplicity** to daily life through a **hybrid open-source architecture** and **AI-first governance**.

---

## ✨ Key Features

- 🎤 **Natural conversation** with GPT-4o Realtime (audio ↔ audio)  
- 📞 **Hands-free emergency calls** (112 / trusted contacts)  
- ⏰ **Reminders & timers** via voice  
- 📧 **Gmail integration** (read, summarize, draft replies)  
- 🎶 **Spotify** (playlists, playback control)  
- 📄 **Document scanning & classification** (OCR → save + smart search)  
- 📅 **Agenda management** (local or Google Calendar sync)  
- 🌐 **Web search** adapted for audio-first answers  

---

## 🛡️ Security & Privacy First

- 🔑 No API keys stored on device (ephemeral credentials only)  
- 🔒 All local data encrypted (SQLite + system keychain/keystore)  
- 🚫 Zero voice recordings by default (opt-in storage only)  
- ✅ Critical actions require vocal confirmation (e.g., “Yes, call emergency”)  

---

## ⚡ Quick Demo

1. Clone & enter the workspace:
   ```bash
   git clone https://github.com/luna-assistant-ai/luna-assistant-ai-core.git
   cd luna-assistant-ai-core
   ```
2. Test and build the core crate:
   ```bash
   cd core
   cargo test
   cargo build
   ```
3. No code yet? Watch [@luna-assistant-ai](https://github.com/luna-assistant-ai) to get notified when the first modules land.

---

## 🛠️ Getting Started

1. Clone the repository and enter the workspace:
   ```bash
   git clone https://github.com/luna-assistant-ai/luna-assistant-ai-core.git
   cd luna-assistant-ai-core
   ```
2. Install prerequisites:
   - Rust ≥ 1.81 (required)  
   - Xcode for iOS development (required if you target iOS)  
   - Node.js (optional — only for local OAuth backend testing)  
   If you do not have Rust yet, run `rustup default stable` after installing rustup.
   > Note: The `core/` crate and client apps will appear as code lands in this repository. If you cloned a skeleton repo without those folders yet, keep an eye on the roadmap section or GitHub issues for the drop.
3. Bootstrap the core crate to verify your toolchain:
   ```bash
   cd core
   cargo test
   cargo build
   ```
4. (Optional) Open the iOS client to run Luna on a simulator:
   ```bash
   open ../clients/ios/App.xcodeproj
   ```
   Then press `Cmd + R` inside Xcode. Refer to the sections below for more platform-specific instructions, environment variables, and integration secrets.
   Additional deep-dive guides will live in the [`luna-assistant-ai-docs`](https://github.com/luna-assistant-ai/luna-assistant-ai-docs) repository as they are published.

**Not ready to code yet?**  
- ⭐ Star this repo to get notified of first commits  
- 💬 Join [Discussions](https://github.com/luna-assistant-ai/luna-assistant-ai-core/discussions) to shape the roadmap  
- 📖 Browse accessibility research and guidelines in [`luna-assistant-ai-docs`](https://github.com/luna-assistant-ai/luna-assistant-ai-docs) (coming soon)  

---

## 📁 Repository Structure

- **[luna-assistant-ai-core](./)** ← you are here (Rust engine, FFI bindings, basic clients)  
- **[luna-assistant-ai-docs](https://github.com/luna-assistant-ai/luna-assistant-ai-docs)** (guides, accessibility standards, prompt library)  
- **[luna-assistant-ai-pro](https://github.com/luna-assistant-ai/luna-assistant-ai-pro)** — private (AI copilots, premium integrations, caregiver dashboard)  
- **[@luna-assistant-ai](https://github.com/luna-assistant-ai)** (organization-level roadmap & discussions)  

---

## 🏗️ Technical Architecture

### Core (Rust)
- Event bus + intent detection  
- Modular plugins (skills): SOS, reminders, OCR, Gmail, Spotify, search  
- Exposed via FFI (iOS/Android) or service (Raspberry Pi)  

### Clients
- 📱 **iOS (SwiftUI)** – first target  
- 🤖 Android (Jetpack Compose) – planned  
- 💻 Raspberry Pi Hub (mic + speaker + SOS button) – planned  

### Cloud
- **OpenAI GPT-4o Realtime** (audio ↔ audio)  
- Lightweight backend (ephemeral keys, OAuth callbacks)  

### Architecture Diagram

```mermaid
flowchart TD
    User["👤 User (Voice)"]

    subgraph Device["📱 Device"]
        App["Luna App"]
        Core["🦀 Rust Core"]
        Skills["🔧 Skills\nSOS · OCR · Reminders"]
    end

    subgraph Cloud["☁️ Services"]
        GPT["🎤 GPT-4o Realtime"]
        Gmail["📧 Gmail API"]
        Emergency["🚨 Emergency Services"]
        Storage["🗂️ Secure Storage"]
    end

    User <--> App
    App <--> Core
    Core <--> Skills
    Skills --> GPT
    Skills --> Gmail
    Skills --> Emergency
    Core --> Storage

    style User fill:#e1f5fe,stroke:#90caf9
    style Skills fill:#f3e5f5,stroke:#ce93d8
    style Emergency fill:#ffebee,stroke:#ef9a9a
    style Storage fill:#f1f8e9,stroke:#aed581
```

*Luna's governance and development process (including AI-assisted analysis) is detailed in the [AI-Assisted Governance](#-ai-assisted-governance-human-in-the-loop) section below.*

---

## 🏗️ Architecture Decisions

- **Why Rust core?** Deterministic performance and memory safety for accessibility-critical flows.  
- **Why AI copilots?** Scale expert analysis (compliance, forecasting, benchmarking) in a niche domain with limited human bandwidth.  
- **Why hybrid licensing?** Keep the foundation open-source while funding long-term accessibility R&D through premium integrations.

---

## 🤖 AI-Assisted Governance (Human-in-the-Loop)

Luna uses AI copilots to accelerate analysis while humans retain decision ownership.

- **AI role**: Draft reports, benchmarks, compliance reviews, financial forecasting.  
- **Human role**: Approve strategic choices, user-facing changes, and safety-critical flows.  
- **Process**: AI output → Maintainer review → Validation with stakeholders → Action in product or roadmap.  

CrewAI orchestrates structured pipelines and AutoGen supports exploratory debates when evaluating trade-offs.

- **Marketing AI** → competitive benchmarking, user studies (visually impaired in NZ), market sizing  
- **Finance AI** → cost analysis, financial forecasting, ROI scenarios  
- **Tech AI** → architecture choices, infra costs, security validation  
- **Legal AI** → GDPR/NZ compliance, open-source licenses, funding opportunities  

---

## 🔑 Open Source Strategy

### Hybrid Model
- **Public (open source)**  
  - Rust Core (event bus, intents API)  
  - Minimal iOS/Android clients  
  - Essential skills: SOS, reminders, basic OCR  

- **Private (proprietary)**  
  - AI Copilots (Marketing, Finance, Legal, Tech)  
  - Premium integrations (advanced Gmail, Spotify, LLM-powered search)  
  - Caregiver dashboard (activity insights, opt-in sharing)  

### License Choice
- **Core & clients**: MIT → maximize adoption and contributions  
- **Premium modules**: proprietary license (e.g., SSPL or BUSL to restrict commercial reuse)  

### Community Governance
- **Maintainers**: core team (Rust/iOS/Android devs)  
- **External contributors**: PR → human review required → CI/CD green → auto-merge via Mergify  
- **CONTRIBUTING.md**: clear rules (tests, accessibility, Conventional Commits)  
- **CODE_OF_CONDUCT.md**: inclusivity & respect  
- **Public roadmap** (Zenhub boards in read-only)  
- **GitHub Discussions**: open channel for visually impaired users & caregivers  

---

## 🚀 Development & CI/CD

### Requirements
- [Rust](https://www.rust-lang.org/tools/install) ≥ 1.81  
- [Xcode](https://developer.apple.com/xcode/) (iOS)  
- [Node.js](https://nodejs.org/) (OAuth backend)  
- [Drone CI](https://www.drone.io/) (CI/CD)  

### Build the Core
```bash
cd core
cargo build --release
cargo test
```

- Generate iOS XCFramework: `bash core/build-ios.sh`
- Run the iOS client: `open clients/ios/App.xcodeproj` then press `Cmd + R`

### CI/CD Workflow
- Drone CI → build, test, iOS artifacts, Raspberry Pi Docker image  
- Mergify → auto-merge if CI passes and a review approves  
- Zenhub → backlog & roadmap fed by CrewAI pipelines  

### Security & Privacy
- 🔑 Ephemeral keys for Realtime (never stored client-side)  
- 🔒 Local data encrypted (SQLite + Keychain/Keystore)  
- 🚫 No voice recordings stored by default (opt-in only)  
- ✅ Critical actions (e.g., 112 calls) require vocal confirmation  

---

## 🛣️ Roadmap & Status *(last updated: December 2024)*

- **Weeks 1–4**: Core engine scaffold + iOS skeleton → first public commit  
- **Months 1–2**: SOS + Reminders skills → closed alpha with testers in NZ  
- **Months 3–4**: OCR pipeline + end-to-end voice loop → MVP release and feedback cycle  
- **Post-MVP**: Gmail/Spotify integrations, Raspberry Pi hub, AI governance pilots shared via Docs repo  

Progress snapshots and issues live in Zenhub boards and GitHub Discussions.

---

## 👩‍💻 Contributing

Luna thrives on community input—especially from people with lived accessibility experience. Whether you are new to open source or a seasoned maintainer, we can pair you with a starter issue or mentoring buddy.

- Fork then branch (`feat/...`, `fix/...`)  
- Run tests (`cargo test`, `xcodebuild test`) before opening a PR  
- Fill the PR template (objective, tests, accessibility, rollback)  
- Human review required → CI green → Mergify auto-merge  
- Follow Conventional Commits and keep PRs small, tested, documented  

---

## 🤝 Get Involved

- **Users with visual impairments**: Join early testing via [GitHub Discussions](https://github.com/luna-assistant-ai/luna-assistant-ai-core/discussions)  
- **Developers**: Pick up tasks in [Issues](https://github.com/luna-assistant-ai/luna-assistant-ai-core/issues) and review [CONTRIBUTING.md](./CONTRIBUTING.md)  
- **Organizations & partners**: Start a thread in [GitHub Discussions](https://github.com/luna-assistant-ai/luna-assistant-ai-core/discussions)  
- **Community**: Watch [@luna-assistant-ai](https://github.com/luna-assistant-ai) for release updates  

---

## 📜 License

- Core: MIT (open source)  
- Basic clients: MIT  
- Premium modules (AI copilots, advanced integrations): proprietary license  
