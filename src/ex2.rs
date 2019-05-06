// Exercise 2 - Variables

fn main() {
    // A variable in Rust is declared using the let keyword
    let the_answer = 42;
    let a_string = "The answer:";

    println!("{} is {}", a_string, the_answer);

    // By default, Rust variables are *immutable*, so re-assigning to them
    // will trigger a compiler error.

    // EX: Fix this by removing the following line.
    //     You can also "shadow" the previous declaration by reusing "let".
    the_answer = 43;

    // To declare a mutable variable, use 'mut' following the let binding
    let mut i_can_change = 1;

    // EX: Increment i_can_change and print its new value

    // In most cases, Rust is able to automatically infer a variable's type
    // without the need to write it down.
    // To explicitly declare a variable's type, add it after the binding's name.

    let a: i32 = -123; // a 32-bit integer
    let b: u32 = 1_210_000; // a 32-bit unsigned integer

    println!("a = {}, b = {}", a, b);

    // A list of Rust primitve types is available at
    // https://doc.rust-lang.org/1.30.0/book/2018-edition/ch03-02-data-types.html

    // EX: Declare and print a char, an arch-dependent unsigned and an array of u8s.
}
