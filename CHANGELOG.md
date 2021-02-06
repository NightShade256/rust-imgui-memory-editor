# Changelog

All changes to the crate will be documented here.

## v0.3.0

### Changes

- Getters and Setters of `HighlightColor` now take in `ImColor32` due to an
  upstream change.

- The crates are now dual-licensed under the terms of MIT or Apache-2.0.

- Switch to `chlorine` from `cty` as a provider of system C type aliases.

- Update dependencies and submodules to latest versions.

## v0.2.0

### Changes

- Changed `get_cols` and `set_cols` to require/return a `cty::c_int` instead
  of a `i32`.

### Additions

- Added getters and setters for all the remaining fields.
  The bindings can now be considered "feature complete".

## v0.1.0

Initial Release

- Wraps the original widget in a rust-y way.
- Supports getters and setters for most of the fields.
