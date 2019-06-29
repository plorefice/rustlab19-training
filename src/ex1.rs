// Exercise 1 - println!

fn main() {
    // let is how variables are declared in Rust. More on this later!
    let x = 2;
    let y = 3;

    // This writes "Hello World!" to stdout
    println!("Hello World!");

    // Like many other languages, println! accepts a formatter string
    // and a variable number of arguments containing expressions
    // to be evaluated and printed.
    println!("The answer to life, the universe and everything is {}", 42);

    // EX: Use println! to print "x + y = 5" using the x and y above.
    println!("x + y = {}", x + y);

    // Advanced formatting can be performed by using the {:} syntax.
    // More on this at https://doc.rust-lang.org/std/fmt/index.html

    // EX: Try printing a number in decimal, hexadecimal, octal and binary format.
    println!("{0} {0:x} {0:o} {0:b}", 127);

    // Rust has recently introduced dbg!() to perform quick printf-debugging.
    // This is documented at https://doc.rust-lang.org/std/macro.dbg.html

    // EX: Modify the statement below to debug the value of x * y.
    let z = dbg!(x * y) + x;
    println!("z is {}", z);
}
