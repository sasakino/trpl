//! # Cargo and Crate.io
//! 
//! `cargo_and_cratesio` is a collection of utilities tomake performing certain
//! calculations more convenient.

/// Adds one to the number given.
/// 
/// # Examples
/// 
/// ```
/// let five = 5;
/// 
/// assert_eq!(6, cargo_and_cratesio::add_one(5));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}