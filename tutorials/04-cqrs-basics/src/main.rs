//! Tutorial 04: CQRS Basics
//!
//! Learn Command Query Responsibility Segregation with directives and queries.

use hexer::prelude::*;

#[derive(HexDomain, Entity, Clone)]
struct Order {
    id: String,
    customer_id: String,
    total: f64,
}

#[derive(HexDirective)]
struct CreateOrderDirective {
    customer_id: String,
    total: f64,
}

struct CreateOrderHandler {
    next_id: u32,
}

impl CreateOrderHandler {
    fn new() -> Self {
        Self { next_id: 1 }
    }
}

impl DirectiveHandler<CreateOrderDirective> for CreateOrderHandler {
    fn handle(&self, directive: CreateOrderDirective) -> HexResult<()> {
        if directive.total <= 0.0 {
            return Err(HexError::validation("Total must be positive")
                .with_field("total"));
        }

        println!("Creating order:");
        println!("  ID: {}", self.next_id);
        println!("  Customer: {}", directive.customer_id);
        println!("  Total: ${:.2}", directive.total);

        Ok(())
    }
}

#[derive(HexQuery)]
struct GetOrdersByCustomerQuery {
    customer_id: String,
}

struct GetOrdersHandler {
    orders: Vec<Order>,
}

impl GetOrdersHandler {
    fn new() -> Self {
        Self {
            orders: vec![
                Order {
                    id: String::from("1"),
                    customer_id: String::from("customer_123"),
                    total: 99.99,
                },
            ],
        }
    }
}

impl QueryHandler<GetOrdersByCustomerQuery, Vec<Order>> for GetOrdersHandler {
    fn handle(&self, query: GetOrdersByCustomerQuery) -> HexResult<Vec<Order>> {
        let results: Vec<Order> = self
            .orders
            .iter()
            .filter(|o| o.customer_id == query.customer_id)
            .cloned()
            .collect();

        Ok(results)
    }
}

fn main() {
    println!("Tutorial 04: CQRS Basics\n");
    println!("{}", "=".repeat(50));

    println!("\nWrite Path (Directive):");
    let directive_handler = CreateOrderHandler::new();
    let directive = CreateOrderDirective {
        customer_id: String::from("customer_123"),
        total: 149.99,
    };
    directive_handler.handle(directive).unwrap();

    println!("\nRead Path (Query):");
    let query_handler = GetOrdersHandler::new();
    let query = GetOrdersByCustomerQuery {
        customer_id: String::from("customer_123"),
    };
    let orders = query_handler.handle(query).unwrap();
    println!("  Found {} orders", orders.len());

    let graph = HexGraph::current();

    println!("\nArchitecture:");
    graph.pretty_print();

    println!("\nCQRS Components:");
    println!("  Directives: {}", graph.nodes_by_role(Role::Directive).len());
    println!("  Queries: {}", graph.nodes_by_role(Role::Query).len());

    println!("\nKey insights:");
    println!("  Writes go through directives (validation, business logic)");
    println!("  Reads go through queries (optimized for retrieval)");
    println!("  Separate concerns enable independent scaling");

    println!("\n{}", "=".repeat(50));
    println!("Next: Tutorial 05 - Graph Analysis");
}
