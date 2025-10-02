//! DomainService marker trait for domain operations.
//!
//! Domain services encapsulate domain logic that doesn't naturally belong to
//! an entity or value object. They represent operations that involve multiple
//! aggregates or that don't have a natural home in a specific entity.
//! Domain services contain only domain logic and have no dependencies on infrastructure.
//!
//! Revision History
//! - 2025-10-01T00:00:00Z @AI: Initial DomainService marker trait definition.

/// Marker trait for domain services.
///
/// Domain services contain domain logic that doesn't naturally fit into
/// entities or value objects. They operate on domain concepts and maintain
/// the ubiquitous language of the domain.
///
/// # Example
///
/// ```rust
/// use hex::domain::DomainService;
/// use hex::HexResult;
///
/// struct TransferService;
///
/// impl DomainService for TransferService {}
///
/// impl TransferService {
///     fn transfer_funds(
///         &self,
///         from_account: &str,
///         to_account: &str,
///         amount: u64,
///     ) -> HexResult<()> {
///         // Domain logic for transferring funds
///         Ok(())
///     }
/// }
/// ```
pub trait DomainService {}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestPricingService;

    impl DomainService for TestPricingService {}

    impl TestPricingService {
        fn calculate_price(&self, base_price: f64, discount: f64) -> f64 {
            base_price * (1.0 - discount)
        }
    }

    #[test]
    fn test_domain_service_implementation() {
        let service = TestPricingService;
        let price = service.calculate_price(100.0, 0.1);
        assert_eq!(price, 90.0);
    }
}
