fn main() {
	const say_hello: bool = false;

	if say_hello {
		println!("Hello World!");
	} else {
		println!("Bye World!");
	}

	if !say_hello {
		println!("`say_hello` is not `true`");
	}

	if true {
		//
	} else if false {
		println!("testing")
	} else {
		println!("nope")
	}

	let mut x = 0;
	loop {
		println!("Round: {x}");
		x += 1;
		if x > 99 {
			break;
		}

		let mut y = 0;
		loop {
			print!(y);
			y = y + 1;
			if y >= 99 {
				println!();
				break;
			}
		}
	}
}
