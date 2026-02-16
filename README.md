# Lox Interpreter in Rust

A high-performance, tree-walk interpreter for the Lox programming language, written in Rust.

This implementation follows the design patterns described in Bob Nystrom's [Crafting Interpreters](https://craftinginterpreters.com/). It features a full lexical scanner, recursive descent parser, and an AST-walking interpreter.

## Features

- **Dynamic Typing**: Variables can hold any type of value.
- **Garbage Collection**: Uses Rust's ownership model and `Rc` for memory management (no manual GC implementation required).
- **Functions**: First-class functions with closures.
- **Control Flow**: `if`, `while`, and `for` loops.
- **Expressions**: Basic arithmetic, comparison, and logical operators.
- **Classes**: *Currently parsed but runtime execution is partially implemented (Work In Progress).*

## Architecture

The interpreter processes Lox code in several stages:

1.  **Scanner (`src/scanner.rs`)**: Reads raw source code and tokenizes it into a stream of meaningful symbols (tokens).
2.  **Parser (`src/parser.rs`)**: Consumes tokens using a recursive descent algorithm to build an Abstract Syntax Tree (AST). The grammar is defined in `grammar.bnf`.
3.  **Resolver (`src/resolver.rs`)**: Performs a semantic analysis pass to resolve variable binding scopes and verify correct usage before runtime.
4.  **Interpreter (`src/interpreter.rs`)**: Walks the AST to execute statements and evaluate expressions.
    -   It maintains environments for variable scope (`src/env.rs`).
    -   It handles native functions and user-defined functions (`src/function.rs`, `src/callable.rs`).

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended)

## Build & Run

### Clone the Repository
```bash
git clone https://github.com/your-username/lox-rust.git
cd lox-rust
```

### Build
```bash
cargo build --release
```

### Usage

**Run the REPL (Interactive Mode):**
To start an interactive session, run `lox` without arguments:
```bash
cargo run
> print "Hello, Lox!";
Hello, Lox!
```

**Run a Script:**
To execute a Lox source file (`.lox`), pass the file path as an argument:
```bash
cargo run -- path/to/script.lox
```

## Syntax & Language Guide

### Variables & Types
Lox is dynamically typed with `nil`, `Boolean`, `Number`, `String`.

```lox
var a = 1;
var b = "hello";
var c = true;
var d = nil;

print a + 2; // 3
```

### Control Flow

**If Statements:**
```lox
if (condition) {
  print "yes";
} else {
  print "no";
}
```

**While Loops:**
```lox
var i = 0;
while (i < 5) {
  print i;
  i = i + 1;
}
```

**For Loops:**
```lox
for (var i = 0; i < 5; i = i + 1) {
  print i;
}
```

### Functions
Functions are declared with `fun`. They support closures.

```lox
fun add(a, b) {
  return a + b;
}

print add(1, 2); // 3

fun makeCounter() {
  var i = 0;
  fun count() {
    i = i + 1;
    print i;
  }
  return count;
}

var counter = makeCounter();
counter(); // 1
counter(); // 2
```

### Classes (WIP)
Class syntax is supported by the parser, but full object-oriented features (inheritance, `this`, `super`) are currently under developement.

```lox
class Breakfast {
  cook() {
    print "Eggs a-fryin'!";
  }

  serve(who) {
    print "Enjoy your breakfast, " + who + ".";
  }
}
```

## Grammar Reference

The full grammar definition can be found in `grammar.bnf`.

```bnf
program    -> declaration* EOF;
declaration -> varDecl | statement | funDecl ;
statement  -> exprStmt | printStmt | block | ifStmt | whileStmt | forStmt | returnStmt ;
...
```

## License

This project is open-source and available under the generic MIT License.
