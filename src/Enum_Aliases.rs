//-------------------------Type Aliases------------------------------//

// If you use a type alias, you can refer to each enum variant via its alias. This might be useful if the enum's name is too long or too generic, and you want to rename it.

#[derive(Debug)]
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

#[derive(Debug)]
struct ArithmaticOperation{
    x:usize,
    y:usize
}

type Operation = VeryVerboseEnumOfThingsToDoWithNumbers;  //Type alias for enum

// The most common place we can this is in impl blocks using the Self alias.

impl Operation {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}


impl ArithmaticOperation {
    fn math(&self)->usize{
        self.x+self.y
    }

    fn sum (x:usize, y:usize) -> usize{
            x+y
        }
    fn sub (x:usize, y:usize) -> usize{
            x-y
        }
    
    fn newassfunc (x:usize, y:usize)->ArithmaticOperation{
        ArithmaticOperation{
            x:x, 
            y:y
        }
    }
}


fn main() {

    let z = ArithmaticOperation{
        x: 7,
        y: 8
    };

    //Method calling (using . dot operator)

    println!("Sub by Method calling: {}",z.math()); 
    
    //To call the associate function (using :: double colon)

    let assfuncsum = ArithmaticOperation::sum(8, 15);
    println!("Summation: {assfuncsum}");

    let assfuncsub = ArithmaticOperation::sub(15, 8);
    println!("Substraction: {assfuncsub}");

    let checkassfunc = ArithmaticOperation::newassfunc(15, 8);
    println!("{:#?}", checkassfunc);


    let x = Operation::run(&Operation::Add, 6, 5);
    let y = Operation::run(&Operation::Subtract, 6, 5);
    println!("Sum : {x}\nDifference: {y}");
}