//Rust program to count numbers

use std::io;
fn main(){

    println!("Enter a lower bound");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let lower_bound:i16 = input2.trim().parse().expect("Failed to input");

    println!("Enter upper_Bound");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let upper_Bound:i16 = input3.trim().parse().expect("Failed to input");

    for x in lower_bound..upper_Bound{ // upper_Bound is not inclusive

      println!("Count Level is {}",x);
    }
}
