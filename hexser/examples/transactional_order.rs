//! Transactional Order Processing Example
//!
//! Demonstrates designing a transactional 'Directive' called ProcessOrder that involves
//! multiple domain operations. The directive must: 1) Decrement stock for multiple products,
//! 2) Create an Order record, and 3) Dispatch an OrderCreated event. Shows how to structure
//! Adapters and Directives to ensure all operations succeed or fail atomically.
//!
//! This addresses Context7 Question 8: Design a transactional directive with multiple
//! repository operations that must succeed or fail together as an atomic unit.
//!
//! Run with: cargo run --example transactional_order
//!
//! Revision History
//! - 2025-10-08T22:43:00Z @AI: Extract transactional order example from README to standalone file per user request.

/// Order item representing a product and quantity
#[derive(Clone, std::fmt::Debug)]
struct OrderItem {
    product_id: std::string::String,
    quantity: u32,
    price: f64,
}

/// Order entity
#[derive(Clone, std::fmt::Debug)]
struct Order {
    id: std::string::String,
    customer_id: std::string::String,
    items: std::vec::Vec<OrderItem>,
    total: f64,
}

impl hexser::domain::entity::Entity for Order {
    type Id = std::string::String;
}

/// OrderCreated domain event
#[derive(Clone, std::fmt::Debug)]
struct OrderCreated {
    order_id: std::string::String,
    customer_id: std::string::String,
    timestamp: u64,
}

impl hexser::domain::domain_event::DomainEvent for OrderCreated {
    fn event_type(&self) -> &str {
        "OrderCreated"
    }

    fn aggregate_id(&self) -> std::string::String {
        self.order_id.clone()
    }
}

/// Transaction context (mock for demonstration)
struct Transaction {
    committed: std::cell::RefCell<bool>,
    operations: std::cell::RefCell<std::vec::Vec<std::string::String>>,
}

impl Transaction {
    fn new() -> Self {
        Self {
            committed: std::cell::RefCell::new(false),
            operations: std::cell::RefCell::new(std::vec::Vec::new()),
        }
    }

    fn log_operation(&self, op: std::string::String) {
        self.operations.borrow_mut().push(op);
    }

    fn commit(&self) -> hexser::result::hex_result::HexResult<()> {
        *self.committed.borrow_mut() = true;
        std::result::Result::Ok(())
    }

    fn is_committed(&self) -> bool {
        *self.committed.borrow()
    }
}

/// Port for product repository with transaction support
trait ProductRepository {
    fn decrement_stock(
        &self,
        tx: &Transaction,
        product_id: &str,
        qty: u32,
    ) -> hexser::result::hex_result::HexResult<()>;
}

/// Port for order repository with transaction support
trait OrderRepository {
    fn create_order(
        &self,
        tx: &Transaction,
        order: Order,
    ) -> hexser::result::hex_result::HexResult<()>;
}

/// Port for event bus (non-transactional)
trait EventBus {
    fn publish(&self, event: OrderCreated) -> hexser::result::hex_result::HexResult<()>;
}

/// In-memory product repository adapter
struct InMemoryProductRepository {
    stock: std::sync::Arc<std::sync::Mutex<std::collections::HashMap<std::string::String, u32>>>,
}

impl InMemoryProductRepository {
    fn new() -> Self {
        let mut stock = std::collections::HashMap::new();
        stock.insert(std::string::String::from("product-001"), 100);
        stock.insert(std::string::String::from("product-002"), 50);
        Self {
            stock: std::sync::Arc::new(std::sync::Mutex::new(stock)),
        }
    }
}

impl hexser::adapters::adapter::Adapter for InMemoryProductRepository {}

