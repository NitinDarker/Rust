fn main() {
    let tup = (5, true, 'c'); // Implicit type declaration
    let tup2: (i32, bool, char) = (55, false, ';'); // Explicit type declaration

    // println!("tuple: {tup2}"); // Cannot print the whole tuple
    println!("tuple[2] = {}", tup2.2); // Accessing the index through .
    // println!("tuple[2] = {tup2.2}"); // Doesn't work for some reason
}