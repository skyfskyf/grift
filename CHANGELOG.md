# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Source location tracking (line/column) in `ArenaError::ParseError` for
  improved error diagnostics.
- MSRV (Minimum Supported Rust Version) policy: **Rust 1.85** (edition 2024).
  The `rust-version` field is set in all workspace crates.
- Miri CI job to detect undefined behaviour edge cases.
- MSRV CI job to verify the minimum supported Rust version.
- `clippy::pedantic` lint group enabled (warn level) for `grift` and
  `grift_arena` crates.
- `CHANGELOG.md` for release tracking.
- `SPECIFICATION.md` formal reference specification for the Grift dialect.
- Criterion benchmarks for arena allocation, GC, and eval hot paths.
- Property-based tests (proptest) for parser and evaluator fuzz testing.

### Changed

- Upgraded `#![warn(missing_docs)]` to `#![deny(missing_docs)]` in `grift`
  and `grift_arena` crates.

### Fixed

- CI `no-std-check` job referenced non-existent crates (`grift_core`,
  `grift_parser`, `grift_eval`); corrected to actual workspace crates
  (`grift`, `grift_unicode`).

## [1.5.0] - 2025-01-01

### Added

- Initial public release of the Grift Lisp interpreter.
- `no_std`, `no_alloc` arena allocator with mark-and-sweep GC.
- Kernel-style vau calculus (fexprs) with tail-call optimization.
- Standard library with `map`, `filter`, `append`, `length`, and more.
- Unicode character operations via `grift_unicode` crate.
- Proc-macro stdlib loader via `grift_macros` crate.
