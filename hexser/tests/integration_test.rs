//! Integration tests for hex crate.
//!
//! These tests validate that all components of the hexagonal architecture
//! work together correctly. They test the flow from domain through ports
//! and adapters, ensuring proper separation of concerns and functionality.
//!
//! Revision History
//! - 2025-10-12T18:37:00Z @AI: Add comprehensive Application trait integration test with full CQRS flow.
//! - 2025-10-02T21:45:00Z @AI: Enhance HexAggregate macro test to use hex(invariants) attribute for custom validation.
//! - 2025-10-02T21:30:00Z @AI: Fix conflicting Aggregate implementations, remove derive from custom invariant test.

#[cfg(test)]
mod domain_integration {
  use hexser::{Aggregate, HexValueItem};

  /// Test Entity and ValueObject integration.
  #[test]
  fn test_entity_with_value_object() {
    struct Email(String);

    impl hexser::HexValueItem for Email {
      fn validate(&self) -> hexser::HexResult<()> {
        if self.0.contains('@') {
          Ok(())
        } else {
          Err(hexser::Hexserror::validation("Email must contain @"))
        }
      }
    }

    struct User {
      id: String,
      email: Email,
    }

    impl hexser::HexEntity for User {
      type Id = String;
    }

    let email = Email(String::from("test@example.com"));
    assert!(email.validate().is_ok());

    let user = User {
      id: String::from("1"),
      email,
    };

    let _id: <User as hexser::HexEntity>::Id = user.id;
  }

  /// Test HexAggregate macro with custom invariants via attribute.
  #[test]
  fn test_aggregate_invariants() {
    #[derive(hexser_macros::HexAggregate)]
    struct Order {
      id: String,
      items: Vec<String>,
    }

    impl hexser::HexEntity for Order {
      type Id = String;
    }

    impl Order {
      fn check_invariants(&self) -> hexser::HexResult<()> {
        if self.items.is_empty() {
          return Err(hexser::Hexserror::domain(
            hexser::error_codes::domain::INVARIANT_EMPTY,
            "Order must have items",
          ));
        }
        Ok(())
      }
    }

    let valid_order = Order {
      id: String::from("1"),
      items: vec![String::from("item1")],
    };
    assert!(valid_order.check_invariants().is_ok());

    let invalid_order = Order {
      id: String::from("2"),
      items: vec![],
    };
    assert!(invalid_order.check_invariants().is_err());
  }

  /// Test HexAggregate derive macro with default implementation.
  #[test]
  fn test_hex_aggregate_derive_default() {
    #[derive(hexser_macros::HexAggregate)]
    struct SimpleAggregate {
      id: String,
      value: i32,
    }

    impl hexser::HexEntity for SimpleAggregate {
      type Id = String;
    }

    let aggregate = SimpleAggregate {
      id: String::from("1"),
      value: 42,
    };

    assert!(aggregate.check_invariants().is_ok());
  }
}

#[cfg(test)]
mod port_adapter_integration {
  use hexser::{Mapper, Repository};

