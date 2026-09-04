fn main() {
	let Product_1 = "Toshiba";
	let Qty_1: f64 = 2.0; 
	let Amount_1: f64 = 450000.00;
	let Item_1 = Qty_1 * Amount_1;

	let Product_2 = "Mac";
	let Qty_2: f64 = 1.0;
	let Amount_2: f64 = 1500000.00;
	let Item_2 = Qty_2 * Amount_2;


	let Product_3 = "HP";
	let Qty_3: f64 = 3.0;
	let Amount_3: f64 = 750000.00;
	let Item_3 = Qty_3 * Amount_3;


	let Product_4 = "Dell";
	let Qty_4: f64 = 3.0;
	let Amount_4: f64 = 2850000.00;
	let Item_4 = Qty_4 * Amount_4;


	let Product_5 = "Acer";
	let Qty_5: f64 = 1.0;
	let Amount_5: f64 = 250000.00;
	let Item_5 = Qty_5 * Amount_5;

	let Sum = Item_1 + Item_2 + Item_3 + Item_4 + Item_5;

	let Total_Qty = Qty_1 + Qty_2 + Qty_3 + Qty_4 + Qty_5;

	let Average = Sum / Total_Qty;

	println!("Sum is {} and Average is {}", Sum, Average);

}