fn main() {
    let mut x = 10;
    println!("Outer Function");

    another_function();
    string_passed("HelloWorld!", x);
    x = 20;
    string_passed("HelloWorld!", x);
}

fn another_function() {
    println!("Inner function");
}

fn string_passed(str: &str, len: i32) {
    println!("String: {}", str);
    println!("Length: {}", len);
}
