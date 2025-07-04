1. [Adding New Detectors to Aderyn](#adding-new-detectors-to-aderyn)
   1. [Quick Start Checklist](#quick-start-checklist)
   2. [Step 1: Create the Detector File](#step-1-create-the-detector-file)
   3. [Step 2: Implement the Detector](#step-2-implement-the-detector)
   4. [Step 3: Register the Detector](#step-3-register-the-detector)
      1. [3.1 Add to Detector Name Enum](#31-add-to-detector-name-enum)
      2. [3.2 Add to Detector Registry](#32-add-to-detector-registry)
      3. [3.3 Export from Module](#33-export-from-module)
   5. [Step 4: Create Test Contract](#step-4-create-test-contract)
   6. [Step 5: Common Detection Patterns](#step-5-common-detection-patterns)
      1. [Iterating Through AST Nodes](#iterating-through-ast-nodes)
      2. [Using Context Browsers](#using-context-browsers)
      3. [Pattern Matching on Expressions](#pattern-matching-on-expressions)
      4. [Using Helper Functions](#using-helper-functions)
   7. [Step 6: Real-World Examples](#step-6-real-world-examples)
      1. [Example 1: Empty Block Detector (Simple)](#example-1-empty-block-detector-simple)
      2. [Example 2: Arbitrary Transfer From Detector (Complex)](#example-2-arbitrary-transfer-from-detector-complex)
   8. [Step 7: Testing and Validation](#step-7-testing-and-validation)
      1. [Run Specific Tests](#run-specific-tests)
      2. [Test Against Playground](#test-against-playground)
      3. [Validate Output](#validate-output)
   9. [Common Gotchas and Best Practices](#common-gotchas-and-best-practices)
      1. [❌ Don't Do This](#-dont-do-this)
      2. [✅ Do This Instead](#-do-this-instead)
      3. [Best Practices](#best-practices)
   10. [Debugging Tips](#debugging-tips)
       1. [Print Debug Information](#print-debug-information)
       2. [Use Context Information](#use-context-information)
       3. [Test Incrementally](#test-incrementally)
   11. [Performance Considerations](#performance-considerations)
   12. [Documentation](#documentation)
   13. [Conclusion](#conclusion)

# Adding New Detectors to Aderyn

This comprehensive guide walks you through creating new vulnerability detectors for Aderyn.

## Quick Start Checklist

1. ✅ Create detector file in `aderyn_core/src/detect/high/` or `aderyn_core/src/detect/low/`
2. ✅ Add detector to registry in `aderyn_core/src/detect/detector.rs`
3. ✅ Export detector in `aderyn_core/src/detect/high.rs` or `low.rs`
4. ✅ Create test contract in `tests/contract-playground/src/`
5. ✅ Write unit tests
6. ✅ Test and validate

## Step 1: Create the Detector File

Choose severity level and create the detector file:

- **High severity**: `aderyn_core/src/detect/high/your_detector_name.rs`
- **Low severity**: `aderyn_core/src/detect/low/your_detector_name.rs`

Use the provided templates as starting points:

- `aderyn_core/src/detect/high/_template.rs`
- `aderyn_core/src/detect/low/_template.rs`

## Step 2: Implement the Detector

```rust
use std::{collections::BTreeMap, error::Error};
use crate::{
    ast::{NodeID, FunctionDefinition, Expression, MemberAccess},
    capture,
    context::workspace::WorkspaceContext,
    detect::detector::{IssueDetector, IssueDetectorNamePool, IssueSeverity},
};
use eyre::Result;

#[derive(Default)]
pub struct YourDetector {
    // Store found instances - DO NOT modify manually, use capture! macro
    found_instances: BTreeMap<(String, usize, String), NodeID>,
    hints: BTreeMap<(String, usize, String), String>,
}

impl IssueDetector for YourDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // Example: Iterate through all functions
        for function in context.function_definitions() {
            // Your detection logic here
            if is_vulnerable(function) {
                // Capture the problematic node
                capture!(self, context, function);
                // Or with a hint
                capture!(self, context, function, "This function is vulnerable because...");
            }
        }

        // Return true if any instances were found
        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High // or IssueSeverity::Low
    }

    fn title(&self) -> String {
        String::from("Your Vulnerability Title")
    }

    fn description(&self) -> String {
        String::from("Detailed description of the vulnerability and its impact.")
    }

    fn instances(&self) -> BTreeMap<(String, usize, String), NodeID> {
        self.found_instances.clone()
    }

    fn hints(&self) -> BTreeMap<(String, usize, String), String> {
        self.hints.clone()
    }

    fn name(&self) -> String {
        format!("{}", IssueDetectorNamePool::YourDetectorName)
    }
}

// Helper function for detection logic
fn is_vulnerable(function: &FunctionDefinition) -> bool {
    // Your vulnerability detection logic
    false
}

#[cfg(test)]
mod your_detector_tests {
    use super::*;
    use crate::detect::{detector::IssueDetector, test_utils::load_solidity_source_unit};

    #[test]
    fn test_your_detector() {
        let context = load_solidity_source_unit(
            "../tests/contract-playground/src/YourTestContract.sol"
        );

        let mut detector = YourDetector::default();
        let found = detector.detect(&context).unwrap();
        assert!(found);
        assert_eq!(detector.instances().len(), 2); // Expected number of issues
    }
}
```

## Step 3: Register the Detector

### 3.1 Add to Detector Name Enum

In `aderyn_core/src/detect/detector.rs`, add your detector to the `IssueDetectorNamePool` enum:

```rust
#[derive(Debug, PartialEq, EnumString, Display)]
#[strum(serialize_all = "kebab-case")]
pub enum IssueDetectorNamePool {
    // ... existing detectors
    YourDetectorName,  // This will become "your-detector-name" in kebab-case
}
```

### 3.2 Add to Detector Registry

In the same file, add your detector to the `get_all_issue_detectors()` function:

```rust
pub fn get_all_issue_detectors() -> Vec<Box<dyn IssueDetector>> {
    vec![
        // ... existing detectors
        Box::<YourDetector>::default(),
    ]
}
```

### 3.3 Export from Module

In `aderyn_core/src/detect/high.rs` or `aderyn_core/src/detect/low.rs`:

```rust
// Add module declaration
pub(crate) mod your_detector_name;

// Add public export
pub use your_detector_name::YourDetector;
```

## Step 4: Create Test Contract

Create a test contract in `tests/contract-playground/src/YourTestContract.sol`:

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract YourTestContract {
    // Good example - should NOT be flagged
    function goodFunction() external {
        // Safe implementation
    }

    // Bad example - SHOULD be flagged
    function badFunction() external {
        // Vulnerable implementation
    }

    // Edge case example
    function edgeCaseFunction() external {
        // Test edge cases
    }
}
```

## Step 5: Common Detection Patterns

### Iterating Through AST Nodes

```rust
// Functions
for function in context.function_definitions() {
    // Process each function
}

// Contracts
for contract in context.contract_definitions() {
    // Process each contract
}

// Function calls
for call in context.function_calls() {
    // Process each function call
}

// Variables
for var in context.variable_declarations() {
    // Process each variable declaration
}

// Blocks
for block in context.blocks() {
    // Process each block
}
```

### Using Context Browsers

```rust
use crate::context::browser::{
    ExtractFunctionCalls,
    GetClosestAncestorOfTypeX,
    GetImmediateChildren
};

// Extract all function calls from a function
let calls = ExtractFunctionCalls::from(function).extracted;

// Find closest ancestor of specific type
if let Some(contract) = function.closest_ancestor_of_type(context, NodeType::ContractDefinition) {
    // Process contract
}

// Get immediate children
let children = function.immediate_children(context);
```

### Pattern Matching on Expressions

```rust
use crate::ast::{Expression, MemberAccess, FunctionCall, Identifier};

// Match function calls
if let Expression::FunctionCall(func_call) = &some_expression {
    if let Expression::MemberAccess(member_access) = &*func_call.expression {
        if member_access.member_name == "transferFrom" {
            // Found transferFrom call
        }
    }
}

// Match identifiers
if let Expression::Identifier(Identifier { name, .. }) = &some_expression {
    if name == "msg" {
        // Found msg usage
    }
}
```

### Using Helper Functions

```rust
use crate::detect::helpers::{
    get_implemented_external_and_public_functions,
    has_msg_sender_binary_operation,
};

// Get only external and public functions
let public_functions = get_implemented_external_and_public_functions(context);

// Check for msg.sender usage
if has_msg_sender_binary_operation(&function.into()) {
    // Function uses msg.sender
}
```

## Step 6: Real-World Examples

### Example 1: Empty Block Detector (Simple)

```rust
fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
    for empty_block in context.blocks().iter().filter(|b| b.statements.is_empty()) {
        if let Some(ASTNode::FunctionDefinition(f)) =
            &empty_block.closest_ancestor_of_type(context, NodeType::FunctionDefinition)
        {
            if *f.kind() == FunctionKind::Function {
                capture!(self, context, f);
            }
        }
    }
    Ok(!self.found_instances.is_empty())
}
```

### Example 2: Arbitrary Transfer From Detector (Complex)

```rust
fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
    let suspected_functions = get_implemented_external_and_public_functions(context)
        .filter(|function_definition| {
            !has_msg_sender_binary_operation(&((*function_definition).into()))
                && function_definition.modifiers.is_empty()
        });

    for func in suspected_functions {
        let func_parameters_ids = &func.parameters.parameters
            .iter()
            .map(|f| f.id)
            .collect::<Vec<_>>();

        let transfer_func_calls = ExtractFunctionCalls::from(func)
            .extracted
            .into_iter()
            .filter(|function_call| {
                if let Expression::MemberAccess(member_access) = &*function_call.expression {
                    member_access.member_name == "transferFrom"
                        || member_access.member_name == "safeTransferFrom"
                } else {
                    false
                }
            })
            .collect::<Vec<_>>();

        for func_call in transfer_func_calls {
            // Check if first argument comes from function parameters
            let arg_index = if func_call.arguments.len() == 3 { 0 } else { 1 };

            if let Some(arg) = func_call.arguments.get(arg_index) {
                if let Expression::Identifier(Identifier { referenced_declaration: Some(ref_id), .. }) = arg {
                    if func_parameters_ids.iter().any(|r| r == ref_id) {
                        capture!(self, context, func_call);
                    }
                }
            }
        }
    }

    Ok(!self.found_instances.is_empty())
}
```

## Step 7: Testing and Validation

### Run Specific Tests

```bash
# Run your detector tests
cargo test your_detector_tests

# Run all detector tests
cargo test -p aderyn_core
```

### Test Against Playground

```bash
# Test against all playground contracts
cargo run -- ../tests/contract-playground/

# Test with specific output
cargo run -- ../tests/contract-playground/ --output test_output.md
```

### Validate Output

```bash
# Check that your detector appears in the list
cargo run -- --detectors

# Run only your detector
cargo run -- ../tests/contract-playground/ --include your-detector-name
```

## Common Gotchas and Best Practices

### ❌ Don't Do This

```rust
// DON'T manually modify found_instances
self.found_instances.insert(key, node_id);

// DON'T use unwrap() in detection logic
let result = some_operation().unwrap();

// DON'T ignore edge cases
if function.name == "transfer" {
    capture!(self, context, function);
}
```

### ✅ Do This Instead

```rust
// DO use the capture! macro
capture!(self, context, function);
capture!(self, context, function, "helpful hint");

// DO handle errors gracefully
if let Some(result) = some_operation() {
    // Process result
}

// DO consider edge cases
if function.name == "transfer" || function.name == "transferFrom" {
    // Additional checks for safety
    if !has_proper_access_control(function) {
        capture!(self, context, function);
    }
}
```

### Best Practices

1. **Use descriptive names**: `ArbitraryTransferFromDetector` not `TransferDetector`
2. **Add helpful hints**: Use the second parameter of `capture!` for context
3. **Test edge cases**: Include inheritance, modifiers, complex expressions
4. **Consider false positives**: Add checks to reduce noise
5. **Document your logic**: Add comments explaining the vulnerability pattern
6. **Use existing helpers**: Check `aderyn_core/src/detect/helpers.rs` for utilities

## Debugging Tips

### Print Debug Information

```rust
// Add debug prints (remove before committing)
eprintln!("Processing function: {}", function.name);
eprintln!("Found {} calls", calls.len());
```

### Use Context Information

```rust
// Get source location for debugging
let location = context.get_node_sort_key(&function.into());
eprintln!("Function at {}:{}:{}", location.0, location.1, location.2);
```

### Test Incrementally

1. Start with a simple case that should be detected
2. Add complexity gradually
3. Test both positive and negative cases
4. Verify the exact number of expected detections

## Performance Considerations

- Use iterators efficiently (`.filter().map()` chains)
- Avoid nested loops when possible
- Cache expensive computations
- Consider using `rayon` for parallel processing if needed

## Documentation

Add clear documentation to your detector:

```rust
/// Detects functions that allow arbitrary `from` addresses in `transferFrom` calls.
///
/// This vulnerability occurs when:
/// 1. A public/external function accepts a `from` parameter
/// 2. The function calls `transferFrom` or `safeTransferFrom` with that parameter
/// 3. The function doesn't verify `msg.sender` authorization
///
/// This can lead to unauthorized token transfers if users have granted approvals.
pub struct ArbitraryTransferFromDetector {
    // ...
}
```

## Conclusion

Creating effective detectors requires understanding both Solidity vulnerabilities and Aderyn's AST analysis capabilities. Start simple, test thoroughly, and iterate based on real-world contract analysis.

For questions or help, check existing detectors in `aderyn_core/src/detect/` for patterns and inspiration.
