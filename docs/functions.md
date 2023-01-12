# Functions

Functions are self-contained chunks of code that perform a specific task. You give a function a name that identifies what it does, and this name is used to “call” the function to perform its task when needed.

Rym's function syntax is flexible enough to express anything from a simple C-style function with no parameter names to a complex method with names and argument labels for each parameter. Parameters can provide default values to simplify function calls and can be passed as mutable parameters, which allows modifying data outside of the functions body.

Every function in Rym has a type, consisting of the function’s parameter types and return type. You can use this type like any other type in Rym, which makes it easy to pass functions as parameters to other functions, and to return functions from functions. Functions can also be written within other functions to encapsulate useful functionality within a nested function scope.

## Defining Functions

In Rym, functions are self-contained chunks of code that perform a specific task. They are defined using the `func` keyword, followed by the function name, a list of parameters with their type annotations, and the return type. The function body is defined within curly braces `{ .. }`.

For example, the following function greet takes a `String` as a parameter called `person` and returns a `String`:

```rym
func greet(person: String) -> String {
	"Hello, " + person + "!"
}
```

To call this function, you would use its name followed by arguments in parentheses, like this:

```rym
const greeting = greet(person: "Max")
assert_eq(greeting, "Hello, Max!")
```

You can also define default values for parameters to make the function easier to call. For example:

```rym
func greet(person: String, greeting: String = "Hello") -> String {
	greeting + ", " + person + "!"
}

const default_greeting = greet(person: "Max") // "Hello, Max!"
const custom_greeting = greet(person: "Max", greeting: "Bonjour") // "Bonjour, Max!"
```

In addition, you can specify an argument label for each parameter to make the function more expressive and easier to read. To use an argument label, you include it before the parameter name, separated by a space. For example:

```rym
func greet(to person: String, with greeting: String = "Hello") -> String {
	greeting + ", " + person + "!"
}

const default_greeting = greet(to: "Max") // "Hello, Max!"
const custom_greeting = greet(to: "Max", with: "Bonjour") // "Bonjour, Max!"
```

You can also use the special argument label \_ to make a parameter positional, meaning it can be passed without an argument label. This can be useful when you want to allow flexibility in the order of arguments. For example:

```rym
func greet(\_ greeting: String, to person: String) -> String {
greeting + ", " + person + "!"
}

const positional_greeting = greet("Hello", to: "Max") // "Hello, Max!"
```

Finally, you can pass parameters as mutable values to allow them to be modified within the function body. To do this, you prefix the parameter name with the mut keyword. For example:

```rym
func greet(mut greeting: String, to person: String) -> String {
greeting += ", " + person + "!"
greeting
}

let mutable_greeting = "Hi"
const modified_greeting = greet(mutable_greeting, to: "Max") // "Hi, Max!"
```

Every function in Rym has a type, consisting of the function’s parameter types and return type. This allows you to pass functions as parameters to other functions, and to return functions from functions. Functions can also be written within other functions to encapsulate useful functionality within a nested function scope.

## Defining Functions

## Functions Without Parameters

```rym
func hello_world() -> String {
	"Hello World!"
}

assert_eq(hello_world(), "Hello World!")
```

```rym
func no_params() -> bool {
	true
}

assert_eq(no_params(no_params(), true))
```

```rym
func no_params_no_return() {}
func no_params_no_return() -> () {}
```

## Functions With Paramaters

```rym
func greet(person: String) -> String {
	"Hello, " + person + "!"
}

const greeting = greet(person: "Max")
assert_eq(greeting, "Hello, Max!")
```

```rym
func add(_ a: int, _ b: int) -> int {
	a + b
}

const sum = add(4, 3)
assert_eq(sum, 7)
```

## Default Values

```rym
func round(_ number: float, precision = 0) {
	..
}
```

## Rest Parameters

```rym
func print(..values: [impl Dispay], seperator = " ", end = "\n", flush = false) -> @Io {
	..
}

print(1, "+", 2) // Prints: "1 + 2\n"
print(1, 2, 3, seperator: ", ") // Prints: "1, 2, 3\n"
print(1, 2, 3, seperator: ", ") // Prints: "1, 2, 3\n"
```

## Side Effects

- Io: Input Output, like writing to stdout, stderr or any other file
- Div: Diverge, Function might never return
- Ffi: Foreign function interface, Anything could happen
- ...

```rym
func function() -> Number {} // returns a Number, pure function
func function() -> Number, @Io {} // returns a Number, Io side effects allowed
func function() -> @Io {} // returns (), Io side effects allowed
func function() -> (), @Io {} // returns (), Io side effects allowed
func function() -> ! {} // Function that is never going to return
```

```rym
func main() -> Result<(), Error>, @Io {
	might_return_error()?
	const testing = 24
	const maybe_value = do_stuff(testing).even_more(false)
	print(maybe_value)

	Ok(())
}
```

## Clojures

```rym
const add = |a: int, b: int| a + b
```

## Type functions

```rym
  type SizeString = func (value: String) -> Result<(), TypeError> {
  	if value.ends_with("px") {
  		Ok(())
  	} else {
  		Err(TypeError::Mismatch("Expected `px` at the end of a WidthString."))
  	}
  }

  type func SizeString(value: String) -> Result<(), TypeError> {
  	if value.ends_with("px") {
  		Ok(())
  	} else {
  		Err(TypeError::Mismatch("Expected `px` at the end of a WidthString."))
  	}
  }
```
