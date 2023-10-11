# Strings

## Prefix Functions

```rym
/// Parse formatting expressions from string,
/// runs at compile time
const func rusty(string: String) -> Result<Ast, ParseError> {
	..
}

const some_string = rusty"Hello { 1 + 2 }!"
```

```rym
const func include_str(path: String) -> String {
	read_to_string(path).unwrap()
}
```