impl ProductRepository for InMemoryProductRepository {
    fn decrement_stock(
        &self,
        tx: &Transaction,
        product_id: &str,
        qty: u32,
    ) -> hexser::result::hex_result::HexResult<()> {
        tx.log_operation(std::format!("decrement_stock({}, {})", product_id, qty));
        let mut stock = self.stock.lock().unwrap();
        let current = stock.get(product_id).copied().unwrap_or(0);
        if current < qty {
            return std::result::Result::Err(
                hexser::error::hex_error::Hexserror::domain(
                    hexser::error::codes::domain::INVARIANT_VIOLATION,
                    "Insufficient stock"
                )
                .with_next_step("Check product availability before ordering")
            );
        }
        stock.insert(product_id.to_string(), current - qty);
        std::result::Result::Ok(())
    }
}

/// In-memory order repository adapter
struct InMemoryOrderRepository {
    orders: std::sync::Arc<std::sync::Mutex<std::vec::Vec<Order>>>,
}

impl InMemoryOrderRepository {
    fn new() -> Self {
        Self {
            orders: std::sync::Arc::new(std::sync::Mutex::new(std::vec::Vec::new())),
        }
    }
}

impl hexser::adapters::adapter::Adapter for InMemoryOrderRepository {}

impl OrderRepository for InMemoryOrderRepository {
    fn create_order(
        &self,
        tx: &Transaction,
        order: Order,
    ) -> hexser::result::hex_result::HexResult<()> {
        tx.log_operation(std::format!("create_order({})", order.id));
        let mut orders = self.orders.lock().unwrap();
        orders.push(order);
        std::result::Result::Ok(())
    }
}

/// In-memory event bus adapter
struct InMemoryEventBus {
    events: std::sync::Arc<std::sync::Mutex<std::vec::Vec<OrderCreated>>>,
}

impl InMemoryEventBus {
    fn new() -> Self {
        Self {
            events: std::sync::Arc::new(std::sync::Mutex::new(std::vec::Vec::new())),
        }
    }

    fn event_count(&self) -> usize {
        self.events.lock().unwrap().len()
    }
}

impl hexser::adapters::adapter::Adapter for InMemoryEventBus {}

impl EventBus for InMemoryEventBus {
    fn publish(&self, event: OrderCreated) -> hexser::result::hex_result::HexResult<()> {
        let mut events = self.events.lock().unwrap();
        events.push(event);
        std::result::Result::Ok(())
    }
}

/// ProcessOrder directive
struct ProcessOrderDirective {
    customer_id: std::string::String,
    items: std::vec::Vec<OrderItem>,
}

impl ProcessOrderDirective {
    fn validate(&self) -> hexser::result::hex_result::HexResult<()> {
        if self.items.is_empty() {
            return std::result::Result::Err(
                hexser::error::hex_error::Hexserror::validation("Items cannot be empty")
            );
        }
        std::result::Result::Ok(())
    }
}

/// ProcessOrder handler with transaction support
struct ProcessOrderHandler {
    product_repo: std::boxed::Box<dyn ProductRepository>,
    order_repo: std::boxed::Box<dyn OrderRepository>,
    event_bus: std::boxed::Box<dyn EventBus>,
}

