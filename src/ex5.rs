// Exercise 5 - Fibonacci sequence

/// Computes the `n`th value of the Fibonacci sequence
fn fib(n: i32) -> i32 {
    let mut a = 0;
    let mut b = 1;

    for _ in 0..n {
        let tmp = a;
        a = b;
        b += tmp;
    }
    a
}

/// Computes the `n`th value of the Fibonacci sequence using recursion.
fn rfib(n: i32) -> i32 {
    match n {
        0 | 1 => n,
        _ => rfib(n - 1) + rfib(n - 2),
    }
}

fn main() {
    // Print the first 10 digits of the Fibonacci sequence: F(n) = F(n-1) + F(n-2)
    for n in 0..10 {
        println!("fib({}) = {:4} | {:4}", n, fib(n), rfib(n));
    }
}
