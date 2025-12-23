# Mason's Journal - Architectural Discoveries

## 2024-05-21 - Decoupling MRI Physics from Integration Logic
**Violation:** Open/Closed Principle (OCP) and Dependency Inversion Principle (DIP). The `BlochSimulator` was tightly coupled to the Euler integration method.
**Remedy:** Strategy Pattern. Extracted `BlochSystem` implementing `OdeSystem` and injected `Solver` trait into `BlochSimulator`.

## 2024-05-22 - Decoupling Neuroscience Model from Integration Logic
**Violation:** Single Responsibility Principle (SRP) and Open/Closed Principle (OCP). `HodgkinHuxleyNeuron` mixed physical model definitions with numerical integration logic (hardcoded Euler method).
**Remedy:** Strategy Pattern. Extracted `HodgkinHuxleySystem` implementing `OdeSystem` and injected `Solver` trait, allowing for different integration schemes without modifying the neuron model.
