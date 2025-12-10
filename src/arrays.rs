//! Infinite array types.

use std::fmt;
use crate::infinity::Infinity;

/// Base trait for infinite arrays
pub trait InfiniteArray: Send + Sync {
    /// Get item at index (0-based)
    fn get(&self, index: usize) -> f64;
    
    /// Get item at multi-dimensional index
    fn get_multi(&self, indices: &[usize]) -> f64 {
        if indices.len() == 1 {
            self.get(indices[0])
        } else {
            panic!("Multi-dimensional indexing not yet fully supported")
        }
    }
    
    /// Get the shape of the array
    fn shape(&self) -> Shape;
    
    /// Get the dtype (represented as a string for simplicity)
    fn dtype(&self) -> &'static str {
        "f64"
    }
}

/// Shape representation (can contain Infinity)
#[derive(Debug, Clone)]
pub enum Shape {
    Scalar,
    OneD(Option<Infinity>),
    MultiD(Vec<Option<Infinity>>),
}

impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Shape::Scalar => write!(f, "()"),
            Shape::OneD(None) => write!(f, "(finite)"),
            Shape::OneD(Some(_)) => write!(f, "(∞)"),
            Shape::MultiD(dims) => {
                let dim_strs: Vec<String> = dims.iter().map(|d| {
                    match d {
                        None => "finite".to_string(),
                        Some(_) => "∞".to_string(),
                    }
                }).collect();
                write!(f, "({})", dim_strs.join(", "))
            }
        }
    }
}

/// Infinite array filled with ones
pub struct Ones {
    shape: Shape,
    dtype: &'static str,
}

impl Ones {
    pub fn new(shape: Option<Shape>) -> Self {
        let shape = shape.unwrap_or_else(|| Shape::OneD(Some(Infinity)));
        Ones {
            shape,
            dtype: "f64",
        }
    }
}

impl InfiniteArray for Ones {
    fn get(&self, _index: usize) -> f64 {
        1.0
    }
    
    fn shape(&self) -> Shape {
        self.shape.clone()
    }
    
    fn dtype(&self) -> &'static str {
        self.dtype
    }
}

impl fmt::Display for Ones {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ones{}:", self.shape())?;
        for i in 0..12 {
            write!(f, "\n  {}", self.get(i))?;
        }
        write!(f, "\n  ⋮")
    }
}

impl fmt::Debug for Ones {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ones{}", self.shape())
    }
}

/// Infinite array filled with zeros
pub struct Zeros {
    shape: Shape,
    dtype: &'static str,
}

impl Zeros {
    pub fn new(shape: Option<Shape>) -> Self {
        let shape = shape.unwrap_or_else(|| Shape::OneD(Some(Infinity)));
        Zeros {
            shape,
            dtype: "f64",
        }
    }
}

impl InfiniteArray for Zeros {
    fn get(&self, _index: usize) -> f64 {
        0.0
    }
    
    fn shape(&self) -> Shape {
        self.shape.clone()
    }
    
    fn dtype(&self) -> &'static str {
        self.dtype
    }
}

impl fmt::Display for Zeros {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Zeros{}:", self.shape())?;
        for i in 0..12 {
            write!(f, "\n  {}", self.get(i))?;
        }
        write!(f, "\n  ⋮")
    }
}

impl fmt::Debug for Zeros {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Zeros{}", self.shape())
    }
}

/// Infinite array filled with a constant value
pub struct Fill {
    value: f64,
    shape: Shape,
    dtype: &'static str,
}

impl Fill {
    pub fn new(value: f64, shape: Option<Shape>) -> Self {
        let shape = shape.unwrap_or_else(|| Shape::OneD(Some(Infinity)));
        Fill {
            value,
            shape,
            dtype: "f64",
        }
    }
}

impl InfiniteArray for Fill {
    fn get(&self, _index: usize) -> f64 {
        self.value
    }
    
    fn shape(&self) -> Shape {
        self.shape.clone()
    }
    
    fn dtype(&self) -> &'static str {
        self.dtype
    }
}

impl fmt::Display for Fill {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Fill({}){}:", self.value, self.shape())?;
        for i in 0..12 {
            write!(f, "\n  {}", self.get(i))?;
        }
        write!(f, "\n  ⋮")
    }
}

impl fmt::Debug for Fill {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Fill({}){}", self.value, self.shape())
    }
}

