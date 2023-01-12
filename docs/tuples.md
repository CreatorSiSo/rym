# Tuples

```rym
const tuple = (89, "testing", false)
assert_eq(tuple.0, 89)
assert_eq(tuple.1, "testing")
assert_eq(tuple.2, false)

const named_tuple = (number: 89, string: "testing", boolean: false)
assert_eq(named_tuple.number, 89)
assert_eq(named_tuple.string, "testing")
assert_eq(named_tuple.boolean, false)
```
