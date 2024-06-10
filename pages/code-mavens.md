---
title: Code Mavens
timestamp: 2023-11-03T19:30:02
published: true
description:
---

The [Code Mavens](https://www.meetup.com/Code-Mavens/) Meetup group exists for almost 10 years. During those years we had loads of meetings
on a lot of different topics. Recently we started to have meetings about Rust.

## Organizers

The main organizer of the group is [Gabor Szabo](https://szabgab.com/).

## Upcoming events

See the [meetings](/meetings) page.

### 2024.06.23 - [Rust at Microsoft, Tel Aviv in June 2024](https://www.meetup.com/code-mavens/events/301170994/) at Microsift in Tel Aviv

* When 18:00
* Where: Microsoft, Tel Aviv
* Language: Hebrew

* [Yoray Herzberg](https://www.linkedin.com/in/yoray-herzberg-b8155621b/): Implementing a Neural Network in Rust (20 min)

Neural Networks are one of the most popular models in Machine Learning, and are the basis for a lot of popular AI applications, such as Computer Vision (CV), Natural Language Processing (NLP), and more. In this presentation, I show the math behind Neural Nets, how to train them, and how I implemented one in Rust from scratch to recognize digits in images.

The presentation is based on a post Yoray wrote: [Digit Recognition with Rust and WASM](https://vaktibabat.github.io/posts/Rust_WASM_Digit_Recognition_1/).


### 2024.05.23 - [Web development in Rust using Rocket](https://www.meetup.com/code-mavens/events/300974367/) (virtual in Hebrew)

In the most recent in-person meeting I talked about the [Rocket web framework](https://rocket.rs/). I am going to give an updated and extended version of the presentation, this time in Zoom.

This will give the opportunity to everyone who could not attend the in-person meeting to hear the presentation. It will be also a good opportunity to ask questions.

* When 18:00
* Speaker and organizer: [Gabor Szabo](https://szabgab.com/)
* Language: Hebrew
* Location: Zoom
* Length: 1 hour, maybe a bit longer.

### 2024.06.06 - [Rust Maven Workshop: Your first contribution to an Open Source Rust project](https://www.meetup.com/code-mavens/events/301156302/) (virtual in English)

During this workshop you will learn about various side of contribution to an open source project. We will cover both the technical and the human aspects of contributing to an open source project. We will discuss both the technical and cultural differences between working in a company and contributing to an open source project.

Contributing a substantial improvement to an established open source project would probably take a lot more time than we have in this workshop so we are going to make some small, but still useful contributions to several open source projects.

You will learn and practice the technique of sending a pull request first via the web interface of GitHub and then using a cloned version of the project.

The workshop includes presentations and hands-on work.

The workshop will be via Zoom
Language: English.
Workshop lead: [Gabor Szabo](https://szabgab.com/)
Requirements: Basic familiarity with writing Rust will be enough.
Length: up to 3 hours.

This workshop is free of charge thanks to the people who support me via one of the following systems: [Github sponsor](https://github.com/sponsors/szabgab/), [Patreon](https://www.patreon.com/szabgab), or directly via [PayPal](https://www.paypal.com/paypalme/szabgab).


### 2024.06.23 - [Rust at Microsoft, Tel Aviv in June 2024 (in Hebrew)](https://www.meetup.com/code-mavens/events/301170994/)

Agenda:

* 18:00 Meeting amd mingling
* Presentations. TBD, submit your [talk proposals](https://github.com/szabgab/rust.org.il/labels/proposal).
* Pub

------

## Past Events

### 2024.05.09 - [In-person at Microsoft, Tel Aviv](https://www.meetup.com/code-mavens/events/300144781/)

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



### 2024.04.21 - [Using AstroNvim for Rust development (Online, in Hebrew)](https://www.meetup.com/code-mavens/events/300265648/)

In this non-formal meeting [Meir Kriheli](https://www.meirkriheli.com/) a long time Open Source developer and a central figure of the Israeli Open Source community is going to share his decades of experience using AstroNvim and tmux.

He will show his own setup with a focus on how he uses it for **programming in Rust**. He will help the interesting parties to set up their own environment.

He will also show [rusmux](https://www.meirkriheli.com/en/2023/04/introducing-rusmux/), his partial replacement for the tmuxinator written in Rust.

The meeting will be in Zoom. It will start at 19:00 ILT. It will be held in Hebrew.



### 2023.12.17 - [Don't panic! - Our journey to error handling in Rust (Online and in Hebrew)](https://www.meetup.com/code-mavens/events/297334993/)

* Speaker: [Tomer Cohen](https://www.linkedin.com/in/tomercode/) from Microsoft
* View the [blog of Tomer Cohen](https://www.tomercode.com/)
* Language: Hebrew
* Location: Online

In this talk, I will delve into the transformative journey of error handling in our first Rust project.
From an initial state where panic-induced chaos was widespread, making it challenging to pinpoint the
root causes of crashes, we navigated through the Rust ecosystem to establish a robust and stable foundation.

Utilizing some of the most common error handling crates, I will share insights into the evolution
of our error-handling practices, discussing the pitfalls encountered, and lessons learned, that led to a more resilient and maintainable codebase.

* [Video](https://youtu.be/Fi--zxTU-8w)


### 2023.11.12 [Rust Digger](https://www.meetup.com/code-mavens/events/297064458/)

* Speaker: [Gabor Szabo](https://szabgab.com/)
* Author of [Rust Maven](https://rust.code-maven.com/)
* Language: Hebrew
* Location: online

Overview and current status of [Rust Digger](https://rust-digger.code-maven.com/)

The goal of this project is to enhance the Rust ecosystem by analyzing the common practices of Crate developers. From basic things such as code formatting rules through practices of testing.
It also helps finding Crates that can be improved with relative little investment.

* [Video recordings about Rust Digger](https://he.code-maven.com/rust-digger-video)

