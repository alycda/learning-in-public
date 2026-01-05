# Advent of Code, Rust-FFI edition

A learning project exploring Foreign Function Interfaces (FFI) in both directions:
- **Rust → C**: 9 different FFI implementations calling C functions from Rust
- **Rust → Other Languages**: UniFFI bindings for Python, Kotlin, and Swift

## UniFFI Language Bindings

This project uses [UniFFI](https://mozilla.github.io/uniffi-rs/) to generate bindings from Rust to other languages:

### ✅ Working Bindings

- **Python**: All 12 implementations working
  - Test: `just test-python` (in nix-shell)
- **Kotlin**: All 12 implementations working
  - Test: `just test-kotlin` (in nix-shell)

### ⚠️ Partial Support

- **Swift**: Bindings generate successfully but tests fail at runtime
  - Generate: `just gen-swift` (in nix-shell)
  - 11 of 12 implementations available (GLib excluded)
  - See [SWIFT_WORKFLOW.md](SWIFT_WORKFLOW.md) and [day-01/SWIFT_LIMITATIONS.md](day-01/SWIFT_LIMITATIONS.md) for details

## Development Environment

### Option 1: Using Nix + direnv (Recommended)

This project uses Nix and direnv for reproducible development environments.

1. Install [direnv](https://direnv.net/docs/installation.html) if you haven't already
2. Hook direnv into your shell (add to your `.bashrc`/`.zshrc`):
   ```bash
   eval "$(direnv hook bash)"  # or 'zsh', 'fish', etc.
   ```
3. cd into this directory and allow direnv:
   ```bash
   direnv allow
   ```

The environment will automatically load when you enter the directory and unload when you leave.

### Option 2: Manual Installation

If you don't want to use Nix, you'll need these prerequisites installed:
- [Rust](https://rustup.rs/) (via rustup)
- [just](https://github.com/casey/just#installation)
- [bacon](https://github.com/Canop/bacon#installation)
- [presenterm](https://github.com/mfontanini/presenterm#installation) (optional)