use std::io::Write;

fn main() {

    let announce = "week 9 - Rust File Input & Output\n";
    let dept = "Department of Computer Science\n";

    let mut file = std::fs::File::create("Drinks_categories.txt").expect("create failed");
    file.write_all("Welcome to Rust Programming\n".as_bytes()).expect("Write Failed");
    file.write_all(announce.as_bytes()).expect("write failed");
    file.write(dept.as_bytes()).expect("write failed");



    let headers = "Lager, Stout, Non-alcoholic\n";
    let row_1 = "33 Export, Legend, Maltina\n";
    let row_2 = "Desperados, Turbo King, Amstel Malta\n";
    let row_3 = "Goldberg, Williams, Malta Gold\n";
    let row_4 = "Gulder, ,Fayrouz\n";
    let row_5 = "Heineken, , ,\n";
    let row_6 = "Star, , \n";

    file.write_all(headers.as_bytes()).expect("write failed");
    file.write_all(row_1.as_bytes()).expect("write failed");
    file.write_all(row_2.as_bytes()).expect("write failed");
    file.write_all(row_3.as_bytes()).expect("write failed");
    file.write_all(row_4.as_bytes()).expect("write failed");
    file.write_all(row_5.as_bytes()).expect("write failed");
    file.write_all(row_6.as_bytes()).expect("write failed");


}
