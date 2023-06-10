fn apply<F>(mut f: F) where
    F: FnMut() {
    f();
}

fn main() {
    let mut x = 7;
    let mut print = || {
        println!("{}", x); 
        x += 1; 
    };

    apply(&mut print);
    apply(&mut print);
}
