fn main() {
    //  Mutable value changed

    let mut x = 15;
    println!("The value of x: {}", x);
    x = 15 + 5;
    println!("The value of x: {}", x);

    // Without Mutable value Changed

    let x = 15;
    println!("The value of x: {}", x);
    let x = x + 1;
    println!("The value of x: {}", x);
    let x = x + 1;
    println!("The value of x: {}", x);

    // Shadowing

    {
        let x = x + 1;
        println!("The value of x: {}", x);
    }

    let x = x + 1;
    println!("The value of x: {}", x);

    let x = 50;
    println!("The value of x: {}", x);

    let x = "String";
    println!("The value of x: {}", x);

    // cannot concat like this
    // let x = x + "Data Type";
    // println!("The value of x: {}", x);

    // Constants

    const HOURS_IN_DAY: usize = 24; // Can't change the value of constant
    println!("Hours In Day: {}", HOURS_IN_DAY);
}
