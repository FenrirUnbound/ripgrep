// Macros useful for testing.
#[macro_use]
mod macros;

// Corpora and utilities for testing.
mod hay;
mod util;

// Tests, grouped by tests for features, regression tests and miscellaneous
// tests. Generally, tests should fall into either the 'feature' or
// 'regression' modules.
mod feature;
mod misc;
mod regression;
