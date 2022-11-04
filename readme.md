# Rym Lang

```rust
fn main() -> Io; Result<(), Error> {
	const msg = "Hello World";
	println(msg);

	mut num = 2 / 4 * (10 - 1);
	println("Number:\t", num);

	const msg = msg + "!";
	println("Combined:\t", msg, " ", num);

	might_return_error()?;
	const maybe_value = do_stuff(testing).even_more(false);

	Ok(())
}
```

## Content

- [Rym Lang](#rym-lang)
	- [Content](#content)
	- [Name](#name)
	- [Goals](#goals)
	- [Todos](#todos)

## Name

**Rym Lang** or **rym-lang**

- **R**ust**y** ⇒ Heavily borrows 🙃 from Rust
- **M**ulti-paradigm ⇒ Mix of object oriented, procedural and functional programming
- Programming **Lang**uage ⇒ because thats what it is

## Goals

- Works well for scripting and serious projects
- Great interoperabilty with Rust
  - Easy to use (simple)
  - Fast to write Bindings
- ~~Ui Structure and Functionality coding~~ (Maybe later)

## Todos

- [x] `Spanned<T>`
  - [x] contains start index and length or `Range<usize>`
- [x] `AstVisitor<R>` trait
  - [x] takes in some ast and produces `R`
- [ ] Interpreter
  - [x] add custom constructor to define globals `with_globals(&[(String, Into<Value>)])`?
  - [ ] add method for defining variable on interpreter directly
- [x] change lib internal file structure

  ```
  crates
  	/ast        ⇒         Ast Types:  Spanned<T>, AstVisitor<T>, Token, ...
  	? /interpret  ⇒ Interpreter Types:  Value, Interpreter, ...

  	/tokenize   ⇒ API to produce Spanned<Token>
  	/parse      ⇒ API to produce some ast
  	/lint       ⇒ API to visit and generate warnings (dead code, ...) for ast
  	/tree_walk  ⇒ API to visit and evaluate ast

  	/rymx
  ```

- [ ] functions
  - [x] add parsing for declarations
  - [ ] fix nested calls eg. `name()()()`
- [ ] add benchmarking capabilities
  - [ ] cargo alias eg. `cargo bench`
- [ ] use arena allocator for scopes
  - [ ] benchmark before & after
- [ ] use logos lexer generator
- [ ] errors
  - [ ] use `Spanned<T>` where possible
  - [ ] construct error location from `Span` and source (file)
  - [ ] use annotations lib to display errors
  - [ ] implement error recovery to safe expr/stmt
  - [ ] use error codes that link to a more detailed explanation (https://github.com/rust-lang/rust/tree/master/compiler/rustc_error_codes)
- [ ] types

  - [ ] type functions

  ```rust
  type SizeString = fn (value: string) -> Result<(), TypeError> {
  	if value.ends_with("px") {
  		Ok(())
  	} else {
  		Err(TypeError::Mismatch("Expected `px` at the end of a WidthString."))
  	}
  }

  type fn SizeString(value: string) -> Result<(), TypeError> {
  	if value.ends_with("px") {
  		Ok(())
  	} else {
  		Err(TypeError::Mismatch("Expected `px` at the end of a WidthString."))
  	}
  }
  ```

  - number, string, char, bool, ...
