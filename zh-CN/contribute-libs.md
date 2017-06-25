---
layout: zh-CN/default
title: 为 Rust 出力 &mdash; 库 &middot; Rust 程序设计语言
---

# 为 Rust 出力 &mdash; 库

如果你想通过编写 Rust 代码来为 Rust 出力，
那么库很不错：因为 Rust 是一种年轻的语言，
很多种类的库还尚不存在或并不完整，它们需要改进或竞争者。

决定写些什么更有乐趣和影响力是一个常见的问题。这里有些想法供你参考：

* 阅读和参与 [/r/rust] 和 [users.rust-lang.org] 上
  每周的「本周每个人都在做什么」（"what's everyone working on
  this week"）主题。那些包含来自其他需要合作者的 Rust 程序员的激动人心的通知。
* 通过 [awesome-rust] 和 [libs.rs] 亲自熟悉最好的 Rust 库。
* Some larger projects, including the web browser [Servo], the HTTP
  library [hyper], the source-formatter [rustfmt], the Unix library bindings
  [nix], and the lint collection [clippy], tag issues with 'easy' labels for new
  contributors.
* Get involved with one of the active Rust-oriented GitHub
  organizations, such as [PistonDevelopers], [servo], [redox-os],
  [iron], [contain-rs], [hyperium]. It's often easier to find a place
  to fit in with these subcommunities, they are in greater need of
  help than rust-lang itself, and they are filled with experienced
  Rust developers to guide you.
* Help guide libraries from [rust-lang-nursery] into rust-lang proper.
  Unfortunately there is not much documentation on what needs to be
  done here; ask on [#rust-libs].
* 查阅 RFC 问题跟踪器中的[社区已请求的库][requested]并建立它。
* 关注 [Github 趋势][trending]了解目前活跃的 Rust 项目。

作为库的设计者，您有必要了解 [Rust 库的最佳实践][lib-prac]。

在 [#rust-libs] 中能遇到其他 Rust 库的设计者。

<!--
TODO: Not sure #rust-libs is the place to direct people
-->

[#rust-libs]: https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-libs
[/r/rust]: https://reddit.com/r/rust
[PistonDevelopers]: https://github.com/PistonDevelopers
[Servo]: https://github.com/servo/servo
[Servo]: https://github.com/servo/servo
[awesome-rust]: https://github.com/kud1ing/awesome-rust
[clippy]: https://github.com/Manishearth/rust-clippy
[contain-rs]: https://github.com/contain-rs
[hyper]: https://github.com/hyperium/hyper
[hyperium]: https://github.com/hyperium
[iron]: https://github.com/iron
[lib-prac]: https://pascalhertleif.de/artikel/good-practices-for-writing-rust-libraries/
[libs.rs]: http://libs.rs
[nix]: https://github.com/nix-rust/nix/
[redox-os]: https://github.com/redox-os
[requested]: https://github.com/rust-lang/rfcs/labels/A-community-library
[rust-lang-nursery]: https://github.com/rust-lang-nursery
[rustfmt]: https://github.com/rust-lang-nursery/rustfmt
[trending]: https://github.com/trending?l=rust
[users.rust-lang.org]: https://users.rust-lang.org
