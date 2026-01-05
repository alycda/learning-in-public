{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustup
    just
    bacon
    presenterm
    # FFI dependencies
    pkg-config
    glib
    # Python for UniFFI bindings
    python3
    python3Packages.pip
    python3Packages.virtualenv
    # Kotlin for testing Kotlin bindings
    kotlin
    # Swift for testing Swift bindings (macOS has built-in Swift)
    # swift is available on macOS by default
  ];

  shellHook = ''
    rustup update

    # Set up Python virtual environment for UniFFI
    if [ ! -d ".venv" ]; then
      echo "Creating Python virtual environment..."
      python3 -m venv .venv
    fi

    source .venv/bin/activate

    # Install uniffi-bindgen if not present
    if ! python -c "import uniffi_bindgen" 2>/dev/null; then
      echo "Installing uniffi-bindgen..."
      pip install --quiet uniffi-bindgen==0.28.3
    fi

    echo "✓ Rust FFI development environment ready"
    echo "  - Rust toolchain with FFI support"
    echo "  - C dependencies (pkg-config, glib)"
    echo "  - Python with uniffi-bindgen for language bindings"
    echo "  - Kotlin compiler for testing Kotlin bindings"
    echo "  - Swift compiler available (system default on macOS)"
  '';
}