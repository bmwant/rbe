enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

fn main() {
    let x = Operations::Add;
}

impl Operations {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}
