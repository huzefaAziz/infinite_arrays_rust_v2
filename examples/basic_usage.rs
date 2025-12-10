//! Basic usage examples for InfiniteArrays.

use infinite_arrays::*;
use infinite_arrays::arrays;
use std::sync::Arc;

fn main() {
    println!("{}", "=".repeat(60));
    println!("InfiniteArrays - Basic Usage Examples");
    println!("{}", "=".repeat(60));

    println!("\n1. Creating an infinite vector of ones:");
    println!("{}", "-".repeat(60));
    let x = Arc::new(Ones::new(None));
    println!("{}", x);
    println!("x[0] = {}", x.get(0));
    println!("x[5] = {}", x.get(5));

    println!("\n2. Creating an infinite diagonal matrix:");
    println!("{}", "-".repeat(60));
    let d = InfiniteDiagonal::new(|i| (i + 1) as f64);
    println!("D[0, 0] = {}", d.get(0, 0));
    println!("D[1, 1] = {}", d.get(1, 1));
    println!("D[0, 1] = {}", d.get(0, 1));
    println!("D[2, 2] = {}", d.get(2, 2));

    println!("\n3. Broadcasting operations:");
    println!("{}", "-".repeat(60));
    let result = BroadcastArray::new(
        |i| (-((i + 1) as f64)).exp() + 2.0,
        arrays::Shape::OneD(Some(INFINITY)),
    );
    println!("First few values of exp(-i) + 2:");
    for i in 0..10 {
        println!("  result[{}] = {:.10}", i, result.get(i));
    }

    println!("\n4. Element-wise operations:");
    println!("{}", "-".repeat(60));
    let x = Arc::new(Ones::new(None));
    let x_clone1 = x.clone();
    let y = BroadcastArray::new(
        move |i| x_clone1.get(i) + 2.0,
        arrays::Shape::OneD(Some(INFINITY)),
    );
    println!("y[0] = {}", y.get(0));

    let x_clone2 = x.clone();
    let z = BroadcastArray::new(
        move |i| x_clone2.get(i) * 3.0,
        arrays::Shape::OneD(Some(INFINITY)),
    );
    println!("z[0] = {}", z.get(0));

    println!("\n5. Cached (mutable) arrays:");
    println!("{}", "-".repeat(60));
    let c = cache(x.clone());
    println!("Before: C[0] = {}", c.get(0));
    c.set(0, 3.0);
    println!("After: C[0] = {}", c.get(0));
    println!("C[1] = {}", c.get(1));

    println!("\n6. Other infinite array types:");
    println!("{}", "-".repeat(60));
    let zeros = Arc::new(Zeros::new(None));
    println!("zeros[0] = {}", zeros.get(0));

    let filled = Arc::new(Fill::new(42.0, None));
    println!("filled[0] = {}", filled.get(0));
    println!("filled[5] = {}", filled.get(5));

    println!("\n7. Accessing elements:");
    println!("{}", "-".repeat(60));
    let x = Arc::new(Ones::new(None));
    println!("First 10 elements:");
    for i in 0..10 {
        println!("  x[{}] = {}", i, x.get(i));
    }

    println!("\n{}", "=".repeat(60));
    println!("Examples completed!");
    println!("{}", "=".repeat(60));
}

