use std::mem;

// This function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    // Fixed-size array (type signature is superfluous)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value
    let ys: [i32; 400] = [1; 400];
    // let zs: [i32; 400] = [1, 2]; //It will show an error

    // Indexing starts at 0
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // Indexing starts at 0
    println!("first element of the array: {}", ys[0]);
    println!("second element of the array: {}", ys[1]);

    //  // Indexing starts at 0
    // println!("first element of the array: {}", zs[0]); //Error will be displayed
    // println!("second element of the array: {}", zs[1]); //Error will be displayed

    // `len` returns the count of elements in the array
    println!("number of elements in array: {}", xs.len());

    // Arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));
    println!("array occupies {} bytes", mem::size_of_val(&ys));

    // Arrays can be automatically borrowed as slices
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // Slices can point to a section of an array
    // They are of the form [starting_index..ending_index]
    // starting_index is the first position in the slice
    // ending_index is one more than the last position in the slice
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[0 .. 2]);

    // Out of bound indexing causes compile error
    // println!("{}", xs[5]);

     let ks: [i32; 5] = [1, 2, 3, 4, 5];
     let slice: &[i32]=&ks[2..4];
     print!("{:?}\n",slice);
     print!("{:?}\n",slice[1]);

}
