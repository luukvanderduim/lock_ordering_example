# Lock ordering example

Compile-time enforced lock ordering example.

## Purpose

To show how Rust the type system can be employed to enforce lock ordering, to prevent deadlocks.

Watch: ["Safety in an unsafe world" talk by Joshua Liebow-Feeser](https://www.youtube.com/watch?v=qd3x5MCUrhw)
A great talk on Google's Netstack3, Rust and upholding invariants using the type system which lured me into this rabbit hole.

## AI-generated slop?

Yes. The LLM offered to generate me an example which I decided to store for later reference.

## Reference

There is a library crate which offers exactly this, called [lock_ordering](https://github.com/akonradi/lock-ordering) by Alex Bakon (akonradi on Github).
In the talk Joshua credits Alex as the author/designer of this pattern, how lock ordering is upheld in Netstack3.
