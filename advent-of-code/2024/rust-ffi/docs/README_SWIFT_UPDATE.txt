Replace:
### ⚠️ Partial Support

- **Swift**: Bindings generate successfully but tests fail at runtime
  - Generate: `just gen-swift` (in nix-shell)
  - 11 of 12 implementations available (GLib excluded)
  - See [SWIFT_WORKFLOW.md](SWIFT_WORKFLOW.md) and [day-01/SWIFT_LIMITATIONS.md](day-01/SWIFT_LIMITATIONS.md) for details

With:
### ✅ Swift - Now Working!

- **Swift**: All 11 implementations working (GLib excluded for compatibility)
  - Generate: `just gen-swift` (in nix-shell)
  - Test: `just test-swift` (OUTSIDE nix-shell)
  - **Breakthrough**: Fixed by downgrading thiserror to 1.0 and avoiding C-style format strings
  - See [SWIFT_BREAKTHROUGH.md](SWIFT_BREAKTHROUGH.md) for the full story
