---
apply: always
---

# ERRORS_PROMPT: N Lang Error Handling Guidelines for AI Assistance
This file extends SYS_PROMPT.md with specific guidelines for handling errors in N Lang.

## Purpose
All errors encountered within this module should be treated as prompts for an AI agent to generate a clear, actionable remedial action plan. The goal is to maximize accessibility and provide the best possible user experience.

Errors follow a specific format to ensure consistency and clarity. Each error message should be brief, empathetic, and provide actionable next steps and suggestions for the user as well as a way to obtain more information.

## Guidelines
1. **Errors MUST BE Remedial prompts**
  - Errors are designed for formal AI assistance and Users.

2. **Brief Empathetic Communication**
   - Acknowledge the user's situation, but avoid saying "You".
   - Avoid blame or technical jargon; focus on understanding and support.

3. **Accessibility**
   - Use plain, simple language that is easy to understand for all users.
   - Ensure error messages are compatible with screen readers and assistive technologies.

4. **Actionable Remediation**
   - Every error message must include a suggested next step or solution.
   - Where possible, provide examples or links to relevant documentation.

5. **Clarity and Brevity**
   - State the error clearly and concisely.
   - Avoid unnecessary details; focus on what the user needs to know and do.

6. **Consistency**
   - Maintain a uniform style and structure for all error prompts.
   - Use consistent terminology and formatting.

7. **Positive Tone**
   - Encourage and reassure users that errors are a normal part of the process.
   - Offer constructive guidance to help users resolve issues confidently.

## Example Error Prompt

CLI Error Example:
```
Error: Invalid project name. The project name contains unsupported characters.
Next Steps: Try again, using only letters, numbers, and underscores.
Suggestions: Projct_name, Project_Name, ProjectName_123
More: https://nlang.io/errors/E_VM_22
```

Compiler Error Example:
```
Error: Invalid type annotation. Type annotation 'Int' does not match the value '15u'.
Next Steps: Make changes and try again
  - Either:
    - Change the type annotation to match the value.
    - Change the value to match the type annotation.
    - Cast the value to the type annotation.
Suggestions: 
  - Either:
    - `let x: Int = 15;`
    - `let x: Uint = 15u;`
    - `let x: Uint => Int = 15u;`
More: https://nlang.io/errors/E_CMP_0222

Sources:
/file/file_name.n.md:196:13

   196: let x: Int = 15u;
                     ^^^ Cannot assign explicitly set unsigned integer to atomic Int.
```


## Comparison to Rust's Error Handling

N Lang's error system, as currently implemented, draws significant inspiration from Rust's error handling philosophy, especially in terms of:

- **Structured, Rich Error Types:** Like Rust, N Lang uses enums and structs to encode error context, sources, and suggestions.
- **Colorful, Readable Output:** The use of ANSI color codes for error display mirrors Rust's CLI error output, improving accessibility and clarity.
- **Actionable Guidance:** Each error provides next steps, suggestions, and links for more information, going beyond Rust's default error messages by making remediation explicit and user-centric.
- **Source Tracking:** The inclusion of detailed source information (file, row, col, and error snippet) is similar to Rust's compiler diagnostics, supporting traceability and debugging.

### Key Differences

- **AI-First Remediation:** N Lang's errors are explicitly designed as prompts for AI agents to generate remedial action plans, with a strong focus on accessibility and empathy.
- **Hexagonal Architecture & Modularity:** The system enforces a stricter one-item-per-file and fully qualified path rule, promoting extreme modularity and clarity of origin.
- **Extensible Error Metadata:** Errors in N Lang are designed to be easily extended with additional context, suggestions, and links, supporting both human and AI consumers.

---

## Recommendations for Further Improvement

1. **Error Codes and Categorization**
  - Introduce a formal error code system (e.g., `E_CMP_001`) for all error variants, similar to Rust's compiler errors, to aid in documentation, searchability, and automated remediation.

2. **Localization and Internationalization**
  - Add support for localizing error messages, next steps, and suggestions to improve accessibility for non-English speakers.

3. **Rich Source Context**
  - Enhance source reporting to include code context (e.g., lines before/after the error), and consider integrating with editors for clickable diagnostics.

4. **Error Chaining and Causality**
  - Implement error chaining (e.g., `source()` methods) to allow errors to reference underlying causes, as in Rust's `std::error::Error`.

5. **Structured Machine-Readable Output**
  - Provide a way to serialize errors (e.g., to JSON or YAML) for integration with tools, editors, and AI agents.

6. **Consistent Error Construction APIs**
  - Consider builder patterns or macros for constructing complex errors, reducing boilerplate and improving ergonomics.

7. **Extensive Testing and Fuzzing**
  - Expand test coverage, including property-based and fuzz testing, to ensure robustness and catch edge cases.

8. **Documentation and Examples**
  - Continue to expand documentation and real-world usage examples, making it easier for contributors and users to understand and extend the error system.

---

## Next Steps

- Prioritize the introduction of error codes and error chaining.
- Begin designing a serialization format for errors.
- Review and refactor error construction APIs for consistency and ergonomics.
- Expand documentation with more real-world error scenarios and AI remediation examples.

---

## Revision History


## Revision History
- 2024-06-08T15:37:00Z @GitHub Copilot: Initial formal error handling prompt guidelines.

