## 2024-05-23 - Extract Standard Model to Module Directory
**Problem:** `math_explorer/src/physics/standard_model.rs` was a "God File" candidate, mixing Gauge Theory, Higgs Mechanism, Flavor Physics (CKM), QCD, and Neutrino Physics in a single file of ~300 lines. While not huge, the conceptual density was high, and it prevented independent evolution of these complex sub-fields.

**Decision:** Extracted the file into a directory `math_explorer/src/physics/standard_model/` with submodules:
- `gauge.rs`: Gauge couplings and Weak Mixing Angle.
- `higgs.rs`: Higgs potential and boson mass generation.
- `flavor.rs`: CKM matrix construction.
- `qcd.rs`: Running coupling and asymptotic freedom.
- `neutrinos.rs`: Oscillation probabilities.
- `mod.rs`: Re-exports and integration tests.

**Consequence:**
- **Pros:** Clear separation of concerns. Easier to navigate. Each subdomain can now grow (e.g., adding PMNS matrix to neutrinos, or 2-loop QCD) without bloating a single file.
- **Cons:** Slightly more file management overhead.
