fn fib_recurse(n: f64) -> f64 {
	if n == 0. || n == 1. {
		return n;
	}

	fib_recurse(n - 1.) + fib_recurse(n - 2.)
}

fn main() {
	const N: f64 = 30.0;
	print!("fib_recurse({N}) => ");
	println!("{}", fib_recurse(N));
}
