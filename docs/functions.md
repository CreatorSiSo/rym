# Functions

A function is a self-contained block of code that performs a specific task. You give a function a name that identifies what it does, and this name is used to "call" the function to perform its task when needed.

Functions in Rym are defined using the `fn` keyword, followed by the function's name, a list of parameters within parentheses, and a return type. The function's body is defined within curly braces.

## Functions Without Parameters

```rust
fn hello_world() -> String {
	"Hello World!"
}

assert_eq(hello_world(), "Hello World!")
```

```rust
fn no_params() -> bool {
	true
}

assert_eq(no_params(no_params(), true))
```

```rust
fn no_params_no_return() {}
fn no_params_no_return() -> () {}
```

## Functions With Paramaters

```rust
fn greet(person: String) -> String {
	"Hello, " + person + "!"
}

const greeting = greet(person: "Max")
assert_eq(greeting, "Hello, Max!")
```

```rust
fn add(_ a: int, _ b: int) -> int {
	a + b
}

const sum = add(4, 3)
assert_eq(sum, 7)
```

## Default Values

```rust
fn round(_ number: float, precision = 0) {
	..
}
```

## Rest Parameters

```rust
fn print(..values: [Todo], seperator = " ", end = "\n", flush = false) -> Io {
	..
}
```

## Side Effects

- Io: Input Output, like writing to stdout, stderr or any other file
- Div: Diverge, Function might never return
- Ffi: Foreign function interface, Anything could happen
- ...

```rust
fn function() -> Number {} // pure function, returns a Number
fn function() -> Io, Number {} // Io side effects allowed, returns a Number
fn function() -> Io {} // Io side effects allowed, returns ()
fn function() -> Io, () {} // Io side effects allowed, returns ()
fn function() -> ! {} // Function that is never going to return
```

```rust
fn main() -> Io, Result<(), Error> {
	might_return_error()?
	const testing = 24
	const maybe_value = do_stuff(testing).even_more(false)
	print(maybe_value)

	Ok(())
}
```

## Clojures

```rust
const add = |a: int, b: int| a + b
```

## Type functions

```rust
  type SizeString = fn (value: String) -> Result<(), TypeError> {
  	if value.ends_with("px") {
  		Ok(())
  	} else {
  		Err(TypeError::Mismatch("Expected `px` at the end of a WidthString."))
  	}
  }

  type fn SizeString(value: String) -> Result<(), TypeError> {
  	if value.ends_with("px") {
  		Ok(())
  	} else {
  		Err(TypeError::Mismatch("Expected `px` at the end of a WidthString."))
  	}
  }
```
