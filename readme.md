# Rym

**Rym Lang** or just **Rym** is a modern, statically typed programming language inspired by Rust, Swift, Python and others that focuses on ease of use and safety.
Big thanks go to Robert Nystrom who made his book [crafting interpreters](http://craftinginterpreters.com) open source which enabled me to read and learn from it :).

## Content

- [Rym](#rym)
	- [Content](#content)
	- [About Rym](#about-rym)
		- [Name](#name)
		- [Goals](#goals)
	- [Examples](#examples)
	- [How to install](#how-to-install)
	- [Inspirational projects](#inspirational-projects)
	- [Similar projects](#similar-projects)
	- [Project Structure](#project-structure)
		- [Tests](#tests)
- [Todos](#todos)

## About Rym

### Name

- **R**ust**y** ⇒ Heavily borrows 🙃 from Rust
- **M**ulti-paradigm ⇒ Mix of object oriented, procedural and functional programming
- Programming **Lang**uage ⇒ because thats what it is

### Goals

- **MVP**
  - Safety
    - No `null` `nil` `undefined`
    - Optionals wrapped in `Option<T>` enum
    - Errors wrapped in `Result<T>`
    - Statically typed
- **1.0**
  - Nice DX (Development Experience)
    - Inferred types
    - Informative errors and warnings
    - Builtin tools
  - Great interoperabilty with Rust
    - Should just work out of the box
- **Past 1.0**
  - Ui and Apps programming

## Examples

```rust
fn main() -> Result<(), Error>, @Io {
	const msg = "Hello World"
	print(msg)

	mut num = 2/4 * (10 - 1)
	print("Number:", num)

	const msg = msg + "!"
	print("Combined:", msg, num)
}
```

In Rym you can unwrap `Tryable` values like `Option`s or `Result`s

```rust
const inner = maybe_value()!

// Same as
const inner = maybe_value().unwrap()
```

Early returns when unwrapping `Tryable`s

```rust
fn main() -> Result<Number, String> {
	const number = maybe_error()?
	print(number)

	// Same as
	const inner = match maybe_error() {
		Ok(val) => val,
		err => return err,
	}
	print(inner)
}
```

Tryable chaining

```rust
const chained = maybe_error()&.to_string()

// Short form of:
const chained = match maybe_error() {
	Ok(val) => Ok(val.to_string()),
	err => err,
}
// or:
const chained = maybe_error().and_then(|val| Ok(val.to_string()))
```

## How to install

**TODO**

## Inspirational projects

- [HVM](https://github.com/Kindelia/HVM): A massively parallel, optimal functional runtime in Rust
- [Unison](https://www.unison-lang.org/): A friendly programming language from the future (statically-typed, functional)
- [Fused Effects](https://github.com/fused-effects/fused-effects): A fast, flexible, fused effect system for Haskell
- [Rust](https://github.com/rust-lang/rust): Empowering everyone to build reliable and efficient software.
- [Swift](https://github.com/apple/swift): A Swift is a high-performance system programming language.

## Similar projects

- [Boson](https://github.com/Narasimha1997/boson-lang): Hybrid programming language written in Rust.
- [Mun](https://github.com/mun-lang/mun): Programming language empowering creation through iteration.
- [Tao](https://github.com/zesterer/tao): Statically-typed functional language with polymorphism, typeclasses, algebraic effects, sum types, pattern-matching, first-class functions, currying, good diagnostics, and much more!
- [Rhai](https://github.com/rhaiscript/rhai): Embedded scripting language for Rust.
- [Rune](https://github.com/rune-rs/rune): Embeddable dynamic programming language for Rust.

## Project Structure

The project is split into many crates that are part of one Cargo workspace:

`crates/rym_span` ⇒ Span
`crates/rym_tt` ⇒ TokenTree, TokenStream
`crates/rym_lexer` ⇒ Isolated initial lexer

`crates/ast` ⇒ Ast Types: Spanned<T>, AstVisitor<T>, Token, ...
`crates/lex` ⇒ produce Tokens from source
`crates/parse` ⇒ produce ast from tokens
`crates/code_gen` ⇒ generate optimized code (dead code analysis, ...) or give warnings
`crates/tree_walk` ⇒ evaluate ast
`crates/tests` ⇒ integration tests
`crates/rymx` ⇒ command line tool for executing `.rym` files

And some other scripts located in the root directory:

`bench.sh` ⇒ builds and runs various benchmarks
`test.py` ⇒ updates and runs all tests

### Tests

Run `python test.py` to update and execute all tests.

This internally runs `cargo r --bin gen -- ./crates/tests/src/integration` which includes the source code for all tests into `crates/tests/src/integration/mod.rs`.

# Todos

- [ ] use insta snapshot testing crate
- [ ] add benchmarking capabilities
- [ ] use arena allocator for scopes?
  - [ ] benchmark before & after
- [ ] use logos lexer generator?
- [ ] errors
  - [ ] use `Spanned<T>` where possible
  - [ ] implement error recovery to jump to next safe expr/stmt
  - [ ] use error codes that link to a more detailed explanation (https://github.com/rust-lang/rust/tree/master/compiler/rustc_error_codes)
  - [ ] `true && (break)` currently only returns `Error: Expected Literal got RightParen, Span: 14..14`, it should also say something along the lines of: `Tip: insert expression or semicolon after break`
- [ ] data types
  - [ ] `number`s, `string`, `char`, `bool`
  - [ ] (literal) values that come from source code directly:
    - [ ] `Literal<u8>`, `Literal<f32>`, `Literal<string>`, `Literal<char>`, `Literal<bool>`
    - [ ] `1`, `2.2`, `"Hello World!"`, `'\n'`, `false`
  - [ ] type unions: `0 | 1 | bool`
  - [ ] type functions [docs/functions.md](docs/functions.md#type_functions)?
