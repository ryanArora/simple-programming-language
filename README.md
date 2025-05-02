# Simple Programming Language

A minimal programming language implementation with basic features like:

- Variables and assignment
- Basic arithmetic operations
- Control flow (if/else, loops)
- Print statements

It uses a hand-written recursive-descent parser (for flexibility).

## Examples

### Fibonacci Sequence

`cargo run examples/fib.l`

```rust
let a = 0;
let b = 1;

loop {
    let c = a + b;
    print(c);
    a = b;
    b = c;
};
```

<details>
<summary>View Abstract Syntax Tree (AST)</summary>

`cargo run examples/fib.l --output-stage ast`

```rust
Block {
  statements: [
    LetStatement(
      LetStatement {
        identifier: "a",
        expression: Some(
          Literal(
            IntegerLiteral(
              0,
            ),
          ),
        ),
        _mutable: false,
      },
    ),
    LetStatement(
      LetStatement {
        identifier: "b",
        expression: Some(
          Literal(
            IntegerLiteral(
              1,
            ),
          ),
        ),
        _mutable: false,
      },
    ),
    LoopStatement(
      LoopStatement {
        block: Block {
          statements: [
            LetStatement(
              LetStatement {
                identifier: "c",
                expression: Some(
                  BinaryOperation(
                    BinaryOperation {
                      operation_type: Add,
                      left_expression: Identifier(
                        "a",
                      ),
                      right_expression: Identifier(
                        "b",
                      ),
                    },
                  ),
                ),
                _mutable: false,
              },
            ),
            Expression(
              FunctionCall(
                FunctionCall {
                  function_name: "print",
                  argument: Identifier(
                    "c",
                  ),
                },
              ),
            ),
            Assignment(
              AssignmentStatement {
                identifier: "a",
                expression: Identifier(
                  "b",
                ),
              },
            ),
            Assignment(
              AssignmentStatement {
                identifier: "b",
                expression: Identifier(
                  "c",
                ),
              },
            ),
          ],
        },
      },
    ),
  ],
}
```

</details>

<details>
<summary>View Intermediate Representation (IR)</summary>

`cargo run examples/fib.l --output-stage ir`

```
li r1, 0
li r2, 1
L1:
add r3, r1, r2
print r3
li r4, 0
add r1, r2, r4
li r5, 0
add r2, r3, r5
L2:
j L1
L3:
```

</details>
