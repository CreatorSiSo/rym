## Side Effects

```rym
func left() @Io -> Result<(), Error> {}

func right() -> Result<(), Error>, @Io {}

func must_io() -> Result<(), Error>, !Io {}
func might_div() -> Result<(), Error>, ?Div {}
func must_div() -> !Div {} // Cannot have a return type
```

## Functions

```rym
func assert_eq<T: PartialEq>(lhs: T, rhs: T) -> /* @Div | ?Div */ {
	const equal = lhs == rhs
	if !equal {
		panic(f"Left side does not equal right side:\n\t{lhs:?}\n\t{rhs:?}")
	}
}
```

## Strings

```rym
const ascii = "Hello World!"
const utf8 = "€²³"

mut concat = ascii ++ utf8
concat ++= "."

mut concat = ascii + utf8
concat += "."
```