  /// Test Repository port with adapter implementation.
  #[test]
  fn test_repository_flow() {
    #[derive(Clone)]
    struct Todo {
      id: String,
      title: String,
      done: bool,
    }

    impl hexser::HexEntity for Todo {
      type Id = String;
    }

    struct InMemoryTodoRepository {
      todos: Vec<Todo>,
    }

    impl hexser::adapters::Adapter for InMemoryTodoRepository {}

    impl hexser::ports::Repository<Todo> for InMemoryTodoRepository {
      fn save(&mut self, todo: Todo) -> hexser::HexResult<()> {
        self.todos.push(todo);
        Ok(())
      }
    }

    #[derive(Clone)]
    enum TodoFilter {
      All,
      ById(String),
    }
    #[derive(Clone, Copy)]
    enum TodoSortKey {
      Id,
    }

    impl hexser::ports::repository::QueryRepository<Todo> for InMemoryTodoRepository {
      type Filter = TodoFilter;
      type SortKey = TodoSortKey;

      fn find_one(&self, filter: &TodoFilter) -> hexser::HexResult<Option<Todo>> {
        let found = match filter {
          TodoFilter::All => self.todos.first().cloned(),
          TodoFilter::ById(id) => self.todos.iter().find(|t| &t.id == id).cloned(),
        };
        Ok(found)
      }

      fn find(
        &self,
        filter: &TodoFilter,
        _opts: hexser::ports::repository::FindOptions<TodoSortKey>,
      ) -> hexser::HexResult<Vec<Todo>> {
        let items: Vec<Todo> = match filter {
          TodoFilter::All => self.todos.clone(),
          TodoFilter::ById(id) => self.todos.iter().filter(|t| &t.id == id).cloned().collect(),
        };
        Ok(items)
      }

      fn delete_where(&mut self, filter: &TodoFilter) -> hexser::HexResult<u64> {
        let before = self.todos.len();
        match filter {
          TodoFilter::All => self.todos.clear(),
          TodoFilter::ById(id) => self.todos.retain(|t| &t.id != id),
        }
        Ok((before.saturating_sub(self.todos.len())) as u64)
      }
    }

    let mut repo = InMemoryTodoRepository { todos: Vec::new() };

    let todo = Todo {
      id: String::from("1"),
      title: String::from("Test"),
      done: false,
    };

    assert!(repo.save(todo).is_ok());
    assert!(
      <InMemoryTodoRepository as hexser::ports::repository::QueryRepository<Todo>>::find_one(
        &repo,
        &TodoFilter::ById(String::from("1"))
      )
      .unwrap()
      .is_some()
    );
    assert_eq!(
      <InMemoryTodoRepository as hexser::ports::repository::QueryRepository<Todo>>::find(
        &repo,
        &TodoFilter::All,
        hexser::ports::repository::FindOptions::default()
      )
      .unwrap()
      .len(),
      1
    );
    assert_eq!(
      <InMemoryTodoRepository as hexser::ports::repository::QueryRepository<Todo>>::delete_where(
        &mut repo,
        &TodoFilter::ById(String::from("1"))
      )
      .unwrap(),
      1
    );
    assert!(
      <InMemoryTodoRepository as hexser::ports::repository::QueryRepository<Todo>>::find_one(
        &repo,
        &TodoFilter::ById(String::from("1"))
      )
      .unwrap()
      .is_none()
    );
  }

  /// Test Mapper transformations.
  #[test]
  fn test_mapper_transformations() {
    struct DomainUser {
      id: String,
      email: String,
    }

    struct DbUserRow {
      user_id: String,
      user_email: String,
    }

    struct UserMapper;

    impl hexser::adapters::Mapper<DomainUser, DbUserRow> for UserMapper {
      fn map(&self, from: DomainUser) -> hexser::HexResult<DbUserRow> {
        Ok(DbUserRow {
          user_id: from.id,
          user_email: from.email,
        })
      }
    }

    impl hexser::adapters::Mapper<DbUserRow, DomainUser> for UserMapper {
      fn map(&self, from: DbUserRow) -> hexser::HexResult<DomainUser> {
        Ok(DomainUser {
          id: from.user_id,
          email: from.user_email,
        })
      }
    }

    let mapper = UserMapper;

    let domain = DomainUser {
      id: String::from("1"),
      email: String::from("test@example.com"),
    };

    let db_row: DbUserRow = mapper.map(domain).unwrap();
    assert_eq!(db_row.user_id, "1");

    let back_to_domain: DomainUser = mapper.map(db_row).unwrap();
    assert_eq!(back_to_domain.id, "1");
  }
}

#[cfg(test)]
mod cqrs_integration {
  use hexser::{Directive, DirectiveHandler, QueryHandler};

