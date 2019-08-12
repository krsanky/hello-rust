fn main() {
	let x = 5 + /* 90 + */ 5;

	let logical: bool = true;
	let a_float: f64 = 1.0;
	let an_int = 5i32;

	println!("x = {}", x);
	println!("logical:{} flt:{} int:{}", logical, a_float, an_int);

	let _def_flt = 3.0; // f64
	let _def_int = 7; // i32

	let mut inferred_type = 12;
	println!("inf_type:{}", inferred_type);
	inferred_type = 4294967296i64;
	println!("inf_type:{}", inferred_type);

	let inferred_type = true;
	println!("inf_type:{}", inferred_type);

	println!("one million :{}", 1_000_000u32);
}

