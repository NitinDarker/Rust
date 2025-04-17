fn main() {
    // Arrays can only store elements of same type
    // Immutable by default
    let arr = [4, 5, 6];
    println!("{}", arr[0]);
    // println!("{arr[0]}"); // Doesn't work

    // To explicitly define type of array
    let arr2: [u64; 3] = [1, 2, 3];
    // type followed by semi-colon followed by total size enclosed in square braces
    println!("{}", arr2[1]);

    // arr2[0] = 5; // Not allowed
    // Also the elements of array need to be initialized when declared.

    // let arr3 = []; // Empty array without type is not allowed
    // let arr3: [f32; 0] = []; // Works
    // println!("{}", arr3[0]); // Index out of bounds
    
    // let arr4: [u32; 5];
    // println!("{}", arr4[0]); // Index out of bounds

    let arr4: [u32; 5] = [42; 5]; // Quickly assign large number of values
    println!("{}", arr4[3]);
}