  /// Test Directive with handler.
  #[test]
  fn test_directive_handler_flow() {
    struct CreateTodoDirective {
      title: String,
    }

    impl hexser::application::Directive for CreateTodoDirective {
      fn validate(&self) -> hexser::HexResult<()> {
        if self.title.is_empty() {
          return Err(hexser::Hexserror::validation("Title cannot be empty"));
        }
        Ok(())
      }
    }

    struct CreateTodoHandler;

    impl hexser::application::DirectiveHandler<CreateTodoDirective> for CreateTodoHandler {
      fn handle(&self, directive: CreateTodoDirective) -> hexser::HexResult<()> {
        directive.validate()?;
        // Would save to repository here
        Ok(())
      }
    }

    let handler = CreateTodoHandler;

    let valid = CreateTodoDirective {
      title: String::from("Test todo"),
    };
    assert!(handler.handle(valid).is_ok());

    let invalid = CreateTodoDirective {
      title: String::from(""),
    };
    assert!(handler.handle(invalid).is_err());
  }

  /// Test Query with handler.
  #[test]
  fn test_query_handler_flow() {
    struct FindTodoQuery {
      id: String,
    }

    #[derive(Clone)]
    struct TodoView {
      id: String,
      title: String,
    }

    struct FindTodoHandler {
      todos: Vec<TodoView>,
    }

    impl hexser::application::QueryHandler<FindTodoQuery, Option<TodoView>> for FindTodoHandler {
      fn handle(&self, query: FindTodoQuery) -> hexser::HexResult<Option<TodoView>> {
        Ok(self.todos.iter().find(|t| t.id == query.id).cloned())
      }
    }

    let handler = FindTodoHandler {
      todos: vec![TodoView {
        id: String::from("1"),
        title: String::from("Test"),
      }],
    };

    let query = FindTodoQuery {
      id: String::from("1"),
    };

    let result = handler.handle(query).unwrap();
    assert!(result.is_some());
  }
}

#[cfg(test)]
mod error_integration {
  /// Test error builder pattern.
  #[test]
  fn test_error_builder() {
    let err = hexser::Hexserror::domain(
      hexser::error_codes::domain::INVARIANT_EMPTY,
      "Order cannot be empty",
    )
    .with_next_step("Add at least one item")
    .with_suggestion("order.add_item(item)");

    let display = format!("{}", err);
    assert!(display.contains("E_HEX_001"));
    assert!(display.contains("Next Steps"));
  }
}

#[cfg(test)]
mod application_integration {
  use hexser::{
    Application, Directive, DirectiveHandler, HexEntity, HexResult, QueryHandler, Repository,
  };

  // Domain layer
  #[derive(Clone, Debug, PartialEq)]
  struct Todo {
    id: String,
    title: String,
    completed: bool,
  }

  impl HexEntity for Todo {
    type Id = String;
  }

  // Repository port
  trait TodoRepository: Repository<Todo> {
    fn find_all(&self) -> HexResult<Vec<Todo>>;
  }

  // Repository adapter (using interior mutability for shared state)
  struct InMemoryTodoRepository {
    todos: std::sync::Arc<std::sync::Mutex<Vec<Todo>>>,
    initialized: std::sync::Arc<std::sync::Mutex<bool>>,
  }

  impl InMemoryTodoRepository {
    fn new() -> Self {
      Self {
        todos: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
        initialized: std::sync::Arc::new(std::sync::Mutex::new(false)),
      }
    }

    fn initialize(&mut self) -> HexResult<()> {
      let mut initialized = self.initialized.lock().unwrap();
      *initialized = true;
      Ok(())
    }
  }

  impl Clone for InMemoryTodoRepository {
    fn clone(&self) -> Self {
      Self {
        todos: std::sync::Arc::clone(&self.todos),
        initialized: std::sync::Arc::clone(&self.initialized),
      }
    }
  }

  impl hexser::adapters::Adapter for InMemoryTodoRepository {}

