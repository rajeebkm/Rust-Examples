#[derive(Debug)]
#[allow(dead_code)]


struct Point{
    x:f32,
    y:f32
}

#[allow(dead_code)]
struct Rectangle{
    width:f32,
    height:f32
}

fn main(){

    let bottom_left_point:Point=Point { x: 5.0, y: 10.0 };
    let top_left_point:Point=Point { x: 5.0, y: 20.0 }; 
    let bottom_right_point=Point{x:20.0, ..bottom_left_point};
    let top_right_point=Point{x:20.0, ..top_left_point};


    let width=bottom_right_point.x-bottom_left_point.x;
    let height=top_right_point.y-bottom_right_point.y;


    let _rectangle=Rectangle{
        width:width,
        height:height
    };

    let area_of_rectangle=rect_area(&_rectangle);
    println!("Area of Rectangle is: {:#?} pixel squares", area_of_rectangle);


    fn rect_area(rectangle:&Rectangle)->f32{
        rectangle.width*rectangle.height
    }

}