# Rym

-   [Rym](#rym)
    -   [Disclaimer](#disclaimer)
    -   [Intro](#intro)
    -   [Examples](#examples)
    -   [About](#about)
        -   [Name](#name)
        -   [Goals](#goals)
    -   [Installation](#installation)
    -   [Inspired by](#inspired-by)
    -   [Other languages written in Rust](#other-languages-written-in-rust)
    -   [Project Structure](#project-structure)
        -   [Tests](#tests)
    -   [Todos](#todos)

## Disclaimer

This is just a hobby project. It is very far from being production ready!

## Intro

**Rym** is a statically typed programming language inspired by Rust, Swift, Python and others.
It focuses on **ease of use** and **should just work™**. </br>

Big thanks to Robert Nystrom and his book [crafting interpreters](http://craftinginterpreters.com)
which inspired me to start and continue working on this language.

## Examples

```rym
const game = fn(x: Uint) String => {
    const rule = fn(acc, num, word) =>
        if (x mod num) == 0 then acc ++ word else acc;

    const default_rule = fn(acc) =>
        if acc == "" then acc else x.to_string();

    "".rule(3, "Fizz")
      .rule(5, "Buzz")
      .rule(7, "Splash")
      .default_rule()
}

const main = fn() {
    for i in 0..=100:
        println(game(i))
}
```

```rust
const main = fn() @Io, Result[(), Error] => {
    let msg = "Hello World";
    println(msg);

    let mut num = 2/4 * (10 - 1);
    println("Number:", num);

    const msg = msg + "!";
    println("Combined:", msg, num);
}
```

Early returns when unwrapping `Tryable`s

```rust
const main = fn() @Io, Result[i32, String] => {
    let number = maybe_error().try;
    println(number);

    // Same as
    let number = match maybe_error():
        | Ok(inner) => inner,
        | Err(err) => return Err(err);
    println(number);
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

## About

### Name

-   **R**ust**y** ⇒ Heavily borrows 🙃 from Rust
-   **M**ulti-paradigm ⇒ Mix of object oriented, procedural and functional programming
-   Programming **Lang**uage ⇒ because thats what it is

### Goals

The language should be as powerful as possible while only providing a small set of consistent features. (Kind of like Go)

Apart from that Rym just gets used to try out a bunch of my ideas...
I am trying my best to combine all of them in a meaningful manner ;)

-   Features
    -   Static types
        -   inferred
    -   Complex types
        -   struct
        -   enum / tagged union
    -   Iterators
    -   Zig like comp time
-   Nice developer experience
    -   Informative errors and warnings
    -   Builtin tools
        -   First party REPL
        -   Package manager

## Installation

**TODO**

## Inspired by

-   [Rust](https://github.com/rust-lang/rust): Empowering everyone to build reliable and efficient software.
-   [OCaml](): **TODO**
-   [F#](): **TODO**
-   [Swift](https://github.com/apple/swift): A Swift is a high-performance system programming language.
-   [HVM](https://github.com/Kindelia/HVM): A massively parallel, optimal functional runtime in Rust
-   [Unison](https://www.unison-lang.org/): A friendly programming language from the future (statically-typed, functional)
-   [Fused Effects](https://github.com/fused-effects/fused-effects): A fast, flexible, fused effect system for Haskell

## Other languages written in Rust

-   [Boson](https://github.com/Narasimha1997/boson-lang): Hybrid programming language written in Rust.
-   [Mun](https://github.com/mun-lang/mun): Programming language empowering creation through iteration.
-   [Tao](https://github.com/zesterer/tao): Statically-typed functional language with polymorphism, typeclasses, algebraic effects, sum types, pattern-matching, first-class functions, currying, good diagnostics, and much more!
-   [Rhai](https://github.com/rhaiscript/rhai): Embedded scripting language for Rust.
-   [Rune](https://github.com/rune-rs/rune): Embeddable dynamic programming language for Rust.

## Project Structure

The project is split into many crates that are part of one Cargo workspace:

`crates/rymx` ⇒ command line tool for executing `.rym` files

### Tests

For now just run `cargo test`, there is no special setup.

**TODO**

## Todos

-   [ ] add benchmarking capabilities
