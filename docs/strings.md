# Strings

## Prefix Functions

```rym
/// Parse formatting expressions from string,
/// runs at compile time
func f(string: String) -> Result<Ast, ParseError> {
	..
}

const some_string = f"Hello { 1 + 2 }!"
```
