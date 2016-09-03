---
layout: ko-KR/default
title: Rust에 기여하기 &mdash; 도구, IDE 및 인프라 &middot; Rust 프로그래밍 언어
---

# Rust에 기여하기 &mdash; 도구, IDE 및 인프라

Tools play a huge part in the success of a language, and there is a
great deal left to implement. ***A major focus of Rust development now
is [improving the IDE experience][ides]***. This involves work
throughout the Rust stack, from the compiler itself through your
favorite IDE. Follow the link for more information.

Both Cargo, the Rust package manager, and rustdoc,
the Rust documentation generator, while full-featured and functional,
suffer from a lack of developers. Rustdoc has many open issues, under
the main repository's [A-rustdoc] label. They are mostly bugs and
contributing is a matter of fixing the bug and submitting a pull
request. Cargo has [its own repository and issues][Cargo], and those
interested in contributing might want to introduce themselves in
[#cargo].

Although Rust can be run under both the gdb and lldb debuggers with
limited success, there are still many cases where debugging does not
work as expected.  The [A-debuginfo] issue tracks these.

For ideas for more tooling projects to contribute to see
[awesome-rust].

There are often other tooling projects of interest just waiting for
the right people to come along and implement them. Discuss with other
Rust tooling enthusiasts in [#rust-tools].

[#cargo]: https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rustc
[#rust-tools]: https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-tools
[A-debuginfo]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-debuginfo
[A-rustdoc]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-rustdoc
[Cargo]: https://github.com/rust-lang/cargo/issues
[awesome-rust]: https://github.com/kud1ing/awesome-rust
[ides]: https://forge.rust-lang.org/ides.html
