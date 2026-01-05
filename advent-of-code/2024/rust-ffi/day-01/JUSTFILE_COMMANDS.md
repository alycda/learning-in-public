# Just Commands Quick Reference

This project uses [just](https://github.com/casey/just) as a command runner. Run `just` to see all available commands.

## Basic Commands

| Command | Description |
|---------|-------------|
| `just` | Show all available commands |
| `just build` | Build the Rust library and binary (release mode) |
| `just build-lib` | Build only the library (for UniFFI bindings) |
| `just run` | Run the Rust binary (tests all 9 FFI implementations) |
| `just watch` | Watch for changes and rebuild (uses bacon) |
| `just bench` | Run criterion benchmarks |
| `just clean` | Clean build artifacts and generated bindings |

## UniFFI Bindings

| Command | Description |
|---------|-------------|
| `just gen-python` | Generate Python bindings |
| `just test-python` | Generate and test Python bindings |
| `just gen-kotlin` | Generate Kotlin bindings |
| `just gen-swift` | Generate Swift bindings |
| `just gen-all` | Generate bindings for all languages |
| `just bindings-status` | Show which bindings have been generated |

## Workflow Examples

### Test Rust implementations
```bash
just run
```

### Generate and test Python bindings
```bash
nix-shell  # Enter dev environment
just test-python
```

### Generate all language bindings
```bash
just gen-all
```

### Development workflow
```bash
# Terminal 1: Watch for changes
just watch

# Terminal 2: Run tests
just run

# Terminal 3: Test Python bindings
just test-python
```

### Benchmark different approaches
```bash
just bench
```

## Notes

- All commands run from the `day-01` directory (set in justfile)
- UniFFI commands automatically build the library first
- The nix-shell automatically installs `uniffi-bindgen` for you
- Generated bindings are placed in `day-01/bindings/{language}/`
