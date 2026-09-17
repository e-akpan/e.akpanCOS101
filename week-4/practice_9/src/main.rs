fn main() {

    let mut count = 0;

    for num in 3..23 {
        if num > 13 {
            println!("{:?}",num);
            continue;
        }
        count +=3;
    }
    println!(" The count of values greater than 13 (between 3 and 22) is: {}",count);
    //outputs 10
}
