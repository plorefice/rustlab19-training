// Exercise 9 - Error handling

// EX: Copy here the rest of the Op enum from the previous exercise.
enum Op {
    Add(i32, i32),
    Sub(i32, i32),
    Mul(i32, i32),
    Div(i32, i32),
}

// EX: Define a OpParseError type and modify the Op::new() function
//     from the previous exerices to return a Result<Op, OpParseError>.
//     The function *should not* panic in any case.
#[derive(Debug)]
struct OpParseError(char);

impl Op {
    fn new(l: i32, op: char, r: i32) -> Result<Op, OpParseError> {
        match op {
            '+' => Ok(Op::Add(l, r)),
            '-' => Ok(Op::Sub(l, r)),
            '*' => Ok(Op::Mul(l, r)),
            '/' => Ok(Op::Div(l, r)),
            _ => Err(OpParseError(op)),
        }
    }

    fn result(self) -> i32 {
        match self {
            Op::Add(l, r) => l + r,
            Op::Sub(l, r) => l - r,
            Op::Mul(l, r) => l * r,
            Op::Div(l, r) => l / r,
        }
    }
}

fn main() {
    // EX: Modify the following code to make it work.
    //     Copy whatever code is necessary from the previous exercise.
    let sum = Op::new(5, '+', 10).unwrap();
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
        if let Err(OpParseError(c)) = result {
            println!("{} is not a valid operator", c);
        }
    }
}