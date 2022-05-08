use std::io;
#[allow(dead_code)]
fn main(){

    loop {
         println!("Enter your number");
         let mut num=String::new();
         io::stdin().read_line(&mut num).expect("Unable to read_line");

        let num: usize=match num.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue
    };
    println!("{}", fizz_buzz(num));
    break;
}
}

fn fizz_buzz(number:usize)->String{
  match number{
        number if(number % 3 == 0 && number % 5 == 0) => "FizzBuzz".to_string(),
        number if(number % 3 == 0) => "Fizz".to_string(),
        number if(number % 5 == 0) => "Buzz".to_string(),
        _ =>{
            format!("{number}")
            }
    }
}
        
//------------------Without taking user input----------------//

fn main(){
    for number in 1..=100 {
    println!("{}", fizz_buzz(number));
    }
}

fn fizz_buzz(value:usize)-> String {
        match (value % 3, value % 5){
        (0, 0) => "FizzBuzz".to_string(),
        (0,_) => "Fizz".to_string(),
        (_,0) => "Buzz".to_string(),
        _ => format!("{value}")
    }
}




// // //-------------------------------------------------//

fn main(){

    for num in 1..101{

        match num{
            num if(num % 3 == 0 && num % 5 == 0) => println!("FizzBuzz"),
            num if (num % 3 == 0) => println!("Fizz"),
            num if (num % 5 == 0)=>println!("Buzz"),
            _  => println!("{num}")
        }
    }
}

// // //--------------------------//

fn main(){

    for num in 1..101 {
        if num % 3 == 0 && num % 5 == 0 {
            println!("FizzBuzz");
        }else if num % 3 == 0 {
            println!("Fizz");
        }else if num % 5 == 0 {
            println!("Buzz");
        }else {
            println!("{num}");
        }
    }
}

   