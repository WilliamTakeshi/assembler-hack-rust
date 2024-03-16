# Hack Assembler

This is a simple assembler designed to translate programs written in the Hack assembly language into Hack binary code. The Hack assembly language is utilized in the context of the "Nand to Tetris" course/book, which teaches the construction of a computer system from the ground up.

## Caution

The project is not finished, only works for simple programs, this is a Work in progress.

## Usage

To use the assembler, follow these steps:

1. **Installation**: Clone or download this repository to your local machine.

2. **Input File**: Prepare your Hack assembly code in a `.asm` file. Make sure the assembly code follows the syntax and conventions outlined in the "Nand to Tetris" course/book. (Some examples on the `/examples` directory)

3. **Running the Assembler**: Open a terminal or command prompt, navigate to the directory where you have cloned/downloaded the repository, and run the following command:

    ```bash
    cargo run -- -i ./examples/rect/RectL.asm -o rectL.hack
    ```

    Replace `-i` arg with the path to your Hack assembly code input file. `-o` is the path of the output file.

4. **Output**: The assembler will generate a `.hack` file containing the translated binary code corresponding to your assembly program.


## Syntax and Conventions

Ensure that your assembly code adheres to the syntax and conventions specified in the "Nand to Tetris" course/book. Some key points to remember:

- Use the correct mnemonics for instructions (`@` for A-instructions, `dest=comp;jump` for C-instructions).
- Labels must be unique and preceded by `(` and followed by `)`.
- Comments can be included using `//`.

## Acknowledgments

This project was inspired by the "Nand to Tetris" course/book, which provides a comprehensive introduction to computer architecture and construction. Special thanks to the authors and contributors of the course/book for their invaluable resources and guidance.

For further information, refer to the "Nand to Tetris" course/book available at [www.nand2tetris.org](https://www.nand2tetris.org).