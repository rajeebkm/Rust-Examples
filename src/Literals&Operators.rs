fn main() {
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important
    // println!("1 - 2 = {}", 1u32 - 2); //attempt to compute `1_u32 - 2_u32`, which would overflow

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true AND false is {}", true && true);
    println!("true AND false is {}", false && false);
    println!("true AND false is {}", false && true);
    println!("true OR false is {}", true || false);
    println!("true AND false is {}", false || true);
    println!("true AND false is {}", true || true);
    println!("true AND false is {}", false || false);
    
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101); //0001
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101); //0111
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101); //0110
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
}
