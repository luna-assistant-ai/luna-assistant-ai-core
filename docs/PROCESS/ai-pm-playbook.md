# 🎮 AI-PM Playbook (Sequence 1: Lite)

## When a PR arrives
1. Check if it is **structural**  
   - If yes → require **ADR** (`docs/DECISIONS/`) + domain checklists.  
2. Verify **Drone CI**  
   - If failing → request fixes.  
3. Verify **labels**  
   - Add or guide if missing (`sequence:*`, `domain:*`, `adr:*`).  
4. Link **Issue ↔ PR ↔ ADR**  
   - Update `docs/DECISIONS/index.md`.  
5. Move the card in the *Governance Flow* board.  

---

## When an RFC/Decision arrives
1. Classify by `domain:*` (Legal/Marketing/Tech/Finance).  
2. Propose a **Decision Note skeleton** (context, options, risks).  
3. Tag as `adr:required` if structural.  
4. Watch **thresholds** → alert if sequence change is needed.  

---

## Standard messages (copy-paste)

- **PR without ADR**  
  > "This PR is structural. Please add an ADR (`docs/DECISIONS/ADR-0001.md`) and tick the domain checklists before review."  

- **Multiple briefs → synthesis needed**  
  > "Several AI briefs are linked. I suggest drafting a Decision Note (context, options, multi-domain analyses, risks) for arbitration and ADR creation."  

- **Sequence alert**  
  > "We now exceed 5 decisions and >1h synthesis. Recommendation: move to **Sequence 2 (Transition)**."  
