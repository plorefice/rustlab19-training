// Exercise 8 - Enums

// EX: Fill in the Op enum with the missing fields Sub, Mul and Div,
//     following the same convention of Add.
enum Op {
    Add(i32, i32),
    /* ... */
}

// EX: Implement the lhs(&self) and rhs(&self) methods that return a
//     *reference* to the left-hand and right-hand size operands respectively.

// EX: Implement a result(self) method that produces the correct
//     result for each operation.

fn main() {
    // EX: Implement the above to make this compile.
    let sub = Op::Sub(6, 10);
    println!("6-10={}", sub.lhs() - sub.rhs());

    // EX: Implement the above to make this compile.
    let sum = Op::Add(2, 3);
    println!("2+3={}", sum.result());

    // EX: Implement the associated function new() with the correct
    //     signature. It should create the correct kind of Op for the
    //     valid characters: '+', '-', '*' and '/'.
    //     Hint: use the char type as the function's second argument.
    //     Hint: use the panic!() macro for the invalid chars. See docs.
    let mul = Op::new(4, '*', 5);
    println!("4*5={}", mul.result());
}