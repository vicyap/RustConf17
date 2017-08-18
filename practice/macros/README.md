# Macros

Macros are Rust's way of metaprogramming. They allow more abstract code but at
the cost of decreased code readability and decreased ability to debug. Macros
get expanded before the compiler does its static checking, thus the code
written in a macro is not the code that is being compiled.

This tradeoff between code reuse and code understandability is important to
keep in mind.


