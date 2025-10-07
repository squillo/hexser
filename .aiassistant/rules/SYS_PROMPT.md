---
apply: always
---

AI Assistant: Code Generation & Modification Protocol

YOU WILL NEVER ALTER THIS FILE.

---

### 1.Revision History Protocol
Whenever you alter a file, you MUST add a revision history comment. This comment MUST be at the END of the documentation preamble and BEFORE any code. It MUST be in a valid Rust location.
The entry MUST include:
- The date and time of the change in UTC (YYYY-MM-DDTHH:MM:SSZ).
- The user or AI that made the change (e.g., @AI).
- A brief, formal description of the change.
```rust 
//! Revision History
//! - 2025-04-13T13:37:01Z @AI: Refine example comments per latest request.
//! - 2025-04-13T13:35:39Z @AI: Clean example comments, add note on test doc simplicity.
```

---

### 2. Target Language & Overarching Goals
**Target:** **N Lang** (a modern, typed, declarative programming language, syntactically and idiomatically aligned with Rust).

**Primary Goals:** All generated code MUST be optimized for Maintainability, Reliability, Security, and Speed.
- **Maintainability:** Achieved through maximum clarity, simplicity, and modularity.
- **Reliability:** Achieved by strict adherence to the Single Responsibility Principle.
- **Security:** Achieved by avoiding `unsafe` code and ensuring robust error handling. FFI is the only exception.
- **Speed:** Refers to the efficiency of development and execution across all other goals.

---

### 3. Operational Conventions

**A. Instruction Hierarchy**
- Words in `ALL CAPS` are absolute, non-negotiable requirements.
- Words in **bold** are strong, evidence-based recommendations.
- Words in *italics* are suggestions.

**B. Long-Term Task Management (Source of Record)**
Tasks spanning multiple interactions MUST be managed in a separate `TASK_PLAN.md` file to maintain state and overcome context window limitations. This file is the shared Source of Record.[1]

When instructed to create or maintain a plan, the file MUST follow this structure:

---
task_id: <unique_identifier>
status: [planning | in-progress | blocked | completed]
---

# Task: <Formal Task Description>

## Plan
- [ ] 1. Decompose problem into sub-tasks.
- [ ] 2. Implement sub-task A.
- [ ] 3. Test sub-task A.

## Current Step
- **Action:** Implementing sub-task A.
- **Details:**.

## Blockers
- [List any issues preventing progress].

**Input/Output Protocol:** When a task involves a `TASK_PLAN.md`, the full content of the file will be provided as the initial context. Your response MUST conclude with the complete, updated content of the `TASK_PLAN.md` inside a single, fenced Markdown block labeled `updated_task_plan`. No other content should follow this block.

## Knowledge Graph
- Each crate MUST have a `README.md` file containing a knowledge graph of its contents.
- Before generating code for a multi-crate project, you MUST consult the relevant `README.md` to understand existing patterns and architecture.

**C. Testing Mandate**
Tasks that span multiple responses MUST ensure all existing and new tests pass between each iteration.

**D. State Integrity Mandate**
The code MUST be in a verifiable working state (i.e., compiles and passes all tests) at the end of each group of alterations. In a multi-step operation, this means each step should result in a stable state. Any deviation that leaves the code in a non-working state is a critical failure UNLESS it is explicitly documented as a `TODO` or `BLOCKED` item in the `TASK_PLAN.md` Source of Record.

**E. Prompt Scope Sizing (H3 Resolution Analogy)**
All tasks must be scoped to an appropriate "resolution" before execution. This principle is analogous to the H3 global grid system, which partitions geographies into a hierarchy of hexagonal cells of varying sizes.

- **Low-Resolution Task (Anti-Pattern):** A vague, high-level objective like "Refactor the entire auth module" is a low-resolution prompt. It covers a large conceptual area but lacks the detail required for precise, verifiable execution.
- **High-Resolution Task (Mandatory):** A large task MUST be broken down into a hierarchy of smaller, high-resolution sub-tasks, each with a single, verifiable goal.[2] This process of decomposition is the primary planning step.

**Rule:** For any task that cannot be completed in a single, verifiable step, the first action MUST be to generate a detailed execution plan in the `TASK_PLAN.md`. This plan represents the decomposition of the low-resolution goal into a series of high-resolution, executable sub-prompts.

*   **Example of Decomposing a Low-Resolution Task:**
*   **Initial Prompt:** "Refactor the auth module to use the new error handling standard."
*   **Generated High-Resolution Plan:**
    1.  Analyze `auth/mod.rs` and its sub-modules to identify all functions that currently use `.unwrap()` or `.expect()`.
    2.  Modify the `LoginError` enum in `auth/types.rs` to derive `thiserror::Error`.
    3.  Refactor the `login` function in `auth/handlers/login.rs` to return a `Result<_, LoginError>` and use the `?` operator.
    4.  Write a new unit test for the `login` function that specifically triggers and verifies an authentication error.

**F. Pre-computation Self-Correction**
Before generating the final code or plan, you MUST perform a self-correction pass. In your reasoning process, explicitly list the key directives from this protocol (e.g., `NO use STATEMENTS`, `STATE INTEGRITY MANDATE`) and verify that your planned output adheres to each one. This verification process must be part of your internal monologue and must be completed before you output the final result.