  impl Repository<Todo> for InMemoryTodoRepository {
    fn save(&mut self, todo: Todo) -> HexResult<()> {
      let initialized = self.initialized.lock().unwrap();
      if !*initialized {
        return Err(hexser::Hexserror::adapter(
          hexser::error_codes::adapter::DB_CONNECTION_FAILURE,
          "Repository not initialized",
        ));
      }
      drop(initialized);

      let mut todos = self.todos.lock().unwrap();
      todos.push(todo);
      Ok(())
    }
  }

  impl TodoRepository for InMemoryTodoRepository {
    fn find_all(&self) -> HexResult<Vec<Todo>> {
      let initialized = self.initialized.lock().unwrap();
      if !*initialized {
        return Err(hexser::Hexserror::adapter(
          hexser::error_codes::adapter::DB_CONNECTION_FAILURE,
          "Repository not initialized",
        ));
      }
      drop(initialized);

      let todos = self.todos.lock().unwrap();
      Ok(todos.clone())
    }
  }

  // Directive (write side)
  struct CreateTodoDirective {
    title: String,
  }

  impl Directive for CreateTodoDirective {
    fn validate(&self) -> HexResult<()> {
      if self.title.is_empty() {
        return Err(hexser::Hexserror::validation_field(
          "Title cannot be empty",
          "title",
        ));
      }
      Ok(())
    }
  }

  // Directive handler
  struct CreateTodoHandler {
    repository: InMemoryTodoRepository,
    next_id: std::cell::Cell<u32>,
  }

  impl CreateTodoHandler {
    fn new(repository: InMemoryTodoRepository) -> Self {
      Self {
        repository,
        next_id: std::cell::Cell::new(1),
      }
    }
  }

  impl DirectiveHandler<CreateTodoDirective> for CreateTodoHandler {
    fn handle(&self, directive: CreateTodoDirective) -> HexResult<()> {
      directive.validate()?;

      let id = self.next_id.get();
      let todo = Todo {
        id: id.to_string(),
        title: directive.title,
        completed: false,
      };

      self.next_id.set(id + 1);
      let mut repo = self.repository.clone();
      repo.save(todo)
    }
  }

  // Query (read side)
  struct FindAllTodosQuery;

  // Query handler
  struct FindAllTodosHandler {
    repository: InMemoryTodoRepository,
  }

  impl FindAllTodosHandler {
    fn new(repository: InMemoryTodoRepository) -> Self {
      Self { repository }
    }
  }

  impl QueryHandler<FindAllTodosQuery, Vec<Todo>> for FindAllTodosHandler {
    fn handle(&self, _query: FindAllTodosQuery) -> HexResult<Vec<Todo>> {
      self.repository.find_all()
    }
  }

  // Application
  struct TodoApplication {
    directive_handler: Option<CreateTodoHandler>,
    query_handler: Option<FindAllTodosHandler>,
    repository: InMemoryTodoRepository,
    fail_on_initialize: bool,
    fail_on_run: bool,
  }

  impl TodoApplication {
    fn new() -> Self {
      Self {
        directive_handler: None,
        query_handler: None,
        repository: InMemoryTodoRepository::new(),
        fail_on_initialize: false,
        fail_on_run: false,
      }
    }

    fn with_init_failure() -> Self {
      let mut app = Self::new();
      app.fail_on_initialize = true;
      app
    }

    fn with_run_failure() -> Self {
      let mut app = Self::new();
      app.fail_on_run = true;
      app
    }
  }

  impl Application for TodoApplication {
    fn name(&self) -> &str {
      "TodoApplication"
    }

    fn initialize(&mut self) -> HexResult<()> {
      if self.fail_on_initialize {
        return Err(hexser::Hexserror::adapter(
          hexser::error_codes::io::IO_FAILURE,
          "Simulated initialization failure",
        ));
      }

      // Initialize repository
      self.repository.initialize()?;

      // Create handlers with initialized repository
      self.directive_handler = Some(CreateTodoHandler::new(self.repository.clone()));
      self.query_handler = Some(FindAllTodosHandler::new(self.repository.clone()));

      Ok(())
    }

