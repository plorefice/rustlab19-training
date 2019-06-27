// Exercise 10 - Generics and traits

// EX: Copy all of exercises 8 and 9 and make the Op enum work
//     on generic types. Read compiler errors for hints on what to do.
//     Hint: if you are stuck, have a look at one of the numeric traits,
//           for eg. https://doc.rust-lang.org/std/ops/trait.Add.html

fn main() {
    // EX: Everything below should compile and run just fine.

    let sum = Op::new(5_u16, '+', 10).unwrap();
    println!("5+10={}", sum.result());

    let sub = Op::new(-5.0, '-', 10.0).unwrap();
    println!("-5-10={}", sub.result());
    
    let mul = Op::new(20_i64, '*', 4).unwrap();
    println!("20*4={}", mul.result());
    
    let div = Op::new(14.0_f64, '/', 2.0).unwrap();
    println!("14*2={}", div.result());

    let op = Op::Sub(6, 10);
    println!("6-10={}", op.lhs() - op.rhs());
}
