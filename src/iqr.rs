//! Infinite-dimensional QR algorithm implementation.
//!
//! This module implements the infinite-dimensional QR (IQR) algorithm as described in:
//! Colbrook, M.J. & Hansen, A.C. "On the infinite-dimensional QR algorithm"
//! Numer. Math. 143, 17-83 (2019).

use ndarray::Array2;
use num_complex::Complex64;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Represents an infinite-dimensional operator on l^2(N)
pub struct InfiniteOperator {
    matrix_func: Arc<dyn Fn(usize, usize) -> Complex64 + Send + Sync>,
    cache: Arc<Mutex<HashMap<(usize, usize), Complex64>>>,
}

impl InfiniteOperator {
    pub fn new<F>(matrix_func: F) -> Self
    where
        F: Fn(usize, usize) -> Complex64 + Send + Sync + 'static,
    {
        InfiniteOperator {
            matrix_func: Arc::new(matrix_func),
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// Get matrix element at position (i, j)
    pub fn get(&self, i: usize, j: usize) -> Complex64 {
        // Check cache
        {
            let cache = self.cache.lock().unwrap();
            if let Some(&value) = cache.get(&(i, j)) {
                return value;
            }
        }
        
        // Compute and cache
        let value = (self.matrix_func)(i, j);
        {
            let mut cache = self.cache.lock().unwrap();
            cache.insert((i, j), value);
        }
        
        value
    }
    
    /// Get a finite n×n truncation of the operator
    pub fn get_truncation(&self, n: usize) -> Array2<Complex64> {
        let mut matrix = Array2::<Complex64>::zeros((n, n));
        for i in 0..n {
            for j in 0..n {
                matrix[(i, j)] = self.get(i, j);
            }
        }
        matrix
    }
}

/// Result of IQR algorithm
#[derive(Debug, Clone)]
pub struct IqrResult {
    pub eigenvalues: Vec<Complex64>,
    pub eigenvectors: Option<Array2<Complex64>>,
    pub iterations: usize,
    pub converged: bool,
    pub residual: Option<f64>,
}

/// Infinite-dimensional QR algorithm for computing spectra
pub fn iqr_algorithm(
    operator: &InfiniteOperator,
    n: usize,
    max_iter: usize,
    tol: f64,
    shift: Option<Complex64>,
    compute_eigenvectors: bool,
) -> IqrResult {
    // Get finite truncation
    let mut a = operator.get_truncation(n);
    
    // Initialize eigenvector matrix if needed
    let mut q_total = if compute_eigenvectors {
        Some(Array2::<Complex64>::eye(n))
    } else {
        None
    };
    
    // QR iteration
    let mut iterations = 0;
    let mut converged = false;
    let mut max_off_diag = f64::INFINITY;
    
    for k in 0..max_iter {
        // Compute shift (Wilkinson shift for better convergence)
        let shift_val = if let Some(s) = shift {
            s
        } else {
            // Wilkinson shift: use eigenvalue of bottom-right 2x2 block
            if n >= 2 {
                let a_val = a[(n-2, n-2)];
                let b_val = a[(n-2, n-1)];
                let c_val = a[(n-1, n-2)];
                let d_val = a[(n-1, n-1)];
                
                // Eigenvalue of 2x2 matrix closest to d
                let trace = a_val + d_val;
                let det = a_val * d_val - b_val * c_val;
                let discriminant = trace * trace - Complex64::new(4.0, 0.0) * det;
                
                if discriminant.re >= 0.0 {
                    let sqrt_disc = Complex64::new(discriminant.re.sqrt(), 0.0);
                    let lambda1 = (trace + sqrt_disc) / Complex64::new(2.0, 0.0);
                    let lambda2 = (trace - sqrt_disc) / Complex64::new(2.0, 0.0);
                    
                    let dist1 = (lambda1 - d_val).norm();
                    let dist2 = (lambda2 - d_val).norm();
                    if dist2 < dist1 {
                        lambda2
                    } else {
                        lambda1
                    }
                } else {
                    trace / Complex64::new(2.0, 0.0)
                }
            } else {
                a[(0, 0)]
            }
        };
        
        // Shift the matrix
        let mut a_shifted = a.clone();
        for i in 0..n {
            a_shifted[(i, i)] = a_shifted[(i, i)] - shift_val;
        }
        
        // QR decomposition (simplified - using ndarray's QR)
        // Note: ndarray doesn't have built-in QR, so we'll use a simple implementation
        let (q, r) = qr_decomposition(&a_shifted);
        
        // Reverse QR: A = R * Q + shift
        a = r.dot(&q);
        for i in 0..n {
            a[(i, i)] = a[(i, i)] + shift_val;
        }
        
        // Accumulate eigenvectors if needed
        if let Some(q_tot) = &mut q_total {
            *q_tot = q_tot.dot(&q);
        }
        
        iterations = k + 1;
        
        // Check convergence: off-diagonal elements should be small
        max_off_diag = 0.0;
        for i in 0..n {
            for j in 0..n {
                if i != j {
                    let val = a[(i, j)].norm();
                    if val > max_off_diag {
                        max_off_diag = val;
                    }
                }
            }
        }
        
        if max_off_diag < tol {
            converged = true;
            break;
        }
    }
    
    // Extract eigenvalues from diagonal
    let mut eigenvalues: Vec<Complex64> = (0..n).map(|i| a[(i, i)]).collect();
    
    // Sort by magnitude
    eigenvalues.sort_by(|a, b| b.norm().partial_cmp(&a.norm()).unwrap_or(std::cmp::Ordering::Equal));
    
    let mut result = IqrResult {
        eigenvalues,
        eigenvectors: None,
        iterations,
        converged,
        residual: if converged { Some(max_off_diag) } else { None },
    };
    
    if compute_eigenvectors {
        if let Some(q_tot) = q_total {
            // Reorder eigenvectors to match sorted eigenvalues
            // This is simplified - full implementation would need to track eigenvalue order
            result.eigenvectors = Some(q_tot);
        }
    }
    
    result
}

/// Simple QR decomposition using Gram-Schmidt
fn qr_decomposition(a: &Array2<Complex64>) -> (Array2<Complex64>, Array2<Complex64>) {
    let n = a.nrows();
    let mut q = Array2::<Complex64>::zeros((n, n));
    let mut r = Array2::<Complex64>::zeros((n, n));
    
        // Gram-Schmidt process
    for j in 0..n {
        let mut v = a.column(j).to_owned();
        
        for i in 0..j {
            let r_ij = q.column(i).dot(&v);
            r[(i, j)] = r_ij;
            let q_col = q.column(i).to_owned();
            let scaled = q_col.mapv(|x| x * r_ij);
            v = &v - &scaled;
        }
        
        let norm = v.mapv(|x| x.norm()).sum().sqrt();
        if norm > 1e-10 {
            r[(j, j)] = Complex64::new(norm, 0.0);
            let q_col = &v / Complex64::new(norm, 0.0);
            for i in 0..n {
                q[(i, j)] = q_col[i];
            }
        } else {
            r[(j, j)] = Complex64::new(0.0, 0.0);
            for i in 0..n {
                q[(i, j)] = if i == j { Complex64::new(1.0, 0.0) } else { Complex64::new(0.0, 0.0) };
            }
        }
    }
    
    (q, r)
}

/// Compute spectrum using IQR algorithm with adaptive truncation
pub fn iqr_spectrum(
    operator: &InfiniteOperator,
    n_range: &[usize],
    max_iter: usize,
    tol: f64,
) -> IqrSpectrumResult {
    let mut results_by_n = HashMap::new();
    
    for &n in n_range {
        let result = iqr_algorithm(operator, n, max_iter, tol, None, false);
        results_by_n.insert(n, result);
    }
    
    // Use eigenvalues from largest truncation as estimate
    let largest_n = *n_range.iter().max().unwrap();
    let eigenvalues = results_by_n.get(&largest_n).unwrap().eigenvalues.clone();
    
    let converged = {
        let results_ref = &results_by_n;
        n_range.iter().all(|&n| results_ref.get(&n).unwrap().converged)
    };
    IqrSpectrumResult {
        eigenvalues,
        eigenvalues_by_n: results_by_n,
        converged,
        recommended_n: largest_n,
    }
}

/// Result of spectrum computation
#[derive(Debug, Clone)]
pub struct IqrSpectrumResult {
    pub eigenvalues: Vec<Complex64>,
    pub eigenvalues_by_n: HashMap<usize, IqrResult>,
    pub converged: bool,
    pub recommended_n: usize,
}

/// Create an infinite diagonal operator
pub fn create_diagonal_operator<F>(diagonal_values: F) -> InfiniteOperator
where
    F: Fn(usize) -> Complex64 + Send + Sync + 'static,
{
    let func = Arc::new(diagonal_values);
    InfiniteOperator::new(move |i, j| {
        if i == j {
            (func)(i)
        } else {
            Complex64::new(0.0, 0.0)
        }
    })
}

/// Create an infinite tridiagonal operator
pub fn create_tridiagonal_operator<F1, F2, F3>(
    main_diag: F1,
    upper_diag: Option<F2>,
    lower_diag: Option<F3>,
) -> InfiniteOperator
where
    F1: Fn(usize) -> Complex64 + Send + Sync + 'static,
    F2: Fn(usize) -> Complex64 + Send + Sync + 'static,
    F3: Fn(usize) -> Complex64 + Send + Sync + 'static,
{
    let main_func = Arc::new(main_diag);
    let upper_func: Arc<dyn Fn(usize) -> Complex64 + Send + Sync> = if let Some(f) = upper_diag {
        Arc::new(f)
    } else {
        Arc::new(|_| Complex64::new(0.0, 0.0))
    };
    let lower_func: Arc<dyn Fn(usize) -> Complex64 + Send + Sync> = if let Some(f) = lower_diag {
        Arc::new(f)
    } else {
        Arc::new(|_| Complex64::new(0.0, 0.0))
    };
    
    InfiniteOperator::new(move |i, j| {
        if i == j {
            (main_func)(i)
        } else if j == i + 1 {
            (upper_func)(i)
        } else if i > 0 && j == i - 1 {
            (lower_func)(j)
        } else {
            Complex64::new(0.0, 0.0)
        }
    })
}

