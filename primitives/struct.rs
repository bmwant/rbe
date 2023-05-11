#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}


fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle { top_left: Point { x: x1, y: y1 }, bottom_right: Point { x: x2, y: y2 } } = rect;
    let h: f32 = y1 - y2;
    let w: f32 = x2 - x1;
    println!("width is {:?} and height is {:?}", w, h);
    h * w
}

fn square(p: Point, h: f32) -> Rectangle {
    let bottom_right = Point { x: p.x + h, y: p.y - h };
    return Rectangle { top_left: p, bottom_right: bottom_right }; 
}

fn main() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    let point: Point = Point { x: 10.3, y: 0.4 };

    println!("point coordinates: ({} {})", point.x, point.y);

    let mut bottom_right = Point { x: 5.2, ..point };

    println!("second point: ({} {})", bottom_right.x, bottom_right.y);
    bottom_right.y = -0.2;
    let Point { x: left_edge, y: top_edge } = point;
    let Point { x: new_var, .. } = point;

    let _rectangle = Rectangle {
        top_left: Point { x : left_edge, y : top_edge },
        bottom_right: bottom_right,
    };

    let _unit = Unit;

    let pair = Pair(1, 0.1);

    println!("new var is {:?}", new_var);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("Rectangle area is {:?}", rect_area(_rectangle));
    let new_point = Point { x: 1.0, y: 1f32 };
    let rect = square(new_point, 4.0);
    let area = rect_area(rect);
    println!("Area of a square is {:?}", area);
    
}
