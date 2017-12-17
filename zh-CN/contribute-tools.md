---
layout: zh-CN/default
title: 为 Rust 出力 &mdash; 工具、IDE 以及基础设施 &middot; Rust 程序设计语言
---

# 为 Rust 出力 &mdash; 工具、IDE 以及基础设施

工具在一门语言的成功中扮演着重要的角色，还有许多工作要做。***Rust 开发的主要焦点是 [改进 IDE 的体验][ides]***。这涉及了整个 Rust 语言栈的工作，从编译器本身到你偏好的 IDE。点击链接了解更多信息。

Rust 包管理器 Cargo，Rust 文档生成器 rustdoc，虽然功能齐全且实用，但缺乏开发者。Rustdoc 在主仓库里的 [A-rustdoc] 标签下有许多未决的 issue。它们大多是 bug，问题是需要修复 bug 和提交 PR 的关键是贡献。 Cargo 有 [它自己的仓库和 issue][Cargo]，有兴趣贡献的人可以在 [#cargo] 中介绍自己。

尽管 Rust 可以在 gdb 和 lldb 调试器下进行一些有限的调试，但是仍然有很多无法正常调试的情况。 [A-debuginfo] 这个 issue 跟踪这些问题。

更多有关工具项目贡献的想法可以参见 [awesome-rust]。

通常还有其它有意思的工具项目，只是在等待合适的人来实现。在 [#rust-tools] 与其它 Rust 工具爱好者讨论。

[#cargo]: https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rustc
[#rust-tools]: https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-tools
[A-debuginfo]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-debuginfo
[A-rustdoc]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-rustdoc
[Cargo]: https://github.com/rust-lang/cargo/issues
[awesome-rust]: https://github.com/kud1ing/awesome-rust
[ides]: https://forge.rust-lang.org/ides.html
