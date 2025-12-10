//! InfiniteArrays - A Rust library for representing arrays with infinite dimension sizes.
//!
//! This library provides lazy infinite arrays designed to work with numerical computing.
//! It is inspired by and converted from InfiniteArrays.jl.

pub mod infinity;
pub mod ranges;
pub mod arrays;
pub mod broadcasting;
pub mod cache;
pub mod diagonal;
pub mod iqr;
pub mod utils;

// Re-export main types and functions
pub use infinity::Infinity;
pub use ranges::{OneToInf, InfUnitRange, InfStepRange};
pub use arrays::{InfiniteArray, Ones, Zeros, Fill};
pub use broadcasting::BroadcastArray;
pub use cache::{cache, CachedArray};
pub use diagonal::InfiniteDiagonal;

/// Infinity constant for specifying infinite dimensions
pub const INFINITY: Infinity = Infinity;

