fn main() {
    // construct mutable vector
    let mut vector = vec![1, 2, 3];
    println!("vector = {:?}", vector);

    // add element to
    // end of vector
    vector.push(4);
    println!("vector = {:?}", vector);
    // beginning of vector

    // remove element from
    // end of vector
    vector.pop();
    println!("vector = {:?}", vector);
    
}