//! Example usage of the Infinite-dimensional QR (IQR) algorithm.

use infinite_arrays::iqr::*;
use num_complex::Complex64;

fn main() {
    println!("{}", "=".repeat(70));
    println!("Infinite-dimensional QR Algorithm Examples");
    println!("{}", "=".repeat(70));

    // Example 1: Diagonal operator
    println!("\n1. Diagonal Operator (eigenvalues = 1, 2, 3, ...)");
    println!("{}", "-".repeat(70));
    let diag_op = create_diagonal_operator(|i| Complex64::new((i + 1) as f64, 0.0));
    let result = iqr_algorithm(&diag_op, 20, 500, 1e-12, None, false);
    println!("Computed {} eigenvalues", result.eigenvalues.len());
    println!("Iterations: {}, Converged: {}", result.iterations, result.converged);
    println!("First 10 eigenvalues:");
    for (i, ev) in result.eigenvalues.iter().take(10).enumerate() {
        println!("  λ_{} = {:.10} (expected: {:.1})", i + 1, ev.re, i + 1);
    }

    // Example 2: Tridiagonal operator
    println!("\n2. Tridiagonal Operator (discrete Laplacian-like)");
    println!("{}", "-".repeat(70));
    let tridiag_op = create_tridiagonal_operator(
        |_| Complex64::new(2.0, 0.0),
        Some(|_| Complex64::new(-1.0, 0.0)),
        Some(|_| Complex64::new(-1.0, 0.0)),
    );
    let result = iqr_algorithm(&tridiag_op, 50, 1000, 1e-10, None, false);
    println!("Computed {} eigenvalues", result.eigenvalues.len());
    println!("Iterations: {}, Converged: {}", result.iterations, result.converged);
    println!("First 10 eigenvalues:");
    for (i, ev) in result.eigenvalues.iter().take(10).enumerate() {
        println!("  λ_{} = {:.10}", i + 1, ev.re);
    }

    // Example 3: Custom operator
    println!("\n3. Custom Operator (matrix with specific structure)");
    println!("{}", "-".repeat(70));
    let custom_op = InfiniteOperator::new(|i, j| {
        Complex64::new(1.0 / (1.0 + (i as f64 - j as f64).abs()), 0.0)
    });
    let result = iqr_algorithm(&custom_op, 30, 500, 1e-10, None, false);
    println!("Computed {} eigenvalues", result.eigenvalues.len());
    println!("Iterations: {}, Converged: {}", result.iterations, result.converged);
    println!("First 10 eigenvalues (sorted by magnitude):");
    for (i, ev) in result.eigenvalues.iter().take(10).enumerate() {
        println!("  λ_{} = {:.10}", i + 1, ev.re);
    }

    // Example 4: Adaptive spectrum computation
    println!("\n4. Adaptive Spectrum Computation (multiple truncation sizes)");
    println!("{}", "-".repeat(70));
    let n_range = vec![20, 50, 100];
    let spectrum_result = iqr_spectrum(&diag_op, &n_range, 500, 1e-10);
    println!("Recommended truncation size: {}", spectrum_result.recommended_n);
    println!("Converged: {}", spectrum_result.converged);
    println!("Eigenvalues from largest truncation:");
    for (i, ev) in spectrum_result.eigenvalues.iter().take(10).enumerate() {
        println!("  λ_{} = {:.10}", i + 1, ev.re);
    }

    println!("\n{}", "=".repeat(70));
    println!("Examples completed!");
    println!("{}", "=".repeat(70));
}

