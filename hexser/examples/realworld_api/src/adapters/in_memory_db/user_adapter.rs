//! In-memory user repository adapter implementation.
//!
//! Provides an in-memory implementation of the UserRepository port using
//! a thread-safe Vec for storage. Implements both Repository and QueryRepository traits.
//!
//! Revision History
//! - 2025-10-10T10:47:00Z @AI: Add HexAdapter derive macro for automatic registration and graph introspection.
//! - 2025-10-09T22:14:00Z @AI: Initial implementation of in-memory user adapter.

#[derive(hexser::HexAdapter, std::default::Default)]
pub struct InMemoryUserRepository {
    users: std::vec::Vec<crate::domain::user::User>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self {
            users: std::vec::Vec::new(),
        }
    }

    fn matches_filter(user: &crate::domain::user::User, filter: &crate::ports::user_repository::UserFilter) -> bool {
        match filter {
            crate::ports::user_repository::UserFilter::ById(id) => &user.id == id,
            crate::ports::user_repository::UserFilter::ByEmail(email) => &user.email == email,
            crate::ports::user_repository::UserFilter::ByUsername(username) => &user.username == username,
            crate::ports::user_repository::UserFilter::All => true,
        }
    }
}

impl hexser::ports::Repository<crate::domain::user::User> for InMemoryUserRepository {
    fn save(&mut self, user: crate::domain::user::User) -> hexser::HexResult<()> {
        if let std::option::Option::Some(pos) = self.users.iter().position(|u| u.id == user.id) {
            self.users[pos] = user;
        } else {
            self.users.push(user);
        }
        std::result::Result::Ok(())
    }
}

impl hexser::ports::repository::QueryRepository<crate::domain::user::User> for InMemoryUserRepository {
    type Filter = crate::ports::user_repository::UserFilter;
    type SortKey = crate::ports::user_repository::UserSortKey;

    fn find_one(&self, filter: &Self::Filter) -> hexser::HexResult<std::option::Option<crate::domain::user::User>> {
        std::result::Result::Ok(
            self.users
                .iter()
                .find(|u| Self::matches_filter(u, filter))
                .cloned()
        )
    }

    fn find(
        &self,
        filter: &Self::Filter,
        options: hexser::ports::repository::FindOptions<Self::SortKey>,
    ) -> hexser::HexResult<std::vec::Vec<crate::domain::user::User>> {
        let mut results: std::vec::Vec<_> = self.users
            .iter()
            .filter(|u| Self::matches_filter(u, filter))
            .cloned()
            .collect();

        if let std::option::Option::Some(sorts) = options.sort {
            for sort in sorts.into_iter().rev() {
                match (sort.key, sort.direction) {
                    (crate::ports::user_repository::UserSortKey::Id, hexser::ports::repository::Direction::Asc) => {
                        results.sort_by(|a, b| a.id.cmp(&b.id));
                    }
                    (crate::ports::user_repository::UserSortKey::Id, hexser::ports::repository::Direction::Desc) => {
                        results.sort_by(|a, b| b.id.cmp(&a.id));
                    }
                    (crate::ports::user_repository::UserSortKey::Email, hexser::ports::repository::Direction::Asc) => {
                        results.sort_by(|a, b| a.email.cmp(&b.email));
                    }
                    (crate::ports::user_repository::UserSortKey::Email, hexser::ports::repository::Direction::Desc) => {
                        results.sort_by(|a, b| b.email.cmp(&a.email));
                    }
                    (crate::ports::user_repository::UserSortKey::Username, hexser::ports::repository::Direction::Asc) => {
                        results.sort_by(|a, b| a.username.cmp(&b.username));
                    }
                    (crate::ports::user_repository::UserSortKey::Username, hexser::ports::repository::Direction::Desc) => {
                        results.sort_by(|a, b| b.username.cmp(&a.username));
                    }
                    (crate::ports::user_repository::UserSortKey::CreatedAt, _) => {}
                }
            }
        }

        let offset = options.offset.unwrap_or(0) as usize;
        let limit = options.limit.map(|l| l as usize).unwrap_or(results.len());

        std::result::Result::Ok(
            results
                .into_iter()
                .skip(offset)
                .take(limit)
                .collect()
        )
    }

    fn delete_where(&mut self, filter: &Self::Filter) -> hexser::HexResult<u64> {
        let before = self.users.len();
        self.users.retain(|u| !Self::matches_filter(u, filter));
        std::result::Result::Ok((before - self.users.len()) as u64)
    }
}

impl crate::ports::user_repository::UserRepository for InMemoryUserRepository {}

#[cfg(test)]
mod tests {
    #[test]
    fn test_save_and_find_user() {
        let mut repo = super::InMemoryUserRepository::new();
        let user = crate::domain::user::User::new(
            std::string::String::from("1"),
            std::string::String::from("test@example.com"),
            std::string::String::from("testuser"),
            std::string::String::from("hashed_pass"),
        );

        hexser::ports::Repository::save(&mut repo, user.clone()).unwrap();

        let filter = crate::ports::user_repository::UserFilter::ById(std::string::String::from("1"));
        let found = hexser::ports::repository::QueryRepository::find_one(&repo, &filter).unwrap();
        std::assert!(found.is_some());
        std::assert_eq!(found.unwrap().email, "test@example.com");
    }

    #[test]
    fn test_find_by_email() {
        let mut repo = super::InMemoryUserRepository::new();
        let user = crate::domain::user::User::new(
            std::string::String::from("1"),
            std::string::String::from("test@example.com"),
            std::string::String::from("testuser"),
            std::string::String::from("hashed_pass"),
        );

        hexser::ports::Repository::save(&mut repo, user).unwrap();

        let filter = crate::ports::user_repository::UserFilter::ByEmail(
            std::string::String::from("test@example.com")
        );
        let found = hexser::ports::repository::QueryRepository::find_one(&repo, &filter).unwrap();
        std::assert!(found.is_some());
    }
}
