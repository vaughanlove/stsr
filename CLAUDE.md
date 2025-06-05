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

The codebase follows a modular design centered around typed syntax trees:

- **Type System (`types.rs`)**: Defines `DataType` (Integer, Float) and `Shape` (Scalar, Vector, Matrix) that form the foundation of the type constraints
- **Node Structure (`node.rs`)**: Core `Node` struct with arena-based indexing, supporting both Terminal and NonTerminal node types. Uses `Box<dyn Any>` for type-erased values and implements `MatchesTerminal` trait for type safety
- **Operations (`ops.rs`)**: Mathematical operations (Add, Subtract, Multiply, Divide) for NonTerminal nodes
- **Arena Management (`arena.rs`)**: Currently commented out tree generation logic for Full and Grow methods
- **Display (`display.rs`)**: Tree visualization functionality
- **Variable Registry (`variable.rs`, `registry.rs`)**: Support for named variables and type registries

The system uses arena-based allocation where nodes reference each other by index rather than direct pointers, allowing for efficient memory management and tree manipulation operations.

Key design principle: All nodes must satisfy type constraints - the root returns the required problem type, and each non-root node returns a type that matches its parent's argument requirements.