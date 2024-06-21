# [name]

A simple bytecode compiler written in Rust with stack-based VM.

## Overview





## Getting Started

Try the language in the [playground]([name].vercel.app).

### Example

```bash
cargo install [name]
echo "let a = 10; print(a);" > example.rs
[name] run example.rs
```

Try more examples in the [examples](./examples) directory.

## Features

- [x] Variables
- [x] Arithmetic operations
- [x] Logical operations
- [x] Control flow (if, else)
- [x] Loops (while)
- [ ] Functions
- [ ] Closures
- [ ] Autograd
- [ ] Tensor Support

## Implementation

The source code goes through the following stages:

1. [Tokenization](#tokenization)
2. [Parsing](#parsing)
3. [Compilation](#compilation)
4. [Execution](#execution)

### Tokenization

[scanner.rs](./src/scanner.rs)

Here the scanner/lexer take the raw source code and converts in into a list of [predefined tokens](./src/scanner.rs).

Here I am using rust [logos](https://github.com/maciejhirsz/logos) to create the lexer, which converts the source code into vector of tokens.

For example, the source code:

```rust
let a = 10;
print(a);
```

Get's converted into:

```rust
[LET, Identifier, EQUAL, Number(10.0), SEMICOLON, PRINT, LeftParen, Identifier, RightParen, SEMICOLON]
```

### Parsing

[parser.rs](./src/ast.rs)

The parser takes the list of tokens and converts it into an Abstract Syntax Tree (AST). Here we also handle the precedence of the operators, syntax errors, syntatic sugar like `+=`, `-=`, etc.

For example, the tokens:

```rust
[LET, Identifier, EQUAL, Number(10.0), SEMICOLON, PRINT, LeftParen, Identifier, RightParen, SEMICOLON]
```

Get's converted into:

```rust
Let("a", [Number(10.0)])
Print([Identifier("a")])
```

I am using a combination of recursive descent parser and [pratt parser](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html) to parse the tokens.

Pratt parser is used to parse the expressions, and recursive descent parser for everything else.

### Compilation

[compiler.rs](./src/compiler.rs)

The compiler takes the AST and converts it into a list of bytecode instructions.

For example, the AST:

```rust
Let("a", [Number(10.0)])
Print([Identifier("a")])
```

Get's converted into:

```
0    OP_CONSTANT         cons->[1]     | tnsr->10
2    OP_DEFINE_GLOBAL    cons->[0]     | intr->a
4    OP_GET_GLOBAL       cons->[2]     | intr->a
6    OP_PRINT
7    OP_RETURN
```

[Chunk](./src/chunk.rs) - It's stored the bytecode instructions along with any constants like strings, numbers, etc.

### Execution

[vm.rs](./src/vm.rs)

The VM takes the list of bytecode instructions and executes them. It is simply a loop that reads the instructions and executes them.

The VM has a stack-based architecture, which means that the instructions are executed by pushing and popping values from the stack.

For example, the bytecode:

```
0    OP_CONSTANT         cons->[1]     | tnsr->10
2    OP_DEFINE_GLOBAL    cons->[0]     | intr->a
4    OP_GET_GLOBAL       cons->[2]     | intr->a
6    OP_PRINT
7    OP_RETURN
```

Get's executed and prints:

```
10
```

## References

- [Crafting Interpreters](https://craftinginterpreters.com/)
- [Pratt Parsing](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html)
- [Logos](https://github.com/maciejhirsz/logos)
