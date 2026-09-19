    use std::io;
    //Quadratic Roots   
    fn main () {


        let mut a = String::new();
        let mut b = String::new();
        let mut c = String::new();

        println!("What is the value for a?");
        io::stdin().read_line(&mut a).expect("Failed to read a");
        let a:f32 = a.trim().parse().expect("Failed to read a");
        println!("The value for a is {}",a);

        println!("What is the value for b?");
        io::stdin().read_line(&mut b).expect("Failed to read b");
        let b:f32 = b.trim().parse().expect("Failed to read b");
        println!("The value for b is {}",b);

        println!("What is the value for c?");
        io::stdin().read_line(&mut c).expect("Failed to read c");
        let c:f32 = c.trim().parse().expect("Failed to read c");
        println!("The value for c is {}",c);


        println!("Finding Discriminant");
        let d:f32 = b*b - 4.0*a*c;
        println!("Discriminant (d) = {}",d);

        if d > 0.0 {
            let root = d.sqrt();

            let x1 = (-b + root) / (2.0 * a);
            let x2 = (-b - root) / (2.0 * a);

            println!(" There are two distinct roots");
            println!("x1 = {}",x1);
            println!("x2 = {}",x2);


        } else if d == 0.0 {
            let x = -b / (2.0 * a);
            println!("There is exactly one real root");
            println!("x = {}",x);


        } else if d < 0.0 {
            println!("no real roots");

        } else {
            println!("No record");
        }
    } 

