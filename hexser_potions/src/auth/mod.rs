//! Auth potions: simple signup flow showcasing a Repository and a Directive.
//!
//! This module provides a minimal end-to-end example of a signup use case:
//! - A domain entity (User)
//! - A repository port (UserRepository)
//! - An adapter (InMemoryUserRepository)
//! - A directive (SignUpUser) with validation
//! - A small application function to wire it together
//!
//! Copy, paste, and adapt as needed.
//!
//! Revision History
//! - 2025-10-07T11:43:00Z @AI: Migrate to v0.4 QueryRepository API; remove id-centric methods; add filter-based querying; fix ID generation.

use hexser::prelude::*;

/// Domain entity representing a user.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct User {
  pub id: String,
  pub email: String,
}

impl HexEntity for User {
  type Id = String;
}

/// Repository port for users, extending the generic Repository.
pub trait UserRepository: Repository<User> {
  fn find_by_email(&self, email: &str) -> HexResult<Option<User>>;
}

/// A simple in-memory adapter implementing the user repository.
#[derive(Default)]
pub struct InMemoryUserRepository {
  pub users: Vec<User>,
}

impl Repository<User> for InMemoryUserRepository {
  fn save(&mut self, entity: User) -> HexResult<()> {
    // overwrite if exists, else push
    if let Some(existing) = self.users.iter_mut().find(|u| u.id == entity.id) {
      *existing = entity;
    } else {
      self.users.push(entity);
    }
    Ok(())
  }
}

impl UserRepository for InMemoryUserRepository {
  fn find_by_email(&self, email: &str) -> HexResult<Option<User>> {
    Ok(self.users.iter().find(|u| u.email == email).cloned())
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UserFilter {
  All,
  ByEmail(String),
  ById(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserSortKey {
  Email,
  Id,
}

impl hexser::ports::repository::QueryRepository<User> for InMemoryUserRepository {
  type Filter = UserFilter;
  type SortKey = UserSortKey;

  fn find_one(&self, filter: &UserFilter) -> HexResult<Option<User>> {
    let found = match filter {
      UserFilter::All => self.users.first().cloned(),
      UserFilter::ByEmail(e) => self.users.iter().find(|u| &u.email == e).cloned(),
      UserFilter::ById(id) => self.users.iter().find(|u| &u.id == id).cloned(),
    };
    Ok(found)
  }

  fn find(
    &self,
    filter: &UserFilter,
    opts: hexser::ports::repository::FindOptions<UserSortKey>,
  ) -> HexResult<Vec<User>> {
    let mut items: Vec<User> = match filter {
      UserFilter::All => self.users.clone(),
      UserFilter::ByEmail(e) => self
        .users
        .iter()
        .filter(|u| &u.email == e)
        .cloned()
        .collect(),
      UserFilter::ById(id) => self.users.iter().filter(|u| &u.id == id).cloned().collect(),
    };
    if let Some(mut sorts) = opts.sort {
      for s in sorts.drain(..).rev() {
        items.sort_by(|a, b| {
          let mut ord = match s.key {
            UserSortKey::Email => a.email.cmp(&b.email),
            UserSortKey::Id => a.id.cmp(&b.id),
          };
          if let hexser::ports::repository::Direction::Desc = s.direction {
            ord = ord.reverse();
          }
          ord
        });
      }
    }
    if let Some(offset) = opts.offset {
      let offset_usize: usize = std::convert::TryInto::try_into(offset).unwrap_or(usize::MAX);
      if offset_usize < items.len() {
        items = items.split_off(offset_usize);
      } else {
        items.clear();
      }
    }
    if let Some(limit) = opts.limit {
      let limit_usize: usize = std::convert::TryInto::try_into(limit).unwrap_or(usize::MAX);
      if items.len() > limit_usize {
        items.truncate(limit_usize);
      }
    }
    Ok(items)
  }

  fn exists(&self, filter: &UserFilter) -> HexResult<bool> {
    Ok(self.find_one(filter)?.is_some())
  }

  fn count(&self, filter: &UserFilter) -> HexResult<u64> {
    let n = match filter {
      UserFilter::All => self.users.len(),
      UserFilter::ByEmail(e) => self.users.iter().filter(|u| &u.email == e).count(),
      UserFilter::ById(id) => self.users.iter().filter(|u| &u.id == id).count(),
    };
    Ok(n as u64)
  }

  fn delete_where(&mut self, filter: &UserFilter) -> HexResult<u64> {
    let before = self.users.len();
    match filter {
      UserFilter::All => {
        self.users.clear();
      }
      UserFilter::ByEmail(e) => {
        self.users.retain(|u| &u.email != e);
      }
      UserFilter::ById(id) => {
        self.users.retain(|u| &u.id != id);
      }
    }
    let removed = before.saturating_sub(self.users.len());
    Ok(removed as u64)
  }
}

/// Directive representing a signup request.
pub struct SignUpUser {
  pub email: String,
}

impl Directive for SignUpUser {
  fn validate(&self) -> HexResult<()> {
    if self.email.contains('@') {
      Ok(())
    } else {
      Err(hexser::Hexserror::validation_field(
        "Invalid email",
        "email",
      ))
    }
  }
}

/// Application helper that executes the signup flow.
/// - Validates the directive
/// - Ensures email is unique
/// - Creates and persists a new user
pub fn execute_signup<R>(repo: &mut R, cmd: SignUpUser) -> HexResult<User>
where
  R: UserRepository + hexser::ports::repository::QueryRepository<User, Filter = UserFilter>,
{
  cmd.validate()?;

  if repo.find_by_email(&cmd.email)?.is_some() {
    return Err(hexser::Hexserror::domain(
      "E_HEXSER_POTIONS_EMAIL_TAKEN",
      "Email already registered",
    ));
  }

  // naive ID generation without dependencies: count existing users via QueryRepository
  let count =
    <R as hexser::ports::repository::QueryRepository<User>>::count(&*repo, &UserFilter::All)?;
  let next_id = std::format!("user-{}", count + 1);

  let user = User {
    id: next_id,
    email: cmd.email,
  };

  repo.save(user.clone())?;
  Ok(user)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn signup_happy_path() {
    let mut repo = InMemoryUserRepository::default();
    let user = execute_signup(
      &mut repo,
      SignUpUser {
        email: "a@b.com".into(),
      },
    )
    .unwrap();
    assert_eq!(user.email, "a@b.com");
    assert!(repo.find_by_email("a@b.com").unwrap().is_some());
  }

  #[test]
  fn signup_rejects_invalid_email() {
    let mut repo = InMemoryUserRepository::default();
    let res = execute_signup(
      &mut repo,
      SignUpUser {
        email: "not-an-email".into(),
      },
    );
    assert!(res.is_err());
  }

  #[test]
  fn signup_rejects_duplicates() {
    let mut repo = InMemoryUserRepository::default();
    execute_signup(
      &mut repo,
      SignUpUser {
        email: "a@b.com".into(),
      },
    )
    .unwrap();
    let duplicate = execute_signup(
      &mut repo,
      SignUpUser {
        email: "a@b.com".into(),
      },
    );
    assert!(duplicate.is_err());
  }
}
