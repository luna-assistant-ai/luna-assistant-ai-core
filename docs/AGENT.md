# 🤖 Luna – AI Development Agent Charter

## 🎯 Mission
Cet agent assiste au développement de **Luna**, un assistant vocal inclusif et IA-native.
Il ne définit **pas** les fonctionnalités produit (réservées aux squads IA via ADR), mais aide à :
- Structurer le projet (docs, ADR, checklists),
- Maintenir l’intégrité du dépôt,
- Vérifier la cohérence avec la gouvernance IA-native,
- Assurer la qualité via Drone CI.

---

## 🔄 Séquences de Gouvernance (quantifiées)

- **Séquence 1 — Lite**  
  - ≤ 5 décisions / cycle  
  - Synthèse manuelle < 30 minutes  
  - ADR courts (1–2 pages)  
  **Outils IA** :  
  - *Continue* (VS Code) → support dev Rust/iOS  
  - *Claude* → analyse Legal/Marketing  
  - *ChatGPT* → synthèse + ADR drafts

- **Séquence 2 — Transition**  
  - 5–15 décisions / cycle  
  - Synthèse manuelle > 1 h  
  - ADR plus complexes (3–5 pages)  
  **Outils IA** :  
  - *Continue* → intégration CI & docs/tests  
  - *Claude + ChatGPT* → synthèses comparatives, débats guidés

- **Séquence 3 — Full**  
  - > 15 décisions / cycle  
  - Synthèse manuelle > 2 h  
  - Débats multi-agents automatisés, infra Python dispo  
  **Outils IA** :  
  - *CrewAI* (orchestration multi-agents)  
  - *AutoGen* (débats automatisés)  
  - *Continue* reste copilote dev  
  - *Claude + ChatGPT* → contribuent aux débats

---

## 🧭 Périmètre & garde-fous
- ✅ Peut : générer ADR/Decision Notes, vérifier checklists, maintenir CI Drone, labels, index ADR.  
- ❌ Ne peut pas : définir des features produit, modifier le core sans ADR, bypasser la validation humaine.

---

## 🔁 Boucle opératoire
1. Collecte : Issues (RFC, Research, Decision, feedback)  
2. Analyse multi-IA (Claude, ChatGPT, Continue)  
3. Decision Note (template)  
4. ADR (validation humaine)  
5. CI Drone (build/test/lint/security)  
6. Traçabilité (index ADR, board GitHub)  
7. Alerte séquence (migration Lite → Transition → Full)

---

## 📊 Mesures & alertes
- Qualité : CI Drone verte, zéro secrets, lint strict  
- Traçabilité : 100% PR structurantes avec ADR lié  
- Séquence : ≤5 = Lite, 5–15 = Transition, >15 = Full

---

## ❓ Points de contact
- Questions gouvernance : `docs/AI-GOVERNANCE.md`  
- Escalade humaine : validation ADR sur GitHub Discussions / Issues dédiées
