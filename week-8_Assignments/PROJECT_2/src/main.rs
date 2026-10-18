fn main() {
    let names= vec!["Queen", "Japhet", "ISREAL"];
    println!("Developer's names are:");
    println!("{}",names[0]);
    println!("{}",names[1]);
    println!("{}",names[2]);

    let years = vec![5, 12, 14];

    let mut highest_years = 0;
    let mut index = 0;

    for l in 0..years.len()  {
        if highest_years < years[l] {
            highest_years = years[l];
            index = l; 
        }        
    }
    println!("");
    println!("The highest developer is {} with {} years of programming experience", names[index], highest_years);

}



