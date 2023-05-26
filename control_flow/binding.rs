fn age() -> u32 {
    15
}

fn main2() {
    println!("Tell me what type of person you are");

    match age() {
        0 => println!("I haven't celebrated my first birthday yet"),
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        n @ 13..=19 => println!("I'm a teen of age {:?}", n),
        n => println!("I'm an old person of age {:?}", n),
    }
}

fn some_number() -> Option<u32> {
    Some(43)
}

fn main() {
    match some_number() {
        Some(n @ 42) => println!("The Answer: {}!", n),
        Some(43) => println!("Not interesting.. {}", 43),
        _ => (),
    }
}
