```bash
cargo install markdown-test-report
cargo install -f cargo-binutils
rustup component add llvm-tools-preview
```

Integration tests
```bash
cargo test --test *
cargo test --test * -- -Z unstable-options --report-time --format json > tests.json
```

Windows integration tests
```bash
cargo build --release
cargo test --release --test * -- -Z unstable-options --report-time --format json | Out-File -Encoding default tests.json
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