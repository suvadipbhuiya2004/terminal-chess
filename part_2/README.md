# terminal-chess - Part 2: Introducing Structs and Enums

Welcome to the second part of this Rust chess project. In this phase, we have refactored the initial board representation to create a more robust, readable, and type-safe foundation using some of Rust's powerful features.

The initial version used a simple `[[i8; 8]; 8]` array where integers represented the pieces. This new code introduces `struct`s and `enum`s to define the game's components more explicitly.

### What We Implemented

The core of this update was to replace the "magic numbers" (like `1` for a white pawn or `-6` for a black rook) with well-defined data structures.

1. **`Color` Enum**: An `enum` to represent the two possible colors, `White` and `Black`. This ensures that a piece can only have one of these two valid colors.
2. **`PieceType` Enum**: An `enum` to define the six types of chess pieces: `Pawn`, `King`, `Queen`, `Bishop`, `Knight`, and `Rook`.
3. **`Piece` Struct**: A `struct` that encapsulates all the data related to a single chess piece. It contains two fields:
    * `piece_type`: An instance of the `PieceType` enum.
    * `color`: An instance of the `Color` enum.
4. **Implementation (`impl`) Block**: We added an `impl` block for the `Piece` struct to associate methods with it.
    * `piece_new()`: A constructor function to create new `Piece` instances.
    * `piece_get_value()`: A method that returns the original `i8` value for a piece, allowing us to still use the `[[i8; 8]; 8]` board representation for now.

### Concepts Learned

This refactoring exercise provides a practical application of several fundamental Rust concepts:

* **Enums (Enumerations)**: Enums allow you to define a type by enumerating its possible variants. We used them to represent a set of fixed choices, like the type of a piece or its color. This prevents invalid values and makes the code self-documenting. For example, `PieceType::Queen` is much clearer than the number `3`.
* **Structs (Structures)**: Structs are custom data types that let you group related data together. Our `Piece` struct bundles the `piece_type` and `color`, treating a chess piece as a single, cohesive unit. This approach is fundamental to organizing complex data in a program.
* **Type Safety**: By using `enum`s and `struct`s, we leverage Rust's compiler to enforce correctness. It's now impossible to create a piece with an invalid color or a non-existent type, which was a risk when using plain integers. This helps prevent a whole class of bugs at compile time.


### Further Reading

To learn more about the concepts used in this part of the project, you can explore the official Rust documentation and other helpful resources:

* **Structs**:
    * The Rust Programming Language Book: [Defining and Instantiating Structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)
    * Rust by Example: [Structs](https://doc.rust-lang.org/rust-by-example/custom_types/structs.html)
* **Enums**:
    * The Rust Programming Language Book: [Defining an Enum](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)
    * Rust by Example: [Enums](https://doc.rust-lang.org/rust-by-example/custom_types/enum.html)
* **Match Control Flow**:
    * The Rust Programming Language Book: [The `match` Control Flow Construct](https://doc.rust-lang.org/book/ch06-02-match.html)[^11]
    * Rust by Example: [Match](https://doc.rust-lang.org/rust-by-example/flow_control/match.html)
    