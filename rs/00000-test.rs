fn main() {
    let mut array: Vec<i32> = vec![0, 0, 1, 1, 1, 2, 2, 3, 4, 4, 4, 4];

    println!("array: {:?}", array);

    // Collect the iterator produced by `drain` into a new vector
    array = array.drain(_..=50);

    println!("array after draining: {:?}", array);
    //writeln!("removed elements: {:?}", removed_elements);
}
