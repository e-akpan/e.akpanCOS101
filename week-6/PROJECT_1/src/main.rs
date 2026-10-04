fn main() {
    use std::io;

    //Authentication Process. Pin and Username verification

    println!("Create Username! ");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Failed Username");
    println!("Your username is {}",username);
    println!("Hi {}",username);
    println!("");

    
    let mut attempts = 0;
    let mut five_digit_pin = String::new();

    while attempts < 3 {
        println!("Please Enter Your Five Digits Pin (Attempts left: {}", 3 - attempts);

        five_digit_pin.clear();

        io::stdin().read_line(&mut five_digit_pin).expect("invalid password");


        if five_digit_pin.trim().len() == 5 {
        println!("Pin sucessfully created 🎉");
        break;

        } else if five_digit_pin.trim().len() < 5 {
            attempts +=1;
        println!(" Too Short! Pin should be Five Digits. 
                    Try again");
        } else if five_digit_pin.trim().len() > 5 {
            attempts +=1;
        println!(" Too Long! Pin should be Five Digits. 
                    Try again"); 
       }
    }


    // This should only happen if the length of pin is correct and if they ran out of tries.        println!("Please re-enter your pin to login");
    if attempts == 3 {
        println!("Too many failed attempts.");
        return;
    }

    let mut login_attempts_verification = 0;
    let mut authenticication = false;

    while login_attempts_verification < 3 {
        println!("PLease re-enter your pin to login");

        let mut login_pin = String::new();
        io::stdin().read_line(&mut login_pin).expect("Invalid pin");

        if login_pin.trim() == five_digit_pin.trim() {

            println!("Correct pin. Access granted {}",username.trim());
            authenticication = true;
            break;
        } 
        else {
             login_attempts_verification += 1;
            println!("Pins Not matching. Attempts left: {}", 3 - login_attempts_verification);
        }
    }

    if !authenticication {
        println!("Too many incorrect login attempts. Account Blocked. 
            Please call Account mananger");
        return;
    }


    loop {
        // Clear screen 

        println!("");

        println!("==MENU==");
        println!("╔══════════════════════════════════════════════════════╗");
        println!("║                  AVAILABLE FOOD AND PRICES           ║");
        println!("╠══════════════════════════════════════════════════════╣");
        println!("║  [P] Poundo Yam / Edinkaiko Soup       - ₦3,200      ║");
        println!("║  [F] Fried Rice & Chicken              - ₦3,000      ║");
        println!("║  [A] Amala & Ewedu Soup                - ₦2,500      ║");
        println!("║  [E] Eba & Egusi Soup                  - ₦2,000      ║");
        println!("║  [W] White Rice & Stew                 - ₦2,500      ║");
        println!("╚══════════════════════════════════════════════════════╝");
        print!("   Select Food Letter (P, F, A, E, W): ");

        println!("Hi! {} What will you like to order?",username);


        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).expect("Failed to read input");
        
        let order = input1.trim();

        if order == "P" {
            let food_name = "Pounded Yam / Edinkaiko Soup";
            let price = 3200;

            println!("Food: {}",food_name);
            println!("Price: {}", price);

            let mut quantity_input = String::new();
            println!("Quantity: ");
            io::stdin().read_line(&mut quantity_input).expect("Failed to read quantity");
            let Quantity:u32 = quantity_input.trim().parse().expect("Not a valid number");

            let total = price * Quantity;
            println!("Total: ₦{}", total);
            println!("");


            if total > 10_000 {
                println!("You have a 5% discount 🥳🥳");
                let discount = total * 5 / 100;
                let final_price = total - discount;
                println!("");
                println!("The price is: ₦{}", final_price);
                println!("");
            }
            println!("");
            println!("How would you like to pay?");
            println!("Option 1. Bank Transfer");
            println!("Option 2. eTranzact");
            let mut payment = String::new();
            io::stdin().read_line(&mut payment).expect("Failed to read Payment");
            let payment = payment.trim();

            println!("");
            if payment == "1" {

                println!("Payment method: Bank Transfer");
            } else if payment == "2" {
                println!("Payment method: eTranzact");
            }
            else {
                println!("Invalid Payment Method. SORRY!!");
            }

            println!("Please enter your PIN to confirm payment:");

            let mut payment_pin = String::new();
            io::stdin().read_line(&mut payment_pin).expect("Failed to read PIN");

            if payment_pin.trim() == five_digit_pin.trim() {
                println!("PIN verified successfully!");
                println!("Payment successful! 🎉");
            } else {
                println!("Incorrect PIN.");
                println!("Payment failed.");
            }
        }
        else if order == "F" {
            let food_name = "Fried Rice and Chicken";
            let price = 3000;

            println!("Food: {}",food_name);
            println!("Price: {}", price);

            let mut quantity =  String::new();
            println!("Quantity: ");

            io::stdin().read_line(&mut quantity).expect("Failed to read Quantity");
            let Quantity:u32 = quantity.trim().parse().expect("Not a valid number");
            let Total = price * Quantity;
            println!("");
            println!("Total: ₦{}", Total);
            println!("");

            if Total > 10_000 {
                println!("Hi Dear, you have a 5% discount 🥳🥳");
                let discount = Total * 5 / 100;
                let price_to_pay = Total - discount;
                println!("");
                println!("The price is: ₦{}", price_to_pay);
            }
            println!("");
            println!("How would you like to pay?");
            println!("Option 1. Bank Transfer");
            println!("Option 2. eTranzact");
            let mut payment = String::new();
            io::stdin().read_line(&mut payment).expect("Failed to read Payment");
            let payment = payment.trim();

            println!("");
            if payment == "1" {

                println!("Payment method: Bank Transfer");
            } else if payment == "2" {
                println!("Payment method: eTranzact");
            }
            else {
                println!("Invalid Payment Method. SORRY!!");
            }

            println!("Please enter your PIN to confirm payment:");

            let mut payment_pin = String::new();
            io::stdin().read_line(&mut payment_pin).expect("Failed to read PIN");

            if payment_pin.trim() == five_digit_pin.trim() {
                println!("PIN verified successfully!");
                println!("Payment successful! 🎉");
            } else {
            println!("Incorrect PIN.");
            println!("Payment failed.");
            }
        }
        else if order == "A" {
            let food_name = "Amala and Ewedu";
            let Price = 2500;
            println!("Food: {}",food_name);
            println!("Price: {}", Price);

            let mut quantity = String::new();
            println!("Quantity: ");

            io::stdin().read_line(&mut quantity).expect("No Quantity");
            let Quantity:u32 = quantity.trim().parse().expect("Wrong Quantity");
            let Total = Price * Quantity;
            println!("Total amount is: ₦{}", Total);
            println!("");

            if Total > 10_000 {
                println!("Hi Dear, you have a 5% discount 🥳🥳");
                let discount = Total * 5 / 100;
                let price_to_pay = Total - discount;
                println!("");
                println!("The price is: ₦{}", price_to_pay);
            }
            println!("");
            println!("How would you like to pay?");
            println!("Option 1. Bank Transfer");
            println!("Option 2. eTranzact");
            let mut payment = String::new();
            io::stdin().read_line(&mut payment).expect("Failed to read Payment");
            let payment = payment.trim();

            println!("");
            if payment == "1" {

                println!("Payment method: Bank Transfer");
            } else if payment == "2" {
                println!("Payment method: eTranzact");
            }
            else {
                println!("Invalid Payment Method. SORRY!!");
            }

            println!("Please enter your PIN to confirm payment:");

            let mut payment_pin = String::new();
            io::stdin().read_line(&mut payment_pin).expect("Failed to read PIN");

            if payment_pin.trim() == five_digit_pin.trim() {
                println!("PIN verified successfully!");
                println!("Payment successful! 🎉");
            } else {
                println!("Incorrect PIN.");
                println!("Payment failed. ");
            }
        }
        else if order == "E"{
            let food_name = "Eba and Egusi Soup";
            let Price = 2000;
            println!("Food: {}",food_name);
            println!("Price: {}", Price);

            let mut quantity = String::new();
            println!("Quantity: ");

            io::stdin().read_line(&mut quantity).expect("No Quantity");
            let Quantity:u32 = quantity.trim().parse().expect("Wrong Quantity");
            let Total = Price * Quantity;
            println!("Total amount is: ₦{}", Total);
            println!("");

            if Total > 10_000 {
                println!("Hi Dear, you have a 5% discount 🥳🥳");
                let discount = Total * 5 / 100;
                let price_to_pay = Total - discount;
                println!("");
                println!("The price is: ₦{}", price_to_pay);
            }
            println!("");
            println!("How would you like to pay?");
            println!("Option 1. Bank Transfer");
            println!("Option 2. eTranzact");
            let mut payment = String::new();
            io::stdin().read_line(&mut payment).expect("Failed to read Payment");
            let payment = payment.trim();

            println!("");
            if payment == "1" {

                println!("Payment method: Bank Transfer");
            } else if payment == "2" {
                println!("Payment method: eTranzact");
            }
            else {
                println!("Invalid Payment Method. SORRY!!");
            }

            println!("Please enter your PIN to confirm payment:");

            let mut payment_pin = String::new();
            io::stdin().read_line(&mut payment_pin).expect("Failed to read PIN");

            if payment_pin.trim() == five_digit_pin.trim() {
                println!("PIN verified successfully!");
                println!("Payment successful! 🎉");
            } else {
                println!("Incorrect PIN.");
                println!("Payment failed. ");
            }
        }
        else if order == "W" {
            let food_name = "White Rice and Stew";
            let Price = 2500;
            println!("Food: {}",food_name);
            println!("Price: {}", Price);

            let mut quantity = String::new();
            println!("Quantity: ");

            io::stdin().read_line(&mut quantity).expect("No Quantity");
            let Quantity:u32 = quantity.trim().parse().expect("Wrong Quantity");
            let Total = Price * Quantity;
            println!("Total amount is: ₦{}", Total);
            println!("");

            if Total > 10_000 {
                println!("You have a 5% discount 🥳🥳");
                let discount = Total * 5 / 100;
                let price_to_pay = Total - discount;
                println!("");
                println!("The price is: ₦{}", price_to_pay);
            }
            println!("");
            println!("How would you like to pay?");
            println!("Option 1. Bank Transfer");
            println!("Option 2. eTranzact");
            let mut payment = String::new();
            io::stdin().read_line(&mut payment).expect("Failed to read Payment");
            let payment = payment.trim();

            println!("");
            if payment == "1" {

                println!("Payment method: Bank Transfer");
            } else if payment == "2" {
                println!("Payment method: eTranzact");
            }
            else {
                println!("Invalid Payment Method. SORRY!!");
            }

            println!("Please enter your PIN to confirm payment:");

            let mut payment_pin = String::new();
            io::stdin().read_line(&mut payment_pin).expect("Failed to read PIN");

            if payment_pin.trim() == five_digit_pin.trim() {
                println!("PIN verified successfully!");
                println!("Payment successful! 🎉");
            } else {
                println!("Incorrect PIN.");
                println!("Payment failed.");
            }
        } else {
            println!("Invalid food selection. Food not Found in Menu Bar");
            break;
        }
    }
}