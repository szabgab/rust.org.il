---
title: Developing Redis Modules in Rust
speakers:
    - people/gavrie-philipson.md
length: 0
language: Hebrew
---

Redis is well known and loved in the open source world. It has long been possible to write modules for Redis, but so far this has been done mostly in C. This is due to Redis itself being written in C, and its API being in the form of a C header file.
It's not easy to write code that is both memory-safe and performant, which is what Rust excels at. This makes it a natural match for writing modules for Redis: They running in the same memory space as Redis, and a bad memory access can crash the whole process.
Writing bindings between Rust and C code is not hard. Our main challenge has been (and still is) to come up with a clean, safe and idiomatic Rust API that hides all the ugly stuff and allows easily writing modules.

Gavrie's current role is Cluster Architect at Redis Labs. He has been hacking away for far too many years at a variety of startup companies. On a never-ending quest for the ultimate programming language, recent favorites have included Kotlin, Go and Python. Since starting with Rust he hasn't looked back.


