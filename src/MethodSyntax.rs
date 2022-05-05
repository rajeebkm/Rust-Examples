//-------------------Method Syntax------------------------//
// Methods are similar to functions: we declare them with the fn keyword and a name, they can have parameters and a return value, and they contain some code that’s run when the method is called from somewhere else. Unlike functions, methods are defined within the context of a struct (or an enum or a trait object respectively), and their first parameter is always self, which represents the instance of the struct the method is being called on.

//-------------------Defining Methods----------------//
// Let’s change the area function that has a Rectangle instance as a parameter and instead make an area method defined on the Rectangle struct.

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    //we can choose to give a method the same name as one of the struct’s fields. For example, we can define a method on Rectangle also named width.
    //We can define methodS those have self as their first parameter. Because they need an instance of the type to work with.

    fn width(&self)->bool{
        self.width > 0
    }  

    fn can_hold(&self, other:&Rectangle)->bool{
        self.width > other.width && self.height > other.height
    }
   
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    //we can use a field within a method of the same name for any purpose. In main, when we follow rect1.width with parentheses, Rust knows we mean the method width. When we don’t use parentheses, Rust knows we mean the field width.
    if rect1.width(){
    println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
    
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    //---------------------Methods with More Parameters------------------------//

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));


//-----------------------------Associated functions-------------------------------------//

//All functions defined within an impl block are called associated functions because they’re associated with the type named after the impl. We can define associated functions that don’t have self as their first parameter (and thus are not methods) because they don’t need an instance of the type to work with. We’ve already used one function like this: the String::from function that’s defined on the String type.

// Associated functions that aren’t methods are often used for constructors that will return a new instance of the struct. For example, we could provide an associated function that would have one dimension parameter and use that as both width and height, thus making it easier to create a square Rectangle rather than having to specify the same value twice:


impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}


    let sq = Rectangle::square(3);

    println!("The square rectangle is:\n {:#?}", sq);

    //------------------Multiple impl Blocks--------------------------//
    
    // Each struct is allowed to have multiple impl blocks. There’s no reason to separate these methods into multiple impl blocks here, but this is valid syntax.

    // #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//     let rect2 = Rectangle {
//         width: 10,
//         height: 40,
//     };
//     let rect3 = Rectangle {
//         width: 60,
//         height: 45,
//     };

//     println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
//     println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
// }


}




