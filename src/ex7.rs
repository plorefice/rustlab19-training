// Exercise 7 - Structs

// A _struct_ is used to group together related custom data.
// It may contain zero or more _fields_.
// Each _struct_ declares a new type in Rust's type system.
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Point {/* TODO: complete me (see last EX below) */}

fn manhattan(p1: &Point, p2: &Point) -> i32 {
    /* TODO: complete me (see last EX below) */
    0
}

fn main() {
    // When creating an instance of a struct, *all* of its fields *must* be initialized.

    // EX: Try removing one of the fields from the initialization and build, see what happens.
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // EX: Try modifying the variable binding above so that you are able to mutate
    //     its fields.

    // EX: Make it so the line below compiles and runs successfully.
    //     Hint: have a look at https://doc.rust-lang.org/std/fmt/trait.Debug.html
    println!("{:?}", user1);

    // EX: Declare a struct named Point with two i32 fields, 'x' and 'y'.
    //     Complete the function 'manhattan' so that it computes and returns
    //     the Manhattan distance [1] between the points.
    //     Hint: https://doc.rust-lang.org/std/primitive.i32.html#method.abs
    //
    //     [1]: https://xlinux.nist.gov/dads/HTML/manhattanDistance.html
    let p1 = Point { x: 15, y: 30 };
    let p2 = Point { x: -4, y: 18 };

    println!(
        "The Manhattan distance between p1 and p2 is {}",
        manhattan(&p1, &p2)
    );
}
