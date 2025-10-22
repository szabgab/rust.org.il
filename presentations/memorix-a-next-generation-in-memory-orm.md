---
title: "Memorix: A Next-Generation In-Memory ORM Written in Rust for Microservices and IoT"
speakers:
    - people/yuval-saraf.md
length: 30
language: Hebrew
---

In distributed systems, developers face significant challenges managing in-memory data stores like Redis and message brokers like Kafka across multiple programming languages while maintaining type safety. While tools like GraphQL, Swagger, Prisma have solved persistent data management and messaging layers, the in-memory layer has been largely overlooked.

This presentation introduces Memorix, an open-source in-memory ORM written in Rust that addresses this gap. Memorix uses a schema-first approach to generate fully-typed client code for multiple programming languages, enabling seamless communication with microservices and IoT devices regardless of their implementation language.

## What you'll see:

* The problem: type safety challenges in distributed in-memory systems
* Memorix overview: schema-first, multi-language code generation
* Live demo: creating a schema and using it across TypeScript, Python, and Rust
* Why Rust: how Rust’s features make this tool possible (performance, macros, and more)

The talk includes practical examples and live coding to show how Memorix brings the same developer experience to in-memory data that modern ORMs provide for databases.

[Memorix project](https://uvop.github.io/memorix/)

[slides](https://unimonkiez.github.io/rust-conf-presentation-memorix/)
