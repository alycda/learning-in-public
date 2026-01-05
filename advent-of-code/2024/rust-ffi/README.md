# Advent of Code, Rust-FFI edition

A learning project exploring Foreign Function Interfaces (FFI) in both directions:
- **Rust → C**: 9 different FFI implementations calling C functions from Rust
- **Rust → Other Languages**: UniFFI bindings for Python, Kotlin, and Swift

## 📊 Presentation

View the complete FFI learning journey:
```bash
nix-shell
just present
```

Or manually with presenterm:
```bash
presenterm ffi-learning-presentation.md
```

## UniFFI Language Bindings

This project uses [UniFFI](https://mozilla.github.io/uniffi-rs/) to generate bindings from Rust to other languages:

### ✅ Working Bindings

- **Python**: All 12 implementations working
  - Test: `just test-python` (in nix-shell)
- **Kotlin**: All 12 implementations working
  - Test: `just test-kotlin` (in nix-shell)

### ✅ Swift - Now Working!

- **Swift**: All 11 implementations working (GLib excluded for compatibility)
  - Generate: `just gen-swift` (in nix-shell)
  - Test: `just test-swift` (OUTSIDE nix-shell)
  - **Breakthrough**: Fixed by downgrading `thiserror` to 1.0 and avoiding C-style format strings
  - See [SWIFT_BREAKTHROUGH.md](./docs/SWIFT_BREAKTHROUGH.md) for the full story

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