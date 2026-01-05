# Advent of Code, Rust-FFI edition

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