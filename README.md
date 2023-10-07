# Rust API Automation Testing Framework

Welcome to the Rust API Automation Testing Framework. This project is designed to offer a robust and efficient solution for automating API testing in Rust. Below, you'll find comprehensive information on how to set up and run tests, as well as an overview of our testing patterns and best practices.

## Prerequisites

Before you can start running tests, please make sure you have the following prerequisites installed:

- **NextTest**: NextTest is the backbone of our testing framework. To install NextTest, please follow the instructions available at [https://nexte.st/book/installation.html](https://nexte.st/book/installation.html).

- **Environment Variables**: Copy the `example.env` file to `.env` and populate it with the necessary environment variables specific to your testing environment.

## Running Tests

Our testing framework encompasses different types of tests to ensure thorough and comprehensive coverage.

### Integration Tests

Integration tests are designed to validate the interactions between various components of your API. To run integration tests, use the following command:

```bash
cargo next
```

### Unit Tests

Unit tests focus on testing individual functions and methods within your automation code. To execute unit tests, use this command:

```bash
cargo test --lib
```

### Doc Tests

Doc tests play a vital role in verifying the accuracy of your code's documentation. Run doc tests using this command:

```bash
cargo test --doc
```

## Project Structure

Our project follows a well-organized structure to maintain clarity and organization. Here's an overview:

```bash
├── Cargo.lock
├── Cargo.toml
├── README.md
├── example.env
├── src
│   ├── base
│   │   ├── types
│   │   │   ├── config_types.rs
│   │   ├── api_client.rs # Base API client
│   │   ├── middleware.rs # Middleware for intercepting requests and responses
│   │   ├── mock_server.rs # Mock server for testing
│   ├── models # Structs for each service in the API
│   │   ├── requests
│   │   ├── responses 
│   │   ├── shared
│   ├── services # Groups of functions that interact with the API
│   ├── utils
```

## Testing Patterns and Guidelines

### Fixtures

We employ the fixture testing pattern in this project. Each test relies on specific systems and features, which we refer to as fixtures. Fixtures are created and destroyed before and after each test to ensure independence. Tests should not rely on each other, and if dependencies exist, is better to include the assertions in the same test.

### Testing Services with Connection Limits

When testing services with connection limits, use test groups and thread limits. This approach allows tests to run in parallel without exceeding connection limits. For detailed information, refer to the [NextTest documentation](https://nexte.st/book/test-groups.html).

### Unit Tests

To maintain high-quality testing, we emphasize testing functions and methods within the automation code. This practice ensures that the code works as expected and that potential failures are confined to API and integration tests logic. Consider organizing tests for each service within the same file as the service code and using the `tests` folder primarily for integration tests.

### Doc Tests

Doc tests validate code documentation to ensure it remains accurate and aligned with code functionality.

## CI/CD

Our continuous integration and continuous deployment (CI/CD) pipeline is powered by GitHub Actions, configured in the `.github/workflows/ci.yml` file. This pipeline automates test execution and deployment processes, ensuring code quality and reliability.

Thank you for choosing our Rust API Automation Testing Framework. We are committed to providing a robust testing solution for your API. If you have any questions or encounter issues, please reach out to us. Happy testing!