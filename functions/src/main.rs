fn main() {
    println!("Outer Function");

    another_function();
}

fn another_function() {
    println!("Inner function");
}
