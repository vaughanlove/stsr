# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust implementation of Strongly Typed Genetic Programming (STGP) based on Montana's 1995 paper. The system generates and evolves syntax trees with strict type constraints where each node must return values compatible with its parent's type requirements.

## Commands

**Build and run:**
```bash
cargo build
cargo run
```

**Testing:**
```bash
cargo test
```

**Code formatting and linting:**
```bash
cargo fmt
cargo clippy
```

## Architecture

The codebase follows a modular design centered around typed syntax trees with the following key components:

- **Type System (`types.rs`)**: Core foundation with `DataType` (Integer, Float), `Shape` (Scalar, Vector, Matrix), and `TypeInfo` struct. Includes `VariableDefinitions` and `Dataset` for strongly-typed data handling
- **Node Structure (`node.rs`)**: Arena-based `Node` struct with type-erased values using `Box<dyn Any>`. Supports both Terminal and NonTerminal node types with parent/child indexing
- **NonTerminal Grammar (`nonterminal.rs`)**: Implements `NonTerminalRule` and `NonTerminalGrammar` for defining operations and their type constraints with user-provided computation functions
- **Operations (`ops.rs`)**: Mathematical operations (Add, Subtract, Multiply, Divide) for NonTerminal nodes
- **Tree Building (`tree_builder.rs`)**: `TreeOrchestrator` manages parse trees, datasets, and variable definitions for genetic programming evolution
- **Variable System (`variable.rs`)**: Runtime variable context and strongly-typed variable definitions

The system uses arena-based allocation where nodes reference each other by index rather than direct pointers. The `NonTerminalGrammar` allows users to define custom operations with type constraints and computation functions.

Key design principle: All nodes must satisfy type constraints - the root returns the required problem type, and each non-root node returns a type that matches its parent's argument requirements.