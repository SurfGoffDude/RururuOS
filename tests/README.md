# RururuOS Testing

## Test Structure

```
tests/
├── unit/           # Unit tests (per-package)
├── integration/    # Integration tests
├── hardware/       # Hardware compatibility tests
├── performance/    # Performance benchmarks
└── e2e/            # End-to-end tests
```

## Running Tests

### Unit Tests

```bash
cargo test --workspace
```

### Specific Package

```bash
cargo test -p rururu-file-handler
cargo test -p rururu-color
```

### Integration Tests

```bash
cargo test --test integration
```

### Performance Benchmarks

```bash
cargo bench
```

## Hardware Compatibility Matrix

See `hardware/compatibility.md` for supported hardware list.

## Codec Compatibility

See `codecs/compatibility.md` for codec support matrix.
