// Exercise 3 - Functions

fn plus_one(n: i32) -> i32 {
    // Each block of code in Rust is composed of a number of _statements_ and
    // optionally a single _expression_ at the end.

    // Statements are instructions that perform some action and _do not_ return
    // any value. Every line of code seen up to this point is a statement.
    // Statements are *always* terminated by a semicolon (';').

    // Expressions evaluate to something (eg. "5 + 6" is an expression evaluating
    // to the number 11). Expressions are *not* terminated by a semicolon.

    // If an expression is present at the end of a block, that block is evaluated
    // to the result of that expression. If the block of code in question is
    // a function's body, the value of the expression is the function's return value.

    // EX: Add an expression to return the value of n incremented by 1
}

fn main() {
    // Functions are declared using the fn keyword, just like above.
    // A function declaration must include its name, arguments (if any)
    // and return type (if any).

    // For example, 'plus_one' returns its argument incremented by 1:
    println!("4 + 1 = {}", plus_one(4));

    // EX: Write a function named 'say_hello' that prints a greeting.
    say_hello();

    // EX: Write a *one-line* function named 'get_age' that returns a number.
    println!("I'm {} years old", get_age());

    // EX: Rewrite 'get_age' to use at least one meaningful statement.
    //     Hint: try declaring a 'let' binding.

    // EX: Write the sum_two() function to take two i32s and return an i32
    //     representing their sum.
    println!("{} + {} = {}", 2, 3, sum_two(2, 3))
}
