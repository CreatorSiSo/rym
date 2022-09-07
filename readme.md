# Rym Lang

- [Rym Lang](#rym-lang)
	- [Name](#name)
	- [Goals](#goals)
	- [ToDo](#todo)

```rust
pr main() -> Io, Result<(), Error> {
	const msg = "Hello World"
	print msg

	mut num = 2 / 4 * (10 - 1)
	print "Number:\t" + num

	const msg = msg + "!"
	print "Combined:\t" + msg + " " + num

	might_return_error()?
	const maybe_value = do_stuff(testing).even_more(false)

	Ok(())
}
```

## Name

**Rym Lang** or **rym-lang**

- **R**ust**y** ⇒ Heavily borrows ;) from Rust
- **M**ulti-paradigm ⇒ Mix of object oriented, procedural and functional programming
- Programming **Lang**uage ⇒ because thats what it is

## Goals

- Scripting
- Great interoperabilty with Rust
  - Easy to use (simple)
  - Fast to write Binding
- ~~Ui Structure and Functionality coding~~ (Maybe later)

## ToDo

- [ ] Make assingments to mut variables work
- [ ] Have a look at
  - [ ] Go Routines
