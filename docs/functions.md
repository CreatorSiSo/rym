# Functions

Functions are self-contained chunks of code that perform a specific task. You give a function a name that identifies what it does, and this name is used to “call” the function to perform its task when needed.

Rym's function syntax is flexible enough to express anything from a simple C-style function with no parameter names to a complex method with names and argument labels for each parameter. Parameters can provide default values to simplify function calls and can be passed as mutable parameters, which allows modifying data outside of the functions body.

Every function in Rym has a type, consisting of the function’s parameter types and return type. You can use this type like any other type in Rym, which makes it easy to pass functions as parameters to other functions, and to return functions from functions. Functions can also be written within other functions to encapsulate useful functionality within a nested function scope.

## Defining Functions

In Rym, functions are self-contained chunks of code that perform a specific task.
They are defined using the `fn` keyword, followed by the function name,
a list of parameters with their type annotations, and the return type.
The function body is defined within curly braces `{ .. }`.

For example, the following function greet takes a `String` as a parameter called `person` and returns a `String`:

```rym
fn greet(person: String) String => {
    "Hello, " + person + "!"
}
```

To call this function, you would use its name followed by arguments in parentheses, like this:

```rym
const greeting = greet(person: "Max");
assert_eq(greeting, "Hello, Max!");
```

You can also define default values for parameters to make the function easier to call. For example:

```rym
fn greet(person: String, greeting: String = "Hello") String => {
    greeting + ", " + person + "!"
}

const default_greeting = greet(person: "Max"); // "Hello, Max!"
const custom_greeting = greet(greeting: "Bonjour", person: "Max"); // "Bonjour, Max!"
```

Finally, you can pass parameters as mutable values to allow the outer value to be modified within the function body.
To do this, you prefix the parameter type with the `mut` keyword. For example:

```rym
use std.fs.{Path, read_to_string};
use std.io;

fn get_content(file_cache: mut HashMap[Path, String], path: std.fs.Path) io.Result[String] => {
    Ok(file_cache.entry(path).or_insert(read_to_string(path).try))
}

fn other() {
    let mut file_cache = HashMap.new();

    let path = Path.from("./test.txt");
    let file_content = get_content(mut file_cache, path);
    assert(file_chache.contains_key(path));
}
```

Every function in Rym has a type, consisting of the function’s parameter types and return type.
This allows you to pass functions as parameters to other functions, and to return functions from functions.
Functions can also be written within other functions to encapsulate useful functionality within a nested function scope.

## Defining Functions

## Functions Without Parameters

```rym
fn hello_world() String => {
    "Hello World!"
}

assert_eq(hello_world(), "Hello World!");
```

```rym
fn no_params() Bool => {
    Bool.True
}

assert_eq(no_params(no_params(), Bool.True))
```

```rym
fn no_params_no_return() => {}
fn no_params_no_return() () => {}
```

## Functions With Paramaters

```rym
fn greet(person: String) String => {
    "Hello, " + person + "!"
}

const greeting = greet(person: "Max")
assert_eq(greeting, "Hello, Max!")
```

```rym
fn add(lhs: Int, rhs: Int) Int => lhs + rhs;

const sum = add(4, 3);
assert_eq(sum, 7);
```

## Default Values

```rym
fn round(number: Float, precision: u32 = 0) Int => {
    // ..
}
```

## Rest Parameters

```rym
fn print(
    ...values: []impl Display,
    seperator = " ",
    end = "",
    flush = False,
) @Io => {
    // ..
}

print(1, "+", 2); // Prints: "1 + 2"
print(1, 2, 3, seperator: ", "); // Prints: "1, 2, 3"
const values = [1, 2, 3, 4];
print(...values, seperator: ", "); // Prints: "1, 2, 3, 4"
```

## Partial application

```rym
fn print(
    ...values: []impl Display,
    seperator = " ",
    end = "",
    flush = False,
) @Io => {
    // ..
}

const println = print.with(end: "\n");

println(1, "+", 2); // Prints
```

## Side Effects

- Io: Input Output, like writing to stdout, stderr or any other file
- Div: Diverge, Function might never return
- Ffi: Foreign function interface, Anything could happen
- ...

```rym
fn function() Number => {} // returns a Number, pure function
fn function() @Io, Number => {} // returns a Number, Io side effects allowed
fn function() @Io => {} // returns (), Io side effects allowed
fn function() @Io, () => {} // returns (), Io side effects allowed
fn function() @Div => {} // Function might never return
fn function() Never => {} // Function is never going to return
```

```rym
fn main() @Io, Result[(), Error] => {
    might_return_error().try;
    const testing = 24;
    const maybe_value = do_stuff(testing).even_more(false);
    print(maybe_value);

    Ok(())
}
```

## Clojures

```rym
fn main() => {
    let mut outer = 0;
    fn increment(by: Int) => outer += by;

    increment(1);
    increment(10);
    increment(-10);

    assert_eq(outer, 1);
}
```
