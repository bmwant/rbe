fn main() {
    #[allow(unused_variables)]
    let x = 10;
    println!("X = {:X}", 69420);
    println!("{:>5}", 1);
    println!("My {0} is {1}", "zero", "one");

    // #[allow(dead_code)] // disable `dead_code` which warn against unused module
    #[derive(Debug)]
    struct Structure(i32);

    println!("structure {:#?}", Structure(3));

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:*>width$}");
}
