use std::io::stdin;
fn main(){
    
   //Creating a mutable variableof string type

    let mut number=String::new();

//Creating a loop for different inputs to take

    loop{
    print!("Please enter the number!\n");

//Taking user input from terminal

    stdin().read_line(&mut number).expect("Failed to readline");

//Converting input string to u32 type by triming the whitespace and parsing

    let number: u32=number.trim().parse().expect("Please enter a a valid number");

    if number % 4 ==0{
        print!("The entered number is divisible by 4\n");
    }else if number % 3==0 {
        print!("The entered number is divisible by 3\n");
    }else if number % 2==0 {
        print!("The entered number is divisible by 2\n");
    }else{
        print!("The entered number is not divisible by 4, 3 and 2\n");
        break; //loop breaks here
    }
}

//Using if in a let Statement//
//--------------------------------------------------------------//
fn main(){
let condition =true;
let number = if condition {5}else{6}; //both the if arm and else arm should be same type, here i32 type
print!("The number is: {}", number);
}
// let number = if condition {5}else{"six"}; //It will show error, expected integer, found `&str`
// print!("The number is: {}", number);

//------------------------------------------------------//

//...............Rust has three kinds of loops: loop, while, and for. Let’s try each one............//

// 1. Repeating Code with loop
//------------------------------------------//
fn main() {
    loop {
        println!("again!");
    }
}
//-----------------------------------------//

    //loop with label (nested loop)
    //-------------------------------------//
    fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count); //0, 1, 2
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining); //10, 9, 10, 9, 10
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count); //2
}
//------------------------------------------------//

//Returning Values from Loops
//-------------------------------------------------//
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
//---------------------------------------------------//

// 2. Conditional Loops with while

//-------------------------------------------------//
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
//--------------------------------------------------//
// 3. Looping Through a Collection with for
//------------------------------------------------//
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
// This approach is error prone; we could cause the program to panic if the index value or test condition are incorrect. For example, if you changed the definition of the a array to have four elements but forgot to update the condition to while index < 4, the code would panic. It’s also slow, because the compiler adds runtime code to perform the conditional check of whether the index is within the bounds of the array on every iteration through the loop.//
//-----------------------------------------------------------------------------------------//

// As a more concise alternative, you can use a for loop and execute some code for each item in a collection.//
//--------------------------------------------------------------------------------------------//
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }
}
//------------------------------------------------------//
//Using for loop to reverse a range//
//---------------------------------------------//
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
//-------------------------------------------------//



}