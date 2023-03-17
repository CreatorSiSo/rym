# Rym Examples Part 1

## Greeting

```rym
// This is a comment
print("What's your name?")
let name: String = read_line(stdin)
print("Hi, ", name, "!")
```

With type inference:

```rym
// This is a comment
print("What's your name?")
let name = read_line(stdin)
print("Hi, ", name, "!")
```

## Comments

```rym
// A comment

/// A documentation comment
let myVariable: Int

/*
You can have any Rym code text commented
out inside this with no indentation restrictions.
			yes("May I ask a pointless question?")
	/*
		 Note: these can be nested!!
	*/
*/
```

## Numbers

```rym
1_000_000
```

## Let Statement

```rym
// declares x and y to have the type `Int`
let x: Int
let y: Int
```

```rym
let x = "abc" // introduces a new variable `x` and binds a value to it
x = "xyz"		 // Illegal: assignment to `x`

let mut x = "abc" // introduces a new mutable variable `x` and binds a value to it
x = "xyz"		 // Legal: assignment to `x`
```

## Constants

Constants are symbols which are bound to a value. The constant's value cannot change.
The compiler must be able to evaluate the expression in a constant declaration at compile time:

```rym
// the constant x contains the string "abc"
const x = "abc"
```

## Control Flow Expressions

### If Expression

```rym
let name = readLine(stdin)
if name == "" then
	print("Poor soul, you lost your name?")
else if name == "name" then
	print("Very funny, your name is name.")
else
	print("Hi, ", name, "!")
```

### Match Expression

```rym
let name = readLine(stdin)
match name with
	"" => print("Poor soul, you lost your name?"),
	"name" => print("Very funny, your name is name."),
	"Dave" | "Frank" => print("Cool name!"),
	else => print("Hi, ", name, "!"),
```

```rym
print("A number please: ")
let num = readLine(stdin).parse()
match num with
	0..2 | 4..7 => print("The number is in the set: {0, 1, 2, 4, 5, 6, 7}"),
	3 | 8 => print("The number is 3 or 8"),
	else => Unit,
```

### Loop Expression

```rym
print("What's your name? ")
let mut name = readLine(stdin)
loop {
	if name == "" then break
	print("Please tell me your name: ")
	name = readLine(stdin)
}
```

### While Expression

```rym
print("What's your name? ")
let mut name = readLine(stdin)
while name == "" do {
	print("Please tell me your name: ")
	name = readLine(stdin)
}
```

### For Expression

```rym
print("Counting to ten: ")
for i in 1..=10 do
	print(i)
```

Does the same as

```rym
print("Counting to ten: ")
let mut i = 1
while i <= 10 do {
  print(i)
  i += 1
}
```

### Ranges

#### Desugaring

```rym
for i in 0..10 do
	print(i)
```

```rym
for i in Range { start: 0, end: 10 } do
	print(i)
```

```rym
let mut range = Range { start: 0, end: 10 }
while let Some(i) = range.next() do
	print(i)
```

```rym
let mut range = Range { start: 0, end: 10 }
loop {
	if let Some(i) = range.next() then
		print(i)
	else
		break
}
```

#### Collection Indexing

```rym
let string = "some string"
for i in 0..string.len() do
	print(string.get(i))
```

```rym
let string = "some string"
for i in string.chars() do
	print(string.get(i))
```

```rym
let string = "some string"
let chars: List = strings.chars().collect()
for char in chars do
	print(char)
```

```rym
let chars = ['s', 'o', 'm', 'e', ' ', 's', 't', 'r', 'i', 'n', 'g']
for char in chars do
	print(char)
```
