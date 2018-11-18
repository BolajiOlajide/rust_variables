fn main() {
    // ABOUT CONSTANTS

    // Constants aren’t just immutable by default—they’re always immutable.
    // constants may be set only to a constant expression, not the result of a
    // function call or any other value that could only be computed at runtime.
    const MAX_POINTS: u32 = 100_000;
    println!("{}", MAX_POINTS);

    // SHADOWING
    let x = 5;

    let x = x + 1; // this is simply shadowing the value of x
    // it is different from `x = x + 1`
    println!("x = {}", x);
}
