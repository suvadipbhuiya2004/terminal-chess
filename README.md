***

# terminl-chess

A command-line chess game written in Rust, designed as a step-by-step tutorial for learning the language.

## About Rust

Rust is a modern systems programming language that focuses on **safety, speed, and concurrency**. It is known for its memory efficiency, as it does not use a garbage collector, and can power performance-critical services. Rust's ownership model is a key feature, guaranteeing memory and thread safety at compile time. This project leverages these strengths to build a reliable and efficient application.

***


## Part 1: Board Setup \& Display

The first part of this project establishes the basic foundation: representing and displaying a chessboard in the terminal using print statements and integers.
**Read more:** [Part_1 README.md](./part_1/README.md)

## Part 2: Introducing Type-Safe Pieces with Structs and Enums

In the second part, we refactor the initial implementation to create a more robust and readable representation of chess pieces. Instead of using raw integers (known as "magic numbers"), we now use `enum`s and `struct`s to define the game's components with type safety. This update makes the code easier to understand, maintain, and extend.
**Read more:** [Part_2 README.md](./part_2/README.md)