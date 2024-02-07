fn main() {
    // exclude upper bound
    for i in 1..10 {
        if i == 7 {
            continue;
        }
        println!("{i}");
    }

    println!("---");

    // include upper bound
    for i in 1..=10 {
        println!("{i}");
    }

    println!("---");

    // rev() method
    for i in (1..=50).rev() {
        if i == 40 {
            break;
        }
        println!("{i}");
    }

    println!("---");

    // step_by() method
    for i in (0..50).step_by(5) {
        println!("{i}");
    }
}