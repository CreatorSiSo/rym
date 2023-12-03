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
  - [Inspired by](#inspired-by)
  - [Other languages written in Rust](#other-languages-written-in-rust)
  - [Project Structure](#project-structure)
    - [Tests](#tests)
- [Todos](#todos)

## About Rym

### Name

- **R**ust**y** ⇒ Heavily borrows 🙃 from Rust
- **M**ulti-paradigm ⇒ Mix of object oriented, procedural and functional programming
- Programming **Lang**uage ⇒ because thats what it is

### Goals

The language should be as powerful as possible while only providing a small set of consistent features.
(Kinda like Go)

- Features
  - Static types
    - inferred
  - Complex types
    - struct
    - enum / tagged union
  - Iterators
  - Zig like comp time
- Nice developer experience
  - Informative errors and warnings
  - Builtin tools
    - First party REPL
    - Package manager

## Examples

```dart
const main = fn() @Io, Result<(), Error> => {
	const msg = "Hello World";
	println(msg);

	let mut num = 2/4 * (10 - 1);
	println("Number:", num);

	const msg = msg + "!";
	println("Combined:", msg, num);
}
```

Early returns when unwrapping `Tryable`s

```dart
const main = fn() Result<Number, String> => {
	const number = maybe_error()?;
	println(number);

	// Same as
	const inner = match maybe_error() with
		| Ok(val) => val,
		| err => return err;
	println(inner);
}
```

<!-- Tryable chaining

```dart
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

## Inspired by

- [Rust](https://github.com/rust-lang/rust): Empowering everyone to build reliable and efficient software.
- [OCaml](): **TODO**
- [F#](): **TODO**
- [Swift](https://github.com/apple/swift): A Swift is a high-performance system programming language.
- [HVM](https://github.com/Kindelia/HVM): A massively parallel, optimal functional runtime in Rust
- [Unison](https://www.unison-lang.org/): A friendly programming language from the future (statically-typed, functional)
- [Fused Effects](https://github.com/fused-effects/fused-effects): A fast, flexible, fused effect system for Haskell

## Other languages written in Rust

- [Boson](https://github.com/Narasimha1997/boson-lang): Hybrid programming language written in Rust.
- [Mun](https://github.com/mun-lang/mun): Programming language empowering creation through iteration.
- [Tao](https://github.com/zesterer/tao): Statically-typed functional language with polymorphism, typeclasses, algebraic effects, sum types, pattern-matching, first-class functions, currying, good diagnostics, and much more!
- [Rhai](https://github.com/rhaiscript/rhai): Embedded scripting language for Rust.
- [Rune](https://github.com/rune-rs/rune): Embeddable dynamic programming language for Rust.

## Project Structure

The project is split into many crates that are part of one Cargo workspace:

`crates/rymx` ⇒ command line tool for executing `.rym` files

### Tests

For now just run `cargo test`, there is no special setup.

**TODO**

# Todos

- [ ] add benchmarking capabilities
