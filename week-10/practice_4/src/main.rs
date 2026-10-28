fn main() {
    let v  = vec![15, 20, 25, 25, 45, 55];
    print_vector(v.clone());
    println!("{:?}", print_vector(v.clone()));
}
fn print_vector(w:Vec<i32>) {
    println!("Inside print_vector function {:?}", w)
}