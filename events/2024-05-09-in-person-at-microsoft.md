---
date: 2024.05.09
title: In-person at Microsoft, Tel Aviv
register: https://www.meetup.com/code-mavens/events/300144781/
online: false
language: Hebrew
---

Agenda:

* 18:00 Meeting amd mingling
* Presentations. TBD, submit your [talk proposals](https://github.com/szabgab/rust.org.il/labels/proposal).

* [Maor Malka](https://www.linkedin.com/in/maor-malka-a46640134/): Are We Embedded Yet? - Implementing tiny HTTP server on a microcontroller (30-40 min)

Given Rust's destiny to become the go-to replacement for C, a clear target for such a change would be the world of embedded microcontrollers.
we sometimes tend to ignore these devices, but they cover so much of our basic appliances and they all are running C.
The embedded world is notorious for its lack of capabilities of reuse and following proper safe code guidelines. this is especially true when most Embedded C developers use many tricks to get the most optimal result in regards to speed and size.

I wanted to try and leverage the power of Rust to show both its capabilities to create advanced projects easily, and do so in a "safe" manner.

I will show an example project, done on a custom STM32 microcontroller board with only 128KB Flash and 40KB running a HTTP server by becoming a USB Ethernet Adapter.

Link to [Repo of stamrust](https://github.com/maor1993/stamrust/)

"Are we embedded yet" slides in [pptx](slides/are-we-embedded-yet.pptx) and converted to [pdf](slides/are-we-embedded-yet.pdf).

About Maor:

Currently a Digital Design Engineer at [ARBE robotics](https://arberobotics.com/)
Been doing embedded, board design, FPGA and system design for the past 11 years.
Now starting to add rust to my toolbox 😉


* [Gabor Szabo](https://www.linkedin.com/in/szabgab/): Web application development with Rocket

I am working on my first web application in Rust using the [Rocket web framework](https://rocket.rs/).
In this presentation I'll show what I've learned so far and how can you get started developing a web application using Rust Rocket.

We used the [rocket-starter](https://crates.io/crates/rocket-starter) crate and you saw [articles about Rocket](https://rust.code-maven.com/rocket).

About Gabor:

Gabor has been self-employed since 2000 providing [consulting and training services](https://szabgab.com/).
For many years his main language was Perl, then Python, and these days his focus is Rust. Part of his efforts is running
these meetings and trying to energize the [Rust community in Israel](https://rust.org.il/).


