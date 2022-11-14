# Functions

```rust
fn add(a: int, b: int) -> int {
	a + b
}

const added = add(4, 3)
assert_eq(added, 7)
```

## Side Effects

- Io = Input Output, like writing to stdout, stderr
- Div = Diverge, Function might never exit
- Ffi = Foreign function interface, Anything could happen
- ...

```rust
fn function() {} // pure function
pr procedure() {} // side effects allowed
```

```rust
fn function() -> Number {} // returns a Number
pr procedure() -> Io, Number {} // Io side effects allowed, returns a Number
pr procedure() -> Io {} // Io side effects allowed, returns ()
```

```rust
pr main() -> Io, Result<(), Error> {
	might_return_error()?
	const testing = 24
	const maybe_value = do_stuff(testing).even_more(false)
	stdout::display(maybe_value)

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
