//! Diagonal infinite arrays.

use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};
use crate::arrays::{InfiniteArray, Shape};
use crate::infinity::Infinity;

/// Infinite diagonal matrix with values from a sequence
pub struct InfiniteDiagonal {
    values: Arc<dyn Fn(usize) -> f64 + Send + Sync>,
    value_cache: Arc<Mutex<HashMap<usize, f64>>>,
    shape: Shape,
    dtype: &'static str,
}

impl InfiniteDiagonal {
    pub fn new<F>(values: F) -> Self
    where
        F: Fn(usize) -> f64 + Send + Sync + 'static,
    {
        InfiniteDiagonal {
            values: Arc::new(values),
            value_cache: Arc::new(Mutex::new(HashMap::new())),
            shape: Shape::MultiD(vec![Some(Infinity), Some(Infinity)]),
            dtype: "f64",
        }
    }
    
    fn get_value(&self, i: usize) -> f64 {
        // Check cache
        {
            let cache = self.value_cache.lock().unwrap();
            if let Some(&value) = cache.get(&i) {
                return value;
            }
        }
        
        // Compute and cache
        let value = (self.values)(i);
        {
            let mut cache = self.value_cache.lock().unwrap();
            cache.insert(i, value);
        }
        
        value
    }
    
    pub fn get(&self, row: usize, col: usize) -> f64 {
        if row == col {
            self.get_value(row)
        } else {
            0.0
        }
    }
}

impl InfiniteArray for InfiniteDiagonal {
    fn get(&self, index: usize) -> f64 {
        // For 1D indexing, return diagonal element
        self.get_value(index)
    }
    
    fn get_multi(&self, indices: &[usize]) -> f64 {
        if indices.len() == 2 {
            self.get(indices[0], indices[1])
        } else if indices.len() == 1 {
            self.get(indices[0], indices[0])
        } else {
            panic!("Invalid index dimensions for diagonal matrix")
        }
    }
    
    fn shape(&self) -> Shape {
        self.shape.clone()
    }
    
    fn dtype(&self) -> &'static str {
        self.dtype
    }
}

impl fmt::Display for InfiniteDiagonal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "InfiniteDiagonal{}:", self.shape())?;
        let n = 15;
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    write!(f, "{}", self.get(i, j))?;
                } else if j == n - 1 {
                    write!(f, "…")?;
                    break;
                } else {
                    write!(f, "⋅")?;
                }
                if j < n - 1 {
                    write!(f, "  ")?;
                }
            }
            if i < n - 1 {
                write!(f, "\n")?;
            }
        }
        write!(f, "\n⋮")
    }
}

impl fmt::Debug for InfiniteDiagonal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "InfiniteDiagonal{}", self.shape())
    }
}