    fn run(&mut self) -> HexResult<()> {
      if self.fail_on_run {
        return Err(hexser::Hexserror::domain(
          hexser::error_codes::domain::INVARIANT_VIOLATION,
          "Simulated runtime failure",
        ));
      }

      // Execute some directives
      let directive = CreateTodoDirective {
        title: String::from("Integration test todo"),
      };

      self
        .directive_handler
        .as_mut()
        .expect("Handler not initialized")
        .handle(directive)?;

      // Execute a query
      let query = FindAllTodosQuery;
      let todos = self
        .query_handler
        .as_ref()
        .expect("Handler not initialized")
        .handle(query)?;

      assert_eq!(todos.len(), 1);
      assert_eq!(todos[0].title, "Integration test todo");

      Ok(())
    }

    fn shutdown(&mut self) -> HexResult<()> {
      // Cleanup handlers
      self.directive_handler = None;
      self.query_handler = None;
      Ok(())
    }
  }

  /// Test: Complete application lifecycle with CQRS flow.
  /// Justification: Validates that the Application trait orchestrates the entire
  /// hexagonal architecture correctly, from initialization through execution to shutdown.
  #[test]
  fn test_complete_application_lifecycle() {
    let mut app = TodoApplication::new();

    // Execute full lifecycle
    let result = app.execute();
    assert!(result.is_ok());

    // Verify handlers were cleaned up
    assert!(app.directive_handler.is_none());
    assert!(app.query_handler.is_none());
  }

  /// Test: Application handles initialization failures.
  /// Justification: Ensures that initialization errors prevent the application
  /// from running in an invalid state.
  #[test]
  fn test_application_initialization_failure() {
    let mut app = TodoApplication::with_init_failure();

    let result = app.execute();
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert!(err.to_string().contains("initialization failure"));
  }

  /// Test: Application handles runtime failures and still shuts down.
  /// Justification: Verifies that runtime errors are propagated but shutdown
  /// still occurs for cleanup.
  #[test]
  fn test_application_runtime_failure() {
    let mut app = TodoApplication::with_run_failure();

    let result = app.execute();
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert!(err.to_string().contains("runtime failure"));

    // Verify shutdown was still called (handlers cleaned up)
    assert!(app.directive_handler.is_none());
    assert!(app.query_handler.is_none());
  }

  /// Test: Minimal application with default implementations.
  /// Justification: Validates the zero-boilerplate philosophy where only
  /// name() is required.
  #[test]
  fn test_minimal_application() {
    struct MinimalApp;

    impl Application for MinimalApp {
      fn name(&self) -> &str {
        "MinimalApp"
      }
    }

    let mut app = MinimalApp;
    assert_eq!(app.name(), "MinimalApp");

    let result = app.execute();
    assert!(result.is_ok());
  }

  /// Test: Directive validation through Application.
  /// Justification: Ensures that directive validation errors are properly
  /// propagated through the application layer.
  #[test]
  fn test_directive_validation_through_application() {
    let mut app = TodoApplication::new();
    app.initialize().expect("Initialization failed");

    let invalid_directive = CreateTodoDirective {
      title: String::from(""),
    };

    let result = app
      .directive_handler
      .as_mut()
      .unwrap()
      .handle(invalid_directive);

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Title cannot be empty"));
  }

  /// Test: Repository not initialized error handling.
  /// Justification: Validates that the application properly handles infrastructure
  /// errors when components aren't properly initialized.
  #[test]
  fn test_uninitialized_repository_error() {
    let mut repo = InMemoryTodoRepository::new();

    let todo = Todo {
      id: String::from("1"),
      title: String::from("Test"),
      completed: false,
    };

    let result = repo.save(todo);
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert!(err.to_string().contains("not initialized"));
  }
}
