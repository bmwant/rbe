fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() -> String {
    let mut text = "FnMut".to_owned();

    let ret_fun = move || {
        println!("This is a: {}", text);
        text.push_str("!!!");
        text.to_owned()
        // text.to_owned()
    };
    ret_fun
}

fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();

    move || println!("This is a: {}", text)
}

fn main() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    let text = fn_mut();
    fn_once();
    println!("This is mutated text: {}", text);
    println!("This is mutated text: {}", fn_mut());
    println!("This is mutated text: {}", fn_mut());
}

