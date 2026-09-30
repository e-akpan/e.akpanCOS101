fn main() {
    let name_1 = "Ayomide Adesokan";
    println!("My name is {}", name_1);

    //find and replace
    let name_2 = name_1.replace("Ayomide","Adebare");
    println!("You can also call me {}",name_2);
    let faculty = "Faculty of Science and Technology";
    

    //find and replace 
    let school = faculty.replace("Faculty","School");
    println!("I am a student of the {}", school);
    

}
