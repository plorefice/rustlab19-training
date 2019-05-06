// Exercise 5 - Fibonacci sequence

/// Computes the `n`th value of the Fibonacci sequence
fn fib(n: i32) -> i32 {
    // EX: Implement using iteration
    0
}

/// Computes the `n`th value of the Fibonacci sequence using recursion.
fn rfib(n: i32) -> i32 {
    // EX: Implement using recursion
    0
}

fn main() {
    // Print the first 10 digits of the Fibonacci sequence: F(n) = F(n-1) + F(n-2)
    for n in 0..10 {
        println!("fib({}) = {:4} | {:4}", n, fib(n), rfib(n));
    }
}
