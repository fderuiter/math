# Curator's Log - Documentation Decision Records (DDR)

## 2024-10-24 - Initial Documentation Audit
**Gap:** The repository has "Knowledge Rot" (broken links in README) and potential mismatches between memory/plans and actual code (Favoritism module).
**Strategy:**
1. Fix broken links in README (specifically `chaos.rs` -> `chaos/mod.rs`).
2. Verify `CONTRIBUTING.md` content against the "Golden Rule" and Mermaid diagram requirements.
3. Enhance docstrings in `lib.rs` and key modules to explain "WHY" rather than just "HOW".
**Outcome:** Reduced confusion for new users and contributors.

## 2024-10-24 - Diagramming Standard
**Gap:** Visualizing complex flows is inconsistent.
**Strategy:** All structural and flow diagrams will use Mermaid.js.
**Outcome:** Standardized visual language that renders natively in GitHub.
