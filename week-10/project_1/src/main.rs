use std::io;

#[derive(Debug)]
struct Mr_Ogbeifuna_Electronics {
    Brand:String,
    Quantity:u32,
    Price:i32
}

impl Mr_Ogbeifuna_Electronics {
    fn calculate_cost(&self, quantity: u32) -> i32 {
        self.Price * quantity as i32
    }
}

fn main() {
    println!("==============================================");
    println!("    ALABA INTERNATIONAL MARKET. LAGOS NIGERIA ");
    println!("==============================================");
    println!("Note: Out of Enough Stock                     ");
    println!("Available products: Laptop                    ");
    println!("\nMaximum Quantity of purchase per time = 3     ");
    println!("\nTypes of Brand: \nHP \nIBM \nToshiba \nDell   ");
    println!("");

    let electronics1 = Mr_Ogbeifuna_Electronics {
        Brand:String::from("HP"),
        Quantity: 10,
        Price: 650_000
    };

    let electronics2 = Mr_Ogbeifuna_Electronics {
        Brand:String::from("IBM"),
        Quantity: 6,
        Price: 755_000
    };
    let electronics3 = Mr_Ogbeifuna_Electronics {
        Brand:String::from("Toshiba"),
        Quantity: 10,
        Price: 550_000
    };
    let electronics4 = Mr_Ogbeifuna_Electronics {
        Brand:String::from("Dell"),
        Quantity: 4,
        Price: 850_000
    };
    display(&electronics1);
    display(&electronics2);
    display(&electronics3);
    display(&electronics4);

    let hp_cost = electronics1.calculate_cost(3);
    let ibm_cost = electronics2.calculate_cost(3);
    let toshiba_cost = electronics3.calculate_cost(3);
    let dell_cost = electronics4.calculate_cost(3);

    let total_cost = hp_cost + ibm_cost + toshiba_cost + dell_cost;

    println!("Total cost for 3 laptops from each brand: ₦{}", total_cost);

    println!("");
    println!("What Brand will you like to purchase from");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).unwrap();
    println!("Quantity:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).unwrap();
    let mut input:u32 = input2.trim().parse().unwrap();
    let inputA = input1.trim().to_lowercase();

    println!("Please Verify Product and Quanity: {}, {}",inputA, input);
    if inputA == electronics1.Brand.to_lowercase() {
        if input <= 3 {
            let total_cost = input as i32 * electronics1.Price;
            println!("Brand: {} \nQuantity: {} \nTotal Cost To Be Paid: {}", electronics1.Brand, input, total_cost);
        }
        else {
            println!("Above Maximum quantity of purchase per time. \n Contact our Customer Service: +234 001 2004 890");
        }

        let remaining_quantity = electronics1.Quantity - input;

        println!("Available Quantity After Purchase: {}", remaining_quantity);
    }
    else if inputA == electronics2.Brand.to_lowercase() {
        if input <= 3 {
            let total_cost = input as i32 * electronics2.Price;
            println!("Brand: {} \nQuantity: {} \nTotal Cost To Be Paid: {}", electronics2.Brand, input, total_cost);
        }else {
            println!("Above Maximum quantity of purchase per time. \n Contact our Customer Service: +234 001 2004 890");
        }

        let remaining_quantity = electronics2.Quantity - input;

        println!("Available Quantity After Purchase: {}", remaining_quantity);
    }
    else if inputA == electronics3.Brand.to_lowercase() {
        if input <= 3 {
            let total_cost = input as i32 * electronics3.Price;
            println!("Brand: {} \nQuantity: {} \nTotal Cost To Be Paid: {}", electronics3.Brand, input, total_cost);
        }
        else {
            println!("Above Maximum quantity of purchase per time. \n Contact our Customer Service: +234 001 2004 890");
        }
        let remaining_quantity = electronics3.Quantity - input;

        println!("Available Quantity After Purchase: {}", remaining_quantity);
    }
    else if inputA == electronics4.Brand.to_lowercase() {
        if input <= 3 {
            let total_cost = input as i32 * electronics4.Price;
            println!("Brand: {} \nQuantity: {} \nTotal Cost To Be Paid: {}", electronics4.Brand, input, total_cost);
        }
        else {
            println!("\n Purchase is above maximum quantity of purchase per time. \nContact our Customer Service: (+234 001 2004 890) for further purchases");
        }
        let remaining_quantity = electronics4.Quantity - input;

        println!("Available Quantity After Purchase: {}", remaining_quantity);
    }
    else {
        println!("Brand Not Found");
    }

}

fn display( electronics: &Mr_Ogbeifuna_Electronics) {
    println!("Brand is: {} \nQuanity: {} \nPrice is {}", electronics.Brand, electronics.Quantity, electronics.Price);    
}