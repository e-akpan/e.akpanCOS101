fn fn main() {
	let p:f64 = 520,000,000;
	let R:f64 = 10.0;
	let n:f64 = 5.0;

	// compound interest 
	let a = p * ( 1.0 + (r/ 100.0)).powf(n);
	let ci = a - p;
	println!("Amount is {}", a);
    println!("Compound Interest is {}", ci);
}
  

}