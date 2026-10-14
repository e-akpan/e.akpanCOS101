fn main() {

    // create an empty vector "CITY"
    let mut city : Vec<String> = Vec::new();
    //PRINT CITY VECTOR
    println!("The city vector has element {}",city.len());
    //PUSH NEW ELEMENTS INTO
    let mut input1 = String::new();
    println!("HOW MANY CITIES DO YOU WANT TO ENTER");
    std::io::stdin().read_line(&mut input1).expect("FAILED TO READ INPUT");
    let city_num:i32 = input1.trim().parse().expect("INVALID INPUT");
    for count in 0..city_num {
        let mut input2 = String::new();
        println!("Enter city {}",count+1);
        std::io::stdin().read_line(&mut input2).expect("Failed to read input");
        let new_city:String = input2.trim().parse().expect("Invalid input");
        city.push(new_city);
    }
    print!("Your preferred cities are:\n");
    let mut count=1;
    //LOOP TO ITERATE ELEMENTS IN VECTOR
    for i in city
    {
        //iterating through i on the vector
        println!("{} {}", count, i); 
    }
}