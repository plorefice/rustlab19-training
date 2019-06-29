// Exercise 6 - Ownership & Borrowing

fn main() {
    // It is possible to create empty variable bindings, but an empty binding
    // *cannot* be used until it has been initialized.
    // The variable type may be omitted: it will be inferred the first time
    // the binding is initialized.
    let s4;

    let s = String::from("Hello!");

    // EX: Try uncommenting the next line and see what happens.
    // println!("s4 contains {}", s4);

    // Here we declare an inner scope to demonstrate value lifetimes.
    {
        // A reference to a value is declared by prepending an '&' to the variable
        // owning that value. The reference can be used just like the original variable,
        // but _cannot_ be mutated.
        let s1 = &s;
        println!("s1 says '{}'", s1);

        // It's okay to have multiple references to the same value.
        let s2 = &s;
        println!("s2 also says '{}'", s2);

        // It is *not* allowed to declare a _mutable_ reference to an _immutable_
        // value.

        // EX: Try uncommenting the next line and see what happens.
        // let s3 = &mut s;

        // In this inner scope we initialize s4, declared at the very top.
        // This is allowed: s is valid in this scope.
        s4 = &s;
        println!("s4 is valid and says '{}' too", s4);
    }

    // In this outer scope, the value of s is no longer valid, since it has been dropped.
    // s, s1 and s2 are also not visible in this parent scope. However, s4 was declared
    // in this scope so we might be tempted to use it.

    // EX: Uncomment this line and try to understand what the compiler tells you.
    println!("s4 is still here and says '{}'", s4);

    // EX: Try modifying the code so that the print above compiles and runs fine.
}
