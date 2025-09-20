# 🛡️ Privacy Policy (Foundational)

## Current state
- This repository currently ships documentation, process files, and a Rust skeleton.
- **No voice/audio recordings are stored by default**; any future storage will be opt-in and defined via ADR.

## Principles
- **Data minimization**: only collect/store what is strictly necessary, justified by an ADR.
- **Purpose limitation**: each data handling path must reference its ADR and Decision Note.
- **Local encryption**: if/when client-side storage appears, use SQLCipher + OS keychain/keystore.
- **Third-party APIs**: request minimal scopes only; avoid “restricted” scopes unless explicitly justified in an ADR.

## User rights (when user data exists)
- **Access & export**: users must be able to retrieve their data in a portable format.
- **Deletion**: users must be able to request deletion of their data.
- **Transparency**: any new data processing must be documented via ADRs.

## Contact
- Preferred: open a discussion or issue tagged `privacy`.  
- Alternative: contact the maintainers (replace with a valid inbox if/when established).
