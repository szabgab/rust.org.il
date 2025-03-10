---
title: Are We Embedded Yet? - Implementing tiny HTTP server on a microcontroller
speakers:
    - people/maor-malka.md
length: 40
language: Hebrew
video_he: https://www.youtube.com/watch?v=hgOfR8iuKjM
---

Given Rust's destiny to become the go-to replacement for C, a clear target for such a change would be the world of embedded microcontrollers.
we sometimes tend to ignore these devices, but they cover so much of our basic appliances and they all are running C.
The embedded world is notorious for its lack of capabilities of reuse and following proper safe code guidelines. this is especially true when most Embedded C developers use many tricks to get the most optimal result in regards to speed and size.

I wanted to try and leverage the power of Rust to show both its capabilities to create advanced projects easily, and do so in a "safe" manner.

I will show an example project, done on a custom STM32 microcontroller board with only 128KB Flash and 40KB running a HTTP server by becoming a USB Ethernet Adapter.

Link to [Repo of stamrust](https://github.com/maor1993/stamrust/)

About Maor:

Currently a Digital Design Engineer at [ARBE robotics](https://arberobotics.com/)
Been doing embedded, board design, FPGA and system design for the past 11 years.
Now starting to add rust to my toolbox 😉


