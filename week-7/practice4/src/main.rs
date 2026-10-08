use std::io;

fn add(a: i32, b:i32) {
    let sum = a + b;

    println!("Sum of A and B = {}", sum);
}

fn main () {
    let mut input1 = String::new();
    println!("Enter Input For Parameter A:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:i32 = input1.trim().parse().expect("Invalid input");

    let mut input4 = String::new();
    println!("Enter Input For Parameter B:");
    io::stdin().read_line(&mut input4).expect("WRONG INPUT");
    let b:i32 = input4.trim().parse().expect("Wrong");

    //Call add funcion with arguments
    add(a, b);
}
