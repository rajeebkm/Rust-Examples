//------------------------------------------------------------------Casting---------------------------------------------------------------------------//

// Rust provides no implicit type conversion (coercion) between primitive types. But, explicit type conversion (casting) can be performed using the as keyword.

// Rules for converting between integral types follow C conventions generally, except in cases where C has undefined behavior. The behavior of all casts between integral types is well defined in Rust.

// Suppress all warnings from casts which overflow.
#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    // Error! No implicit conversion
    // let integer: u8 = decimal;
    

    // Explicit conversion
    let integer = decimal as u8;
    let character = integer as char;

    // Error! There are limitations in conversion rules. 
    // A float cannot be directly converted to a char.
    // let character = decimal as char;
  

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // when casting any value to an unsigned type, T,
    // T::MAX + 1 is added or subtracted until the value
    // fits into the new type

    // 1000 already fits in a u16
    println!("1000 as a u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    // Under the hood, the first 8 least significant bits (LSB) are kept,
    // while the rest towards the most significant bit (MSB) get truncated.
    println!("1000 as a u8 is : {}", 1000 as u8);  //u8 ~ (0 - 255) 
//     // -1 + 256 = 255
    println!("  -1 as a u8 is : {}", (-1i8) as u8); // 256 as u8 is 0

    // For positive numbers, this is the same as the modulus
    println!("1000 mod 256 is : {}", 1000 % 256);

    // When casting to a signed type, the (bitwise) result is the same as
    // first casting to the corresponding unsigned type. If the most significant
    // bit of that value is 1, then the value is negative.

    // Unless it already fits, of course.
    println!(" 128 as a i16 is: {}", 128 as i16);   //o/p: 128      //i16 ~ Range: -32,768 to 32,767
    // 128 as u8 -> 128, whose two's complement in eight bits is:
    println!(" 128 as a i8 is : {}", 128 as i8);  //o/p: -128      // i8 ~ Range: -128 to 127
    println!(" 128 as a u8 is : {}", 128 as u8);  //o/p: 128        ////u8 ~ Range: (0 - 255) 
    // repeating the example above
    // 1000 as u8 -> 232
    println!("1000 as a u8 is : {}", 1000 as u8); // o/p: 232
    // and the two's complement of 232 is -24
    println!(" 232 as a i8 is : {}", 232 as i8);  //o/p: -24
    
    // Since Rust 1.45, the `as` keyword performs a *saturating cast* 
    // when casting from float to int. If the floating point value exceeds 
    // the upper bound or is less than the lower bound, the returned value 
    // will be equal to the bound crossed.
    
    // 300.0 is 255
    println!("300.0 is {}", 300.0_f32 as u8);  //o/p: 255
    // -100.0 as u8 is 0
    println!("-100.0 as u8 is {}", -100.0_f32 as u8); //o/p: 0
    // nan as u8 is 0
    println!("nan as u8 is {}", f32::NAN as u8);
    
    // This behavior incurs a small runtime cost and can be avoided 
    // with unsafe methods, however the results might overflow and 
    // return **unsound values**. Use these methods wisely:
    unsafe {
        // 300.0 is 44
        println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>()); //0/p: 44
        // -100.0 as u8 is 156
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());  //o/p: 156
        println!("-256.0 as u8 is {}", (-256.0_f32).to_int_unchecked::<u8>()); //o/p: 0
        // nan as u8 is 0
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }
}

// //Each signed variant can store numbers from -(2^(n-1)) to (2^(n-1))-1 inclusive, where n is the number of bits that variant uses. So an i8 can store numbers from -(2^7) to 2^7 - 1, which equals -128 to 127. Unsigned variants can store numbers from 0 to 2^n - 1, so a u8 can store numbers from 0 to 2^8 - 1, which equals 0 to 255.//
