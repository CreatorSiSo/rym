# Rym

**Rym** is a statically typed programming language inspired by Rust, Swift, Python and others that focuses on **ease of use** and **should just work™**.
Big thanks go to Robert Nystrom and his book [crafting interpreters](http://craftinginterpreters.com) open source which inspired me to continue working on this language.

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

- Features
  - Static types
  - Complex types (structs, enums, ..)
  - Union types
  - Iterators
  - Inferred types
  - Constant evaluation context
  - Tryable chaining?
- Nice Development Experience
  - Informative errors and warnings
  - Builtin tools
    - First party REPL
    - Package manager

## Examples

```rust
fn main() -> Result<(), Error>, ?Io => {
	const msg = "Hello World";
	print(msg);

	let mut num = 2/4 * (10 - 1);
	print("Number:", num);

	const msg = msg + "!";
	print("Combined:", msg, num);
}
```

Early returns when unwrapping `Tryable`s

```rust
fn main() -> Result<Number, String> => {
	const number = maybe_error()?;
	print(number);

	// Same as
	const inner = match maybe_error() {
		Ok(val) => val,
		err => return err,
	};
	print(inner);
}
```

<!-- Tryable chaining

```rust
const chained = maybe_error()&.to_string()

// Short form of:
const chained = match maybe_error() {
	Ok(val) => Ok(val.to_string()),
	err => err,
}
// or:
const chained = maybe_error().and_then(|val| Ok(val.to_string()))
``` -->

## How to install

**TODO**

## Inspirational projects

- [Rust](https://github.com/rust-lang/rust): Empowering everyone to build reliable and efficient software.
- [Swift](https://github.com/apple/swift): A Swift is a high-performance system programming language.
- [HVM](https://github.com/Kindelia/HVM): A massively parallel, optimal functional runtime in Rust
- [Unison](https://www.unison-lang.org/): A friendly programming language from the future (statically-typed, functional)
- [Fused Effects](https://github.com/fused-effects/fused-effects): A fast, flexible, fused effect system for Haskell

## Similar projects

- [Boson](https://github.com/Narasimha1997/boson-lang): Hybrid programming language written in Rust.
- [Mun](https://github.com/mun-lang/mun): Programming language empowering creation through iteration.
- [Tao](https://github.com/zesterer/tao): Statically-typed functional language with polymorphism, typeclasses, algebraic effects, sum types, pattern-matching, first-class functions, currying, good diagnostics, and much more!
- [Rhai](https://github.com/rhaiscript/rhai): Embedded scripting language for Rust.
- [Rune](https://github.com/rune-rs/rune): Embeddable dynamic programming language for Rust.

## Project Structure

The project is split into many crates that are part of one Cargo workspace:

`crates/rymx` ⇒ command line tool for executing `.rym` files

### Tests

**TODO**

# Todos

- [ ] use insta snapshot testing crate
- [ ] add benchmarking capabilities
