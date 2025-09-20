# 🔒 Security Policy

## Scope
This repository currently ships documentation, process files, and a Rust skeleton.  
No production credentials or sensitive user data should ever be committed.

## Reporting a vulnerability
- Preferred: use **GitHub Security Advisories** (*Security → Report a vulnerability*).  
- Alternative: if needed, contact the maintainers via the repository discussions.  

Reports are acknowledged within 72h, triaged within 7 days, and remediation guidance shared when available.

## Principles
- **Ephemeral credentials only**: never commit tokens; avoid long-lived keys.  
- **No secrets in CI logs**: Drone must not echo sensitive values.  
- **Least privilege**: minimal scopes for CI, bots, and agents.  
- **Encryption required** if/when local data is introduced (e.g., SQLCipher + keychain/keystore).  
- **Zero voice/audio recordings by default**; any future storage must be opt-in and documented via ADR.  

## Supported versions
- Only the `main` branch is maintained. Draft branches are unsupported once merged/closed.
