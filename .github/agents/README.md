# Custom Agents for Math Explorer

This directory contains custom agent configurations for GitHub Copilot to assist with various aspects of the Math Explorer project.

## Available Agents

### Mathematical Implementation Architect (`math-architect.agent.md`)

**Purpose**: Expert agent for adding comprehensive mathematical implementations to the math_explorer codebase following SOLID/DRY principles, strong typing, separation of concerns, and academic rigor.

**When to Use**:
- Adding new mathematical domains or subdomains to the codebase
- Implementing new numerical algorithms or solvers
- Creating mathematical models following best practices
- Ensuring architectural consistency with existing code
- Refactoring mathematical code to follow project patterns

**Key Features**:
- Enforces separation of concerns and avoids "God Files"
- Mandates strong typing (Newtypes over primitive types)
- Requires dependency injection for testability
- Ensures deterministic behavior for stochastic processes
- Validates against academic literature
- Follows SOLID and DRY principles

**Tools Available**: `read`, `search`, `edit`, `execute`, `agent`

## How to Use Custom Agents

### For GitHub Copilot Coding Agent

Custom agents are automatically available when working on this repository through GitHub Copilot. The agent can be:

1. **Automatically Inferred**: The coding agent will automatically select the appropriate custom agent based on the task context
2. **Manually Selected**: You can explicitly request the agent by name in your prompt

Example prompts:
```
@math-architect Implement a new numerical integration method for stiff ODEs
```

```
Using the Mathematical Implementation Architect, add a Fourier transform module to pure_math
```

### For VS Code and JetBrains IDEs

These agents are configured to work in both GitHub Copilot coding agent and supported IDEs.

## Agent Design Philosophy

All custom agents in this project follow these core principles:

1. **Comprehensive Guidance**: Provide detailed instructions for every aspect of implementation
2. **Best Practices**: Enforce SOLID, DRY, and Rust-specific idioms
3. **Type Safety**: Leverage Rust's type system for correctness
4. **Testability**: Ensure all code is deterministic and testable
5. **Documentation**: Require comprehensive documentation with academic citations
6. **Architectural Consistency**: Maintain alignment with existing codebase patterns

## Contributing Custom Agents

When adding new custom agents:

1. Follow the GitHub Copilot custom agent YAML frontmatter specification
2. Include clear `name` and `description` in the frontmatter
3. Specify appropriate `tools` for the agent's purpose
4. Set `infer: true` if the agent should be automatically selected by context
5. Provide comprehensive guidance in the markdown content (max 30,000 characters)
6. Update this README with information about the new agent

## References

- [GitHub Copilot Custom Agents Documentation](https://docs.github.com/en/copilot/customizing-copilot/creating-custom-agents)
- [Math Explorer Contributing Guide](../../CONTRIBUTING.md)
- [Math Explorer Agent Instructions](../../AGENTS.md)
