/*
CLIPLY by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Declaring the "modules"
/// directory as a module.
pub mod modules;

/// Re-exporting the main module
/// for this crate.
pub use modules::cliply::*;

/// Re-exporting the module
/// to handle any errors
/// that might occur.
pub use modules::errors::*;