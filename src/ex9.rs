// Exercise 9 - Error handling

// EX: Copy here the rest of the Op enum from the previous exercise.
enum Op {
    Add(i32, i32),
    /* ... */
}

// EX: Define a OpParseError type and modify the Op::new() function
//     from the previous exerices to return a Result<Op, OpParseError>.
//     The function *should not* panic in any case.

fn main() {
    // EX: Modify the following code to make it work.
    //     Copy whatever code is necessary from the previous exercise.
    let sum = Op::new(5, '+', 10);
    println!("5+10={}", sum.result());

    // EX: Modify the following code to make it work.
    //     For each element, it should print the incorrect operator
    //     in the form contained in the println!() macro.
    //     Hint: you might need to add some more info to OpParseError
    //           and use pattern matching.
    for result in [
        Op::new(1, '!', 2),
        Op::new(0, '?', 5),
        Op::new(18, '#', 2),
        Op::new(-16, '&', 21),
    ]
    .iter()
    {
        println!("{} is not a valid operator", result);
    }
}