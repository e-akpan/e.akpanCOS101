use::std::fs::File;
use::std::io::Write;

fn main() {
    let mut students_info = File::create("student.txt").unwrap();


    let student_name = vec!["Oluchi Mordi", "Adams Aliyu  ", "Shania Bolade", "Adekunle Gold", "Blanca Edemoh",];
    let matric_number = vec!["ACC10211111","ECO10110101  ", "CSC10328828", "EEE11020202", "MEE10202001"];
    let department = vec!["Accounting", "Economics", "Computer", "Electrical", "Mechanical"];
    let  level = vec![300, 100, 200,  200, 100];
    // a for loop
    for i in 0..student_name.len() {
        println!("Name: {}   |  Matric Number: {}    | Department: {}     | Level: {}\n", student_name[i], matric_number[i], department[i], level[i]);
        students_info.write_all(format!("Name: {}   |  Matric Number: {}    | Department: {}     | Level: {}\r\n\r\n", student_name[i], matric_number[i], department[i], level[i]).as_bytes()).unwrap();
    }

}
