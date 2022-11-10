#[rustfmt::skip]
pub const SOURCES: [(&'static str, &'static str); 16] = [(r#"/home/simon/dev/rym/crates/tests/src/integration/test_hello_world.rym"#, r#"println("Hello World!")
"#), (r#"/home/simon/dev/rym/crates/tests/src/integration/test_if.rym"#, r#"if true {
	println("test")
} else {
	panic()
}

if false {
	panic()
} else {
	println("test")
}
"#), (r#"/home/simon/dev/rym/crates/tests/src/integration/lex_valid.rym"#, r#"//! fail parse exec

ident	ifier

- + / *
. , ; ( ) { }
! != = == > >= < <=
&& ||

!== === <== >== ==>

0 1 2 3 4 5 6 7 8 9
12392347.230873460 1 23428934
34957533457 96

"""asdf\"sd"""
"WOoooOOoooWEEeee!"

"
- + / *
. , ; ( ) { }
! != = == > >= < <=
if else for while loop return break
 false true && ||
 fn const mut
 struct self
 print
"

if else for while loop return break
false true
fn const mut
struct self

testing(arg1,, arg2)
"#), (r#"/home/simon/dev/rym/crates/tests/src/integration/lex_invalid.rym"#, r#"//! fail lex

if ³ true {

} else {

}

~@€½³²
"#), (r#"/home/simon/dev/rym/crates/tests/src/integration/expr_results.rym"#, r#"//!

const expr_example_1 = {
	mut index = 0
	mut sum = 0
	loop {
		if index > 50 {
			break sum
		}
		sum = sum + index
		index = index + 1
	}
}
println(expr_example_1)

const expr_example_2 = if expr_example_1 < 1000 {
	expr_example_1 + " ist kleiner als 1000"
} else {
	expr_example_1 + " ist größer als 1000"
}
println(expr_example_2)
"#), (r#"/home/simon/dev/rym/crates/tests/src/integration/control_flow.rym"#, r#"//!

const say_hello = false

if say_hello {
	println("Hello World!")
} else {
	println("Bye World!")
}

if !say_hello {
	println("`say_hello` is not `true`")
}

if true {

} else if false {
	println("testing")
} else {
	println("nope")
}

mut x = 0
loop {
	println("Round: ", x)
	x = x + 1
	if x > 99 { break }

	mut y = 0
	loop {
		print(y)
		y = y + 1
		if y >= 99 {
			println()
			break
		}
	}
}
"#), (r#"/home/simon/dev/rym/crates/tests/src/integration/cylinder.rym"#, r#"// Ein zylinderförmiger Behälter vom Durchmesser d1(m) und der Höhe h(m) wird durch einen Schlauch mit dem Durchmesser d2(cm) mit Wein gefüllt.
// Die Durchflussgeschwindigkeit ist v (m/s).
// Durch ein Programm soll die Zeit ermittelt werden, die zum vollständigen Füllen des Behälters benötigt wird.

// VZ = PI * r * r * h

// VS = PI * r * r * l          | l = vS * t
// VS = PI * r * r * vS * t

// VZ = VS
// PI * rZ * rZ * h = PI * rS * rS * vS * t  | /PI
// rZ * rZ * h = rS * rS * vS * t            | /(rS * rS * vS)
// t = (rZ * rZ * h) / (rS * rS * vS)

fn t_filled(d1 /* m */, h /* m */, d2 /* cm */, vS /* m/s */) {
	const rZ = d1 / 2
	const rS = d2 / 200

	(rZ * rZ * h) / (rS * rS * vS)
}

println("Time: ", t_filled(0.8, 1, 2, 1), "s")
"#), (r#"/home/simon/dev/rym/crates/tests/src/integration/parse_if.rym"#, r#"//! fail parse exec

// TODO Add proper error recovery for parsing
// eg. this should either ignore the whole if expression or
// every thing until the then block
if mut test = 0 {
	println("cool feature but not implemented :(")
} else {
	println("Hää?")
}
"#), (r#"/home/simon/dev/rym/crates/tests/src/integration/variables.rym"#, r#"const test = "Hello World"
mut two = 2 / 4 * (10 - 1)
println("test:\t", test)

const test = test + "!"
println("both:\t", test, " " + two)

println("two:\t", two)
two = 0
println("two:\t", two)
"#), (r#"/home/simon/dev/rym/crates/tests/src/integration/string.rym"#, r#""
\ndgopg\ns\ndgopg\ns2354
pg\npg\n234
\tfisdj		\n\tfisdj		\n
	\n\n\n\n235864				0
\nsdfsg\nsdfsd\nsdfsg\nsd
fsd	gopg\t\tfisdj\t\t\t\t24
\t\t\t\t\t\t235864
\ndgopg\ns\ndgopg\ns2354
pg\npg\n234
\tfisdj		\n\tfisdj		\n
	\n\n\n\n235864				0
\nsdfsg\nsdfsd\nsdfsg\nsd
fsd	gopg\t\tfisdj\t\t\t\t24
\t\t\t\t\t\t235864
";
"
	Proident amet do enim cillum do
		deserunt eiusmod dolore do mollit
			amet id laborum excepteur.
				Amet qui culpa elit reprehenderit
			mollit occaecat labore. Deserunt
		ut duis nisi tempor magna
	adipisicing occaecat nostrud est qui.
";
"
Dolore tempor qui non ipsum amet minim
nisi deseruntdolor consectetur.
Nulla eu aute cupidatat dolor et fugiat
reprehenderit consectetur sunt elit.
Et voluptate sunt ipsum ea in cillum
do nisi Lorem in consequat tempor ipsum.
Ut veniam veniam adipisicing aliquip exercitation.
Non nulla ex amet cupidatat ex velit
exercitation et excepteur enim deserunt
voluptate ut cupidatat.
Non minim aliquip exercitation laboris
minim magna velit.
Magna quis eu sint non consectetur qui
aute amet nostrud proident in eu.
Exercitation esse exercitation laboris
non ea occaecat qui esse proident.
Occaecat laboris cillum ea ea ut
proident pariatur ad irure incididunt
reprehenderit.
Mollit mollit do sit minim sunt.
Est ut excepteur laboris excepteur nostrud incididunt officia ad veniam nulla duis tempor cillum cupidatat.
Nulla eu aute cupidatat dolor et fugiat reprehenderit consectetur sunt elit.
Ut veniam veniam adipisicing aliquip exercitation.
Magna quis eu sint non consectetur qui aute amet nostrud proident in eu.
Est ut excepteur laboris excepteur nostrud incididunt officia ad
veniam nulla duis tempor cillum cupidatat.niam adipisicing aliquip exercitation.Non nulla ex amet cupidatat ex velitexercitation
et excepteur enim deseruntvoluptate ut cupidatat.Non minim aliquip exercitation laborisminim magna velit.
Magna quis eu sint non consectetur quiaute amet nostrud proident in eu.Exercitation e
";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n"; "\t\t\t\n@ŧ€ŋÜsüfÖg§fdfä€€ſ«\n";
"#), (r#"/home/simon/dev/rym/crates/tests/src/integration/scope.rym"#, r#"mut value = 20
{
	const value = 0
	println(value)
}
// value = value-1
println(value)

println("")

const a = "\t\tglobal a"
const b = "\t\tglobal b"
const c = "\t\tglobal c"
{
  const a = "\touter a"
  const b = "\touter b"
  println(a); println(b); println(c)
}
println(a); println(b); println(c)
"#), (r#"/home/simon/dev/rym/crates/tests/src/integration/print.rym"#, r#"println(true)
println(2 + 1)
println("one")
"#), (r#"/home/simon/dev/rym/crates/tests/src/integration/parser.rym"#, r#"//! fail exec

// Literals
false
true
2099
//  34_560_000
560897890.42
0293457346783873434569999999999999999999999.99999
"Hello World!"
".\t.\n	. ."
variable_name

// Comparison
30 < 99 && true
false || true
	// err
0 && true
"0" || "\t"

// Equality
20 == 0 // false
true != false // true
"air" == "air" // true
"air" != true // true

// Comparison
30 < 99
999 <= 9
10 > 9
10 >= 10
	// err
0 < true
"0" >= "\t"

// Term
30 + 99
999 - 9
	// err
0 - true
"0" - "\t"

// Factor
30 * 99
999 / 9
	// err
999 / 0

// Unary
-20
!true
!!false
	// err
-true
--false
!20

// Groups
(20 - 900) * 10
(false == "") + "_suffix"

// Blocks
{
	"outer"
	{ "inner1" } + "\n"
	({ "inner2" } + "\n")
}
{}

// Empty
;

/*1
	and this is a
	multiline
	comment

	/*
		you can
		nest them too
	*/
	/*2 /*3 /*4 */ */ */
*/

// Should not result in an endless loop
// {
"#), (r#"/home/simon/dev/rym/crates/tests/src/integration/ice.rym"#, r#"fn print_ice_value(numIce, ateWhiteIce) {
	fn max(a, b) { if a > b { a } else { b } }

	const numWhiteIce = floor(max(numIce * 0.45 - ateWhiteIce, 0))
	const numDarkIce = floor(max(numIce * 0.55 - 2 * ateWhiteIce, 0))

	println("Value white chok:\t", numWhiteIce * 1.2, "€")
	println("Value dark chok:\t",numDarkIce * 0.9, "€")
}

print_ice_value(10, 2)
print_ice_value(80, 15)
"#), (r#"/home/simon/dev/rym/crates/tests/src/integration/expression.rym"#, r#"(true != !false) // false => correct
(0769570 + 34907 * 569034) < 9 // false => correct
(0769570 + 34907 * 569034) < 9 == (true != !false) // false == false => true => correct
(0769570 + (34907 * 569034 - (34897534889 / 55)))
(0769570 + (34907 * 569034 - ((((((34897534889 / 55)))))))) < 9 == (true != !false)

println(1 + 2)
println(1 / 0)
println(200 * 23406)
"#), (r#"/home/simon/dev/rym/crates/tests/src/integration/fn_envs.rym"#, r#"//! fail exec

fn returns_fn(value, info) /* -> Fn */ {
	println("called outer with value: `", value, "`")
	println("\t", info)

	fn inner(value) /* -> () */ {
		println("called inner with value: `", value, "`")
		println("\t", info)
	}

	inner
}

const info = ()
returns_fn(1, "This should work")
returns_fn(0, "This should not work")(false)
"#), ];