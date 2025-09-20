# 🤝 Contributing Guide — Luna Core

Welcome! We are glad you're interested in contributing to **Luna Assistant Core**.  
This guide explains what contributions are welcome, and how to work within our **AI-native governance model**.

---

## ✅ What You Can Contribute
- **RFCs**: propose new decisions using the [RFC issue template](.github/ISSUE_TEMPLATE/rfc.yml).  
- **Research**: submit findings or analysis via the [Research issue template](.github/ISSUE_TEMPLATE/research.yml).  
- **Documentation**: improve guides, checklists, ADRs, or READMEs.  
- **Tests**: add or improve unit/integration tests.  
- **Accessibility Feedback**: capture user/tester feedback via the [Accessibility Feedback issue template](.github/ISSUE_TEMPLATE/accessibility-feedback.yml).  
- **Translations**: suggest improvements for international accessibility.  

---

## 🚫 What Is Reserved
- **Core code changes without ADR**: structural modifications to the Rust core, FFI bindings, or governance logic require a validated ADR.  
- **Feature definitions**: functionality is defined by the governance process (Decision Notes → ADRs), not directly in PRs.  
- **Bypassing CI**: all PRs must pass Drone CI (build, test, lint, security).  

---

## 📝 Workflow
1. Open an **RFC issue** for significant changes.  
2. Participate in the synthesis → Decision Note → ADR validation process.  
3. Once ADR is validated, implement changes on a feature branch.  
4. Submit a PR referencing the ADR, using the [PR template](.github/PULL_REQUEST_TEMPLATE.md).  
5. Ensure all **domain checklists** are reviewed (Legal, Marketing, Tech, Finance).  
6. CI must pass before merge.  

---

## 📜 Commit Conventions
We use [Conventional Commits](https://www.conventionalcommits.org/):  
- `feat:` → new capability (backed by ADR)  
- `fix:` → bug fix  
- `docs:` → documentation changes  
- `test:` → adding or updating tests  
- `chore:` → tooling, infra, non-core changes  

---

## 🛡️ Governance Safeguards
- **Human-in-the-loop**: all structural decisions require human validation.  
- **Transparency**: every decision is logged in `docs/DECISIONS/index.md`.  
- **Labels**: use `sequence:*`, `domain:*`, and `adr:*` labels for tracking.  

---

## 📌 Summary
You are encouraged to contribute to research, docs, tests, and feedback.  
Structural code changes are possible, but only through the ADR process.  
This ensures that Luna evolves in a way that is transparent, inclusive, and governed.
