    use std::io;
    //Quadratic Roots   
    fn main () {


        let mut a = String::new();
        let mut b = String::new();
        let mut c = String::new();

        println!("What is the value for a?");
        io::stdin().read_line(&mut a).expect("Failed to read a");
        let a:i32 = a.trim().parse().expect("Failed to read a");
        println!("The value for a is {}",a);

        println!("What is the value for b?");
        io::stdin().read_line(&mut b).expect("Failed to read b");
        let b:i32 = b.trim().parse().expect("Failed to read b");
        println!("The value for b is {}",b);

        println!("What is the value for c?");
        io::stdin().read_line(&mut c).expect("Failed to read c");
        let c:i32 = c.trim().parse().expect("Failed to read c");
        println!("The value for c is {}",c);


        println!("Finding Discriminant");
        let d:i32 = b*b - 4*a*c;
        println!("Discriminant (d) = {}",d);

        if d > 0 {
            println!("two distinct roots");

        } else if d == 0 {
            println!("exactly one real root");

        } else if d < 0 {
            println!("no real roots");
            
        } else {
            println!("No record");
        }
    } 

