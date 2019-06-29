// Exercise 10 - Generics and traits

use std::ops::{Add, Div, Mul, Sub};

// EX: Copy all of exercises 8 and 9 and make the Op enum work
//     on generic types. Read compiler errors for hints on what to do.
//     Hint: if you are stuck, have a look at one of the numeric traits,
//           for eg. https://doc.rust-lang.org/std/ops/trait.Add.html
enum Op<T> {
    Add(T, T),
    Sub(T, T),
    Mul(T, T),
    Div(T, T),
}

#[derive(Debug)]
struct OpParseError(char);

impl<T> Op<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    fn new(l: T, op: char, r: T) -> Result<Op<T>, OpParseError> {
        match op {
            '+' => Ok(Op::Add(l, r)),
            '-' => Ok(Op::Sub(l, r)),
            '*' => Ok(Op::Mul(l, r)),
            '/' => Ok(Op::Div(l, r)),
            _ => Err(OpParseError(op)),
        }
    }

    fn lhs(&self) -> &T {
        match self {
            Op::Add(l, _) | Op::Sub(l, _) | Op::Mul(l, _) | Op::Div(l, _) => l,
        }
    }

    fn rhs(&self) -> &T {
        match self {
            Op::Add(_, r) | Op::Sub(_, r) | Op::Mul(_, r) | Op::Div(_, r) => r,
        }
    }

    fn result(self) -> T {
        match self {
            Op::Add(l, r) => l + r,
            Op::Sub(l, r) => l - r,
            Op::Mul(l, r) => l * r,
            Op::Div(l, r) => l / r,
        }
    }
}

fn main() {
    // EX: Everything below should compile and run just fine.

    let sum = Op::new(5_u16, '+', 10).unwrap();
    println!("5+10={}", sum.result());

    let sub = Op::new(-5.0, '-', 10.0).unwrap();
    println!("-5-10={}", sub.result());

    let mul = Op::new(20_i64, '*', 4).unwrap();
    println!("20*4={}", mul.result());

    let div = Op::new(14.0_f64, '/', 2.0).unwrap();
    println!("14/2={}", div.result());

    let op = Op::Sub(6, 10);
    println!("6-10={}", op.lhs() - op.rhs());
}
