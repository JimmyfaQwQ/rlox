# A implementation of the JLox language in Rust

This is a implementation of the JLox language in Rust, based on the book "Crafting Interpreters" by Bob Nystrom. The repo is for practice and learning purposes, and is not intended for production use.

## About the missing Visitor pattern

The Visitor pattern is a design pattern that allows you to separate an algorithm from the objects on which it operates, and it means that it works well in OOP languages. However, in Rust, we can use enums and pattern matching to achieve the same result without the need for a Visitor pattern. This approach is more idiomatic in Rust and allows us to write cleaner and more efficient code. Therefore, the Visitor pattern is not used in this implementation of JLox in Rust.

The <'a> lifetime annotations in the code are used to indicate the maximum lifetime of the raw code str.
