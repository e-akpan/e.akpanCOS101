fn main() {

    // NAME VECTOR
    let name = vec!["MARY", "SAM", "SALLY", "GREG", "ADE", "MARK", "JUNE", "IFE"];

    //AGE VECTOR
    let age = vec![16,17,19,22,20,21,18,23];

    print!("");
    print!("Age allocation:");
    print!("");

    //LOOP TO ITERATE ELEMENTS IN VECTOR
    for i in 0..age .len()
    {
        //ITERATING THOUGH I ON THE VECTOR
        print!("{} is {} years old\n",name[i],age[i]);
    }
}
