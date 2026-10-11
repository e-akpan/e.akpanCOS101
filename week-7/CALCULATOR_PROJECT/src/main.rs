use std::io;

fn trapezium() -> f32 {
    let mut height = String::new();
    let mut base1 = String::new();
    let mut base2 = String::new();

    println!("Enter the height:");
    io::stdin().read_line(&mut height).expect("Failed to read input");
    let height: f32 = height.trim().parse().expect("Input not a number");

    println!("Enter the first base:");
    io::stdin().read_line(&mut base1).expect("Failed to read input");
    let base1: f32 = base1.trim().parse().expect("Input not a number");

    println!("Enter the second base:");
    io::stdin().read_line(&mut base2).expect("Failed to read input");
    let base2: f32 = base2.trim().parse().expect("Input not a number");

    height / 2.0 * (base1 + base2)
}

fn rhombus() -> f32 {
    let mut diagonal1 = String::new();
    let mut diagonal2 = String::new();

    println!("Enter the first diagonal:");
    io::stdin().read_line(&mut diagonal1).expect("Failed to read input");
    let diagonal1: f32 = diagonal1.trim().parse().expect("Input not a number");

    println!("Enter the second diagonal:");
    io::stdin().read_line(&mut diagonal2).expect("Failed to read input");
    let diagonal2: f32 = diagonal2.trim().parse().expect("Input not a number");

    0.5 * diagonal1 * diagonal2
}

fn parallelogram() -> f32 {
    let mut base = String::new();
    let mut altitude = String::new();

    println!("Enter the base:");
    io::stdin().read_line(&mut base).expect("Failed to read input");
    let base: f32 = base.trim().parse().expect("WRONG INPUT not anumber");

    println!("Enter the altitude:");
    io::stdin().read_line(&mut altitude).expect("Failed to read input");
    let altitude: f32 = altitude.trim().parse().expect("Input not a number");

    base * altitude
}

fn cube() -> f32 {
    let mut side = String::new();

    println!("Enter the side of the cube:");
    io::stdin().read_line(&mut side).expect("Failed to read input");
    let side: f32 = side.trim().parse().expect("Input not a number");

    6.0 * side * side
}

fn cylinder() -> f32 {
    let mut radius = String::new();
    let mut height = String::new();

    println!("Enter the radius:");
    io::stdin().read_line(&mut radius).expect("Failed to read input");
    let radius: f32 = radius.trim().parse().expect("Input not a number");

    println!("Enter the height:");
    io::stdin().read_line(&mut height).expect("Failed to read input");
    let height: f32 = height.trim().parse().expect("WRONG INPUT not a number");

    3.14 * radius * radius * height
}

fn main() {
    let mut choice = String::new();

    println!("===== SHAPE CALCULATOR =====");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Surface Area of Cube");
    println!("5. Volume of Cylinder");
    println!("Enter your choice:");

    io::stdin().read_line(&mut choice).expect("Failed to read input");

    if choice.trim() == "1" {
        let result = trapezium();
        println!("Area of Trapezium = {}", result);
    } else if choice.trim() == "2" {
        let result = rhombus();
        println!("Area of Rhombus = {}", result);
    } else if choice.trim() == "3" {
        let result = parallelogram();
        println!("Area of Parallelogram = {}", result);
    } else if choice.trim() == "4" {
        let result = cube();
        println!("Surface Area of Cube = {}", result);
    } else if choice.trim() == "5" {
        let result = cylinder();
        println!("Volume of Cylinder = {}", result);
    } else {
        println!("Invalid choice");
    }
}
