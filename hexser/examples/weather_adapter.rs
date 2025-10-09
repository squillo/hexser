//! Weather REST Adapter Example
//!
//! Demonstrates implementing a Hexser adapter that connects to an external REST API.
//! This example shows how to define a WeatherPort trait, create a domain Forecast model,
//! and implement a REST adapter using reqwest to fetch and map JSON responses.
//!
//! This addresses Context7 Question 7: Implement a Hexser 'Adapter' that connects to
//! an external REST API with proper error handling and domain mapping.
//!
//! Run with: cargo run --example weather_adapter
//!
//! Revision History
//! - 2025-10-08T23:04:00Z @AI: Remove rest-adapter feature gates; reqwest is now a dev-dependency.
//! - 2025-10-08T22:54:00Z @AI: Make example self-contained with internal type definitions.
//! - 2025-10-08T22:43:00Z @AI: Extract weather adapter example from README to standalone file per user request.

use hexser::error::RichError;

// Domain Model: Forecast value object
#[derive(Clone, Debug, PartialEq)]
struct Forecast {
    city: String,
    temperature_c: f64,
    condition: String,
    observed_at_iso: Option<String>,
}

impl Forecast {
    fn new(city: String, temperature_c: f64, condition: String, observed_at_iso: Option<String>) -> hexser::result::hex_result::HexResult<Self> {
        if city.trim().is_empty() {
            return Result::Err(
                hexser::error::hex_error::Hexserror::validation_field(
                    "City must not be empty",
                    "city",
                )
            );
        }
        if !condition.chars().any(|c| c.is_alphanumeric()) {
            return Result::Err(
                hexser::error::hex_error::Hexserror::validation_field(
                    "Condition must contain letters or numbers",
                    "condition",
                )
            );
        }
        Result::Ok(Self { city, temperature_c, condition, observed_at_iso })
    }

    fn city(&self) -> &str { &self.city }
    fn temperature_c(&self) -> f64 { self.temperature_c }
    fn condition(&self) -> &str { &self.condition }
    fn observed_at_iso(&self) -> Option<&str> { self.observed_at_iso.as_deref() }
}

// Port Definition: WeatherPort trait
trait WeatherPort {
    fn get_forecast(&self, city: &str) -> hexser::result::hex_result::HexResult<Forecast>;
}

// Adapter Implementation: REST-based weather adapter
struct RestWeatherAdapter {
    api_base_url: String,
    client: reqwest::blocking::Client,
}

impl RestWeatherAdapter {
    fn new(api_base_url: String) -> Self {
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .expect("Failed to build reqwest client");
        Self { api_base_url, client }
    }
}

impl WeatherPort for RestWeatherAdapter {
    fn get_forecast(&self, city: &str) -> hexser::result::hex_result::HexResult<Forecast> {
        if city.trim().is_empty() {
            return Result::Err(
                hexser::error::hex_error::Hexserror::validation_field("City must not be empty", "city")
            );
        }

        let url = format!("{}?city={}", self.api_base_url, city);

        let response = self.client.get(&url)
            .send()
            .map_err(|e| {
                let adapter_err = hexser::error::adapter_error::AdapterError::new(
                    hexser::error::codes::adapter::API_FAILURE,
                    "Failed to connect to weather API"
                )
                .with_source(e)
                .with_next_steps(&["Verify API endpoint", "Check network connectivity"])
                .with_suggestion("Ensure the API URL is correct and reachable");
                hexser::error::hex_error::Hexserror::Adapter(adapter_err)
            })?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            return Result::Err(
                hexser::error::hex_error::Hexserror::adapter(
                    hexser::error::codes::adapter::API_FAILURE,
                    &format!("Weather API returned error status {}", status)
                )
                .with_next_step("Check API documentation for error codes")
            );
        }

        let body = response.text().map_err(|e| {
            let adapter_err = hexser::error::adapter_error::AdapterError::new(
                hexser::error::codes::adapter::MAPPING_FAILURE,
                "Failed to read response body"
            ).with_source(e);
            hexser::error::hex_error::Hexserror::Adapter(adapter_err)
        })?;

        let api_response: ApiWeatherResponse = serde_json::from_str(&body).map_err(|e| {
            let adapter_err = hexser::error::adapter_error::AdapterError::new(
                hexser::error::codes::adapter::MAPPING_FAILURE,
                "Failed to parse JSON response"
            )
            .with_source(e)
            .with_next_step("Verify API response structure matches expected schema");
            hexser::error::hex_error::Hexserror::Adapter(adapter_err)
        })?;

