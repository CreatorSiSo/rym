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

	let mut num = 0;
	loop {
		println!("{num}");
		num = num + 1;
	}
}
