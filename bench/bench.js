function fib_iter(n) {
	let current = 0;
	let next = 1;

	let i = 0;
	while (true) {
		if (i >= n) {
			break;
		}
		const new_ = next + current;
		current = next;
		next = new_;
		i = i + 1;
	}

	current;
}

const N = 99999; // 78 is max for result without deviation
console.log("fib_iter(", N, ") => ");
console.log(fib_iter(N));
