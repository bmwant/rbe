fn apply<F>(mut f: F) where
    F: FnMut() {
    f();
}

fn apply_to_3<T>(f: T) -> i32 where
    T: Fn(i32) -> i32 {
    
    f(3)
}

fn main() {
    use std::mem;

    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();

    let diary = || {
        println!("I said {}.", greeting);

        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // mem::drop(farewell);
    };

    apply(diary);

    let double = |x| 2 * x;
    println!("3 doubled: {}", apply_to_3(double));
}
