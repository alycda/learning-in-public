#!/usr/bin/env python3
"""
Generate Python bindings for the Rust library using UniFFI.
This script uses the uniffi package to generate Python bindings from the UDL file.
"""

import subprocess
import sys
from pathlib import Path

def main():
    # Paths
    project_root = Path(__file__).parent
    udl_file = project_root / "src" / "aoc_ffi_day01.udl"
    lib_file = project_root / "target" / "release" / "libaoc_ffi_day01.dylib"
    output_dir = project_root / "bindings" / "python"

    # Create output directory
    output_dir.mkdir(parents=True, exist_ok=True)

    # Build the library first
    print("Building Rust library...")
    subprocess.run(["cargo", "build", "--release", "--lib"],
                   cwd=project_root, check=True)

    # Generate Python bindings using cargo uniffi-bindgen
    print(f"Generating Python bindings...")
    print(f"  UDL: {udl_file}")
    print(f"  Library: {lib_file}")
    print(f"  Output: {output_dir}")

    # Try using the uniffi-bindgen binary from cargo
    try:
        result = subprocess.run([
            "cargo", "run", "--manifest-path",
            str(project_root / "Cargo.toml"),
            "--features", "uniffi/cli",
            "--bin", "uniffi-bindgen",
            "generate",
            "--library", str(lib_file),
            "--language", "python",
            "--out-dir", str(output_dir)
        ], capture_output=True, text=True)

        if result.returncode != 0:
            print(f"Error: {result.stderr}")
            print("\nTrying alternative method...")
            raise Exception("First method failed")

    except:
        # Alternative: use Python uniffi package if installed
        try:
            from uniffi_bindgen import generate_bindings
            print("Using Python uniffi_bindgen package...")
            generate_bindings(str(udl_file), "python", str(output_dir), str(lib_file))
        except ImportError:
            print("\nError: Could not generate bindings.")
            print("Please install uniffi-bindgen:")
            print("  pip install uniffi-bindgen")
            sys.exit(1)

    print(f"\n✓ Python bindings generated in {output_dir}/")
    print(f"  Files:")
    for f in output_dir.iterdir():
        print(f"    - {f.name}")

if __name__ == "__main__":
    main()
