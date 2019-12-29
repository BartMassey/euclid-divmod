# Euclidean division and remainder
Bart Massey

Many programming languages define integer division to either
"truncate" / "round toward zero" (C, Rust) or "round down"
(Python). In either case, the integer remainder will
sometimes have to be negative to get the identity

    (a // b) * b + a % b = a

to hold. (This is an identity necessary for arithmetic to
make reasonable sense).

In the [Nickle](http://nickle.org) programming language we
chose a different convention used by languages such as
Pascal, Maple and the Z3 Theorem prover: "Euclidean"
division and remainder. In this system, the remainder is
always non-negative, and integer division is adjusted to
make the identity hold. The definition of integer division
becomes

    a // b = sgn(b) * floor(a / abs(b))

(See
[Wikipedia](https://en.wikipedia.org/wiki/Modulo_operation#In_programming_languages)
for a thorough explanation.)

Unfortunately, Euclidean division and remainder is awkward
to implement directly in Rust in terms of its "normal" `%`
and `/` operators. This crate provides a slow but correct
implementation.

## License

This program is licensed under the "MIT License". Please see
the file `LICENSE` in this distribution for license terms.