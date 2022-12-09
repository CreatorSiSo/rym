# Functions

## Empty

```rust
fn no_params_no_return() {}
fn no_params_no_return() -> () {}
```

```rust
fn no_params() -> bool {
	true
}
```

## Paramaters

```rust
fn add(a: int, b: int) -> int {
	a + b
}

const sum = add(4, 3)
assert_eq(sum, 7)
```

## Default Values

```rust
fn round(number: float, precision = 0) {
	..
}
```

## Rest Parameters

```rust
fn print(..values: [Todo], sep = " ", end = "\n", flush = false) -> Io {
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
fn procedure() -> Io, Number {} // Io side effects allowed, returns a Number
fn procedure() -> Io {} // Io side effects allowed, returns ()
fn procedure() -> Io, () {} // Io side effects allowed, returns ()
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
  type SizeString = fn (value: string) -> Result<(), TypeError> {
  	if value.ends_with("px") {
  		Ok(())
  	} else {
  		Err(TypeError::Mismatch("Expected `px` at the end of a WidthString."))
  	}
  }

  type fn SizeString(value: string) -> Result<(), TypeError> {
  	if value.ends_with("px") {
  		Ok(())
  	} else {
  		Err(TypeError::Mismatch("Expected `px` at the end of a WidthString."))
  	}
  }
```
