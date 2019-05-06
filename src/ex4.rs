// Exercise 4 - Control flow

fn main() {
    // Rust has four control flow constructs: `if-else`, `loop`, `while`, `for`.

    // == `if` ==

    // `if` allows conditional execution of branches of codes.
    // The condition must be an _expression_.
    // Each branch must evaluate to the same type, if an expression is the last
    // line in the block.
    let number = 3;

    if number > 5 {
        println!("condition was met");
    } else {
        println!("condition was not met");
    };

    // The following code is equivalent, but the `if` is used as an expression.
    // In this case, one of the two string literals is assigned to `s`, depending
    // on the condition. Note the absence of a semicolon after the literals.
    let s = if number > 5 {
        "condition was met"
    } else {
        "condition was not met"
    };

    println!("{}", s);

    // == `loop` ==

    // `loop` repeats a block of code forever, or until the process is killed.

    // EX: Try uncommenting the following code and see what happens.
    //     Hint: use Ctrl-C to terminate the program.

    // loop {
    //     use std::thread;
    //     use std::time::Duration;
    //
    //     println!("Once more!");
    //     thread::sleep(Duration::from_secs(1));
    // }

    // == `while` ==

    // `while` repeatedly executes a block of code until a condition remains true.
    // As for `if`, the condition must be an _expression_.
    let mut counter = 3;

    while counter != 0 {
        println!("{}!", counter);

        counter -= 1;
    }

    // == `for`==

    // `for` is used to iterate over items in a collection (eg. an array).

    // In this example, `element` is bound subsequently to each element in the array.
    // The `.iter()` method produces an `Iterator` over which the `for` can loop.
    let array = [10, 20, 30, 40, 50];

    for element in array.iter() {
        println!("{}", element);
    }

    // The idiomatic way to iterate over the integer range [a,b) is to use `for`
    // along with some range syntax.
    for i in 0..10 {
        println!("{}", i);
    }

    // EX: Write a `for` cycle to iterate over integers in the range [9..0].
    //     Hint: look for the `.rev()` method in the language reference.
}
