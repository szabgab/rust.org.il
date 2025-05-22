---
title: From TensorFlow to PyTorch with some help from Rust
speakers:
    - people/gavrie-philipson.md
length: 40
language: Hebrew
---

When moving a machine learning project from TensorFlow to PyTorch, we wanted to keep using our existing training data files that were in the [TFRecord format](https://www.tensorflow.org/tutorials/load_data/tfrecord).
Unfortunately, there was no good method of reading this format from PyTorch without adding TensorFlow as a dependency, and even that caused performance problems.

To address this, I wrote [rustfrecord](https://pypi.org/project/rustfrecord/), a Python package written in Rust with the help of [PyO3](https://pyo3.rs/), and published to PyPI using [Maturin](https://www.maturin.rs/).

In the talk I'll discuss the problem, the solution, and what can be learned from this for other projects.
