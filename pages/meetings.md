---
title: Meetings
timestamp: 2023-11-04T17:30:02
published: true
description:
---

* The [Rust in Israel](https://www.meetup.com/rust-in-israel/) group has both in-person and virtual events.
* The [Rust TLV](https://www.meetup.com/rust-tlv/) is now inactive.
* The [Code Mavens](https://www.meetup.com/code-mavens/) hosts virtual events in English about Rust, Python, and Perl.
* The [Virtual events](https://events.code-maven.com/) site lists Virtual Rust events around the world with a possibility to filter by time and language.

## Upcoming events

## Future Meetings

We are looking for speakers for our meetings. If you are interested to give a presentation, please [open an issue describing it](https://github.com/szabgab/rust.org.il/)
or talk to [Gabor Szabo](https://szabgab.com/contact). Presentation can be at any level and any topic related to Rust.

If you are interested hosting an in-person event or sponsoring one, please contact  [Gabor Szabo](https://szabgab.com/contact).

## Past Events


### 2024.09.17 [Embedded Summer Camp Session #3 - Rust in the Linux Kernel](https://www.meetup.com/abra-rnd-solutions/events/300733258/) (In-person, Hebrew).

Topics We Will Discuss:

* Explore the pioneering integration of Rust into the Linux kernel, poised to transform kernel programming through improved safety and efficiency.
* Get hands-on guidance on setting up, developing, and debugging a simple Rust-based kernel module.
* Learn about the potential impacts and improvements Rust brings to kernel programming, highlighting synergies and differences from traditional C/C++ modules.

When: Tuesday, Sep 17th, 2024, 17:00-20:00

### 2024.09.09 [Rust at Scale](https://coralogix.com/rust-coralogix-meetup/) (In-person, Hebrew)

Host: [Coralogix](https://coralogix.com/)
* Time: 17:30-20:00

At the event, you'll learn about:

* Memory Allocators: Discover the variety of memory allocators available in Rust beyond the default; understand why tuning them is crucial even in a language without garbage collection.
* DRY Principles: Find out how using Cargo Workspaces in Rust can simplify dependency management and reduce production issues.
* Efficient Querying: See how Coralogix efficiently queries metrics in sub-seconds on object storage by decoupling storage from compute.

#### Agenda:

* 17:30-18:00 Gathering and yummy food

Please let us know in advance if you have any special dietary requirements (e.g. Vegan, Kosher, etc.). We will do our best to accommodate.

* 18:00-18:15 The Coralogix Journey by [Ariel Assaraf](https://www.linkedin.com/in/ariel-assaraf-ab621896/), CEO

Learn about the incredible journey of Coralogix in the observability space from the CEO himself.

* 18:15-18:35 Simplifying Dependency Management with Workspaces by [Roy Prager](https://www.linkedin.com/in/roy-prager-40656a119/)

Join us as we share our experience of simplifying dependency management by adopting Cargo Workspaces in Rust. See how this approach helped us to reduce overhead and prevent production issues across our services.

* 18:35-19:10 Exploring Memory Allocators by Ronen Cohen

Discover the variety of memory allocators available in Rust beyond the default, and learn why tuning them is crucial even in a language without garbage collection.

* 19:25-20:00 Querying Metrics in Sub-Seconds on Object Storage by [Oron Sharabi](https://www.linkedin.com/in/oron-sharabi-27615b26/)

When designing our metrics product at Coralogix, we decided to decouple storage from compute. Learn about how this allows our customers to store their metrics data cost-effectively in their own S3 buckets.



### 2024.08.29 [Rust Source Code Reading: The thousands crate](https://www.meetup.com/code-mavens/events/302391142/) (Virtual, English)

The [thousands](https://crates.io/crates/thousands) crate is a small crate that will help "commafy" a number. That is, put commas between triplets of digits to make the number more readable. So it will convert 1234567 to 1,234,567.

During this even we'll go over the implementation of this crate.

* The event will be via Zoom (link will be announced close to the event)
* Language: English.
* Lead: [Gabor Szabo](https://szabgab.com/)
* Requirements: Basic familiarity with writing Rust will be enough.
* Length: up to 2 hours.


### 2024.08.28 [Command Line Tools: Implementing wc in Rust](https://www.meetup.com/code-mavens/events/302151487/) (Virtual, English)

During this presentation we'll re-implement the wc (word count) command line tool in Rust. We could call it "from 0 to full implementation" or "From 00 to WC" as you like.

The event will be via Zoom (link will be announced close to the event)
* Language: English.
* Lead: [Gabor Szabo](https://szabgab.com/)
* Requirements: Basic familiarity with writing Rust will be enough.
* Length: up to 2 hours.



### 2024.08.27 [Declarative macros in Rust](https://www.meetup.com/rust-in-israel/events/302327956/) (Virtual, Hebrew)

I am sure you have already used many macros, but have you written any?

In this presentation you will learn how to write declarative macros. You will also see many examples from both the standard library and 3rd party Crates to get ideas for when declarative macros might help.

* Language: Hebrew
* Organizer and Speaker: [Gabor Szabo](https://szabgab.com/)
* Location: This is a virtual event. We'll publish the Zoom link a few hours before the event.



### 2024.08.06 [Web development in Rust using Rocket - part 2](https://www.meetup.com/code-mavens/events/301736709/) (English)

This is the second part of the Web development with Rocket series. It is strongly recommended that you watch the first part and even play around a bit with the framework before attending this meeting.

The presentation is going to be about 1 hour long and then we open the floor for discussions about Rocket, web development and Rust in general.

* The meeting will be via Zoom
* Language: English.
* Workshop lead: [Gabor Szabo](https://szabgab.com/)
* Requirements: Basic familiarity with writing Rust will be enough.
* Length: up to 2 hours.

* [notes and video](https://rust.code-maven.com/web-development-in-rust-using-rocket-building-a-job-board)

### 2024.07.28 [Implementing a Neural Network in Rust (Virtual)](https://www.meetup.com/rust-in-israel/events/302271449/)

This the extended (~ 1 hour) version of the presentation Yoray gave at our in-person meeting last month.

* [Yoray Herzberg](https://www.linkedin.com/in/yoray-herzberg-b8155621b/): Implementing a Neural Network in Rust

Neural Networks are one of the most popular models in Machine Learning, and are the basis for a lot of popular AI applications, such as Computer Vision (CV), Natural Language Processing (NLP), and more. In this presentation, I show the math behind Neural Nets, how to train them, and how I implemented one in Rust from scratch to recognize digits in images.
The presentation is based on a post Yoray wrote: [Digit Recognition with Rust and WASM](https://vaktibabat.github.io/posts/Rust_WASM_Digit_Recognition_1/).
About Yoray: I am a third-year CS student at the Open University. Other than that, I am very interested in cybersecurity, reverse engineering, Malware Analysis, AI, and Rust (for about 6 months). I run a blog [here](https://vaktibabat.github.io/).

Language: Hebrew

Organizer: [Gabor Szabo](https://szabgab.com/)
Location: This is a virtual event. We'll publish the Zoom link a few hours before the event.
For more communication channels check our the [Rust in Israel](https://rust.org.il/) web site


32 people registered to this event.

* [video](https://www.youtube.com/watch?v=QlBbr_srFTQ)


### 2024.07.19 [Threads in Rust](https://www.meetup.com/rust-in-israel/events/302219468/)

In this presentation we will learn about threads in Rust.

There presentation will take about 1 hour, maybe 1 and a half hour.
After the presentation you'll have the opportunity to practice with the help of Gabor or just discuss any related topics.

* Speaker: [Gabor Szabo](https://szabgab.com/)
* Language: Hebrew
* Location: Zoom

20 people registered to this event.

* [video](https://youtu.be/edhM1vz536A)

### 2024.07.12 [Getting started with Rust](https://www.meetup.com/rust-in-israel/events/301872689/)

In this presentation we get start programming in Rust.
We assume no knowledge of Rust, we start from zero and we'll see where we can get to.

There presentation will take about 1 hour, maybe 1 and a half hour.
After the presentation you'll have the opportunity to practice with the help of Gabor or just discuss any related topics.

* Speaker: [Gabor Szabo](https://szabgab.com/)
* Language: Hebrew
* Location: Zoom

33 people registered to this event.

* [video](https://youtu.be/lbp2E-igAC8)

### 2024.07.11 [Reading JSON files in Rust](https://www.meetup.com/code-mavens/events/301636580/) (English)

During this meeting we will go through a number of sample JSON files and we'll learn how to read them and convert them to data structures in Rust.

* The event will be via Zoom
* Language: English.
* Lead: [Gabor Szabo](https://szabgab.com/)
* Requirements: Basic familiarity with writing Rust will be enough.
* Length: up to 2 hours.

* [notes and video](https://rust.code-maven.com/reading-json-files-in-rust)

### 2024.06.27 [Accepting parameters on the command line using Rust Clap](https://www.meetup.com/code-mavens/events/301506015/) (English)

When you start writing a command line application you usually start out with a very limited number of parameters. Maybe just 2 values. In that case using the `std::env::args` is enough.

However as you start adding more and more parameters that some of them depend on each other then very quickly you can get in trouble.

The clap crate is an excellent solution for such cases.

In this workshop you'll learn quite a few thing using the Clap crate.

If you have any specific use-case in mind, please comment on this event so I might be able to prepare such an example as well.

The workshop includes presentations and hands-on work.

* The workshop will be via Zoom
* Language: English.
* Workshop lead: [Gabor Szabo](https://szabgab.com/)
* Requirements: Basic familiarity with writing Rust will be enough.
* Length: up to 2 hours.

* [notes and video](https://rust.code-maven.com/accepting-parameters-on-the-command-line-using-rust-clap)

### 2024.06.25 [Using the Liquid template system in Rust](https://www.meetup.com/code-mavens/events/301487547/)

When you need to create many well-designed reports with different data, or when you create web pages where the content changes from one-page to another or from one visitor to another, it is common to use a template system.

The template systems are not very complex, but many people, when they start writing a project feel that they only have very simple need and thus the use of a "real" template system is not warranted.

So they create their own simple template system.

Then as the project growth their needs get more and more complex and they end up with a full-fledged template system.

It is so common that creating your own template system seems like a rite of passage in many circles.

In this workshop you'll learn about one of the successful template systems called Liquid. It was originally written at Shopify in Ruby, but it has an implementation in Rust as well.

We will learn how to use and extend the Rust version.

The workshop includes a presentation which is about 1 hour long and then you can practice what we learned while I answer questions and help if there is any need.

* The workshop will be via Zoom
* Language: English.
* Workshop lead: [Gabor Szabo](https://szabgab.com/)
* Requirements: Basic familiarity with writing Rust will be enough.
* Length: up to 3 hours.

* [notes and video](https://rust.code-maven.com/using-the-liquid-template-system-in-rust)

### 2024.06.23 - [Rust at Microsoft, Tel Aviv in June 2024](https://www.meetup.com/rust-in-israel/events/301670916/) at Microsoft in Tel Aviv

* When 18:00
* Where: Microsoft, Tel Aviv
* Language: Hebrew
* Organizer: [Gabor Szabo](https://szabgab.com/)
* Presentations start at 18:30
* [Yoray Herzberg](https://www.linkedin.com/in/yoray-herzberg-b8155621b/): Implementing a Neural Network in Rust (20 min)
* [Ron Zigelman](https://www.linkedin.com/in/ron-zigelman/): Microservices, Message Brokers, and Protobuf with Rust (20 min)
* Pub
* The host requires us to provide the names of attendees at least 24 before the event.

----

* [Yoray Herzberg](https://www.linkedin.com/in/yoray-herzberg-b8155621b/): Implementing a Neural Network in Rust (20 min)

Neural Networks are one of the most popular models in Machine Learning, and are the basis for a lot of popular AI applications, such as Computer Vision (CV), Natural Language Processing (NLP), and more. In this presentation, I show the math behind Neural Nets, how to train them, and how I implemented one in Rust from scratch to recognize digits in images.

The presentation is based on a post Yoray wrote: [Digit Recognition with Rust and WASM](https://vaktibabat.github.io/posts/Rust_WASM_Digit_Recognition_1/).

* [Slides Neural Networks in Rust](https://docs.google.com/presentation/d/1j_wkKKtGZoInpWYDW2IKaOJxqEXneqGCLDSHDDGAfeM)
* [Neural Networks in Rust GitHub repository](https://github.com/vaktibabat/rust-neural-nets)

About Yoray: I am a third-year CS student at the Open University. Other than that, I am very interested in cybersecurity, reverse engineering, Malware Analysis, AI, and Rust (for about 6 months). I run a blog [here](https://vaktibabat.github.io/).

* [Ron Zigelman](https://www.linkedin.com/in/ron-zigelman/): Microservices, Message Brokers, and Protobuf with Rust (20 min)

How to use NATS and Protobuf with Rust to integrate with Microservices

In this presentation, we'll explore how to use Protobuf and NATS to create a distributed system with microservices. We'll cover integrating with existing services to leverage them for faster development and reduced errors, demonstrating the benefits of using these technologies for scalable and efficient communication within a microservices architecture.

[GitHub of Ron Zigelman](https://github.com/r-zig)

[slides of Modern Software Architecture with Rust](https://shorturl.at/XW1yu)

About Ron: I am a Software Engineer, Architect, and Independent Software Consultant. I have extensive experience developing in .NET and, in recent years, have focused on building distributed systems using Rust. I am particularly interested in P2P and decentralized networks.

* If time permits: [Gabor Szabo](https://szabgab.com/): Contribution to an Open Source Rust crate, and update on the [Rust Digger](https://rust-digger.code-maven.com/) project.


56 people registered to this event.



### 2024.06.16 [Web development in Rust using Rocket](https://www.meetup.com/code-mavens/events/301294669/) (English)

In this workshop we will create a web site using the [Rocket framework](https://rocket.rs/).
We don't assume any background in web development. We'll use minimal HTML, we'll try to focus on getting started with the Rocket framework

The workshop includes presentations and hands-on work.

* The workshop will be via Zoom
* Language: English.
* Workshop lead: [Gabor Szabo](https://szabgab.com/)
* Requirements: Basic familiarity with writing Rust will be enough.
* Length: up to 3 hours.

* [notes and video](https://rust.code-maven.com/web-development-in-rust-using-rocket)

### 2024.06.09 [GitHub pages for Rust developers](https://www.meetup.com/code-mavens/events/301215326/) (English)

In this workshop you will create a web site on GitHub while using Rust. During the workshop I will provide explanation on how to setup Github pages first with plain Markdown and then using Rust. You will then have the opportunity to create your own web site and get help from me with any of the issues you encounter.

The workshop includes presentations and hands-on work.

* The workshop will be via Zoom
* Language: English.
* Workshop lead: [Gabor Szabo](https://szabgab.com/)
* Requirements: Basic familiarity with writing Rust will be enough.
* Length: up to 3 hours.

* [notes and video](https://rust.code-maven.com/github-pages-for-rust-developers)

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

* [notes and video](https://rust.code-maven.com/your-first-contribution-to-an-open-source-rust-project)

### 2024.05.23 - [Web development in Rust using Rocket](https://www.meetup.com/code-mavens/events/300974367/) (virtual in Hebrew)

In the most recent in-person meeting I talked about the [Rocket web framework](https://rocket.rs/). I am going to give an updated and extended version of the presentation, this time in Zoom.

This will give the opportunity to everyone who could not attend the in-person meeting to hear the presentation. It will be also a good opportunity to ask questions.

* When 18:00
* Speaker and organizer: [Gabor Szabo](https://szabgab.com/)
* Language: Hebrew
* Location: Zoom
* Length: 1 hour, maybe a bit longer.


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

### 2023.09.06 - [RustTLV @ Final - September Edition](https://www.meetup.com/rust-tlv/events/295441355/)

Hosted and sponsored by [Final](https://www.final.co.il/)

* [Gabor Szabo](https://www.linkedin.com/in/szabgab/) - What I learned from learning Rust
The journey of a veteran programmer from 0 to the first published Crate and beyond.
[video](https://www.youtube.com/watch?v=HRhAkSdvGx4)

[Gabor Szabo](https://szabgab.com/) has been programming for almost 40 years and has been teaching programming for more than 20. He taught a lot of people how to use Perl and then a lot more people how to use Python. His hope is that soon he will be teaching people how to write Rust.

* [Efi Merdler Kravitz](https://www.linkedin.com/in/efime/) - Rustifying Serverless
[video](https://www.youtube.com/watch?v=Luk-nXPaC88)

Rust, known for its performance and security, offers significant advantages when used with AWS Lambda. This session delves into deploying Rust functions using AWS SAM and cargo-lambda, enabling streamlined development right from one's own computer. Attendees will learn strategies to expedite local builds for Lambda deployments, with a spotlight on tools like mold and sccache. Dive into how Rust can enhance Lambda functions, from developing Lambda extensions to seamlessly integrating Rust into existing Python and Node.js Lambda code without necessitating a complete overhaul. Also, gain insights on methods for reducing binary sizes using Rust.

Efi is the Head of engineering @ [Cloudex](https://www.cloudex.pro/), AWS Serverless Hero amd author of [Learning Serverless in Hebrew](https://headstart.co.il/project/70568).

### 2023.02.20 - [Februrary Edition - Redis and BioCatch talking Rust!](https://www.meetup.com/rust-tlv/events/291182881/)

RustTLV is back to celebrate a new year of events, this time hosted by [BioCatch](https://www.biocatch.com/) with Redis! We'll have great speakers from both companies, on a varied list of topics.
We're trying a somewhat different format this time, with one conventional talk, and a bunch of lightning talks, please let us know what do you think!

* Serde tips and tricks - [Ben Avrahami](https://www.linkedin.com/in/ben-avrahami-b4855a1a7/)

#### Lightning Talks!

* The Three R's - Redis, Raft & Rust- [Uri Shachar](https://www.linkedin.com/in/uri-shachar-852b29/) Senior Director at [Redis](https://redis.com/)
* Macros in Rust: Unleashing the Power of Metaprogramming- [Yael Tzirulnikov Ruthenberg](https://www.linkedin.com/in/yael-ruthenberg-04bb18148/) Software Engineer at [Microsoft](https://www.microsoft.com/)
* How to use Rust Const generics- [Gil Dafnai](https://www.linkedin.com/in/gil-dafnai/)
* FFI - how C and Rust became BFFs- [Sharon Rosenfeld](https://www.linkedin.com/in/sharon-rosenfeld-972426b/)
* [slides of the lightning talks](/slides/redis-lightning-talks-2023-02-20.pdf)

### 2022.12.29 - [December Edition - xtask, macros and low level features](https://www.meetup.com/rust-tlv/events/290156141/)

Just before the new year celebrations, Rust TLV is back at it with another event, this time hosted by [Checkpoint](https://www.checkpoint.com/) and [Spectral](https://spectralops.io/)! We have multiple great technical talks this time, including tooling, language features and great debug stories.

* Using xtask & xtaskops by [Dotan Nahum](https://www.linkedin.com/in/jondot/)
Dotan is the CEO and Co-Founder @ Spectral, a Check Point company focused on developer-first security.

* Procedural Macros Intro + Tips by [Dan Aloni](https://www.linkedin.com/in/aloni/)
Developing Procedural Macros is a significant resource in the arsenal of advanced Rust programming. I'll talk about procedural macros and share some methods and tips about how to develop them more easily.

Dan Aloni (@DanAloni) is a systems software developer who specializes in Linux kernel, storage systems, compilation, virtualization, open source software, packaging, and build systems. He is the author of the [coLinux project](https://en.wikipedia.org/wiki/Cooperative_Linux) and has contributed code to various open-source projects, including the Rust compiler and the Linux kernel. Dan is currently an independent contractor.

* Going Low with Rust by [Aviram Hassan](https://www.linkedin.com/in/aviram-hassan/)
Technical walkthrough of implementing low level functionality using Rust - hooking Go functions, writing assembly, unsafe functions, naked functions, full-clothed functions
Aviram is the CEO and Co-Founder of MetalBear, building open source tools for backend developers.

### 2022.09.05 - [September Edition](https://www.meetup.com/rust-tlv/events/287754162/)

After a super successful post-COVID meetup, Rust TLV is back at it with another event strategically scheduled between the end of summer vacation and right before holiday season kicks in.

[Healthy.io](https://healthy.io/) will (once again) be hosting us in their shiny new offices on the 31st floor right in the middle of Tel Aviv, and Final will be graciously taking care of all the logistics, including snacks and drinks to make it a great event.

* Rust @ Final, Doing the Math Safely by [Rotem Yaari](https://www.linkedin.com/in/rotemyaari/)
* Enum Variants by [Yoni Peleg](https://www.linkedin.com/in/yoni-peleg/)
* Arrow DataFusion by [Boaz Berman](https://www.linkedin.com/in/boaz-berman/)

### 2022.07.04 - [Rust TLV - Rust Interop, Rewrites and fun](https://www.meetup.com/rust-tlv/events/286610368/)

After a looong COVID hiatus, Rust TLV is back with three exciting talks hosted by [Pinecone](https://www.pinecone.io/) at WeWork's London Ministore location!

Location: London Ministore - Shaul HaMelech 4/Ibn Gabirol 30.
WeWork (2nd floor), take the internal elevators up (by the Shaul HaMelech entrance).

* Microdosing Rust to your organization by [Aviram Hassan](https://www.linkedin.com/in/aviram-hassan/), CEO and Co-Founder @ MetalBear
You can start using Rust in your organization without refactoring or rewriting the whole system by leveraging extensions of widely used languages. napi for Node and PyO3 for Python.

* Idiomatic Rust by [Dotan Nahum](https://www.linkedin.com/in/jondot/), CEO and Co-Founder @ Spectral, a Check Point company and a Rust foundation member

* Rewriting high performance system in Rust by [Roei Mutay](https://www.linkedin.com/in/roei-mutay-bb3743200/), Engineering Manager @ Pinecone
How and why we abandoned Python and C++ and rewrote our database core in Rust. We’ll explore the challenges and struggles we had along the way,
present the unique tools in Rust ecosystem that helped us in the process, and if time allows - will dive deep into some of the technical pitfalls and lessons we learned.

### 2019.03.11 - [Redis modules and a production story](https://www.meetup.com/rust-tlv/events/259099506/)

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

### 2018.12.09 - [Intro for C\C++ Developers, SGX and Cargo](https://www.meetup.com/rust-tlv/events/256141472/)

* A feature-wise introduction for C/C++ programmers, part 1/3 by [Dan Aloni](https://www.linkedin.com/in/aloni/):

This talk will focuse on Move semantics and Pattern matching, explaining their importance and the differences from the decades-old C / C++ arena.

[Dan Aloni](https://blog.aloni.org/) develops software for more than 20 years. Dan is the original author of coLinux (wikipedia.org/wiki/Cooperative_Linux), has worked on the Linux kernel, gained specialty in virtualization, storage systems, niche languages and operating systems. Currently, Dan is an independent consultant, and helps maintaining the Rust plugin for Vim.

* SGX by [Isan Rivkin](https://www.linkedin.com/in/isan-rivkin/):
Intel SGX protects selected code and data from disclosure or modification. Developers can partition their application into processor-hardened enclaves or protected areas of execution in memory that increase security even on compromised platforms. The talk will focus on understanding SGX and how it can be used with the Rust-SDK that was released not so long ago.

Isan Rivkin, a software engineer part of the protocol team at Enigma.
Enigma is an open-source project building a decentralized network in Rust for private computations.
Isan is working on the Networking, p2p, system architecture and Hardware (SGX).

* A deeper look at Cargo by [Anton Weiss](https://www.linkedin.com/in/antonweiss/):
Cargo is arguably one of the best things about Rust. Not every new language comes with a mature build and package manager out of the box. Let's take a deeper look at this great tool and peek into some of its less known corners.

Anton Weiss is the Principal Consultant at Otomato Software. Internationally acclaimed speaker and trainer in the fields of software delivery and collaboration. Became interested in Rust for hobby projects about half a year ago.

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
