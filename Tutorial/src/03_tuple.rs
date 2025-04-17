fn main() {
    // Tuples can have multiple type of elements inside
    // Immutable by default
    // use mut to  make them mutable

    // let tup = (5, true, 'c'); // Implicit type declaration
    // let tup2: (i32, bool, char) = (55, false, ';'); // Explicit type declaration

    // println!("tuple: {tup2}"); // Cannot print the whole tuple
    // println!("tuple[2] = {}", tup2.2); // Accessing the index through .
    // println!("tuple[2] = {tup2.2}"); // Doesn't work for some reason

    let mut tup3 = (4, 3.4);
    tup3.1 = 6.9; // Works
    // tup3.2 = 5; // Cannot insert even if mutable
    // tup3 = (6, 5, 4); // Doesn't work either -> Type mismatch of tuple
    println!("{}", tup3.1);

}