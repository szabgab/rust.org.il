---
date: 2019.03.11
title: Redis modules and a production story
register: https://www.meetup.com/rust-tlv/events/259099506/
online: false
language: Hebrew
---

Rust TLV is back with two exciting talks, and at a new venue - this time [SimilarWeb](https://www.similarweb.com/) will be hosting us at their amazing offices!

* Rust - a (Pre)production Story by [Yoav Yanilov](https://www.linkedin.com/in/yoav-yanilov/)
In this talk, I'll walk you through my first experience with Rust, a drop-in replacement for an IO-bound and CPU-intensive production service, performing parallel in-memory aggregations on high-volume compressed text.
We'll see how the Rust implementation reduced memory footprint and improved CPU utilization. We'll also highlight some of the popular 3rd-party crates I used in the process (tokio, hyper, and rusoto), and examine a quirk or two. [slides](https://docs.google.com/presentation/d/101z1MBtcn6FkkHQV-rNiRm0PnkdGgTanS3luCxcS1h0/edit#slide=id.g51e85cc9e2_0_0)

Yoav is a software developer at SimilarWeb, working on the B2B platform backend, where he likes to tackle infrastructure and design challenges.

* Developing Redis Modules in Rust by [Gavrie Philipson](https://www.linkedin.com/in/gavrie/)

Redis is well known and loved in the open source world. It has long been possible to write modules for Redis, but so far this has been done mostly in C. This is due to Redis itself being written in C, and its API being in the form of a C header file.
It's not easy to write code that is both memory-safe and performant, which is what Rust excels at. This makes it a natural match for writing modules for Redis: They running in the same memory space as Redis, and a bad memory access can crash the whole process.
Writing bindings between Rust and C code is not hard. Our main challenge has been (and still is) to come up with a clean, safe and idiomatic Rust API that hides all the ugly stuff and allows easily writing modules.

Gavrie's current role is Cluster Architect at Redis Labs. He has been hacking away for far too many years at a variety of startup companies. On a never-ending quest for the ultimate programming language, recent favorites have included Kotlin, Go and Python. Since starting with Rust he hasn't looked back.


