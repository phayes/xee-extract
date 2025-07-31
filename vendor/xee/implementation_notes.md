# Implementation Notes

I'm grateful to all the authors of the various libraries I've used to implement
this code - see the dependencies in the various `Cargo.toml` files.

Here are various non-library resources I've relied on during the
implementation.

## Language implementation

The book [Crafting Interpreters](https://craftinginterpreters.com/) by Robert
Nystrom was very helpful in designing the XPath interpreter, in particular the
operation around variables and function calls.

[loxcraft](https://github.com/ajeetdsouza/loxcraft) is a Rust implementation of
Lox, the language introduced by Crafting Interpreters. I studied its source
code with interest and picked up a few tricks.

The intermediate representation design is inspired by various lecture notes:

https://course.ccs.neu.edu/cs4410/lec_anf_notes.html

https://maxsnew.com/teaching/eecs-483-fa21/lec_anf_notes.html

https://users.dcc.uchile.cl/~etanter/CC5116/lec_let-and-stack_notes.html

## Automated bindings for static functions

[pyo3](https://github.com/PyO3/pyo3) lets you expose Rust functions to Python.
Since I needed to expose functions to XPath, I studied the source code of the
the pyo3 library to help me implement the `#[xpath_fn()]` macro.
