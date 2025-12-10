//! Infinity constant and related utilities.

use std::fmt;
use std::cmp::Ordering;

/// Represents infinity for array dimensions.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Infinity;

impl fmt::Display for Infinity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "∞")
    }
}

impl fmt::Debug for Infinity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "∞")
    }
}

impl PartialOrd for Infinity {
    fn partial_cmp(&self, _other: &Infinity) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

impl Ord for Infinity {
    fn cmp(&self, _other: &Infinity) -> Ordering {
        Ordering::Equal
    }
}

impl Infinity {
    /// Check if a value is infinity
    pub fn is_infinity(value: &dyn std::any::Any) -> bool {
        value.is::<Infinity>()
    }
}

impl From<Infinity> for f64 {
    fn from(_: Infinity) -> Self {
        f64::INFINITY
    }
}

impl From<Infinity> for usize {
    fn from(_: Infinity) -> Self {
        usize::MAX
    }
}

