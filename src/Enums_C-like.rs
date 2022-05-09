// An attribute to hide warnings for unused code.
#![allow(dead_code)]

//For debug
#[derive(Debug)]

// enum with implicit discriminator (starts at 0)
enum Number {
    Zero,
    One,
    Two,
}

// enum with explicit discriminator
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    // `enums` can be cast as integers.
    println!("zero is {:#?}", Number::Zero as i32);
    println!("zero is {:#?}", Number::Zero);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("greens are #{:06x}", Color::Green as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
    println!("//---------------------------------------------------//");
    println!("roses are #{:06b}", Color::Red as i32);
    println!("greens are #{:06b}", Color::Green as i32);
    println!("violets are #{:06b}", Color::Blue as i32);
    println!("//---------------------------------------------------//");
    println!("roses are #{:06o}", Color::Red as i32);
    println!("greens are #{:06o}", Color::Green as i32);
    println!("violets are #{:06o}", Color::Blue as i32);
    println!("//---------------------------------------------------//");
    println!("roses are #{:0x}", Color::Red as i32);
    println!("roses are #{:0x}", Color::Green as i32);
    println!("roses are #{:0x}", Color::Blue as i32);
    println!("roses are #{:x}", Color::Red as i32);
    println!("violets are #{:x}", Color::Green as i32);
    println!("violets are #{:x}", Color::Blue as i32);
    

   
}
