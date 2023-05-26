fn main() {
    // tuples()
    // arrays()
    enums()
}

fn tuples() {
    let triple = (0, -2, 3);

    println!("Tell me about {:?}", triple);

    match triple {
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..) => println!("First is `1` and the rest doesn't matter"),
        (.., 2) => println!("last is `2` and the rest doesn't matter"),
        (3, .., 4) => println!("First is `3`, last is `4`, and the rest doesn't matter"),
        _ => println!("It doesn't matter what they are"),
    }
}

fn arrays() {
    let array = [1, -2, 6];

    match array {
        [0, second, third] =>
            println!("ar[0] = 0 ar[1] = {} ar[2] = {}", second, third),

        [1, _, third] => println!("ar[0] = 1, ar[2] = {} and ar[1] was ignored", third),
        [-1, second, ..] => println!(
            "ar[0] = 1, ar[1] = {} and all the other ones were ignored", second
        ),
        // [-1, second] => println!("just try"),
        [3, second, tail @ ..] => println!(
            "ar[0] = 3, ar[1] = {} and the other elements were {:?}",
            second, tail
        ),
        [first, middle @ .., last] => println!(
            "ar[0] = {}, middle = {:?}, ar[2] = {}",
            first, middle, last
        ),
    }
}

#[allow(dead_code)]
enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn enums() {
    let color = Color::RGB(122, 17, 40);

    println!("What color is it?");
    match color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) =>
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) => 
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yellow {}, key (black): {}!", 
                c, m, y, k)
    }
}
