---
title: Meetings
timestamp: 2023-11-04T17:30:02
published: true
description:
---

* The [Rust TLV](https://www.meetup.com/rust-tlv/) group has both in-person and virtual events.
* The [Code Mavens](https://www.meetup.com/code-mavens/) hosts virtual events in English about Rust, Python, and Perl.
* The [Virtual events](https://events.code-maven.com/) site lists Virtual Rust events around the world with a possibility to filter by time and language.

## Upcoming events

## Future Meetings

We are looking for speakers for our meetings. If you are interested to give a presentation, please [open an issue describing it](https://github.com/szabgab/rust.org.il/)
or talk to [Gabor Szabo](https://szabgab.com/contact). Presentation can be at any level and any topic related to Rust.

If you are interested hosting an in-person event or sponsoring one, please contact  [Gabor Szabo](https://szabgab.com/contact).

## Past Events


### 2018.10.23 - [Under The Hood, Electrum](https://www.meetup.com/rust-tlv/events/254349265/)

Hosted by [Healthy.io](https://healthy.io/).

* Rust Under The Hood by [Isan Rivkin](https://www.linkedin.com/in/isan-rivkin/):

Ownership is the breakout feature of Rust. It allows Rust to be completely memory-safe and efficient while avoiding garbage collection with a set of strict rules.
Rust enforces these rules through lifetimes. Lifetimes are effectively just names for scopes somewhere in the program.
In this session, we'll talk about how these concepts work, how to embrace it and the mental model of a Rust developer.

Isan Rivkin, a software engineer part of the protocol team at Enigma.
Enigma is an open-source project building a decentralized network in Rust for private computations.
Isan is working on the Networking, p2p, system architecture and Hardware (SGX).

* Re-Implementing Electrum Server by [Roman Zeyde](https://www.linkedin.com/in/roman-zeyde/) - [slides](https://docs.google.com/presentation/d/1gZbykgg3oPurBWHB-tJD2jnRzJ61XCiZjaxHqJ7z4ag/edit#slide=id.p)

(Up to date version of the slides can be found [here](https://github.com/romanz/electrs))

In this talk, we will discuss the architecture of the Electrum Bitcoin wallet, current blockchain indexing solutions and their limitations, and present an efficient re-implementation of the [Electrum](bit.ly/electrs) indexing and query server in Rust.

[Roman Zeyde](https://romanzey.de/) has over 14 years of software development experience, working on distributed storage systems, embedded devices, linux kernel development, digital signal processing and scientific computing.
Currently, Roman is working on open-source projects related to the latest breakthroughs in distributed systems and applied cryptography.

### 2018.09.02 - [Cursive, Production and N00bing](https://www.meetup.com/rust-tlv/events/253408497/)

Hosted by [Healthy.io](https://healthy.io/).

September's meetup will feature a great variety of experiences:

* N00bing Through Rust by [Yuval Adam](https://www.linkedin.com/in/yuvaladam/):

The Rust language no doubt has a steep learning curve.
In this talk I will share my experience of getting started with Rust in addition to some ideas that can be applied to making things easier for Rust beginners.

Yuval Adam is a full stack developer, freelance consultant and a Rustacean in the making

* Rusty Life by [Noam Tenne](https://www.linkedin.com/in/noam-tenne/):

Rust may be a server side language, but it doesn't have to cower in the shadows. In this session we'll talk about how we can prettify our Rust programs using a GUI built with Cursive.

Noam is a hacker-hearted symmathecist. He's been working on both cloud based and on-premise platforms, gaining much experience in building scalable, mission critical web applications and microservices. He's now wreaking havoc at Healthy.io

* Rust In Production by [Daniel Brodie](https://www.linkedin.com/in/danielbrodie/) CTO & Co-Founder at [Cynerio](https://www.cynerio.com/):

At a startup, moving from a small and badly developed prototype to an initial working prototype, is an important baby step that is difficult to get right. At Cynerio, we leverage Rust to allow us to deal with the complexity of handling 1-10 Gbps of traffic at the hospital's network, while parsing obscure and problematic protocols, running machine learning algorithms, and staying fault tolerant and secure. We will show how we did a gradual and iterative shift from the experimental Python code to production quality Rust and how we are using performance analytics to improve performance going forward. We will also share how our core developers, who haven't known rust, have handled the on-boarding process.

Daniel Brodie is the CTO & Co-Founder of Cynerio, a cyber security startup aimed at protecting hospital's most sensitive entities - their medical devices. Before that Daniel led the research team and a development team in Lacoon, til it was acquired by Checkpoint. Daniel has been a huge fan of hipster programming languages for many years and has fallen in love with rust 3 years ago.
