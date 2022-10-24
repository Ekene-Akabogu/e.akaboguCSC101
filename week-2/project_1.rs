fn main() {
	let p:f64 = 520000000.0;
	let r:f64 = 10.0;
	let t:f64 = 5.0;
	let m = 1.0 + (r / 100.0);
	let k = f64:: powf(m,t);

	let a = p * k;
	println!("Amount is {}",a);
	
	let ci = a - p;
	println!("Compound Interest is {}",ci);
}