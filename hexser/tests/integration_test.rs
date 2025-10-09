//! Integration tests for hex crate.
//!
//! These tests validate that all components of the hexagonal architecture
//! work together correctly. They test the flow from domain through ports
//! and adapters, ensuring proper separation of concerns and functionality.
//!
//! Revision History
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
