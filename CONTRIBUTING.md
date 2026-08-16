# Contributing to razorpay-rs

Thank you for your interest in contributing to `razorpay-rs`! This guide explains how to set up the codebase locally, run tests, and submit pull requests.

---

## Workspace Structure

The project is structured as a standard Cargo workspace:

* [`razorpay/`](razorpay/): The core SDK crate (`razorpay-rs` on crates.io, imported in code as `razorpay`).
* [`examples/`](examples/): Standalone executable example binaries.
* [`tests/`](tests/): Integration tests, offline wiremock tests, and live API test suites.

---

## Getting Started

### Prerequisites

* [Rust](https://www.rust-lang.org/) (stable, 2024 edition supported).
* Optional: [Docker](https://www.docker.com/) for containerized test builds.

### Setting Up

Clone the repository:

```bash
git clone https://github.com/swar09/razorpay-rs.git
cd razorpay-rs
```

Verify your setup by running the unit test suite:

```bash
cargo test-unit
```

---

## Development Workflow

### 1. Code Formatting & Linting

All contributions must pass strict formatting and clippy linter checks:

```bash
# Check formatting
cargo fmt --all --check

# Format all code
cargo fmt --all

# Run linter with strict warning checks
cargo lint
```

### 2. Testing

#### Offline Unit Tests (Wiremock)
Unit tests run entirely offline against local mock servers:

```bash
cargo test-unit
```

#### Documentation Tests
Ensure all code examples in rustdoc compile:

```bash
cargo test --doc
```

#### Live Integration Tests (Optional)
To run live API tests against `api.razorpay.com`:

1. Copy `.env.example` to `.env`:
   ```bash
   cp .env.example .env
   ```
2. Add your Razorpay Test Key ID and Secret in `.env`:
   ```env
   RAZORPAY_KEY_ID=rzp_test_xxxx
   RAZORPAY_KEY_SECRET=xxxx
   ```
3. Run the live suite:
   ```bash
   cargo test-live
   ```

### 3. Local Documentation

Build and view the documentation locally in your browser:

```bash
cargo doc -p razorpay --no-deps --open
```

---

## Pull Request Guidelines

1. **Branching**: Create a feature branch from `master` (`git checkout -b feature/my-feature`).
2. **Commit Messages**: Use [Conventional Commits](https://www.conventionalcommits.org/) (e.g. `feat(payments): add payment capture endpoint`, `fix(models): handle optional count field`).
3. **No Secrets**: Never commit real API keys, secrets, or `.env` files.
4. **Verification Checklist**: Before submitting a PR, make sure:
   - `cargo fmt --all --check` passes.
   - `cargo lint` passes with 0 warnings.
   - `cargo test-unit` passes (50/50 tests).
   - `cargo test --doc` passes.

---

## Community & Code of Conduct

Please be respectful, constructive, and helpful when opening issues or participating in discussions.
