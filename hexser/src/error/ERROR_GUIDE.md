# Hexser Error System Guide

Complete guide to using the Hexser error handling system for building robust, maintainable applications with rich, actionable error information.

## Table of Contents

1. [Quick Start](#quick-start)
2. [Error Types](#error-types)
3. [Error Construction](#error-construction)
4. [Error Chaining](#error-chaining)
5. [Best Practices](#best-practices)
6. [Error Codes Reference](#error-codes-reference)
7. [Examples](#examples)

---

## Quick Start

### Basic Error Creation

Creating rich, actionable errors is straightforward using the builder pattern:

```rust
use hexser::error::{Hexserror, codes};

// Domain error with next steps
let err = Hexserror::domain(codes::domain::INVARIANT_VIOLATION, "Order must contain items")
    .with_next_step("Add at least one item to the order")
    .with_suggestion("order.add_item(item)");

// Validation error with field context
let err = Hexserror::validation_field("Email format is invalid", "email")
    .with_suggestion("Use format: user@example.com");

// Not found error
let err = Hexserror::not_found("User", "user-123");

```

## Error Display
All errors implement std::fmt::Display with structured, human-readable output:

```text 
Error [E_HEX_002]: Order must contain items
Next Steps: Add at least one item to the order
Suggestions: order.add_item(item)
Source: src/domain/order.rs:45:12
```


## Error Types

The Hexser error system provides specialized error types for different architectural layers and failure scenarios.

Hexserror (Main Error Type)

Hexserror is the primary error enum that wraps all layer-specific errors:

```rust
pub enum Hexserror {
  Domain(DomainError),
  Port(PortError),
  Adapter(AdapterError),
  Validation(ValidationError),
  NotFound(NotFoundError),
  Conflict(ConflictError),
}
```

## Layer-Specific Errors

### DomainError

Represents business rule violations and invariant failures in the domain layer:

```rust
use hexser::error::{Hexserror, codes};

let err = Hexserror::domain(
    codes::domain::INVARIANT_VIOLATION,
    "Account balance cannot be negative"
)
.with_next_step("Ensure sufficient funds before withdrawal")
.with_suggestion("Check account.balance >= amount");
```

Common Use Cases:
- Aggregate invariant violations
- Business rule failures
- Invalid state transitions
- PortError

- Represents failures in port interfaces between domain and adapters:

```rust
use hexser::error::port_error;

let err = port_error::communication_failure("Database connection lost");
let err = port_error::port_timeout("UserRepository");
let err = port_error::port_not_found("PaymentGateway");
```

Common Use Cases:
- Port communication failures
- Missing port implementations
- Port operation timeouts
- AdapterError
Represents infrastructure and external service failures:

```rust
use hexser::error::adapter_error;

let err = adapter_error::connection_failed("PostgreSQL", "Connection refused");
let err = adapter_error::api_failure("Payment API returned 503");
let err = adapter_error::mapping_failure("Cannot map DTO to entity");
```

Common Use Cases:
- Database connection failures
- External API errors
- Data mapping issues
- File I/O failures
- ValidationError

Represents input validation failures with field-specific context:

```rust
use hexser::error::{Hexserror, codes};

let err = Hexserror::validation("Required field missing")
.with_field("username");

let err = Hexserror::validation_field("Must be at least 8 characters", "password")
.with_suggestion("Use a longer password");
```

Common Use Cases:
- Required field validation
- Format validation (email, phone, etc.)
- Range validation
- Business constraint validation
- NotFoundError 

Represents missing resource errors:

```rust
use hexser::error::Hexserror;

let err = Hexserror::not_found("Order", "order-123");
let err = Hexserror::not_found("User", "user@example.com");
```

Common Use Cases:
- Entity not found by ID
- Resource lookup failures
- Missing configuration
- ConflictError

Represents resource state conflicts:

```rust
use hexser::error::Hexserror;

let err = Hexserror::conflict("Email already registered")
.with_existing_id("user-456")
.with_next_step("Use a different email or recover existing account");
```

Common Use Cases:
- Duplicate resource creation
- Concurrent modification conflicts
- State transition conflicts
- Error Construction
- Builder Pattern

All layer errors support the builder pattern for incrementally adding context:

```rust
use hexser::error::{Hexserror, codes};

let err = Hexserror::domain(codes::domain::INVARIANT_VIOLATION, "Invalid order state")
    .with_next_step("Verify order is in draft state")
    .with_next_steps(&["Check order.status", "Ensure no items are shipped"])
    .with_suggestion("Call order.cancel() before modifying items")
    .with_suggestions(&["Use order.is_modifiable()", "Check order lifecycle"]);
```

### Helper Functions
The error system provides helper functions for common error scenarios:

```rust
use hexser::error::{domain_error, port_error, adapter_error};

// Domain helpers
let err = domain_error::invariant_violation("Order must have items");
let err = domain_error::invalid_state_transition("Cannot ship cancelled order");
let err = domain_error::invariant_empty("Cart is empty");

// Port helpers
let err = port_error::communication_failure("Network timeout");
let err = port_error::port_timeout("PaymentService");
let err = port_error::port_not_found("EmailService");

// Adapter helpers
let err = adapter_error::connection_failed("Redis", "Connection refused");
let err = adapter_error::api_failure("Payment API unavailable");
let err = adapter_error::mapping_failure("Invalid JSON structure");
let err = adapter_error::io_failure("Cannot read configuration file");
```

### Source Location Tracking
Add source location information for precise error tracking:

```rust
use hexser::error::{Hexserror, source_location::SourceLocation};

let location = SourceLocation::new("src/domain/order.rs", 45, 12);
let err = Hexserror::domain("E_HEX_001", "Invalid state")
    .with_location(location);
```

Documentation Links
Link errors to documentation for detailed remediation:

```rust
use hexser::error::{Hexserror, RichError};

let err = Hexserror::domain("E_HEX_002", "Invariant violation")
.with_more_info("https://docs.example.com/errors/E_HEX_002");
```

 
### Error Chaining
Error chaining allows tracking the full causal chain of failures.

Adding Source Errors
Use with_source() to chain underlying errors:

```rust
use hexser::error::{Hexserror, adapter_error};
use std::io;

fn load_config() -> Result<Config, Hexserror> {
    let content = std::fs::read_to_string("config.toml")
        .map_err(|io_err| {
            adapter_error::io_failure("Failed to read config file")
                .with_source(io_err)
        })?;
    
    parse_config(&content)
}
```

Multi-Layer Error Propagation
Chain errors across architectural layers:

```rust
use hexser::error::{Hexserror, domain_error, adapter_error};

// Adapter layer
fn fetch_user_data(id: &str) -> Result<UserData, Hexserror> {
  let response = api_client.get(id)
    .map_err(|api_err| {
    adapter_error::api_failure("User service unavailable")
    .with_source(api_err)
    })?;

  Ok(response)
}

// Domain layer
fn load_user(id: &str) -> Result<User, Hexserror> {
  let data = fetch_user_data(id)
  .map_err(|fetch_err| {
    domain_error::invariant_violation("Cannot load user")
    .with_source(fetch_err)
    })?;

  User::from_data(data)
}
```

### Accessing Error Sources
Retrieve the source error chain using `std::error::Error::source()`:

```rust
use std::error::Error;

fn print_error_chain(err: &dyn Error) {
    eprintln!("Error: {}", err);
    let mut source = err.source();
    while let Some(err) = source {
        eprintln!("Caused by: {}", err);
        source = err.source();
    }
}
```


## Best Practices
1. Choose the Right Error Type
   Use domain errors for business logic violations, port errors for interface issues, and adapter errors for infrastructure failures.
   
Good:

```rust
// Domain layer - business rule violation
if order.items.is_empty() {
  return Err(domain_error::invariant_empty("Order must have items"));
}

// Adapter layer - infrastructure failure
let conn = pool.get().map_err(|e| {
  adapter_error::connection_failed("PostgreSQL", e.to_string())
})?;
```

Bad:

```rust
// Don't use generic error for domain violations
if order.items.is_empty() {
    return Err(Hexserror::validation("Order is empty")); // Wrong layer!
}
```

2. Provide Actionable Guidance
   Always include next steps and concrete suggestions:

Good:

```rust
   Hexserror::validation_field("Password too short", "password")
   .with_next_step("Use a password with at least 8 characters")
   .with_suggestion("Try: MyP@ssw0rd2024")
```

Bad:

```rust
Hexserror::validation("Invalid password") // No actionable guidance
```

3. Add Context at Each Layer
   Enrich errors with layer-specific context as they propagate:

```rust
// Adapter layer - raw error
let data = fetch_from_api(id)
  .map_err(|e| adapter_error::api_failure("API request failed").with_source(e))?;

// Domain layer - add business context
let user = User::from_data(data)
  .map_err(|e| domain_error::invariant_violation("Invalid user data").with_source(e))?;

// Application layer - add user-facing context
user.validate()
  .map_err(|e| e.with_next_step("Contact support if problem persists"))?;
```

4. Use Helper Functions
Prefer helper functions over manual error construction:

Good:

```rust
port_error::port_timeout("PaymentService")
```

Bad:

```rust
Hexserror::Port(PortError::new(codes::port::PORT_TIMEOUT, "Port timed out"))
.with_next_step("Increase timeout or check port responsiveness")
```

5. Handle Validation Early
Validate input at system boundaries before processing:

```rust
pub fn create_user(req: CreateUserRequest) -> Result<User, Hexserror> {
    // Validate early
    if req.email.is_empty() {
        return Err(Hexserror::validation_field("Email is required", "email"));
    }
    if !is_valid_email(&req.email) {
        return Err(Hexserror::validation_field("Invalid email format", "email")
            .with_suggestion("Use format: user@example.com"));
    }
    
    // Continue with business logic
    let user = User::new(req.email, req.name)?;
    Ok(user)
}
```

6. Test Error Scenarios
   Write tests for error cases and verify error messages:
 
```rust
   #[test]
   fn test_order_empty_invariant() {
   let order = Order::new();
   let result = order.submit();

   assert!(result.is_err());
   let err = result.unwrap_err();
   assert!(matches!(err, Hexserror::Domain(_)));
   assert!(err.to_string().contains("E_HEX_001"));
   }
```


## **Error Codes Reference**

### **Domain Layer (E\_HEX\_001 \- E\_HEX\_099)**

| Code | Constant | Description | Resolution |
| :---- | :---- | :---- | :---- |
| E\_HEX\_001 | domain::INVARIANT\_EMPTY | Container must contain items | Add items before saving |
| E\_HEX\_002 | domain::INVARIANT\_VIOLATION | Aggregate invariant violated | Check business rules |
| E\_HEX\_003 | domain::INVALID\_STATE\_TRANSITION | Invalid state change | Verify entity lifecycle |

### **Port Layer (E\_HEX\_100 \- E\_HEX\_199)**

| Code | Constant | Description | Resolution |
| :---- | :---- | :---- | :---- |
| E\_HEX\_100 | port::COMMUNICATION\_FAILURE | Port communication failed | Check connectivity |
| E\_HEX\_101 | port::PORT\_NOT\_FOUND | Required port unavailable | Register port properly |
| E\_HEX\_102 | port::PORT\_TIMEOUT | Port operation timed out | Increase timeout setting |

### **Adapter Layer (E\_HEX\_200 \- E\_HEX\_299)**

| Code | Constant | Description | Resolution |
| :---- | :---- | :---- | :---- |
| E\_HEX\_200 | adapter::DB\_CONNECTION\_FAILURE | Database connection failed | Check configuration |
| E\_HEX\_201 | adapter::API\_FAILURE | External API failed | Verify endpoint and credentials |
| E\_HEX\_202 | adapter::MAPPING\_FAILURE | Data mapping failed | Check structure compatibility |

### **Validation Layer (E\_HEX\_300 \- E\_HEX\_399)**

| Code | Constant | Description | Resolution |
| :---- | :---- | :---- | :---- |
| E\_HEX\_300 | validation::REQUIRED\_FIELD | Required field missing | Provide all required fields |
| E\_HEX\_301 | validation::INVALID\_FORMAT | Invalid field format | Check format requirements |
| E\_HEX\_302 | validation::OUT\_OF\_RANGE | Value out of range | Use value within valid range |

### **Resource Layer (E\_HEX\_400 \- E\_HEX\_499)**

| Code | Constant | Description | Resolution |
| :---- | :---- | :---- | :---- |
| E\_HEX\_400 | resource::NOT\_FOUND | Resource not found | Verify resource ID |
| E\_HEX\_401 | resource::ALREADY\_EXISTS | Duplicate resource | Use update or different ID |
| E\_HEX\_402 | resource::CONFLICT | Resource state conflict | Resolve conflict or retry |

### **IO Layer (E\_HEX\_500 \- E\_HEX\_599)**

| Code | Constant | Description | Resolution |
| :---- | :---- | :---- | :---- |
| E\_HEX\_500 | io::FILE\_NOT\_FOUND | File not found | Verify file path |
| E\_HEX\_501 | io::PERMISSION\_DENIED | Permission denied | Check file permissions |
| E\_HEX\_502 | io::IO\_FAILURE | IO operation failed | Check system resources |

## **Examples**
### Example 1: Domain Validation

```rust
use hexser::error::{Hexserror, codes};

pub struct Order {
    items: Vec<OrderItem>,
    status: OrderStatus,
}

impl Order {
    pub fn submit(&self) -> Result<(), Hexserror> {
        // Validate invariants
        if self.items.is_empty() {
            return Err(
                Hexserror::domain(codes::domain::INVARIANT_EMPTY, "Order must contain items")
                    .with_next_step("Add at least one item to the order")
                    .with_suggestion("order.add_item(item)")
            );
        }
        
        if self.status != OrderStatus::Draft {
            return Err(
                Hexserror::domain(
                    codes::domain::INVALID_STATE_TRANSITION,
                    "Can only submit orders in Draft status"
                )
                .with_next_step("Check order status before submitting")
                .with_suggestion("order.status == OrderStatus::Draft")
            );
        }
        
        Ok(())
    }
}
```

### Example 2: Adapter Error with Chaining

```rust
use hexser::error::{Hexserror, adapter_error};

pub struct PostgresUserRepository {
pool: sqlx::PgPool,
}

impl PostgresUserRepository {
pub async fn find_by_id(&self, id: &str) -> Result<User, Hexserror> {
let row = sqlx::query_as::<_, UserRow>("SELECT * FROM users WHERE id = $1")
.bind(id)
.fetch_optional(&self.pool)
.await
.map_err(|db_err| {
adapter_error::connection_failed("PostgreSQL", "Query execution failed")
.with_source(db_err)
.with_next_step("Verify database connection")
.with_suggestion("Check connection pool configuration")
})?;

        match row {
            Some(user_row) => {
                User::from_row(user_row)
                    .map_err(|e| {
                        adapter_error::mapping_failure("Cannot map user row to entity")
                            .with_source(e)
                    })
            }
            None => Err(Hexserror::not_found("User", id)),
        }
    }
}
```

### Example 3: Validation with Field Context

```rust
use hexser::error::{Hexserror, codes};

pub struct CreateUserRequest {
    pub email: String,
    pub username: String,
    pub age: u32,
}

impl CreateUserRequest {
    pub fn validate(&self) -> Result<(), Hexserror> {
        // Email validation
        if self.email.is_empty() {
            return Err(
                Hexserror::validation_field("Email is required", "email")
                    .with_next_step("Provide a valid email address")
            );
        }
        
        if !self.email.contains('@') {
            return Err(
                Hexserror::validation_field("Invalid email format", "email")
                    .with_suggestion("Use format: user@example.com")
            );
        }
        
        // Username validation
        if self.username.len() < 3 {
            return Err(
                Hexserror::validation_field("Username too short", "username")
                    .with_next_step("Use at least 3 characters")
                    .with_suggestion("Try: john_doe, alice_smith")
            );
        }
        
        // Age validation
        if self.age < 13 {
            return Err(
                Hexserror::validation_field("Must be at least 13 years old", "age")
                    .with_next_step("Provide a valid age")
            );
        }
        
        Ok(())
    }
}
```

### Example 4: Multi-Layer Error Propagation

```rust
use hexser::error::{Hexserror, domain_error, port_error};

// Port layer
pub trait UserRepository {
async fn save(&self, user: &User) -> Result<(), Hexserror>;
}

// Domain layer
pub struct UserService<R: UserRepository> {
repo: R,
}

impl<R: UserRepository> UserService<R> {
pub async fn register_user(&self, req: CreateUserRequest) -> Result<User, Hexserror> {
// Validate input
req.validate()?;

        // Check for duplicates
        if self.repo.exists_by_email(&req.email).await? {
            return Err(
                Hexserror::conflict("User with this email already exists")
                    .with_next_step("Use a different email or recover existing account")
                    .with_suggestion("Try the password reset flow")
            );
        }
        
        // Create domain entity
        let user = User::new(req.email, req.username, req.age)
            .map_err(|e| {
                domain_error::invariant_violation("Cannot create user")
                    .with_source(e)
            })?;
        
        // Persist
        self.repo.save(&user)
            .await
            .map_err(|e| {
                port_error::communication_failure("Failed to save user")
                    .with_source(e)
                    .with_next_step("Retry operation or contact support")
            })?;
        
        Ok(user)
    }
}
```

### Example 5: Error Display and Debugging

```rust
use hexser::error::Hexserror;
use std::error::Error;

fn handle_user_creation(req: CreateUserRequest) {
    match service.register_user(req).await {
        Ok(user) => {
            println!("User created successfully: {}", user.id);
        }
        Err(err) => {
            // Display the full error
            eprintln!("{}", err);
            
            // Print error chain
            let mut source = err.source();
            while let Some(cause) = source {
                eprintln!("Caused by: {}", cause);
                source = cause.source();
            }
            
            // Match on specific error types for custom handling
            match &err {
                Hexserror::Validation(_) => {
                    // Return 400 Bad Request
                }
                Hexserror::NotFound(_) => {
                    // Return 404 Not Found
                }
                Hexserror::Conflict(_) => {
                    // Return 409 Conflict
                }
                _ => {
                    // Return 500 Internal Server Error
                }
            }
        }
    }
}
```