        Forecast::new(
            api_response.city,
            api_response.temp_c,
            api_response.condition,
            api_response.observed_at,
        ).map_err(|e| {
            let adapter_err = hexser::error::adapter_error::AdapterError::new(
                hexser::error::codes::adapter::MAPPING_FAILURE,
                "Domain validation failed for API response"
            )
            .with_source(e)
            .with_next_step("Ensure API returns valid city and condition fields");
            hexser::error::hex_error::Hexserror::Adapter(adapter_err)
        })
    }
}

#[derive(serde::Deserialize)]
struct ApiWeatherResponse {
    city: String,
    temp_c: f64,
    condition: String,
    observed_at: Option<String>,
}

fn main() -> hexser::result::hex_result::HexResult<()> {
    std::println!("Weather REST Adapter Example\n");
    std::println!("{}", "=".repeat(60));

    // Demonstrate the domain model
    std::println!("\n1. Domain Model (Forecast):");
    std::println!("   - Represents weather forecast data");
    std::println!("   - Pure domain logic, no infrastructure dependencies");

    let forecast = Forecast::new(
        std::string::String::from("San Francisco"),
        15.5,
        std::string::String::from("Partly Cloudy"),
        std::option::Option::Some(std::string::String::from("2025-10-08T22:00:00Z")),
    )?;

    std::println!("   Created forecast: {}", forecast.city());
    std::println!("   Temperature: {}°C", forecast.temperature_c());
    std::println!("   Condition: {}", forecast.condition());

    // Demonstrate the port (interface)
    std::println!("\n2. Port Definition (WeatherPort trait):");
    std::println!("   - Abstract interface for weather data");
    std::println!("   - Domain-driven contract, technology-agnostic");
    std::println!("   - Signature: fn get_forecast(&self, city: &str) -> HexResult<Forecast>");

    // Demonstrate the adapter
    std::println!("\n3. Adapter Implementation (RestWeatherAdapter):");
    std::println!("   - Concrete implementation using reqwest HTTP client");
    std::println!("   - Maps external JSON to domain Forecast model");
    std::println!("   - Rich error handling with API_FAILURE and MAPPING_FAILURE codes");

    std::println!("\n4. Example Usage (with mock server):");
    std::println!("   Note: This example would connect to a real API endpoint");
    std::println!("   For demonstration, we show the adapter structure:");

    let _adapter = RestWeatherAdapter::new(
        std::string::String::from("https://api.example.com/weather")
    );

    std::println!("   ✓ Adapter created with base URL");
    std::println!("   ✓ HTTP client configured with 10s timeout");
    std::println!("   ✓ Ready to fetch forecasts");

    // In a real scenario, you would call:
    // let forecast = adapter.get_forecast("London")?;
    // For this example, we demonstrate the error handling structure

    std::println!("\n5. Error Handling Layers:");
    std::println!("   API_FAILURE: Network/HTTP errors");
    std::println!("     - Source error preserved via with_source()");
    std::println!("     - Actionable guidance: 'Verify API endpoint', 'Check network'");
    std::println!("   MAPPING_FAILURE: JSON deserialization errors");
    std::println!("     - Source error preserved via with_source()");
    std::println!("     - Clear indication of data structure issues");

    std::println!("\n6. Architecture Benefits:");
    std::println!("   ✓ Port (WeatherPort) is technology-agnostic");
    std::println!("   ✓ Can swap RestWeatherAdapter for MockWeatherAdapter in tests");
    std::println!("   ✓ Can add CachedWeatherAdapter without changing domain");
    std::println!("   ✓ Domain logic (Forecast) has zero infrastructure dependencies");

    std::println!("\n7. Key Implementation Details:");
    std::println!("   - reqwest::blocking::Client for synchronous HTTP calls");
    std::println!("   - serde_json for JSON deserialization");
    std::println!("   - Hexserror::adapter() for layer-specific error creation");
    std::println!("   - with_source() preserves underlying error chain");
    std::println!("   - with_next_steps() provides actionable guidance");

    std::println!("\n{}", "=".repeat(60));
    std::println!("Example complete. All implementation details are in this file:");
    std::println!("hexser/examples/weather_adapter.rs");

    std::result::Result::Ok(())
}
