#!/usr/bin/env python3
"""
Generate Python bindings using uniffi-bindgen-python

Install first with: pip install uniffi-bindgen==0.28.3
"""

import subprocess
import sys
from pathlib import Path

def main():
    project_root = Path(__file__).parent
    lib_file = project_root / "target" / "release" / "libaoc_ffi_day01.dylib"
    udl_file = project_root / "src" / "aoc_ffi_day01.udl"
    output_dir = project_root / "bindings" / "python"

    output_dir.mkdir(parents=True, exist_ok=True)

    print("Generating Python bindings with uniffi-bindgen...")

    try:
        result = subprocess.run([
            "uniffi-bindgen",
            "generate",
            str(udl_file),
            "--lib-file", str(lib_file),
            "--language", "python",
            "--out-dir", str(output_dir)
        ], check=True, capture_output=True, text=True)

        print(result.stdout)
        print(f"✓ Bindings generated in {output_dir}/")

    except FileNotFoundError:
        print("Error: uniffi-bindgen not found")
        print("Install with: pip install uniffi-bindgen==0.28.3")
        sys.exit(1)
    except subprocess.CalledProcessError as e:
        print(f"Error: {e.stderr}")
        sys.exit(1)

if __name__ == "__main__":
    main()
