# Monkey Programming Language Interpreter

A Rust implementation of the Monkey programming language interpreter. This project is an implementation of the Monkey programming language as described in the book "Writing An Interpreter In Go" by Thorsten Ball, but written in Rust.

## Project Structure

The project is organized into a few key components:

- `lexer/`: Tokenizes the input source code
- `parser/`: Parses tokens into an Abstract Syntax Tree (AST)
- `evaluator/`: Evaluates the AST to produce results
- `repl/`: Provides an interactive Read-Eval-Print Loop

## Features

- Interactive REPL (Read-Eval-Print Loop)
- Lexical analysis
- Parsing
- Evaluation
- Support for Monkey programming language features

## Prerequisites

- Rust (2021 edition or later)
- Cargo (comes with Rust)

## Building

To build the project:

```bash
cargo build
```

## Testing

This project includes comprehensive unit tests for all major components. To run the tests:

```bash
cargo test
```

Each component (lexer, parser, evaluator) has its own test suite to ensure correct functionality.

## Running

To run the REPL:

```bash
cargo run
```

## Usage

Once the REPL is running, you can enter Monkey code expressions. To exit the REPL, enter an empty string.

Example Monkey code you can try:
```monkey
let x = 5;
let y = 10;
x + y
```

## Dependencies

- `indoc`: For indented documentation strings

## License

This project is open source and available under the MIT License.
