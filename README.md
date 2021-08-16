# rusty_interpreter

The beginning of a ByteCode interpreter built with rust.


## Instructions

You can run the program by having rust and cargo [installed](https://doc.rust-lang.org/book/ch01-01-installation.html).

Then you can run `cargo build`, `cargo check`, `cargo test` and `cargo run` in the root directory.

## Motivation

This project was the result of an online challenge.

I was given the following prompt:

(1)
    You are a TA at a university, and you want to evaluate your student’s homework
    without executing their (untrusted) code. You decide to write a small
    web-service that takes bytecode as input, and interprets the results.
    The bytecode language you need to support includes basic arithmetic and
    variables. The bytecode language is stack, rather than register based.
    ByteCode (right) is given for the following pseudo code (left):

    function f() {
        x = 1                   LOAD_VAL 1
                                WRITE_VAR ‘x’
        y = 2                   LOAD_VAL 2
                                WRITE_VAR ‘y’
        return (x + 1) * y      READ_VAR ‘x’
                                LOAD_VAL 1
                                ADD
                                READ_VAR ‘y’
                                MULTIPLY
                                RETURN_VALUE
    }

Add a data type `ByteCode` that can represent bytecode like in the example
above, along with an interpreter for said bytecode.