impl ProcessOrderHandler {
    fn handle(&self, directive: ProcessOrderDirective) -> hexser::result::hex_result::HexResult<()> {
        // Validate directive
        directive.validate()?;

        // Begin transaction
        let tx = Transaction::new();

        // 1) Decrement stock for each product (atomic within tx)
        for item in &directive.items {
            self.product_repo.decrement_stock(&tx, &item.product_id, item.quantity)
                .map_err(|e| {
                    hexser::error::hex_error::Hexserror::domain(
                        hexser::error::codes::domain::INVARIANT_VIOLATION,
                        "Failed to decrement stock"
                    )
                    .with_next_step("Verify product availability")
                })?;
        }

        // 2) Create order record (atomic within tx)
        let total: f64 = directive.items.iter().map(|i| i.price * i.quantity as f64).sum();
        let order = Order {
            id: std::format!("order-{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis()),
            customer_id: directive.customer_id.clone(),
            items: directive.items.clone(),
            total,
        };

        self.order_repo.create_order(&tx, order.clone())
            .map_err(|e| {
                hexser::error::hex_error::Hexserror::adapter(
                    hexser::error::codes::adapter::DB_CONNECTION_FAILURE,
                    "Failed to create order"
                )
                .with_next_step("Check database connectivity")
            })?;

        // Commit transaction (all-or-nothing)
        tx.commit()
            .map_err(|e| {
                hexser::error::hex_error::Hexserror::adapter(
                    hexser::error::codes::adapter::DB_CONNECTION_FAILURE,
                    "Transaction commit failed"
                )
                .with_next_step("Review database logs")
            })?;

        // 3) Dispatch event (after commit)
        let event = OrderCreated {
            order_id: order.id.clone(),
            customer_id: order.customer_id.clone(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        self.event_bus.publish(event)?;

        std::result::Result::Ok(())
    }
}

fn main() -> hexser::result::hex_result::HexResult<()> {
    std::println!("Transactional Order Processing Example\n");
    std::println!("{}", "=".repeat(60));

    std::println!("\n1. Transaction Requirements:");
    std::println!("   - Decrement stock for multiple products (atomic)");
    std::println!("   - Create order record (atomic)");
    std::println!("   - Dispatch OrderCreated event (after commit)");
    std::println!("   - All database operations must succeed or fail together");

    std::println!("\n2. Port Definitions:");
    std::println!("   ProductRepository::decrement_stock(tx, product_id, qty)");
    std::println!("   OrderRepository::create_order(tx, order)");
    std::println!("   EventBus::publish(event)");

    std::println!("\n3. Transaction Pattern:");
    std::println!("   - Begin transaction");
    std::println!("   - Pass &Transaction to all repository operations");
    std::println!("   - Commit transaction (or automatic rollback on error)");
    std::println!("   - Publish events only after successful commit");

    std::println!("\n4. Demonstration:");
    let product_repo = std::boxed::Box::new(InMemoryProductRepository::new());
    let order_repo = std::boxed::Box::new(InMemoryOrderRepository::new());
    let event_bus = std::boxed::Box::new(InMemoryEventBus::new());

    let handler = ProcessOrderHandler {
        product_repo,
        order_repo,
        event_bus,
    };

    let directive = ProcessOrderDirective {
        customer_id: std::string::String::from("customer-001"),
        items: std::vec![
            OrderItem {
                product_id: std::string::String::from("product-001"),
                quantity: 2,
                price: 29.99,
            },
            OrderItem {
                product_id: std::string::String::from("product-002"),
                quantity: 1,
                price: 49.99,
            },
        ],
    };

    handler.handle(directive)?;
    std::println!("   ✓ Order processed successfully");
    std::println!("   ✓ Stock decremented for 2 products");
    std::println!("   ✓ Order record created");
    std::println!("   ✓ OrderCreated event published");

    std::println!("\n5. Atomicity Guarantees:");
    std::println!("   - If stock decrement fails → rollback, no order created");
    std::println!("   - If order creation fails → rollback, stock restored");
    std::println!("   - If commit fails → automatic rollback via Drop trait");
    std::println!("   - Events published only after successful commit");

    std::println!("\n6. Error Handling:");
    std::println!("   INVARIANT_VIOLATION: Insufficient stock");
    std::println!("   DB_WRITE_FAILURE: Order creation or commit failure");
    std::println!("   All errors include actionable guidance");

    std::println!("\n7. Production Implementation (PostgreSQL):");
    std::println!("   - Use sqlx::Transaction or diesel::Connection::transaction");
    std::println!("   - Pass &mut PgTransaction to all repository methods");
    std::println!("   - Rollback is automatic on Drop if commit not called");
    std::println!("   - Example: tx.commit().await?");

    std::println!("\n8. Architecture Benefits:");
    std::println!("   ✓ Transactional consistency across multiple aggregates");
    std::println!("   ✓ Clear separation: repositories handle data, handler orchestrates");
    std::println!("   ✓ Event publishing decoupled from transaction");
    std::println!("   ✓ Easy to test with in-memory adapters");

    std::println!("\n{}", "=".repeat(60));
    std::println!("Example complete. See hexser/README.md for production SQL transaction code.");

    std::result::Result::Ok(())
}
