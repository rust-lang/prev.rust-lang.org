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
follow our [Code of Conduct](https://www.rust-lang.org/conduct.html).

## Finding Something to Work On

#### Issues

If you're looking for
[issues](https://github.com/rust-lang/rust/issues) on the Rust repo
and want something easy-ish to start with, look for the
[E-easy](https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AE-easy)
or
[E-mentor](https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AE-easy+label%3AE-mentor)
labels.

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

#### The Rust compiler

The compiler is part of the [main repo](https://github.com/rust-lang),
which also includes the standard library crates and a whole bunch of
supporting code. For questions about the compiler, there is the #rustc
irc channel. Compiler errors (ICE for 'internal compiler errors') can
be searched for in issues using the
[I-ICE](https://github.com/rust-lang/rust/labels/I-ICE) label. These
are usually good bugs to start with because it's easy to know when
you've fixed them, and they're often relatively self-contained. If
you're interested in parsing, macros, syntactic stuff, the
[parsing](https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-parser)
label and the
[macro](https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-parser+label%3AA-macros)
label are a good places to start.

#### Standard Libraries

[Libstd](https://github.com/rust-lang/rust/tree/master/src/libstd),
[libcollections](https://github.com/rust-lang/rust/tree/master/src/libcollections),
[liballoc](https://github.com/rust-lang/rust/tree/master/src/liballoc)
and
[libcore](https://github.com/rust-lang/rust/tree/master/src/libcore)
are the main library crates.  These all appear to users as if they are
part of libstd, the standard library. These tend to be very
fundamental libraries - built-in types, low level io and concurrency,
allocation, essential collections, and so forth. You should join
\#rust-libs if you are interested in contributing to the Rust
libraries.

#### Cargo

Cargo is Rust's package manager. Cargo [issues](https://github.com/rust-lang/cargo/issues).

#### Tests

Contributing tests is extremely valuable to the Rust project. For the
compiler and standard libraries, there are unit tests (usually)
embedded in the source code and regression tests in the
[src/tests](https://github.com/rust-lang/rust/tree/master/src/test)
directory. There is a list of issues which have (probably) been fixed,
but still need a test. Nearly any other project will be extremely
welcoming of new tests too.  Writing test cases is a great way to
begin to understand a new project and get started contributing.

#### Checking Older Bugs

Sometimes, an issue will stay open, even though the bug has been
fixed. And sometimes, the original bug may go stale because something
has changed in the meantime. It can be helpful to go through older bug
reports and make sure that they are still valid. Load up an older
issue, double check that it's still true, and leave a comment letting
us know if it is or is not. The [least recently
updated](https://github.com/rust-lang/rust/issues?q=is%3Aissue+is%3Aopen+sort%3Aupdated-asc)
sort is good for finding issues like this.

#### Documentation

Documentation is never good enough and there's never enough of it.
Writing docs is a really valuable way to contribute to open source
projects. Many aspects of the docs don't require super-deep knowledge,
especially if they're aimed at newcomers. It's also a great way to
learn more about the language or specific libraries.

- [The
  Book](https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-book)
The main guide to learning Rust.
- [Rust
  Documentation](https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-docs)
- [Rust by Example](https://github.com/rust-lang/rust-by-example/issues)
- [Rust for C++
  Programmers](https://github.com/nrc/r4cppp/issues) A tutorial aimed
at experienced C++ programmers coming to Rust.

#### Tools

Tools play a huge part in the success of a language. Rust has some
great tool support, in particular with debugging and package
management, but we need much more.

- [Cargo](https://github.com/rust-lang/cargo/issues) Rust's package manager and build system.
- [Rustdoc](https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-rustdoc)
  Produces documentation for the official libraries and user projects.
- [Racer](https://github.com/phildawes/racer) Code completion.
- [rustfmt](https://github.com/nrc/rustfmt) Code formatting.

#### Other Contributions

Try [Github Trending](https://github.com/trending?l=rust) for
currently active Rust projects. There are a number of other ways to
contribute to Rust that don't deal directly with the Rust repository.

- Answer questions in
[#rust](http://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust),
on the [Rust Forum](https://users.rust-lang.org/) or on [Stack
Overflow](http://stackoverflow.com/questions/tagged/rust).
- Participate in the [RFC process](https://github.com/rust-lang/rfcs).
- Find a [requested community
library](https://github.com/rust-lang/rfcs/labels/A-community-library)
, build it, and publish it to [Crates.io](http://crates.io). Easier
said than done, but very, very valuable!
