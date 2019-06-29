// Exercise 7 - Structs

// A _struct_ is used to group together related custom data.
// It may contain zero or more _fields_.
// Each _struct_ declares a new type in Rust's type system.
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn manhattan(&self, other: &Point) -> i32 {
        manhattan(self, other)
    }
}

fn manhattan(p1: &Point, p2: &Point) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

fn main() {
    // When creating an instance of a struct, *all* of its fields *must* be initialized.

    // EX: Try removing one of the fields from the initialization and build, see what happens.
    let mut user1 = User {
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

    // EX: Declare an associated function that creates a Point from two i32s.
    let p3 = Point::new(5, -2);

    // EX: Declare a method version of the manhattan function above.
    println!(
        "The Manhattan distance between p2 and p3 is {}",
        p2.manhattan(&p3)
    );
}
