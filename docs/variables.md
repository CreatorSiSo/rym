# Variables

## Assignment

A variable is created by defining its mutability with ` const | mut`, followed by a name and assigning some value to it.

```rym
const variable1 = 42
mut variable2 = false
```

Mutable variables can be created without assigning a value to them immediatly by writing `mut variable_name` and leaving the `= _value_` out. Before you can use these variables a value must always be assigned to them via `variable_name = _value_`.

```rym
mut variable1
variable1 = 0
```

Assigning another value to a `const` variable after its creation is forbidden.

## Replacing

Instead you can create another variable with the same name. If you do this the old variable will be dropped and new one will be created.

```rym
const constant_var = 99
const constant_var = "ninety-nine"

mut mutable_var = "Hello World :)"
mut mutable_var = false
```

## Function parameters

```rym
/// var1 can be mutated, var2 not
func test_func(mut var1: T1, var2: T2) {}

// mut_outer passes a mutable reference into test_func()
// const_outer passes a immutable reference into test_func()
test_func(mut mut_outer, const_outer)

// mut_outer gets copied and passed into test_func()
// const_outer passes a immutable reference into test_func()
test_func(mut_outer, const_outer)
```
