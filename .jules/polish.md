## 2025-12-12 - Magic Numbers in Mathematical Implementations

Smell: Magic Numbers (e.g., `1e-4`, `0.02`) are rampant in mathematical/AI implementations, often representing tuning parameters, thresholds, or physical constants.
Remedy: Extract to named constants at the top of the module or struct impl to clarify their meaning (e.g., `DEFAULT_BETA_START`, `TRANSMITTANCE_THRESHOLD`). This prevents "tuning drift" and clarifies intent.
