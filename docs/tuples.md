# Tuples

```rym
let tuple = (89, "testing", False)
assert_eq(tuple.0, 89)
assert_eq(tuple.1, "testing")
assert_eq(tuple.2, False)

let named_tuple = (number: 89, string: "testing", boolean: False)
assert_eq(named_tuple.number, 89)
assert_eq(named_tuple.string, "testing")
assert_eq(named_tuple.boolean, False)
```
