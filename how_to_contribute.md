---
layout: default
title: How To Contribute
---

Contributing to Rust
===

If you're learning Rust, you've no doubt been amazed at how awesome the language is and will be keen to write more Rust code. Hopefully, you'll also want to make a meaningful contribution to the community. If you're not sure how best to do that, then I hope that this page will help.

I'll link to a few resources that make contributing to the Rust project easier. Most of this page is filled with links to lists of issues to help you find something to work on. There are issues on the Rust project itself, a number of supporting projects, and some projects which are large users of Rust.

Writing code isn't the only way to make a meaningful contribution. Writing tests and documentation is how many people get started and is really useful. There are links to these kind of issues too. Likewise finding bugs and filing issues in any of the projects linked here is very much appreciated.

Rust and all the projects listed here use Git and GitHub for version control and to manage contributing. IF you're new to Git and/or GitHub, I wrote a blog post on my workflow which might be useful.
Getting started

If you're interested in the Rust project itself, you should first read contributing.md. It contains loads of useful info about the process of contributing. These slides by kmc (a Rust and Servo engineer at Mozilla), given at a Rust meetup, are a great intro to contributing to Rust. The old wiki has some useful information for contributors, but is a bit hard to wade through. Hopefully it'll get a new home soon.

If you need help with Rust itself, the docs and book are the best place to start.

Rustaceans primarily communicate via irc. If you haven't already, you should join #rust on moznet. For questions about how Rust works, anything about contribution, or to find a mentor, #rust-internals is the right channel. There are also some more specific irc channels noted below. There is a rust-internals discuss instance for discussing the inner workings of Rust.

If you're looking for issues on the Rust (or Servo) repos and want something easy-ish to start with, look for the E-easy or E-mentor labels. If no one is specifically mentioned as a mentor, try pinging the author or assignee on irc to see if they can mentor or recommend someone.
Finding something to work on
rustc

The Rust compiler. The compiler is part of the main repo, which also includes the standard library crates and a whole bunch of supporting code. For questions about the compiler, there is the #rustc irc channel.

    All issues There are a lot of tags in the Rust repo. Finding compiler specific issues is a bit hit and miss. You could look particularly for compiler errors (ICEs); these are usually good bugs to start with because it's easy to know when you've fixed them, and they're often relatively self-contained.
    Parsing/macros If you're interested in parsing/frontend/macros/syntacic stuff, these lists are a good place to start
    Backend/codegen Some knowledge of LLVM is helpful here.

libraries

    In tree Libstd, libcollections, liballoc, and libcore are the main in-tree library crates. These all appear to users as if they are part of libstd, the standard library. These tend to be very fundamental libraries - built-in types, low level io and concurrency, allocation, essential collections, and so forth.
    Out of tree The 'official' libraries which are not part of the standard library are in various repos under the rust-lang GitHub organisation. There is everything from random numbers to regular expressions. There are also numerous other libraries which can be found on crates.io which might be interesting to contribute to.

You should join #rust-libs if you are interested in contributing to the Rust libraries.
tests

Contributing tests is extremely valuable to the Rust project. For the compiler and standard libraries, there are unit tests (usually) embedded in the source code and regression tests in the src/tests directory. There is a list of issues which have (probably) been fixed, but still need a test.

Nearly any other project will be extremely welcoming of new tests too. Writing test cases is a great way to begin to understand a new project and get started contributing.
docs

Documentation is never good enough and there's never enough of it. Writing docs is a really valuable way to contribute to open source projects. Many aspects of the docs don't require super-deep knowledge, especially if they're aimed at newcomers. It's also a great way to learn more about the language or specific libraries.

    The book The main guide to learning Rust.
    Documentation Better documentation throughout the Rust project - mostly for libraries.
    Rust by example an alternate tutorial for learning Rust, based around learning by example. Initially a community project, now official.
    Rust for C++ programmers A tutorial aimed at experienced C++ programmers coming to Rust.

tools

Tools play a huge part in the success of a language. Rust has some great tool support (in particular with debugging and package management), but we need much more.

    Cargo Rust's package manager and build system (#cargo is an irc channel dedicated to Cargo).
    Rustdoc Produces documentation for the official libraries and user projects.
    Debugging Rust support in GDB and LLDB. Mostly compiler issues.
    DXR Source code exploration and cross-referencing.
    Racer Code completion.
    rustfmt Code formatting.

infrastructure

These projects support the Rust project. They're mostly not written in Rust, but if you have experience with GitHub, Python, or other such technologies you can make really helpful impact here.

    Homu (bors) Continuous integration and queue management for the Rust project.
    Highfive Generally useful bot.
    Rustaceans.org Rust community phonebook.
    Rustbook Used for making the book/guide.

other projects

If you want more ideas for open source Rust projects to contribute to, these are all excellent projects and very welcoming to new contributors.

    Servo Parallel browser engine from Mozilla Research.
    Piston A game engine in Rust.
    Iron Web development framework.
    Hyper HTTP library.
    MIO IO library.
