use std::io;
//The Incentive Calculator

fn main() {

    let mut age = String::new();
    let mut years_of_experience = String::new();

    println!("Your years of experience under this field?");
    println!("");
    io::stdin().read_line(&mut years_of_experience).expect("Failed to see experience");
    let years_of_experience:u32 = years_of_experience.trim().parse().expect("Failed to see experience");
    println!("");
    println!("Your Years of experience is {}",years_of_experience);
    println!("");

    println!("How old are you?");
    println!("");
    io::stdin().read_line(&mut age).expect("failed to read age");
    let age:u16 = age.trim().parse().expect("Failed to read age");
    println!("");
    println!("Your age is {}",age);


    if years_of_experience > 0 && age >= 40 {
        println!("Your annual incentive is $1,560,000");
    } else if years_of_experience > 0 && age >= 30 && age <=39 {
        println!("Your annual incentive is $1,480,000");

    } else if years_of_experience >0 && age < 28 {
        println!("Your annual incentive is $1,300,000");
    } else if years_of_experience == 0 {
        println!("Your annual incentive is $100,000");
    } else {
        println!("No record found");
    }

}
