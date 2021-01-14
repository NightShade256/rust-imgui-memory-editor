# Changelog

All changes to the crate will be documented here.

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
