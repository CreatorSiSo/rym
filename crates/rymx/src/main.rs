mod debug;
mod file;
mod repl;

fn main() -> Result<(), std::io::Error> {
	let mut args = std::env::args().skip(1);

	match args.next() {
		Some(value) => match value.as_str() {
			"help" | "-h" | "--help" => println!("Usage: rys [path to script]"),
			path => file::exec(path)?,
		},
		None => repl::exec(),
	}

	Ok(())
}
