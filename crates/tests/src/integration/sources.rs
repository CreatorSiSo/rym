#[rustfmt::skip]
pub const SOURCES: [(&'static str, &'static str); 5] = [(r#"/home/simon/dev/rym/crates/tests/src/integration/test_hello_world.rym"#, r#"println("Hello World!")
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

$ €
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
"#), ];