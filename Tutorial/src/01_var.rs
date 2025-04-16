fn main() {
    let x = 5;
    // can be redeclared but cannot be reassigned
    // to reassign, use mut
    // by-default a let variable is immutable

    // let x = 4;  // works 
    // x = 4;  // doesn't work
    
    // let mut x = 5;
    // x = 3; // Now works

    println!("x is: {}", x);
}