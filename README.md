```bash
https://nexte.st/book/installation.html
cargo install markdown-test-report
cargo install -f cargo-binutils
rustup component add llvm-tools-preview
```

Integration tests
```bash
cargo integration-o
```

Integration tests with html
```bash
cargo integration > tests.json
OR
cargo integration | Out-File -Encoding default tests.json
markdown-test-report tests.json -o tests.md
```

Unit tests
```bash
cargo test --lib
```

Doc tests
```bash
cargo test --doc
```

There are two options to run the tests:
```bash
cargo integration-o
```
and
```bash
cargo next-o
```

The first one uses the default runner and can share dependencies between tests. But does not run tests in the same spec in totally parallel.

The second one uses the nextest runner and ach individual test is executed as a separate process.

Both are really blazingly fast.
If you don't need to share dependencies between tests, use the second one.

In the case that is strictly required to share dependencies between tests because of some limitation,
you can check: https://nexte.st/book/test-groups.html or use the first one