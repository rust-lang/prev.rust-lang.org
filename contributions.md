---
layout: default
title: How To Contribute
---

# Contributing to Rust

If you're learning Rust, you've no doubt been amazed at how awesome
the language is and will be keen to write more Rust code. Hopefully,
you'll also want to make a meaningful contribution to the community.
If you're not sure how best to do that, then this page will
help.

**Just want to report a bug in Rust?** [Follow the Rust bug reporting
guide][bugs]. Thanks in advance!

[bugs]: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

Rust is an expansive system of projects, some officially maintained
within the [rust-lang] organization on GitHub, but with many
increasingly-important efforts driven from without by its enthusiastic
community. Newcomers will be interested in [an overview of the
organization, processes, and policies of The Rust Project][dev_proc]
and the project's [CONTRIBUTING.md] file, which explains the specifics
of contributing to [rust-lang/rust].

There are many ways to contribute to the success of Rust: filing,
triaging and fixing bugs; writing documentation and tests; partipating
in the the design of the language and standard libraries through the
RFC process; improving compile-time and run-time performance;
spreading the tendrils of the Rust community ever-outward.

This guide focuses on a few avenues for the new contributor:

* [Bugs, triage, and maintenance](#bugs). Finding issues to fix,
  triaging, adding test cases.
* [Documentation](#doc). Not just official documentation, but also
  for crates, blog posts, and other unofficial sources.
* [Community building](#comm). Expanding the reach of Rust, and
  maintaining its excellence.
* [Tooling, IDEs, and infrastructure](#tool). The hard work of making
  Rust accessible to programmers of all kinds.
* [Language and compiler](#comp). Language design, feature
  implementation, performance improvement.
* [Libraries](#lib). Including the standard library, but also the
  equally-important unofficial crates that make Rust usable.

As a reminder, all contributors are expected to
follow our [Code of Conduct][coc].

[dev_proc]: dev_process.html
[rust-lang]: https://github.com/rust-lang
[rust-lang/rust]: https://github.com/rust-lang/rust
[CONTRIBUTING.md]: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md
[coc]: https://www.rust-lang.org/conduct.html

<a name="bugs"></a>
## Bugs, triage, and maintenance

The day-to-day maintenance of the project revolves around Rust's
[issue tracker] and [pull requests], and more help is always
needed.

The most basic way to get started contributing to Rust is to look for
the [E-easy][e_easy_issues] or [E-mentor][e_mentor_issues]
labels. These are meant to be approachable for new Rust programmers.

Rust is always in need of people to [triage] issues: reproduce bugs,
minimize test cases, apply labels.

Sometimes, an issue will stay open, even though the bug has been
fixed. And sometimes, the original bug may go stale because something
has changed in the meantime. It can be helpful to go through older bug
reports and make sure that they are still valid. Load up an older
issue, double check that it's still true, and leave a comment letting
us know if it is or is not. The [least recently updated][lru_issues]
sort is good for finding issues like this.  

While Rust has an [extensive test suite][test] there is always more to
test. The [E-needstest] label indicates issues that are thought to be
fixed but don't have tests. Writing test cases is a great way to begin
to understand a new project and get started contributing. For those
especially interested in testing, there are usually *entirely new
classes* of tests that need to be implemented, and asking #rust-internals
what needs to be tested can be fruitful.

Once you've found your way around the project and have created a few
pull requests in a particular area of the project, consider actively
reviewing others' pull requests: reviewership is a rare skill and good
reviewers are always appreciated.

[lru_issues]: https://github.com/rust-lang/rust/issues?q=is%3Aissue+is%3Aopen+sort%3Aupdated-asc
[issue tracker]: https://github.com/rust-lang/rust/issues
[pull requests]: https://github.com/rust-lang/rust/pulls
[e_easy_issues]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AE-easy
[e_mentor_issues]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AE-easy+label%3AE-mentor
[triage]: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#issue-triage
[E-needstest]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AE-needstest
[test]: https://github.com/rust-lang/rust-wiki-backup/blob/master/Note-testsuite.md

<a name="doc"></a>
## Documentation

Documentation is never good enough and there's never enough of it.
Writing docs is a really valuable way to contribute to open source
projects. Many aspects of the docs don't require super-deep knowledge,
especially if they're aimed at newcomers. It's also a great way to
learn more about the language or specific libraries.

- [The Book][rustbook_issues] The main guide to learning Rust.
- [Rust Documentation][rustdoc_issues]
- [Rust by Example][rust_by_example_issues]
- [Rust for C++ Programmers][rust_for_cpp_issues] A tutorial aimed
at experienced C++ programmers coming to Rust.

[rustbook_issues]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-book
[rustdoc_issues]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-docs
[rust_by_example_issues]: https://github.com/rust-lang/rust-by-example/issues
[rust_for_cpp_issues]: https://github.com/nrc/r4cppp/issues


<a name="comm"></a>
## Community building

<a name="tool"></a>
## Tooling, IDEs, and infrastructure

Tools play a huge part in the success of a language. Rust has some
great tool support, in particular with debugging and package
management, but we need much more.

- [cargo](https://github.com/rust-lang/cargo/issues) Rust's package manager and build system.
- [rustdoc](https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-rustdoc)
  Produces documentation for the official libraries and user projects.
- [racer](https://github.com/phildawes/racer) Code completion.
- [rustfmt](https://github.com/nrc/rustfmt) Code formatting.
- [multirust](https://github.com/brson/multirust/issues) For managing
  multiple installations of the Rust toolchain.
- [homu](https://github.com/barosl/homu/issues) Acts as a gatekeeper for commits.

<a name="libs"></a>
## Libraries

[Libstd][libstd], [libcollections][libcollections],
[liballoc][liballoc] and [libcore][libcore] are the main library
crates.  These all appear to users as if they are part of libstd, the
standard library. These tend to be very fundamental libraries -
built-in types, low level IO and concurrency, allocation, essential
collections, and so forth. You should join [#rust-libs][libs_irc] if
you are interested in contributing to the Rust libraries.

[libstd]: https://github.com/rust-lang/rust/tree/master/src/libstd
[libcollections]: https://github.com/rust-lang/rust/tree/master/src/libcollections
[liballoc]: https://github.com/rust-lang/rust/tree/master/src/liballoc
[libcore]: https://github.com/rust-lang/rust/tree/master/src/libcore
[libs_irc]: irc://moznet/rust-libs

<a name="comp"></a>
## Language and compiler

The compiler is part of the [main repo][main_repo], which also
includes the standard library crates and a whole bunch of supporting
code. For questions about the compiler, there is the
[#rustc][rustc_irc] IRC channel. Compiler errors (ICE for 'internal
compiler errors') can be searched for in issues using the
[I-ICE][i_ice_issues] label. These are usually good bugs to start with
because it's easy to know when you've fixed them, and they're often
relatively self-contained. If you're interested in parsing, macros,
syntactic stuff, the [parsing][parsing_issues] label and the
[macro][macro_issues] label are a good places to start.

[main_repo]: https://github.com/rust-lang
[rust_irc]: irc://moznet/rustc
[i_ice_issues]: https://github.com/rust-lang/rust/labels/I-ICE
[parsing_issues]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-parser
[macro_issues]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-parser+label%3AA-macros

#### Other Contributions

Try [Github Trending][trending] for currently active Rust projects.
There are a number of other ways to contribute to Rust that don't deal
directly with the Rust repository.

- Answer questions in [#rust][rust_irc], on the [Rust Forum][forum] or
  on [Stack Overflow][stack_overflow].
- Participate in the [RFC process][rfcs].
- Find a [requested community library][requested], build it, and
  publish it to [Crates.io][crates]. Easier said than done,
but very, very valuable!

[trending]: https://github.com/trending?l=rust
[rust_irc]: irc://moznet/rust
[forum]: https://users.rust-lang.org/
[stack_overflow]: http://stackoverflow.com/questions/tagged/rust
[rfcs]: https://github.com/rust-lang/rfcs
[requested]: https://github.com/rust-lang/rfcs/labels/A-community-library
[crates]: http://crates.io
