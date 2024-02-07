// Declaring immutable variables
fn main() {
    // Declare the variables
    let x = 1;
    let y = 2;
    let z = x + y;

    println!("x = {}", x);
    println!("y = {}", y);
    println!("z = {}", z);
}

// Trying to change immutable variables (error)
// fn main() {
//     let x = 1;
//     let y = 2;
//     let z = x + y;

//     println!("x = {}", x);
//     println!("y = {}", y);
//     println!("z = {}", z);
    
//     x = 2;

//     println!("---");
//     println!("x = {}", x);
//     println!("y = {}", y);
//     println!("z = {}", z);
// }

// Mutable variables and updating after using the variable
fn main() {
    let mut x = 1;
    let y = 2;
    let z = x + y;

    println!("x = {}", x);
    println!("y = {}", y);
    println!("z = {}", z);
    
    x = 2;

    println!("---");
    println!("x = {}", x);
    println!("y = {}", y);
    println!("z = {}", z);
}