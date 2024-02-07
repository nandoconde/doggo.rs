fn main() {
    // construct mutable array
    let mut array = [1, 2, 3];

    // display array in a single row
    println!("array = {:?}", array);

    // display array in a single column
    println!("array = {:#?}", array);

    // access specific element
    println!("2nd element is: {}", array[1]);
    
    // mutate array element value
    array[1] = 20;
    println!("2nd element is now: {}", array[1]);

    // array length
    println!("The array has {} elements", array.len());

    // sum array elements
    let total: i32 = array.iter().sum();
    println!("array sum is {}", total);

    // sort array elements
    // in descending index order
    array.reverse();
    println!("reversed array = {:?}", array);

    // in ascending value order
    array.sort();
    println!("sorted array = {:?}", array);

    // in descending value order
    array.reverse();
    println!("sort-reversed array = {:?}", array);

    // array of arrays
    let arr_of_arr = [
        [1, 2, 3],
        [4, 5, 6]
    ];
}