fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    let mut s1 = 5;
    let s2 = s1;
    s1 = 8;
    println!("{}", s2);
    println!("{}", s1);

    // Clone Deeply copied
    let s1 = String::from("Tony Stark");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    let fr = String::from("Odin crate");
    get_data(fr);
}

fn get_data(data: String) {
    println!("{}", data);
}
