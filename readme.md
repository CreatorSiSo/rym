# Rym

**Rym Lang** or just **Rym** is a statically typed 'mid-level' programming language inspired by and written in Rust for educational purposes. Big thanks go to Robert Nystrom who made his book [crafting interpreters](http://craftinginterpreters.com) open source which enabled me to read it and learn a lot :).

![Screenshot from 2022-11-06 04-48-30](https://user-images.githubusercontent.com/64036709/200153194-31819cec-809c-4fa7-b7db-feda44a1fa9b.png)

## Content

- [Rym](#rym)
	- [Content](#content)
	- [About Rym](#about-rym)
		- [Goals](#goals)
		- [Name](#name)
	- [Similar projects](#similar-projects)
	- [How to install](#how-to-install)
	- [Project Structure](#project-structure)
		- [Tests](#tests)
- [Todos](#todos)

## About Rym

### Goals

- Enjoyable DX (Development Experience)
  - Informative errors and warnings
  - Built in tools
- Safe programming language
  - No `null` `nil` `undefined`
  - Static but inferred types
- Should work as a scripting language
- Great interoperabilty with Rust
  - Should just work out of the box
- Ui and apps programming

### Name

- **R**ust**y** ⇒ Heavily borrows 🙃 from Rust
- **M**ulti-paradigm ⇒ Mix of object oriented, procedural and functional programming
- Programming **Lang**uage ⇒ because thats what it is

## Similar projects

- [Boson](https://github.com/Narasimha1997/boson-lang): Hybrid programming language written in Rust.
- [Mun](https://github.com/mun-lang/mun): Programming language empowering creation through iteration.
- [Tao](https://github.com/zesterer/tao): Statically-typed functional language with polymorphism, typeclasses, algebraic effects, sum types, pattern-matching, first-class functions, currying, good diagnostics, and much more!
- [Rhai](https://github.com/rhaiscript/rhai): Embedded scripting language for Rust.
- [Rune](https://github.com/rune-rs/rune): Embeddable dynamic programming language for Rust.

## How to install

**TODO**

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

- [ ] Interpreter
  - [ ] add method for defining variable on interpreter directly
- [ ] add benchmarking capabilities
  - [ ] cargo alias eg. `cargo bench`
- [ ] use arena allocator for scopes
  - [ ] benchmark before & after
- [ ] use logos lexer generator
- [ ] errors
  - [ ] use `Spanned<T>` where possible
  - [x] construct error location from `Span` and source (file)
  - [x] use annotations lib to display errors
  - [ ] implement error recovery to safe expr/stmt
  - [ ] use error codes that link to a more detailed explanation (https://github.com/rust-lang/rust/tree/master/compiler/rustc_error_codes)
  - [ ] `true && (break)` currently only returns `Error: Expected Literal got RightParen, Span: 14..14`, it should also say something along the lines of: `Tip: insert expression or semicolon after break`
- [ ] types
  - [ ] `Number`, `String`, `Char`, `Bool`
  - [ ] (literal) values that come from source code directly:
    - [ ] `LiteralNumber`, `LiteralString`, `LiteralChar`, `LiteralBool`
    - [ ] `1`, `2.2`, `"Hello World!"`, `'\n'`, `false`
  - [ ] type unions: `0 | 1 | Bool`
  - [ ] type functions [docs/functions.md](docs/functions.md#type_functions)
