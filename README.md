```bash
cargo install -f cargo-binutils
rustup component add llvm-tools-preview
```

Integration tests
```bash
cargo test --test *
```

Unit tests
```bash
cargo test --lib
```

Doc tests
```bash
cargo test --doc
```