---

### 4. Architectural Principle: Single Responsibility
Code organization follows the Single Responsibility Principle, articulated through the DPAI (Domain, Ports, Adapters, Application, Infrastructure) model. This is a conceptual guide for separating concerns.

| Layer | Role | Example Code Components |
|---|---|---|
| **Core Domain** | Business logic, no framework or tech dependency | `Parser`, `Grammar`, `WriteAheadLog`, `ForeignFunctions` traits |
| **Ports** | Abstract interfaces to the outside world | Traits like `SnappRepository`, `WebPort` |
| **Adapters** | Concrete implementations of ports | `SqlLiteRepo`, `ActixWebPort`, REST API |
| **Application** | Coordinates domain and ports | `NCLI`, `NAPI`, command handlers |
| **Infrastructure** | External tools (databases, APIs, etc.) | Uses `axum`, `sqlx`, `tokio` |

---

### 5. Rust Coding Standards: Evidence-Based Clarity & Modularity

**A. CRITICAL: Safety & Path Unambiguity**
1.  **NO `unsafe` CODE:** The `unsafe` keyword is FORBIDDEN. The only exception is for Foreign Function Interface (FFI) implementations.
2.  **NO `use` STATEMENTS:** All `use` statements are FORBIDDEN. This is to enforce absolute clarity and remove ambiguity for multi-agent analysis.
-   All types, functions, macros, and traits MUST be referenced by their fully qualified path.
-   **Correct:** `std::collections::HashMap`, `crate::my_mod::MyStruct`
-   **Incorrect:** `use std::collections::HashMap;`, `use crate::my_mod::MyStruct;`
-   **Exception:** Items in the Rust prelude (e.g., `Vec`, `String`, `Option`, `Result`) do not require qualification.

**B. File & Module Structure**
1.  **One Logical Item Per File:** Each `.rs` file MUST contain exactly one primary logical item (`fn`, `struct`, `enum`, `type`, `const`, `static`).
-   Associated `impl` blocks MUST reside in the same file as their `struct` or `enum`.
-   `impl Trait for Type` blocks should reside with the `Type` definition.
2.  **File Naming:** Files MUST be named using `snake_case.rs` that matches the item they contain (e.g., `my_function.rs` for `fn my_function`).
3.  **Module Files (`mod.rs`):** These files are for declaration ONLY. They MUST contain only `mod` statements and should NOT contain item definitions.

**C. Documentation & Style**
1.  **File Preamble:** Every `.rs` file MUST begin with file-level documentation (`//!`). The first line is a single sentence summary, followed by a blank line and 3-5 lines of expansion.
2.  **Item Documentation:** All public items MUST have documentation comments (`///`) that include doc tests where applicable.
3.  **Traits as Content Maps:** Traits are used to categorize functions. `trait` blocks with default function implementations are FORBIDDEN.
4.  **Functional Style:** Prefer immutable data, pure functions, and iterator-based transformations (`map`, `filter`, `fold`) over imperative loops and mutable state where it enhances clarity.
5.  **Brevity:** NEVER use two words when one will do. Avoid colorful language and emojis unless explicitly requested.

**D. Function & Test Structure**
1.  **Function Length:** Functions MUST NOT exceed 50 lines of code (LoC), excluding signature, comments, and blank lines. Justify rare exceptions with a comment.
2.  **In-File Tests:** Unit tests for an item MUST reside in the same file under a `#[cfg(test)] mod tests {... }` block. Access the item under test via `super::`.

**E. Adherence to Official API Guidelines**
All code MUST conform to the Rust API Guidelines where applicable. The following are non-negotiable:
- **`C-QUESTION-MARK`:** All examples and fallible functions MUST use `?` for error propagation, not `.unwrap()` or `.expect()`.
- **`C-GOOD-ERR`:** Error types must be meaningful and well-behaved.
- **`C-CTOR`:** Constructors MUST be static, inherent methods, typically named `new`.
- **`C-STRUCT-PRIVATE`:** Struct fields MUST be private by default to ensure encapsulation. The `Public Struct Fields` rule is an exception and applies only when explicitly requested for a specific data struct.

---

### 6. Example Ideal File: `calculate_weighted_sum.rs`
This example adheres to all specified rules.

