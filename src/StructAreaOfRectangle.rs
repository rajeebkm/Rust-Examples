struct Rectangle{
    width: u32,
    height: u32
}

fn main(){

    let rect1=Rectangle{
        width:30,
        height:40
    };

    println!("THe area of rectangle is: {:#?} pixel squares", area(&rect1));
}

fn area(rectangle: &Rectangle)->u32{
    rectangle.width*rectangle.height
}

