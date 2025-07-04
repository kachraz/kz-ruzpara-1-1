1. [Aderyn Developer Guide](#aderyn-developer-guide)
   1. [Overview](#overview)
   2. [Repository Structure](#repository-structure)
      1. [Core Directories](#core-directories)
         1. [`/aderyn` - CLI Interface](#aderyn---cli-interface)
         2. [`/aderyn_core` - Analysis Engine](#aderyn_core---analysis-engine)
         3. [`/aderyn_driver` - Orchestration Layer](#aderyn_driver---orchestration-layer)
         4. [`/tools/xtask` - Build Tools](#toolsxtask---build-tools)
      2. [Test Directories](#test-directories)
         1. [`/tests` - Integration Tests](#tests---integration-tests)
         2. [`/benchmarks` - Performance Tests](#benchmarks---performance-tests)
      3. [Configuration Files](#configuration-files)
   3. [Adding New Functionality](#adding-new-functionality)
      1. [1. Adding a New Detector](#1-adding-a-new-detector)
      2. [2. Adding New CLI Commands](#2-adding-new-cli-commands)
      3. [3. Adding New Output Formats](#3-adding-new-output-formats)
      4. [4. Adding AST Analysis Features](#4-adding-ast-analysis-features)
      5. [5. Adding Configuration Options](#5-adding-configuration-options)
   4. [Development Workflow](#development-workflow)
      1. [Building](#building)
      2. [Testing](#testing)
      3. [Running Benchmarks](#running-benchmarks)
      4. [Adding Test Cases](#adding-test-cases)
   5. [Key Design Patterns](#key-design-patterns)
      1. [Detector Pattern](#detector-pattern)
      2. [Context Pattern](#context-pattern)
      3. [Visitor Pattern](#visitor-pattern)
   6. [Dependencies](#dependencies)
      1. [Core Dependencies](#core-dependencies)
      2. [Development Dependencies](#development-dependencies)
   7. [Release Process](#release-process)

# Aderyn Developer Guide

## Overview

Aderyn is a Rust-based Solidity static analyzer that detects vulnerabilities and issues in smart contracts. The project follows a modular architecture with clear separation of concerns.

## Repository Structure

### Core Directories

#### `/aderyn` - CLI Interface

**Purpose**: Main CLI application and entry point

- `src/main.rs` - CLI argument parsing and command routing
- `src/lsp.rs` - Language Server Protocol implementation
- `src/birdsong.rs` - Version checking and updates

**Modify when**: Adding new CLI commands, options, or LSP features

#### `/aderyn_core` - Analysis Engine

**Purpose**: Core analysis logic, AST processing, and detection algorithms

**Key subdirectories**:

- `src/ast/` - AST node definitions and utilities
- `src/detect/` - All vulnerability detectors
  - `detect/high/` - High severity detectors
  - `detect/low/` - Low severity detectors
  - `detect/detector.rs` - Detector trait and registry
  - `detect/entrypoint.rs` - Detection orchestration
- `src/context/` - Analysis context and data structures
  - `context/browser/` - AST navigation utilities
  - `context/graph/` - Call graph analysis
  - `context/flow/` - Control flow analysis
- `src/visitor/` - AST traversal patterns
- `src/stats/` - Code metrics and statistics

**Modify when**: Adding new detectors, AST analysis, or core functionality

#### `/aderyn_driver` - Orchestration Layer

**Purpose**: Coordinates compilation, analysis, and report generation

- `src/driver.rs` - Main orchestration logic
- `src/compile.rs` - Solidity compilation handling
- `src/config.rs` - Configuration management
- `src/process.rs` - Context creation and setup
- `src/runner.rs` - Execution modes (CLI, LSP, audit)
- `src/interface/` - Output format implementations
  - `markdown.rs` - Markdown report generation
  - `json.rs` - JSON report format
  - `sarif.rs` - SARIF format support
  - `lsp.rs` - LSP-specific reporting

**Modify when**: Changing compilation process, adding output formats, or execution modes

#### `/tools/xtask` - Build Tools

**Purpose**: Development and release automation

- `src/main.rs` - Task runner entry point
- `src/reportgen.rs` - Report generation utilities
- `src/cut_release.rs` - Release automation

**Modify when**: Adding build tasks or release processes

### Test Directories

#### `/tests` - Integration Tests

- `contract-playground/` - Test contracts for detector validation
- `adhoc-sol-files/` - Specific test cases
- `ast/` - AST parsing test fixtures

#### `/benchmarks` - Performance Tests

- Individual detector benchmarks
- Performance regression tracking

### Configuration Files

- `Cargo.toml` - Workspace configuration
- `aderyn.toml` - Project configuration template
- `rust-toolchain.toml` - Rust version specification

## Adding New Functionality

### 1. Adding a New Detector

**Files to modify**:

1. Create detector file: `aderyn_core/src/detect/high/your_detector.rs` or `aderyn_core/src/detect/low/your_detector.rs`
2. Update registry: `aderyn_core/src/detect/detector.rs`
   - Add to `get_all_issue_detectors()` function
   - Add to detector enum if needed
3. Update module exports: `aderyn_core/src/detect/high.rs` or `aderyn_core/src/detect/low.rs`
4. Add tests: `tests/contract-playground/src/YourTestContract.sol`

**Template structure**:

```rust
use crate::detect::detector::IssueDetector;

#[derive(Default)]
pub struct YourDetector {}

impl IssueDetector for YourDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // Implementation
    }

    fn name(&self) -> String { "your-detector".to_string() }
    fn description(&self) -> String { "Description".to_string() }
    fn severity(&self) -> IssueSeverity { IssueSeverity::High }
}
```

### 2. Adding New CLI Commands

**Files to modify**:

1. `aderyn/src/main.rs` - Add command to `Commands` enum and handling
2. `aderyn_driver/src/driver.rs` - Add execution logic if needed
3. `aderyn_driver/src/runner.rs` - Add new execution mode if required

### 3. Adding New Output Formats

**Files to modify**:

1. Create format handler: `aderyn_driver/src/interface/your_format.rs`
2. Update interface module: `aderyn_driver/src/interface/mod.rs`
3. Update runner: `aderyn_driver/src/runner.rs`
4. Update CLI args: `aderyn/src/main.rs`

### 4. Adding AST Analysis Features

**Files to modify**:

1. AST utilities: `aderyn_core/src/ast/` (if new node types needed)
2. Context browsers: `aderyn_core/src/context/browser/` (for navigation)
3. Visitors: `aderyn_core/src/visitor/` (for traversal patterns)

### 5. Adding Configuration Options

**Files to modify**:

1. `aderyn_driver/src/config.rs` - Configuration structure
2. `aderyn/templates/aderyn.toml` - Default configuration template
3. `aderyn/src/main.rs` - CLI argument parsing
4. `aderyn_driver/src/driver.rs` - Configuration usage

## Development Workflow

### Building

```bash
cargo build --release
```

### Testing

```bash
cargo test
```

### Running Benchmarks

```bash
cargo bench
```

### Adding Test Cases

1. Add Solidity contracts to `tests/contract-playground/src/`
2. Run tests to validate detector behavior
3. Update integration tests as needed

## Key Design Patterns

### Detector Pattern

- All detectors implement `IssueDetector` trait
- Detectors are stateless and thread-safe
- Use `WorkspaceContext` for AST access
- Return `Issue` objects with source locations

### Context Pattern

- `WorkspaceContext` provides unified access to all analysis data
- Browser utilities for AST navigation
- Graph structures for call/control flow analysis

### Visitor Pattern

- AST traversal using visitor pattern
- Extensible for new analysis types
- Parallel processing support

## Dependencies

### Core Dependencies

- `solidity_ast` - Solidity AST parsing
- `serde` - Serialization
- `rayon` - Parallel processing
- `clap` - CLI parsing
- `tower-lsp` - Language server protocol

### Development Dependencies

- `criterion` - Benchmarking
- `serial_test` - Test coordination

## Release Process

Use the xtask tool for releases:

```bash
cargo xtask cut-release
```

This handles version bumping, changelog generation, and publishing.
