---
title: Rust is the future of Linux drivers
speakers:
    - people/leon-vak.md
length: 60
language: Hebrew
---

I gave a talk about Rust in the Linux kernel back in 2024. Back then, it was mostly an experiment and honestly, there was a lot of debate around it. Even though I was excited, for my daily work—writing DMA and PCIe drivers for our customers—I still stuck with C. The situation felt shaky: one day Linus would say something positive, and the next day a lead maintainer would quit because of the "non-technical nonsense."
But while the headlines were messy, the developers kept working quietly and closing the gaps. Fast forward to today, the "experiment" tag is officially gone. With the new Nova driver for NVIDIA and subsystems like Binder already running in Rust on millions of devices, it is no longer a question of "if," but "when." I decided it’s the right time to take a deep dive into what we have today and see if we are finally ready to move our daily driver work from C to Rust.

