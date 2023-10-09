fn main() {
    // comparison operators
    // less than | less than or equal to
    println!("3 < 2 is {}", 3 < 2);
    println!("3 <= 3 is {}", 3 <= 3);
    // greater than | greater than or equal to
    println!("3 >= 2 is {}", 3 >= 2);
    println!("3 >= 4 is {}", 3 >= 4);
    // equality
    println!("3 == 3 is {}", 3 == 3);
    // inequality
    println!("3 != 3 is {}", 3 != 3);

    // lazy boolean operators
    // the minimum number of evaluations are performed
    // && Lazy Boolean AND operator
    println!("1 == 1 && 1 > 1 is {}", 
        1 == 1 && 1 > 1
    );
    // || Lazy Boolean OR operator
    println!("1 == 1 || 1 > 1 is {}", 
        1 == 1 || 1 > 1
    );
}