# Strings

## Prefix Functions

```rym
use std.fmt.fstring.{Formatter};

type Error = /* TODO */;

type StringFormatter = struct {
    strings: [_]String,
    index: usize = 0,
};

impl Formatter for StringFormatter {
    type Value = impl Display;
    type Result = String;

    fn fill(mut self: Self, value: Value) Self => {
        std.fmt.write(mut self.[self.index], value);
        self.index += 1;
        self
    }

    fn finish(self: Self) String => {
        self.strings.join("")
    }
}


/// Creates the correct `StringFormatter` from the interpolation string.
/// Could also check that the string parts are inserted at the correct locations. (for inline SQL/HTML)
/// Always runs at compile time.
const fn f(strings: [_]String) Result[StringFormatter, Error] => {
    Ok(StringFormatter { strings })
}

let name = "Robot";
let greeting = f"Hello {name}!";

// Desugars to:
let name = "Robot";
let greeting = f(["Hello", "!"]).fill(name).finish();
```

```rym
const fn include_str(path: String) -> String {
	read_to_string(path).unwrap()
}
```
