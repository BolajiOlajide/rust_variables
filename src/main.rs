fn main() {
    // ABOUT CONSTANTS

    // Constants aren’t just immutable by default—they’re always immutable.
    // constants may be set only to a constant expression, not the result of a
    // function call or any other value that could only be computed at runtime.
    const MAX_POINTS: u32 = 100_000_000;
    println!("{}", MAX_POINTS);

    // SHADOWING
    let x = 5;

    let x = x + 1; // this is simply shadowing the value of x
    // it is different from `x = x + 1`

    let x = x * 2;

    println!("x = {}", x);

    // Rust has 4 scalar types:
    // - integer
    // - Boolean
    // - floating-point numbers
    // - characters

    // Each signed variant can store numbers from -(2n - 1) to 2n - 1 - 1 inclusive,
    // where n is the number of bits that variant uses. So an i8 can store numbers from
    // -(27) to 27 - 1, which equals -128 to 127. Unsigned variants can store numbers from 0 to 2n - 1,
    // so a u8 can store numbers from 0 to 28 - 1, which equals 0 to 255.

    // NUMERIC OPERATIONS

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    println!(
        "
            sum = {}
            difference = {}
            product = {}
            quotient = {}
            remainder = {}
        ", sum, difference, product, quotient, remainder
    );

    let _f: bool = false; // with explicit type annotation

    // Rust’s char type is the language’s most primitive alphabetic type
    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1); // define a tuple with type annotation
    let (_x, y, _z) = tup; // this is called destructuring

    println!("The value of y is: {}", y);

    // we can also access the individual elements of the tuple by querying the index
    // with dot notation
    println!(" =====> {}", tup.0);


    // Unlike a tuple, every element of an array must have the same type.
    // Arrays in Rust are different from arrays in some other languages because arrays in
    // Rust have a fixed length: once declared, they cannot grow or shrink in size.
    let _a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("The third month is {}", months[2]);

    another_function();
}

fn another_function() {
    println!("Another function.");
}
