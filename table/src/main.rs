use std::io;
fn main() {
    println!("Please enter a table number: ");
    let mut table = String::new();
    io::stdin()
        .read_line(&mut table)
        .expect("Failed to read line");
    let table: i32 = table.trim().parse().expect("Please type a number!");
    table_function(table);
}
fn table_function(x: i32) {
    for i in 1..11 {
        println!("{} * {} = {}", x, i, x * i);
    }
}
