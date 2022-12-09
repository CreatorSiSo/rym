# Strings

## Prefix Functions

```rust
/// Parse formatting expressions from string,
/// runs at compile time
fn f(string: String) -> Result<Ast, ParseError> {
	..
}

const some_string = f"Hello { 1 + 2 }!"
```
