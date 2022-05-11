//-----------------------------------------------------------Inference----------------------------------------------------------------//

//The type inference engine is pretty smart. It does more than looking at the type of the value expression during an initialization. It also looks at how the variable is used afterwards to infer its type. Here's an advanced example of type inference:

fn main() {
    // Because of the annotation, the compiler knows that `elem` has type u8.
    let elem = 5u8;

    // Create an empty vector (a growable array).
    let mut vec = Vec::new();
    // At this point the compiler doesn't know the exact type of `vec`, it
    // just knows that it's a vector of something (`Vec<_>`).

    // Insert `elem` in the vector.
    vec.push(elem);
    // Aha! Now the compiler knows that `vec` is a vector of `u8`s (`Vec<u8>`)
    let elem2=6u16 as u8;
    vec.push(elem2);

    let mut vec2=Vec::new();
    let elem3=7usize;
    let elem4=elem3+1usize;
    vec2.push(elem3);
    vec2.push(elem4);

    println!("{:?}", vec);
    println!("{:?}", vec2);
}

//No type annotation of variables was needed, the compiler is happy and so is the programmer!
