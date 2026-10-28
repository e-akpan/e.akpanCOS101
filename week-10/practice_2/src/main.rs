fn main() {
    let v = vec![10, 20, 30, 40, 70];
    let v2 = v.clone();
    display(v2);
    display(v.clone());
    println!("In main {:?}", display(v.clone()));
}

fn display(v:Vec<i32>) {
    println!("Inside display {:?}", v);
}
