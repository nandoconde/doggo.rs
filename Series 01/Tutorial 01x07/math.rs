// this is a single-line comment

// keyboard shortcut in VS Code for toggling comments is Ctrl + /

// this is a
// multi-line
// comment

fn main() {
    // Addition (sum)
    // println!("1 + 1 = {}", 2); // Print dummy value
    println!("1 + 1 = {}", 1 + 1); // Print computed value (use whitespace)
    // println!("1 + 1 = {}", 1+1); // Print computed value (no whitespace)
    
    // Subtraction (minus)
    println!("2 - 1 = {}", 2 - 1);

    // Multiplication
    println!("2 * 3 = {}", 2 * 3);

    // Division
    // Integer division always returns an integer
    println!("4 / 2 = {}", 4 / 2); // = 2
    println!("4 / 2 = {}", 5 / 2); // = 2

    // Modulo (remainder after division)
    println!("5 % 2 = {}", 5 % 2); // = 1

    // exponentiation
    // `pow(base, power)`
    // `i32::` type declaration as 32-bit integer
    println!("2^3 = {}", i32::pow(2,3)); // no builtin operator

}