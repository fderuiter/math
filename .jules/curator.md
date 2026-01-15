# Curator's Log - Documentation Decision Records (DDR)

## 2025-05-15 - Visualizing the Domain Ecosystem
**Gap:** The root README lists domains but lacks a visual map of how they relate or what they contain, making the project feel like a disconnected list of files.
**Strategy:** Introduce a Mermaid.js diagram in the "Features" section to visualize the project hierarchy.
**Outcome:** Users can instantly grasp the scope of the library without reading the table of contents.

## 2025-05-15 - Elevating Biology
**Gap:** The "Biology" module is a core feature but is completely absent from the "Deep Dive" sections in the READMEs, violating the "Show, Don't Tell" principle.
**Strategy:** Add a Hodgkin-Huxley neuron simulation example to both the root and crate READMEs.
**Outcome:** Demonstrates the library's capability in complex biological systems modeling.

## 2025-05-16 - Closing the "What is this?" Gap
**Gap:** `pure_math` lacked a top-level description, and `radar_gating` was a wall of text describing a pipeline.
**Strategy:** Added a summary module doc to `pure_math` and a Mermaid pipeline diagram to `radar_gating`.
**Outcome:** Users can now visualize the data flow in radar processing and understand the scope of the pure math module at a glance.

## 2025-05-17 - Demystifying the AI Black Box
**Gap:** The `ai` module documentation was a dry list of files, failing to convey the "From Scratch" educational philosophy or the relationships between submodules (e.g., how SDS relates to NeRF).
**Strategy:** Overhauled `ai/mod.rs` with a "Deep Learning & AI" header, a Mermaid ecosystem diagram, and a runnable Transformer example. Also added a process diagram to `sds/mod.rs`.
**Outcome:** Users can now visualize the AI learning path and run a Transformer model in seconds.
