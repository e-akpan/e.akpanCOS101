use std::fs::File;
use std::io::Write;
use std::io;

fn main () {
    loop {

        println!("EFCC RECORD SYSTEM");
        println!("--------------------");
        println!("==MENU==");
        println!("1. Merge and Save Records  \n2. Search Records By Zone \n3. EXIT");
        println!("Enter your choice: ");

        let mut decision = String::new();
        io::stdin().read_line(&mut decision).unwrap();
        let decion = decision.trim();
        let commissioners = vec!["Aigbogun Alamba Daudu", "Murtala Afeez Bendu", "Okorocha Calistus Ogbona", "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye"];
        let ministry = vec!["Internal Affairs", "Justice", "Defense", "Power & Steel", "Petroleum"];
        let zone = vec!["South West", "North East", "South South", "South West","South East"];


        if decion == "1" {
            let mut efcc_file = File::create("EFCCC_MERGER").unwrap();



            for z in 0..zone.len() {
                efcc_file.write_all(format!("COMMISSIONERS: {} | MINISTRY: {} | ZONE: {}\n", commissioners[z], ministry[z], zone[z]).as_bytes()).unwrap();
            }
            println!("All records merged and saved successfully!");
            println!("Total records saved: {}", zone.len());
            println!("File created: EFCCC_MERGER");
            println!("Status: SUCCESS");
            println!("Returning to main menu...\n");
        }   
        else if decion == "2" {
            println!("Enter the geopolitical zone to search: ");
        
            let mut geopolitical_zone = String::new();
            io::stdin().read_line(&mut geopolitical_zone).unwrap();
        
            let clean_geopolitical_zone = geopolitical_zone.trim();
            let mut matches_data = Vec::new();
            for z in 0..zone.len() {
                if clean_geopolitical_zone.to_lowercase() == zone[z].to_lowercase() { 
                    println!("
                    COMMISSIONERS: {}
                    MINISTRY: {} 
                    ZONE: {}\n", commissioners[z], ministry[z], zone[z]);
                    println!("{} is a commisioner serving in {}, from {}", commissioners[z], ministry[z], zone[z]);
                    matches_data.push(commissioners[z]);
                }
            }
             println!("Total Matching Records: {}", matches_data.len());
            if matches_data.len() == 0 {
                println!("No records found for the geopolitical zone you Entered");
                println!("\nAvailable Zones");
                println!("==================");
                println!(" \n South West \n North East\n South South\n South East");
            }
        }
        else if decion == "3" {
            println!("Exiting EFCC RECORD SYSTEM.");
            break;
        }
        else {
            println!("Invalid choice. Please select 1, 2, or 3.");
        }   
    }   
       
}
