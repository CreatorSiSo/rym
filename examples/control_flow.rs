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

	let mut x = 0;
	loop {
		println!("Round: {x}");
		x = x + 1;
		if x > 3 {
			break;
		}

		let mut y = 0;
		loop {
			println!("{y}");
			y = y + 1;

			if y > 3 {
				break;
			}
		}
	}
}
