## ARCHITECT'S JOURNAL - CRITICAL DECISIONS ONLY

## 2026-02-10 - Strategy Pattern for Q-Function
**Problem:** `TabularQAgent` was monolithically coupled to a `HashMap` implementation, preventing the use of other storage mechanisms (e.g., neural networks for Deep Q-Learning) or different backing stores.
**Decision:** Applied the **Strategy Pattern** by extracting a `QFunction` trait. The original `TabularQAgent` is now a type alias for `QAgent<S, A, HashMapQFunction<S, A>>`.
**Consequence:** The agent logic (learning rule, exploration) is now decoupled from data storage. New Q-function implementations (e.g., `DeepQNetwork`) can be injected without modifying the agent. The `HashMapQFunction` requires cloning state/action keys, incurring a minor performance cost for the sake of flexibility.
