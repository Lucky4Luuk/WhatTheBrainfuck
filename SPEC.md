# WhatTheBrainfuck specification
I'm so sorry

## Terminology
- Cell: 8-bit value at the current memory location
- Pointer: 8-bit memory location (points at a cell)

## Instructions
| Instruction | Description |
|-------------|-------------|
| `+` | Increment the current cell by 1 (wrap on overflow) |
| `-` | Decrement the current cell by 1 (wrap on underflow) |
| `*` | Multiply the current cell by 2 (wrap on overflow) |
| `/` | Divide the current cell by 2 |
|||
| `>` | Move the pointer one cell to the right |
| `<` | Move the pointer one cell to the left (cannot go below 0) |
| `#` | Jump the pointer back to 0 |
|||
| `[` | Jump past the matching `]` if the current cell is 0 |
| `]` | Jump back to the matching `[` if the current cell is !0 |
|||
| `:` | Push the current cell value to the stack (leaves current cell intact) |
| `;` | Pops the current cell from the stack and stores it in the current cell |
|||
| `^text` | Defines a label with name `text` to jump to with `@text` |
| `@text` | Jumps to a label defined with the name `text` |

Any and all other text is ignored, and can be seen as comments.

## TODO
Missing from the specification:
- Importing code from other files
