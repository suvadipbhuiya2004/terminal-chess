***

# terminal-chess - Part 1: Board Setup

This `README.md` documents the progress and learning outcomes for the first part of the Rust Chess tutorial project. The objective of this initial phase is to establish a basic, visual representation of a chessboard in the terminal.

***

## What We Will Learn in Part 1

By completing this first milestone, you will gain hands-on experience with the foundational concepts of Rust programming. These are the building blocks for any Rust application. Below are the key concepts covered, with links to the official Rust documentation for further reading.

- **Project Initialization \& Structure**
Learn how to create a new Rust project using `cargo new` and understand the basic file structure it generates. Cargo is Rust's build tool and package manager.
    - **Read more:** [Hello, World! - The Rust Programming Language](https://doc.rust-lang.org/book/ch01-02-hello-world.html)
- **Variables and Mutability**
Understand the core concept of immutability in Rust and how to make variables mutable using the `mut` keyword. This is fundamental to Rust's approach to safety.
    - **Read more:** [Variables and Mutability - The Rust Programming Language](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
- **Primitive Data Types \& Arrays**
Use basic integer types (`i8`) to represent data and learn how to use fixed-size arrays (`[T; N]`) to store a collection of elements of the same type. In this project, a 2D array models the chessboard.
    - **Read more:** [Data Types - The Rust Programming Language (Array Type Section)](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- **Control Flow (Loops and Conditionals)**
Use `for` loops to iterate over the board's dimensions and `if/else` statements for conditional logic to handle different pieces and empty squares.
    - **Read more:** [Control Flow - The Rust Programming Language](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
- **Functions**
Define and call a basic function (`board_print`) that takes arguments and encapsulates a specific task—in this case, rendering the board.
    - **Read more:** [Functions - The Rust Programming Language](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
- **Console Output**
Use the powerful `println!` macro to print formatted text to the terminal, which is essential for displaying the game state and debugging.
    - **Read more:** [Hello, World! - Anatomy of a Rust Program](https://doc.rust-lang.org/book/ch01-02-hello-world.html)

***

## What We Have Achieved in Part 1

This initial version of the project successfully sets up and displays the chessboard.

- **Board Representation:** The chessboard is implemented as a mutable 8x8 2D array (`[[i8; 8]; 8]`), where each element holds a value representing a piece or an empty square.
- **Piece Representation:** A simple numerical system is used to define the pieces:
    - **Color:** The sign of the number indicates the piece's color (positive for White, negative for Black).
    - **Type:** The absolute value identifies the piece type (1 for Pawn, 2 for King, 3 for Queen, etc.).
- **Initial Board State:** The `main` function programmatically populates the 2D array, placing all 32 pieces in their correct starting positions.
- **Visual Display:** A `board_print` function has been created to render the board in the terminal. It includes row and column labels (1-8, A-H) to mimic standard algebraic notation, making the output clear and readable.
