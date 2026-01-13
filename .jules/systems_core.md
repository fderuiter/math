# Systems Core Engineering Log

## 2025-02-14 - Strategy Pattern for MFG Hamiltonian
**Violation:** The Mean Field Game `FixedPointSolver` had hardcoded logic for the Hamiltonian ($H(p) = p^2/2$) and drift calculation, violating OCP and making it impossible to simulate games with different kinetic energy models (e.g., relativistic or control costs).
**Refactor:** Applied the **Strategy Pattern**. Extracted a `Hamiltonian` trait with `evaluate` and `derivative` methods. Updated `FixedPointSolver` to be generic over `H: Hamiltonian`.
**Trade-off:** Minimal complexity increase (generic parameters) for significant flexibility gain. Default implementation (`QuadraticHamiltonian`) preserves backward compatibility.
