---
layout: default
title: How To Contribute
---

# Contributing to Rust

If you're learning Rust, you've no doubt been amazed at how awesome
the language is and will be keen to write more Rust code. Hopefully,
you'll also want to make a meaningful contribution to the community.
If you're not sure how best to do that, then hopefully this page will
help. Most of this page is filled with links to lists of issues to
help you find something to work on. There are issues on the Rust
project itself, a number of supporting projects, and some projects
which are large users of Rust.

Writing code isn't the only way to make a meaningful contribution.
Writing tests and documentation is how many people get started and is
really useful. There are links to these kind of issues too. Likewise
finding bugs and filing issues in any of the projects linked here is
very much appreciated. As a reminder, all contributors are expected to
follow our [Code of Conduct][coc].

[coc]: https://www.rust-lang.org/conduct.html

## Finding Something to Work On

#### Issues

If you're looking for [issues][rust_issues] on the Rust repo
and want something easy-ish to start with, look for the [E-easy][e_easy_issues]
or [E-mentor][e_mentor_issues] labels.

Contributors with sufficient permissions on the Rust repo can help by
adding labels to triage issues:

- **Yellow A-prefixed** labels state which area of the project an
  issue relates to.
- **Magenta B-prefixed** labels identify bugs which belong elsewhere.
- **Green E-prefixed** labels explain the level of experience
  necessary to fix the issue.
- **Red I-prefixed** labels indicate the importance of the issue. The
  I-nominated label indicates that an issue has been nominated for
prioritizing at the next triage meeting.
- **Orange P-prefixed** labels indicate a bug's priority. These labels
  are only assigned during triage meetings, and replace the
I-nominated label.
- **Blue T-prefixed** bugs denote which team the issue belongs to.
- **Dark blue beta-** labels track changes which need to be backported
  into the beta branches.
- **Purple** metabug labels mark lists of bugs collected by other
  categories.

[rust_issues]: https://github.com/rust-lang/rust/issues
[e_easy_issues]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AE-easy
[e_mentor_issues]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AE-easy+label%3AE-mentor

#### The Rust compiler

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

#### Standard Libraries

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

#### Cargo

[Cargo][cargo_issues] is Rust's package manager.

[cargo_issues]: https://github.com/rust-lang/cargo/issues

#### Tests

Contributing tests is extremely valuable to the Rust project. For the
compiler and standard libraries, there are unit tests (usually)
embedded in the source code and regression tests in the
[src/tests][tests] directory. There is a list of issues which have
(probably) been fixed, but still need a test. Nearly any other project
will be extremely welcoming of new tests too.  Writing test cases is a
great way to begin to understand a new project and get started
contributing.

[test]: https://github.com/rust-lang/rust/tree/master/src/test

#### Checking Older Bugs

Sometimes, an issue will stay open, even though the bug has been
fixed. And sometimes, the original bug may go stale because something
has changed in the meantime. It can be helpful to go through older bug
reports and make sure that they are still valid. Load up an older
issue, double check that it's still true, and leave a comment letting
us know if it is or is not. The [least recently updated][lru_issues]
sort is good for finding issues like this.  

[lru_issues]: https://github.com/rust-lang/rust/issues?q=is%3Aissue+is%3Aopen+sort%3Aupdated-asc

#### Documentation

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

#### Tools

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
