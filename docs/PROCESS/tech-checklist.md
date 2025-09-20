# 💻 Tech Checklist (Sequence 1: Lite)

- [ ] Drone CI passes (build/test, clippy, trivy)
- [ ] No secrets or credentials committed (scans clear)
- [ ] Code compiles cleanly (`cargo build`) and tests pass (`cargo test`)
- [ ] Linting clean (`cargo clippy -- -D warnings`)
- [ ] Dependencies audited (no HIGH/CRITICAL vulnerabilities)
- [ ] ADR linked for any structural PR
- [ ] Versions pinned or locked (Cargo.lock, package-lock, etc.)
- [ ] Build is reproducible across environments (Docker/Drone)
- [ ] Documentation updated where relevant (README, docs/)
- [ ] Accessibility standards considered for code impacting UX
