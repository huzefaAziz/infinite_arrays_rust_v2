//! Broadcasting support for infinite arrays.

use std::fmt;
use crate::arrays::{InfiniteArray, Shape};

/// Lazy broadcasted array that computes values on-demand
pub struct BroadcastArray {
    func: Box<dyn Fn(usize) -> f64 + Send + Sync>,
    shape: Shape,
    dtype: &'static str,
}

impl BroadcastArray {
    pub fn new<F>(func: F, shape: Shape) -> Self
    where
        F: Fn(usize) -> f64 + Send + Sync + 'static,
    {
        BroadcastArray {
            func: Box::new(func),
            shape,
            dtype: "f64",
        }
    }
}

impl InfiniteArray for BroadcastArray {
    fn get(&self, index: usize) -> f64 {
        (self.func)(index)
    }
    
    fn shape(&self) -> Shape {
        self.shape.clone()
    }
    
    fn dtype(&self) -> &'static str {
        self.dtype
    }
}

impl fmt::Display for BroadcastArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BroadcastArray{}:", self.shape())?;
        for i in 0..12 {
            write!(f, "\n  {}", self.get(i))?;
        }
        write!(f, "\n  ⋮")
    }
}

impl fmt::Debug for BroadcastArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BroadcastArray{}", self.shape())
    }
}

