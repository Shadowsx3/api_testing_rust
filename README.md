# Automation API Testing on Rust

This project aims to provide a robust and efficient solution for automating API testing in Rust. Follow the steps below to set up and run the tests.

## Prerequisites

Before running the tests, ensure you have the following prerequisites installed:

- Rust: To install Rust, follow the instructions at [https://nexte.st/book/installation.html](https://nexte.st/book/installation.html).
- `markdown-test-report`: Install it using Cargo with the following command:

```bash
cargo install markdown-test-report
```

- `cargo-binutils`: Install it using Cargo with the following command:

```bash
cargo install -f cargo-binutils
```

- `llvm-tools-preview`: Add the LLVM tools preview component using Rustup:

```bash
rustup component add llvm-tools-preview
```

- `next-test`: Install it using Cargo with the following command:

```bash
cargo install next-test
```

## Running Tests

### Integration Tests

To run integration tests, use the following command:

```bash
cargo integration-o
```

### Integration Tests with HTML Output

You can run integration tests and generate an HTML report using the following commands:

```bash
cargo integration-j > tests.json
# OR
cargo integration-j | Out-File -Encoding default tests.json
markdown-test-report tests.json -o tests.md
```

### Unit Tests

To run unit tests, execute the following command:

```bash
cargo test --lib
```

### Doc Tests

Run doc tests using the following command:

```bash
cargo test --doc
```

## Test Execution Options

There are two options to run the tests:

### Option 1: Using Default Runner

To use the default runner, which allows sharing dependencies between tests, but does not run tests in the same spec in parallel, use the following command:

```bash
cargo integration-o
```

### Option 2: Using Nextest Runner

The second option utilizes the nextest runner, where each individual test is executed as a separate process. This option is incredibly fast and is recommended when you don't need to share dependencies between tests. To use this runner, execute:

```bash
cargo next-o
```

If you have specific requirements that demand sharing dependencies between tests despite any limitations, refer to [https://nexte.st/book/test-groups.html](https://nexte.st/book/test-groups.html) for more information or use the first option.

Choose the appropriate test execution option based on your project's requirements and enjoy efficient API testing in Rust!