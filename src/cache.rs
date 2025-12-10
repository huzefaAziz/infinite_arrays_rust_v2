//! Caching functionality for infinite arrays to enable mutation.

use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};
use crate::arrays::{InfiniteArray, Shape};

/// Cached version of an infinite array that allows mutation
pub struct CachedArray {
    base: Arc<dyn InfiniteArray>,
    cache: Arc<Mutex<HashMap<usize, f64>>>,
    shape: Shape,
    dtype: &'static str,
}

impl CachedArray {
    pub fn new(array: Arc<dyn InfiniteArray>) -> Self {
        let shape = (*array).shape();
        let dtype = (*array).dtype();
        CachedArray {
            base: array,
            cache: Arc::new(Mutex::new(HashMap::new())),
            shape,
            dtype,
        }
    }
    
    pub fn set(&self, index: usize, value: f64) {
        let mut cache = self.cache.lock().unwrap();
        cache.insert(index, value);
    }
}

impl InfiniteArray for CachedArray {
    fn get(&self, index: usize) -> f64 {
        // Check cache first
        {
            let cache = self.cache.lock().unwrap();
            if let Some(&value) = cache.get(&index) {
                return value;
            }
        }
        
        // Otherwise get from base array
        let value = self.base.get(index);
        
        // Cache it
        {
            let mut cache = self.cache.lock().unwrap();
            cache.insert(index, value);
        }
        
        value
    }
    
    fn shape(&self) -> Shape {
        self.shape.clone()
    }
    
    fn dtype(&self) -> &'static str {
        self.dtype
    }
}

impl fmt::Display for CachedArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CachedArray{}:", self.shape())?;
        for i in 0..12 {
            write!(f, "\n  {}", self.get(i))?;
        }
        write!(f, "\n  ⋮")
    }
}

impl fmt::Debug for CachedArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CachedArray{}", self.shape())
    }
}

/// Convert an infinite array to a cached (mutable) version
pub fn cache(array: Arc<dyn InfiniteArray>) -> CachedArray {
    CachedArray::new(array)
}

