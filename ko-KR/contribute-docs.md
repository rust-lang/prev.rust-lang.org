---
layout: ko-KR/default
title: Rust에 기여하기 &mdash; 문서 &middot; Rust 프로그래밍 언어
---

# Rust에 기여하기 &mdash; 문서

Documentation is never good enough and there's never enough of it.
Many aspects of Rust's documentation don't require deep knowledge to
improve, and writing, reviewing, and editing documentation are great
ways to learn Rust. Furthermore, improvements to documentation are
easy to identify and limitless. Don't like the way something reads?
Discover some information that wasn't documented?  Your pull request
will be gleefully embraced.

***The most impactful documentation you can write is [for the crates
that make up the Rust ecosystem][crate_docs]***. While the in-tree
documentation is relatively complete, the same is not yet true for
[many of the popular crates and tools][awesome-rust] that Rust
programmers interact with every day. Contributing API documentation to
a popular Rust project will earn you the enduring love of its maintainer.

[The Book] is the primary documentation for Rust, maintained in the
main repository. It has its own issue label, [A-book] and
is continually being refined. Other documentation in the main
repository include [The Rust Reference], the [standard library API
documentation][std], [The Rustonomicon] (a guide to using `unsafe`
correctly). The [Rust Style Guidelines] are so incomplete they are not
linked prominently; an ambitious contributor can make much headway
there. The [error index][err] provides extended explanations of the
errors produced by the compiler. As new errors are added this
documentation [must be maintained][err-issue], so there always are
errors not reflected in the index to be added. Most in-tree
documentation lives in the [src/doc] directory. These are all covered by
the [A-docs] label on that issue tracker. Finally, this document
and other website materials are maintained in the [Rust website Git repository].
To contribute simply edit it and submit a pull request.

A great deal of important Rust documentation does not live in the main
repository, or is not maintained by the project, but is still
critically important to Rust's success. Examples of excellent Rust
documentation that is actively developed and in need of contributors
include [Rust By Example], [Rust Design Patterns], and [rust-rosetta].
For other existing documentation projects to contribute to see [rust-learning].

Meet other Rust documentarians in [#rust-docs].

<!--
TODO: blogging, translation
-->

[#rust-docs]: https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs
[A-book]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-book
[A-docs]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-docs
[Rust By Example]: https://github.com/rust-lang/rust-by-example
[Rust Design Patterns]: https://github.com/nrc/patterns
[Rust Style Guidelines]: https://doc.rust-lang.org/style/index.html
[The Book]: https://doc.rust-lang.org/book/index.html
[The Rust Reference]: https://doc.rust-lang.org/reference.html
[The Rustonomicon]: https://doc.rust-lang.org/nomicon/index.html
[awesome-rust]: https://github.com/kud1ing/awesome-rust
[crate_docs]: https://users.rust-lang.org/t/lets-talk-about-ecosystem-documentation/2791
[err-issue]: https://github.com/rust-lang/rust/issues/24407
[err]: https://doc.rust-lang.org/error-index.html
[rust-learning]: https://github.com/ctjhoa/rust-learning
[rust-rosetta]: https://github.com/Hoverbear/rust-rosetta
[src/doc]: https://github.com/rust-lang/rust/tree/master/src/doc
[std]: https://doc.rust-lang.org/std/index.html
[Rust website Git repository]: https://github.com/rust-lang/rust-www
