---
layout: zh-CN/default
title: 为 Rust 出力 &mdash; 找出、分类和修复问题 &middot; Rust 程序设计语言
---

# 为 Rust 出力 &mdash; 找出、分类和修复问题

Rust 项目的的日常维护工作是围绕着[问题跟踪器][issue tracker]和[拉取请求][PR]，并且始终需要更多的贡献。
开始为 Rust 出力的最简单方式是寻找标有 [E-easy] 或 [E-mentor] 标签的问题。
它们最适合新手 Rust 程序员。

在标有 `E-mentor` 的问题上会有一名经验丰富的 Rust 开发人员来自愿帮助你解决该问题和[提交 GitHub 拉取请求][pull]来修复它。
在评论、IRC 中用 [@mentioning] 来联系他们（也可通过电子邮件）。
温馨提醒，Rust 开发人员经常收到大量的通知，所以有时可能错过某些；
不要犹豫！如果你认为他没有注意到你。

Rust 的其他项目也有其他类似的入门级任务，包括网页浏览器 [Servo]，HTTP 库 [hyper]，源代码格式化器 [rustfmt]，Unix 库绑定 [nix]，以及 lint 集 [clippy]。

虽然 Rust 有一个[广泛的测试套件][test]，但测试总是不嫌多的嘛。
[E-needstest] 标签指出了被认为已修复但尚未测试的问题。编写测试用例也是很好的一个了解新项目并开始贡献的方法。

Rust 始终需要人们来[分流][triage]问题：重现 bug，
最小化的测试用例，应用标签，关闭已解决的问题。
请注意，您需要被提升的 GitHub 权限才能应用标签，
不过卓有经验的人很容易得到它，相关事宜请联系[团队成员][team]。

在您找到自己的方式为项目出力并创建一些拉取请求后，可以考虑审阅其他人的拉取请求：良好的审阅是一个不可多得的技能。无需事先批准 &mdash; 只需开始礼貌并有建设性地在您感兴趣的拉取请求中评论。如果您想在代码审核的基础上有效分流，[阅读这份指南][reviews]。

<!--
TODO: weekly triage email?
TODO: @nrc says suggesting everybody review w/o training is bad
-->

[@mentioning]: https://github.com/blog/821
[E-easy]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AE-easy
[E-mentor]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AE-easy+label%3AE-mentor
[E-needstest]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AE-needstest
[PR]: https://github.com/rust-lang/rust/pulls
[Servo]: https://github.com/servo/servo
[clippy]: https://github.com/Manishearth/rust-clippy
[hyper]: https://github.com/hyperium/hyper
[issue tracker]: https://github.com/rust-lang/rust/issues
[nix]: https://github.com/nix-rust/nix/
[pull]: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#pull-requests
[reviews]: http://blog.originate.com/blog/2014/09/29/effective-code-reviews/
[rustfmt]: https://github.com/rust-lang-nursery/rustfmt
[team]: team.html
[test]: https://github.com/rust-lang/rust-wiki-backup/blob/master/Note-testsuite.md
[triage]: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#issue-triage
