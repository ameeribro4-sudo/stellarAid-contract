# Fuzz Tests for Campaign Contract

This directory contains fuzz tests for the critical financial functions in the campaign contract:
- `donate` - handles donations to the campaign
- `release_milestone` - releases funds for completed milestones

## Setup Requirements

1. Install cargo-fuzz:
   ```bash
   cargo install cargo-fuzz
   ```

2. Install nightly Rust toolchain (required for libfuzzer):
   ```bash
   rustup install nightly
   ```

## Running the Fuzz Tests

To run the donate function fuzz test:
```bash
cargo +nightly fuzz run fuzz_donate
```

To run the release_milestone function fuzz test:
```bash
cargo +nightly fuzz run fuzz_release_milestone
```

## What These Tests Do

The fuzz tests generate random inputs to exercise edge cases and potential overflow conditions:

### fuzz_donate
- Generates random donation amounts (including very large numbers that could cause overflow)
- Tests with random asset codes to validate asset validation logic
- Ensures that only expected errors are encountered, never unexpected panics

### fuzz_release_milestone
- Generates random milestone indices (including out-of-bounds indices)
- Tests milestone release order enforcement
- Verifies that only properly unlocked and sequential milestones can be released

## Expected Behavior
The fuzz tests should run indefinitely without finding any unexpected panics. If a panic occurs outside of the predefined error codes, it will be reported as a failure, and the failing input will be saved for debugging.