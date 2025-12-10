//! Internal utility functions for infinite arrays.

use crate::infinity::Infinity;

/// Get the infinity instance
pub fn get_infinity() -> Infinity {
    Infinity
}

/// Check if a value is infinity
pub fn is_infinity<T: 'static>(_value: &T) -> bool {
    std::any::TypeId::of::<Infinity>() == std::any::TypeId::of::<T>()
}