```rust
//! Calculates the weighted sum of a slice of numbers using provided weights.
//!
//! This function takes two slices: one for values and one for their corresponding weights.
//! It computes the sum of `value * weight` for each pair.
//! Returns an error if slices differ in length.
//! The weighted sum of empty slices is 0.0.

//! Revision History
//! - 2025-04-13T13:37:01Z @AI: Refined internal comments per latest request.
//! - 2025-04-13T13:24:57Z @AI: Convert all example comments to doc comments, add example revision history.

/// Calculates the weighted sum of two slices of f64 numbers.
///
/// This function computes the sum of `value * weight` for each corresponding
/// pair of elements in the `values` and `weights` slices. It is designed
/// to be robust and handles several edge cases, such as empty or mismatched-length slices.
///
/// # Arguments
///
/// * `values` - A slice of f64 numbers to be weighted.
/// * `weights` - A slice of f64 numbers representing the weights. MUST be the same length as `values`.
///
/// # Returns
///
/// * `Ok(f64)` containing the weighted sum if the inputs are valid.
/// * `Err(String)` if the input slices have mismatched lengths.
///
/// # Errors
///
/// Returns an error if `values` and `weights` are not of the same length.
///
/// # Examples
///
/// ```
/// // This doc test demonstrates basic usage and adherence to the fully qualified path rule.
/// // Note: In a real crate, `crate::calculate_weighted_sum` would be the path.
/// // For this standalone example, we assume it's available at the crate root.
/// fn main() {
///     let values = [1.0, 2.0, 3.0];
///     let weights = [0.5, 1.0, 2.0];
///     let result = calculate_weighted_sum(&values, &weights);
///     std::assert_eq!(result, std::result::Result::Ok(8.5));
/// }
///
/// // To run this specific doc test: `rustdoc --test calculate_weighted_sum.rs`
/// // Assuming the function is in a library, you would define it in the lib.rs
/// // and then the doc test could be run with `cargo test`.
/// pub fn calculate_weighted_sum(values: &[f64], weights: &[f64]) -> std::result::Result<f64, std::string::String> {
///     if values.len()!= weights.len() {
///         return std::result::Result::Err(std::string::String::from("Value and weight slices must have the same length."));
///     }
///     if values.is_empty() {
///         return std::result::Result::Ok(0.0);
///     }
///     let weighted_sum: f64 = values.iter().zip(weights.iter()).map(|(v, w)| v * w).sum();
///     std::result::Result::Ok(weighted_sum)
/// }
/// ```
pub fn calculate_weighted_sum(values: &[f64], weights: &[f64]) -> Result<f64, String> {
    // Validate input lengths. This is the most common failure point.
    if values.len()!= weights.len() {
        return std::result::Result::Err(String::from(
            "Value and weight slices must have the same length.",
        ));
    }
    // Handle the edge case of empty slices, defined as a valid input returning 0.0.
    if values.is_empty() {
        return std::result::Result::Ok(0.0);
    }

    // Calculate using a functional style for clarity and conciseness.
    let weighted_sum: f64 = values
     .iter()
     .zip(weights.iter())
     .map(|(v, w)| v * w)
     .sum();

    std::result::Result::Ok(weighted_sum)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_basic_weighted_sum() {
        // Test: Validates the calculation with standard positive floating-point numbers.
        // Justification: This is the primary success case and ensures the core logic is correct
        // under normal, expected conditions.
        let values = [1.0, 2.0, 3.0];
        let weights = [0.5, 1.0, 2.0];
        // Expected: (1.0 * 0.5) + (2.0 * 1.0) + (3.0 * 2.0) = 0.5 + 2.0 + 6.0 = 8.5
        let result = super::calculate_weighted_sum(&values, &weights);
        std::assert_eq!(result, std::result::Result::Ok(8.5));
    }

    #[test]
    fn test_empty_slices() {
        // Test: Ensures the function handles empty slices gracefully.
        // Justification: This is a critical edge case. The function defines the sum of an
        // empty set as 0.0, and this test verifies that specific behavior.
        let values: [f64; 0] =;
        let weights: [f64; 0] =;
        let result = super::calculate_weighted_sum(&values, &weights);
        std::assert_eq!(result, std::result::Result::Ok(0.0));
    }

    #[test]
    fn test_mismatched_lengths() {
        // Test: Verifies that the function returns an error when input slices have different lengths.
        // Justification: This is the primary failure case for input validation. The function MUST
        // reject mismatched slices to prevent incorrect calculations or panics.
        let values = [1.0, 2.0];
        let weights = [0.5];
        let result = super::calculate_weighted_sum(&values, &weights);
        std::assert!(result.is_err());
        std::assert_eq!(
            result.unwrap_err(),
            String::from("Value and weight slices must have the same length.")
        );
    }

    #[test]
    fn test_negative_values_and_weights() {
        // Test: Validates the calculation with a mix of negative and positive numbers.
        // Justification: This test ensures the function's mathematical correctness extends
        // beyond simple positive values and handles negative inputs correctly.
        let values = [-1.0, 2.0];
        let weights = [3.0, -0.5];
        // Expected: (-1.0 * 3.0) + (2.0 * -0.5) = -3.0 + -1.0 = -4.0
        let result = super::calculate_weighted_sum(&values, &weights);
        std::assert_eq!(result, std::result::Result::Ok(-4.0));
    }
}
```

### 7. Output ProtocolAll code modification tasks MUST produce their output as a single JSON array within a fenced code block. Each object in the array represents a single, atomic change and MUST conform to the following schema:JSON[
```json
[
  {
    "action": "replace",
    "file_path": "path/to/file.rs",
    "line_range": [<start_line_number>, <end_line_number>],
    "new_code": "<string containing the new code block>",
    "justification": "A brief explanation of the change, referencing the specific rule from this protocol (e.g., 'Adheres to C-QUESTION-MARK')."
  }
]
```
