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

    let mut electronics1 = Mr_Ogbeifuna_Electronics {
        Brand:String::from("HP"),
        Quantity: 10,
        Price: 650_000
    };

    let mut electronics2 = Mr_Ogbeifuna_Electronics {
        Brand:String::from("IBM"),
        Quantity: 6,
        Price: 755_000
    };
    let mut electronics3 = Mr_Ogbeifuna_Electronics {
        Brand:String::from("Toshiba"),
        Quantity: 10,
        Price: 550_000
    };
    let mut electronics4 = Mr_Ogbeifuna_Electronics {
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

    
    let mut attempts = 0;
    let mut pin_created:bool = false;
    let mut transfer_pin = String::new();

    while attempts < 3 {
        let mut four_digits_pin = String::new();

        println!("");
        println!("");
        println!("Create a Four value transactions pin \nPin must be four value \n Attempts left: {}", 3 - attempts);
        io::stdin().read_line(&mut four_digits_pin).unwrap();
        let pin = four_digits_pin.trim();

        if pin.len() == 4 {
            println!("Pin created successfully");
            transfer_pin = pin.to_string();
            pin_created = true;
            break;
        }
        else if pin.len() < 4 {
            attempts += 1;
            println!("Too Short \nTry again. Attempts left {}",3 - attempts);
        }
        else if pin.len() > 4 {
            attempts  += 1;
            println!("Too Long \nTry again. Attempts left {}", 3 - attempts);
        }
    }
    if !pin_created {
        println!("Too many failed Attempts. \n- Unable to create pin");
        return;
    }
    

    loop {
        println!("");
        println!("What Brand will you like to purchase from");
        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).unwrap();
        println!("Quantity:");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).unwrap();
        let input:u32 = input2.trim().parse().unwrap();
        let inputA = input1.trim().to_lowercase();

        println!("Please Verify Product and Quantity: {}, {}",inputA, input);

        if inputA == electronics1.Brand.to_lowercase() {
            if electronics1.Quantity == 0 {
                println!("Out Of Stock");
            }
            else if input >0 && input <= 3 && input <= electronics1.Quantity {
                let total_cost = input as i32 * electronics1.Price;
                println!("Brand: {} \nQuantity: {} \nTotal Cost To Be Paid: {}", electronics1.Brand, input, total_cost);
                if verification_pin(&transfer_pin) {
                    let remaining_quantity = electronics1.Quantity - input;
                    electronics1.Quantity = remaining_quantity;

                    println!("Purchase successful.");
                    println!("Available Quantity After Purchase: {}", remaining_quantity);
                }
                else {
                    println!("payment not confirmed");
                }
            }
            else {
                println!("Above Maximum quantity of purchase per time. \n Contact our Customer Service: +234 001 2004 890");
            }
        }
        else if inputA == electronics2.Brand.to_lowercase() {
            if electronics2.Quantity == 0 {
                println!("out of stock");
            }
            else if input >0 && input <= 3 && input <= electronics2.Quantity {
                let total_cost = input as i32 * electronics2.Price;
                println!("Brand: {} \nQuantity: {} \nTotal Cost To Be Paid: {}", electronics2.Brand, input, total_cost);
                 if verification_pin(&transfer_pin) {
                    let remaining_quantity = electronics2.Quantity - input;
                    electronics2.Quantity = remaining_quantity;

                    println!("Purchase successful.");
                    println!("Available Quantity After Purchase: {}", remaining_quantity);
                }
                else {
                    println!("payment not confirmed");
                }
            }
            else {
                println!("Above Maximum quantity of purchase per time. \n Contact our Customer Service: +234 001 2004 890");
            }
        }
        else if inputA == electronics3.Brand.to_lowercase() {
            if electronics3.Quantity == 0 {
                println!("out of stock");
            }
            else if input >0 && input <= 3 && input <= electronics3.Quantity {
                let total_cost = input as i32 * electronics3.Price;
                println!("Brand: {} \nQuantity: {} \nTotal Cost To Be Paid: {}", electronics3.Brand, input, total_cost);    
                 if verification_pin(&transfer_pin) {
                    let remaining_quantity = electronics3.Quantity - input;
                    electronics3.Quantity = remaining_quantity;

                    println!("Purchase successful.");
                    println!("Available Quantity After Purchase: {}", remaining_quantity);
                }
                else {
                    println!("payment not confirmed");
                }
            }
            else {
                println!("Above Maximum quantity of purchase per time. \n Contact our Customer Service: +234 001 2004 890");
            }
            
        }
        else if inputA == electronics4.Brand.to_lowercase() {
            if electronics4.Quantity == 0 {
                println!("Out of stock");
            }
            else if input >0 && input <= 3 && input <= electronics4.Quantity {
                let total_cost = input as i32 * electronics4.Price;
                println!("Brand: {} \nQuantity: {} \nTotal Cost To Be Paid: {}", electronics4.Brand, input, total_cost);
                 if verification_pin(&transfer_pin) {
                    let remaining_quantity = electronics4.Quantity - input;
                    electronics4.Quantity = remaining_quantity;

                    println!("Purchase successful.");
                    println!("Available Quantity After Purchase: {}", remaining_quantity);
                }
                else {
                    println!("payment not confirmed");
                }
            }
            else {
                println!("\n Purchase is above maximum quantity of purchase per time. \nContact our Customer Service: (+234 001 2004 890) for further purchases");
            }
        }
        else {
            println!("Brand Not Found");
        }
        println!("Would you like to purchase again/ or let another customer purchase? \n\nYes/No");
        let mut request = String::new();
        io::stdin().read_line(&mut request).unwrap();
        let response = request.trim().to_lowercase();

        let response1 = "yes";
        let reponse2 = "no";

        if  response == response1 {
            println!("proceed");
        }
        else if response == reponse2 {
            println!("Goodbye");
            break;
        }
        else {
            println!("Not a valid response");
            break;
        }
    }
}

fn display( electronics: &Mr_Ogbeifuna_Electronics) {
    println!("Brand is: {} \nQuanity: {} \nPrice is {}", electronics.Brand, electronics.Quantity, electronics.Price);    
}

fn verification_pin (transfer_pin: &str) -> bool {
    println!("How do you want to pay?");
    println!("* eTranszact \n* Transfer");
    let mut ans = String::new();
    io::stdin().read_line(&mut ans).unwrap();
    let opt = ans.trim().to_lowercase();

    if opt == "etranszact" {
        println!("Please pay at Cashier Desk");
        println!("Payment must be confirmed by the Cashier");
        false
    }
    else if opt == "transfer" {
        println!("");
        println!("Please enter your pin to proceed");
        let mut okay = String::new();
        io::stdin().read_line(&mut okay).unwrap();
        let proceed = okay.trim();
        if proceed == transfer_pin {
            println!("Purchase successful.");
            true
        }
        else {
            println!("Invalid transaction pin");
            false
        }
    }
    else {
        println!("Invalid Payment method");
        false
    }

}