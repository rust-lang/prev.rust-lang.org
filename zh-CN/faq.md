---
layout: zh-CN/faq
title: 常被问到的问题 &middot; Rust 程序设计语言
---

# 常被问到的问题

<p class="faq-intro">
此页面回答了有关 Rust 编程语言的常见问题。它不是这个语言的完整指南，也不是教学该语言的工具。这是 Rust 社区常遇到的人们重复提问的回答，及澄清了 Rust 某些设计决策幕后的理由。
</p>

<p class="faq-intro">
如果你觉得有一些常见或重要的问题没有在此列出，欢迎<a href="https://github.com/rust-lang/rust-www/blob/master/CONTRIBUTING.md">帮我们解决</a>。
</p>

<div id="toc">
    <h2>目录</h2><a href="#toggle-toc"></a>
    <div class="contents">
        <ol id="toc-contents">
            <li><a href="#project">Rust 项目</a></li>
            <li><a href="#performance">性能</a></li>
            <li><a href="#syntax">语法</a></li>
            <li><a href="#numerics">数字</a></li>
            <li><a href="#strings">字符串</a></li>
            <li><a href="#collections">集合</a></li>
            <li><a href="#ownership">所有权</a></li>
            <li><a href="#lifetimes">生命周期</a></li>
            <li><a href="#generics">泛型</a></li>
            <li><a href="#input-output">输入 / 输出</a></li>
            <li><a href="#error-handling">错误处理</a></li>
            <li><a href="#concurrency">并发</a></li>
            <li><a href="#macros">宏</a></li>
            <li><a href="#debugging">调试和工具</a></li>
            <li><a href="#low-level">底层</a></li>
            <li><a href="#cross-platform">跨平台</a></li>
            <li><a href="#modules-and-crates">模块和包装箱</a></li>
            <li><a href="#libraries">库</a></li>
            <li><a href="#design-patterns">设计模式</a></li>
            <li><a href="#other-languages">其他语言</a></li>
            <li><a href="#documentation">文档</a></li>
        </ol>
    </div>
</div>


<h2 id="project">Rust 项目</h2>

<h3><a href="#what-is-this-projects-goal" name="what-is-this-projects-goal">
此项目的目标是什么？
</a></h3>

设计和实现一个安全、并发、实用的系统语言。

Rust 存在的原因是，其他语言在这个抽象层次和效率上不能令人满意。尤其是：

1. 太少注重安全性。
2. 对并发的支持不够好。
3. 缺乏实用的可见功能。
4. 对资源的控制能力有限。

Rust 作为一种备选方案，可以提供高效的代码和舒适的抽象级别，同时改进上述四项要点。

<h3><a href="#is-this-project-controlled-by-mozilla" name="is-this-project-controlled-by-mozilla">
这个项目由 Mozilla 控制？
</a></h3>

不是。Rust 始于 2006 年 Graydon Hoare 的兼职项目，并持续了三年。Mozilla 在语言成熟到足以运行基本测试并展示其核心概念时于 2009 年开始参与。虽然它由 Mozilla 资助，但 Rust 是由来自世界各地的多样化的发烧友开发的。[Rust 团队](https://www.rust-lang.org/team.html) 由 Mozilla 和非 Mozilla 成员组成，`rustc`（Rust 的编译器）至今也有超过[一千名独立贡献者](https://github.com/rust-lang/rust/)。

就[项目治理](https://github.com/rust-lang/rfcs/blob/master/text/1068-rust-governance.md)而言，
Rust 由一个从全局角度出发，为项目设定愿景和优先事项的核心团队管理。
我们还有小组指导和促进特定兴趣领域的发展，包括核心语言、编译器、Rust 库、Rust 工具和官方 Rust 社区的管理。这些领域中的设计都先进地经过一个[RFC 流程](https://github.com/rust-lang/rfcs)。对于不需要RFC的更改，决定是通过 [`rustc` 代码库](https://github.com/rust-lang/rust)上的拉取请求决定。

<h3><a href="#what-are-some-non-goals" name="what-are-some-non-goals">
哪些不是 Rust 的目标？
</a></h3>

1. 我们不使用任何特别尖端的技术。旧的、成熟的技术更好。
2. 我们不会夸奖其他目标的表现力、极简主义或优雅。这些是可取的，但不是主要目标。
3. 我们不打算覆盖 C++ 或任何其他语言的完整功能集。Rust 应提供较常使用功能。
4. 我们不打算一切都是100%静态，100%安全，100%反思或任何其他意义上的太教条。存在权衡。
5. 我们不打算使 Rust 能在“每个可能的平台”上运行。它必须没有不必要的妥协，最终适用于广泛使用的硬件和软件平台。

<h3><a href="#how-does-mozilla-use-rust" name="how-does-mozilla-use-rust">
Mozilla 的哪些项目使用 Rust？
</a></h3>

主要项目是 [Servo](https://github.com/servo/servo)，它是 Mozilla 正在雕琢的一个实验性的浏览器引擎。Mozilla 还在努力[整合 Rust 组件](https://bugzilla.mozilla.org/show_bug.cgi?id=1135640) 到 Firefox。

<h3><a href="#what-examples-are-there-of-large-rust-projects" name="what-examples-are-there-of-large-rust-projects">
有哪些大型的 Rust 项目？
</a></h3>

目前两个最大的 Rust 开源项目是 [Servo](https://github.com/servo/servo) 和 [Rust 编译器](https://github.com/rust-lang/rust) 本身。

<h3><a href="#who-else-is-using-rust" name="who-else-is-using-rust">
还有谁在使用 Rust？
</a></h3>

[越来越多的组织在使用 Rust！](friends.html)

<!--
### What projects are good examples of idiomatic Rust code?

TODO: Write this answer.
-->

<h3><a href="#how-can-i-try-rust-easily" name="how-can-i-try-rust-easily">
怎样轻松地尝试 Rust？
</a></h3>

[playpen](https://play.rust-lang.org/) 是尝试 Rust 的最简单方法，它是一个可编写和运行 Rust 代码的在线应用。如果您想在您的系统上尝试 Rust，[安装](https://www.rust-lang.org/install.html)并尝试[猜谜游戏](https://doc.rust-lang.org/stable/book/guessing-game.html)教程。

<h3><a href="#how-do-i-get-help-with-rust-issues" name="how-do-i-get-help-with-rust-issues">
如何就 Rust 问题取得帮助？
</a></h3>

那有很多种方式。您可以尝试：

- 在 [users.rust-lang.org](https://users.rust-lang.org/) 发帖，这是官方的 Rust 用户论坛
- 在官方的 [Rust IRC 频道](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust) 询问 (irc.mozilla.org 上的 #rust)
- 在 [Stack Overflow](https://stackoverflow.com/questions/tagged/rust) 询问，加上 "rust" 标签
- 在 [/r/rust](https://www.reddit.com/r/rust) 发帖，这是非官方的 Rust 板块

<h3><a href="#why-has-rust-changed-so-much" name="why-has-rust-changed-so-much">
为什么 Rust 随时间推移在大幅变化？
</a></h3>

Rust 起步的目标是创建一个安全且合用的系统编程语言。为了追求这个目标，它探索了很多想法，其中一些（生命周期、traits）被保留，而其他则被丢弃（类型体系系统、绿色线程）。 Also, in the run up to 1.0 a lot of the standard library was rewritten as early designs were updated to best use Rust's features and provide quality, consistent cross-platform APIs. 现在的 Rust 已达到了 1.0，语言保证为“稳定”（stable）；虽然它可能继续演变，但对于目前的 Rust 来说，代码在未来的发行版本上能继续工作。

<h3><a href="#how-does-rust-language-versioning-work" name="how-does-rust-language-versioning-work">
Rust 语言的版本控制是怎样的？
</a></h3>

Rust 的语言版本遵循 [SemVer](http://semver.org/)，如果更改修复了编译器错误、补丁安全漏洞，或更改调度或类型推断需要附加注明，则稳定 API 的向后不兼容更改允许在次要版本中出现。次要版本更改的更详细指南可以在[语言](https://github.com/rust-lang/rfcs/blob/master/text/1122-language-semver.md)和[标准库](https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md)的已批准 RFC 中找到。

Rust 维护三个“发行频道”：稳定版（stable）、测试版（beta）和每夜版（nightly）。稳定版和测试版每六周更新一次，而在那时的每夜版会变为新的测试版，测试版变为新的稳定版。语言和标准库特性 marked unstable or hidden behind feature gates may only be used on the nightly release channel. New features land as unstable, and are "ungated" once approved by the core team and relevant subteams. 这种方法允许实验性变更，并同时为稳定频道提供强有力的向后兼容保证。

就相关的其他详细信息，请阅读 Rust 博客["Stability as a Deliverable."](http://blog.rust-lang.org/2014/10/30/Stability.html)

<h3><a href="#can-i-use-unstable-features-on-the-beta-or-stable-channel" name="can-i-use-unstable-features-on-the-beta-or-stable-channel">
我可以在 Beta 或稳定频道上使用不稳定的功能吗？
</a></h3>

并不能。Rust works hard to provide strong guarantees about the stability of the features provided on the beta and stable channels. When something is unstable, it means that we can't provide those guarantees for it yet, and don't want people relying on it staying the same. This gives us the opportunity to try changes in the wild on the nightly release channel, while still maintaining strong guarantees for people seeking stability.

Things stabilize all the time, and the beta and stable channels update every six weeks, with occasional fixes accepted into beta at other times. If you're waiting for a feature to be available without using the nightly release channel, you can locate its tracking issue by checking the [`B-unstable`](https://github.com/rust-lang/rust/issues?q=is%3Aissue+is%3Aopen+tracking+label%3AB-unstable) tag on the issue tracker.

<h3><a href="#what-are-feature-gates" name="what-are-feature-gates">
什么是“特性门控”（Feature Gates）？
</a></h3>

“特性门控”（Feature Gates）是 Rust 用来稳定编译器、语言和标准库特性的机制。 A feature that is "gated" is accessible only on the nightly release channel, and then only when it has been explicitly enabled through `#[feature]` attributes or the `-Z unstable-options` command line argument. When a feature is stabilized it becomes available on the stable release channel, and does not need to be explicitly enabled. At that point the feature is considered "ungated". Feature gates allow developers to test experimental features while they are under development, before they are available in the stable language.

<h3><a href="#why-a-dual-mit-asl2-license" name="why-a-dual-mit-asl2-license">
为什么采用 MIT/ASL2 双许可证？
</a></h3>

Apache 许可证包含对专利侵权的重要保护，但它与 GPL 第二版不兼容。为避免 Rust 使用 GPL2 会遇到的问题，因而同时采用 MIT 许可证。

<h3><a href="#why-a-permissive-license" name="why-a-permissive-license">
为什么是 BSD 风格的许可证，而不是 MPL 或三个许可证？
</a></h3>

这一部分是由于原始开发者（Graydon）的偏好，另一部分原因在于，语言倾向于拥有更广泛的受众群体，以及将其更多样的嵌入和提供到终端，例如网页浏览器。。我们希望呼吁尽可能多地潜在贡献者。

<h2 id="performance">性能</h2>

<h3><a href="#how-fast-is-rust" name="how-fast-is-rust">
Rust 有多快？
</a></h3>

飞快！Rust 在一些基准测试中（例如 [Benchmarks Game](https://benchmarksgame.alioth.debian.org/u64q/compare.php?lang=rust&lang2=gpp)、[等等](https://github.com/kostya/benchmarks)）已经有与惯用的 C 和 C++ 竞争的能力。

与 C++ 类似，Rust 采用[零成本抽象](http://blog.rust-lang.org/2015/05/11/traits.html)作为其核心原则之一：Rust 的抽象都没有施加全局性能损失，也不会有任何运行时系统的开销。

鉴于 Rust 建立在 LLVM 上，以及力求从 LLVM 的角度来看 C 语言，任何 LLVM 的性能改进也都有助于 Rust。从长远来看，Rust 的类型系统中更丰富的信息也应该能够实现对于 C / C++ 代码来说困难或不可能的优化。

<h3><a href="#is-rust-garbage-collected" name="is-rust-garbage-collected">
Rust 会垃圾收集吗？
</a></h3>

不。Rust 的重要创新之一就是保证内存安全（无 segfaults），*无需*垃圾收集。

通过避免垃圾收集（GC），Rust 可以提供许多益处：可预测的资源清理，降低内存管理开销，基本上没有运行时系统。这些特征使 Rust 很容易嵌入到任意上下文，并使其更容易[集成 Rust 代码到有 GC 的语言](http://calculist.org/blog/2015/12/23/neon-node-rust/)。

Rust 通过其所有权和借用（borrowing）系统避免了垃圾收集的需求，而同一系统也有助于解决许多其他问题，包括
[一般的资源管理](http://blog.skylight.io/rust-means-never-having-to-close-a-socket/)和[并发](http://blog.rust-lang.org/2015/04/10/Fearless-Concurrency.html)。

如果单一的所有权不够，Rust 程序依赖标准的引用计数智能指针类型，[`Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html)，及其线程安全版的[`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html)，而不是 GC。

不过，我们也在调查*可选*的垃圾收集作为一项未来扩展。
其目标旨在顺利与有垃圾收集的运行时整合，
例如由 
[Spidermonkey](https://developer.mozilla.org/en-US/docs/Mozilla/Projects/SpiderMonkey)
和 [V8](https://developers.google.com/v8/?hl=en) JavaScript 引擎所提供的那些。
最后，有人已调研了无编译器支持实现的
[纯 Rust 垃圾收集器](https://manishearth.github.io/blog/2015/09/01/designing-a-gc-in-rust/)
。

<h3><a href="#why-is-my-program-slow" name="why-is-my-program-slow">
为什么我的程序很慢？
</a></h3>

如非要求，Rust 编译器不会优化编译，[因为优化会编译速度，且在开发过程中通常是不合需要](https://users.rust-lang.org/t/why-does-cargo-build-not-optimise-by-default/4150/3)。

如果您使用 `cargo` 编译，请使用 `--release` 标志。如果您直接使用 `rustc`，使用 `-O` 标志。两者都将优化编译。

<h3><a href="#why-is-rustc-slow" name="why-is-rustc-slow">
Rust 编译似乎很慢。这是为什么？
</a></h3>

代码转换和优化。Rust提供了高级抽象，它可以编译成高效的机器代码，而这些翻译需要时间来运行，特别是优化。

但 Rust 的编译时间并不像看起来那么糟糕，并有理由相信会有所改善。当比较 C++ 与 Rust 的类似大小的项目时，整个项目的编译时间一般被认为是可比的。Rust 编译缓慢的一般认识很大程度上是由于 C++ 与 Rust 的*编译模型*的差异：C++ 的编译单元是文件，而 Rust 则是包装箱，它由很多文件组成。因此，在开发过程中，修改单个 C++ 文件导致的重新编译比 Rust 更少。正在进行的重大工作将重构编译器来引入[增量编译](https://github.com/rust-lang/rfcs/blob/master/text/1298-incremental-compilation.md)，这将使 Rust 的编译时间变得比 C++ 的模型更有优势。

除了编译模型外，Rust 的语言设计和编译器实现的其他几个方面也影响了编译时性能。

首先，Rust 有中等复杂类型的系统，并且必须花费不可忽略的编译时间来强制在运行时使 Rust 安全的约束。

其次，Rust 编译器遭受着长期的技术债务，特别是生成质量差的LLVM IR，而LLVM必须花时间“修复”。这有希望在未来[基于MIR](https://github.com/rust-lang/rfcs/blob/master/text/1211-mir.md)优化和转换传递来减轻 Rust 编译器在 LLVM 上的负担。

第三，Rust 使用的 LLVM 代码生成是一把双刃剑：虽然它能够使 Rust 具有世界一流的运行时性能，但 LLVM 是一个不重视编译时性能的大型框架，特别是在使用较差输入质量时。

最后，虽然 Rust 的单性泛型（ala C ++）的首选策略产生快速代码，但它需要比其他翻译策略产生更多的代码。Rust 程序员可以使用 trait 对象通过动态调度来抵消这个代码膨胀。

<h3><a href="#why-are-rusts-hashmaps-slow" name="why-are-rusts-hashmaps-slow">
为什么 Rust 的 <code>HashMap</code> 很慢？
</a></h3>

默认情况下，Rust 的 [`HashMap`][HashMap] 使用 [SipHash](https://131002.net/siphash/) 哈希算法，which is designed to prevent [hash table collision attacks](http://programmingisterrible.com/post/40620375793/hash-table-denial-of-service-attacks-revisited) while providing [reasonable performance on a variety of workloads](https://www.reddit.com/r/rust/comments/3hw9zf/rust_hasher_comparisons/cub4oh6).

While SipHash [demonstrates competitive performance](http://cglab.ca/%7Eabeinges/blah/hash-rs/) in many cases, one case where it is notably slower than other hashing algorithms is with short keys, such as integers. This is why Rust programmers often observe slow performance with [`HashMap`][HashMap]. The [FNV hasher](https://crates.io/crates/fnv) is frequently recommended for these cases, but be aware that it does not have the same collision-resistance properties as SipHash.

<h3><a href="#why-is-there-no-integrated-benchmarking" name="why-is-there-no-integrated-benchmarking">
为什么没有集成的基准测试基础设施？
</a></h3>

有，但它只在夜间发行频道上可用。我们最终计划为集成的基准建立一个可插拔系统，但同时，目前的系统[被认为是不稳定的](https://github.com/rust-lang/rust/issues/29553)。

<h3><a href="#does-rust-do-tail-call-optimization" name="does-rust-do-tail-call-optimization">
Rust 是否做尾呼优化？
</a></h3>

Not generally, no. Tail-call optimization may be done in [limited circumstances](http://llvm.org/docs/CodeGenerator.html#sibling-call-optimization), but is [not guaranteed](https://mail.mozilla.org/pipermail/rust-dev/2013-April/003557.html). As the feature has always been desired, Rust has a keyword (`become`) reserved, though it is not clear yet whether it is technically possible, nor whether it will be implemented. There was a [proposed extension](https://github.com/rust-lang/rfcs/pull/81) that would allow tail-call elimination in certain contexts, but it is currently postponed.

<h3><a href="#does-rust-have-a-runtime" name="does-rust-have-a-runtime">
Rust 有运行时吗？
</a></h3>

Not in the typical sense used by languages such as Java, but parts of the Rust standard library can be considered a "runtime", providing a heap, backtraces, unwinding, and stack guards. There is a [small amount of initialization code](https://github.com/rust-lang/rust/blob/33916307780495fe311fe9c080b330d266f35bfb/src/libstd/rt.rs#L43) that runs before the user's `main` function. The Rust standard library additionally links to the C standard library, which does similar [runtime initialization](http://www.embecosm.com/appnotes/ean9/html/ch05s02.html). Rust code can be compiled without the standard library, in which case the runtime is roughly equivalent to C's.

<h2 id="syntax">语法</h2>

<h3><a href="#why-curly-braces" name="why-curly-braces">
Why curly braces? Why can't Rust's syntax be like Haskell's or Python's?
</a></h3>

Use of curly braces to denote blocks is a common design choice in a variety of programming languages, and Rust's consistency is useful for people already familiar with the style.

Curly braces also allow for more flexible syntax for the programmer and a simpler parser in the compiler.

<h3><a href="#why-brackets-around-blocks" name="why-brackets-around-blocks">
I can leave out parentheses on <code>if</code> conditions, so why do I have to put brackets around single line blocks? Why is the C style not allowed?
</a></h3>

Whereas C requires mandatory parentheses for `if`-statement conditionals but leaves brackets optional, Rust makes the opposite choice for its `if`-expressions. This keeps the conditional clearly separate from the body and avoids the hazard of optional brackets, which can lead to easy-to-miss errors during refactoring, like Apple's [goto fail](https://gotofail.com/) bug.

<h3><a href="#why-no-literal-syntax-for-dictionaries" name="why-no-literal-syntax-for-dictionaries">
Why is there no literal syntax for dictionaries?
</a></h3>

Rust's overall design preference is for limiting the size of the *language* while enabling powerful *libraries*. While Rust does provide initialization syntax for arrays and string literals, these are the only collection types built into the language. Other library-defined types, including the ubiquitous [`Vec`][Vec] collection type, use macros for initialization like the [`vec!`][VecMacro] macro.

This design choice of using Rust's macro facilities to initialize collections will likely be extended generically to other collections in the future, enabling simple initialization of not only [`HashMap`][HashMap] and [`Vec`][Vec], but also other collection types such as [`BTreeMap`][BTreeMap]. In the meantime, if you want a more convenient syntax for initializing collections, you can [create your own macro](https://stackoverflow.com/questions/27582739/how-do-i-create-a-hashmap-literal) to provide it.

<h3><a href="#when-should-i-use-an-implicit-return" name="when-should-i-use-an-implicit-return">
我什么时候应该使用隐式 return？
</a></h3>

Rust is a very expression-oriented language, and "implicit returns" are part of that design. Constructs like `if`s, `match`es, and normal blocks are all expressions in Rust. For example, the following code checks if an [`i64`][i64] is odd, returning the result by simply yielding it as a value:

```rust
fn is_odd(x: i64) -> bool {
    if x % 2 != 0 { true } else { false }
}
```

Although it can be simplified even further like so:

```rust
fn is_odd(x: i64) -> bool {
    x % 2 != 0
}
```

In each example, the last line of the function is the return value of that function. It is important to note that if a function ends in a semicolon, its return type will be `()`, indicating no returned value. Implicit returns must omit the semicolon to work.

Explicit returns are only used if an implicit return is impossible because you are returning before the end of the function's body. While each of the above functions could have been written with a `return` keyword and semicolon, doing so would be unnecessarily verbose, and inconsistent with the conventions of Rust code.

<h3><a href="#why-arent-function-signatures-inferred" name="why-arent-function-signatures-inferred">
为什么不推断函数签名？
</a></h3>

在 Rust 中，声明倾向于使用显式类型，而实际代码则推断其类型。这种设计有几个原因：

- Mandatory declaration signatures help enforce interface stability at both the module and crate level.
- Signatures improve code comprehension for the programmer, eliminating the need for an IDE running an inference algorithm across an entire crate to be able to guess at a function's argument types; it's always explicit and nearby.
- Mechanically, it simplifies the inference algorithm, as inference only requires looking at one function at a time.

<h3><a href="#why-does-match-have-to-be-exhaustive" name="why-does-match-have-to-be-exhaustive">
Why does <code>match</code> have to be exhaustive?
</a></h3>

To aid in refactoring and clarity.

First, if every possibility is covered by the `match`, adding variants to the `enum` in the future will cause a compilation failure, rather than an error at runtime. This type of compiler assistance makes fearless refactoring possible in Rust.

Second, exhaustive checking makes the semantics of the default case explicit: in general, the only safe way to have a non-exhaustive `match` would be to panic the thread if nothing is matched. Early versions of Rust did not require `match` cases to be exhaustive and it was found to be a great source of bugs.

It is easy to ignore all unspecified cases by using the `_` wildcard:

```rust
match val.do_something() {
    Cat(a) => { /* ... */ }
    _      => { /* ... */ }
}
```

<h2 id="numerics">数字</h2>

<h3><a href="#which-type-of-float-should-i-use" name="which-type-of-float-should-i-use">
Which of <code>f32</code> and <code>f64</code> should I prefer for floating-point math?
</a></h3>

The choice of which to use is dependent on the purpose of the program.

If you are interested in the greatest degree of precision with your floating point numbers, then prefer [`f64`][f64]. If you are more interested in keeping the size of the value small or being maximally efficient, and are not concerned about the associated inaccuracy of having fewer bits per value, then [`f32`][f32] is better. Operations on [`f32`][f32] are usually faster, even on 64-bit hardware. As a common example, graphics programming typically uses [`f32`][f32] because it requires high performance, and 32-bit floats are sufficient for representing pixels on the screen.

If in doubt, choose [`f64`][f64] for the greater precision.

<h3><a href="#why-cant-i-compare-floats" name="why-cant-i-compare-floats">
Why can't I compare floats or use them as <code>HashMap</code> or <code>BTreeMap</code> keys?
</a></h3>

Floats can be compared with the `==`, `!=`, `<`, `<=`, `>`, and `>=` operators, and with the `partial_cmp()` function. `==` and `!=` are part of the [`PartialEq`][PartialEq] trait, while `<`, `<=`, `>`, `>=`, and `partial_cmp()` are part of the [`PartialOrd`][PartialOrd] trait.

Floats cannot be compared with the `cmp()` function, which is part of the [`Ord`][Ord] trait, as there is no total ordering for floats. Furthermore, there is no total equality relation for floats, and so they also do not implement the [`Eq`][Eq] trait.

There is no total ordering or equality on floats because the floating-point value [`NaN`](https://en.wikipedia.org/wiki/NaN) is not less than, greater than, or equal to any other floating-point value or itself.

Because floats do not implement [`Eq`][Eq] or [`Ord`][Ord], they may not be used in types whose trait bounds require those traits, such as [`BTreeMap`][BTreeMap] or [`HashMap`][HashMap]. This is important because these types *assume* their keys provide a total ordering or total equality relation, and will malfunction otherwise.

There [is a crate](https://crates.io/crates/ordered-float) that wraps [`f32`][f32] and [`f64`][f64] to provide [`Ord`][Ord] and [`Eq`][Eq] implementations, which may be useful in certain cases.

<h3><a href="#how-can-i-convert-between-numeric-types" name="how-can-i-convert-between-numeric-types">
如何在数字类型间进行转换？
</a></h3>

There are two ways: the `as` keyword, which does simple casting for primitive types, and the [`Into`][Into] and [`From`][From] traits, which are implemented for a number of type conversions (and which you can implement for your own types). The [`Into`][Into] and [`From`][From] traits are only implemented in cases where conversions are lossless, so for example, `f64::from(0f32)` will compile while `f32::from(0f64)` will not. On the other hand, `as` will convert between any two primitive types, truncating values as necessary.


<h3><a href="#why-doesnt-rust-have-increment-and-decrement-operators" name="why-doesnt-rust-have-increment-and-decrement-operators">
Why doesn't Rust have increment and decrement operators?
</a></h3>

Preincrement and postincrement (and the decrement equivalents), while convenient, are also fairly complex. They require knowledge of evaluation order, and often lead to subtle bugs and undefined behavior in C and C++. `x = x + 1` or `x += 1` is only slightly longer, but unambiguous.

<h2 id="strings">字符串</h2>

<h3><a href="#how-to-convert-string-or-vec-to-slice" name="how-to-convert-string-or-vec-to-slice">
How can I convert a <code>String</code> or <code>Vec&lt;T&gt;</code> to a slice (<code>&amp;str</code> and <code>&amp;[T]</code>)?
</a></h3>

Usually, you can pass a reference to a `String` or `Vec<T>` wherever a slice is expected.
Using [Deref coercions](https://doc.rust-lang.org/stable/book/deref-coercions.html), [`String`s][String] and [`Vec`s][Vec] will automatically coerce to their respective slices when passed by reference with `&` or `& mut`.

Methods implemented on `&str` and `&[T]` can be accessed directly on `String` and `Vec<T>`. For example, `some_string.char_at(0)` will work even though `char_at` is a method on `&str` and `some_string` is a `String`.

In some cases, such as generic code, it's necessary to convert manually. Manual conversions can be achieved using the slicing operator, like so: `&my_vec[..]`.

<h3><a href="#how-to-convert-between-str-and-string" name="how-to-convert-between-str-and-string">
How can I convert from <code>&amp;str</code> to <code>String</code> or the other way around?
</a></h3>

The [`to_string()`][to_string] method converts from a [`&str`][str] into a [`String`][String], and [`String`s][String] are automatically converted into [`&str`][str] when you borrow a reference to them. Both are demonstrated in the following example:

```rust
fn main() {
    let s = "Jane Doe".to_string();
    say_hello(&s);
}

fn say_hello(name: &str) {
    println!("Hello {}!", name);
}
```

<h3><a href="#what-are-the-differences-between-str-and-string" name="what-are-the-differences-between-str-and-string">
两种不同的字符串类型有什么区别？
</a></h3>

[`String`][String] is an owned buffer of UTF-8 bytes allocated on the heap. Mutable [`String`s][String] can be modified, growing their capacity as needed. [`&str`][str] is a fixed-capacity "view" into a [`String`][String] allocated elsewhere, commonly on the heap, in the case of slices dereferenced from [`String`s][String], or in static memory, in the case of string literals.

[`&str`][str] is a primitive type implemented by the Rust language, while [`String`][String] is implemented in the standard library.

<h3><a href="#how-do-i-do-o1-character-access-in-a-string" name="how-do-i-do-o1-character-access-in-a-string">
如何以 O(1) 复杂度访问一个 <code>String</code> 中的字符？
</a></h3>

并不能这样做。At least not without a firm understanding of what you mean by "character", and preprocessing the string to find the index of the desired character.

Rust strings are UTF-8 encoded. A single visual character in UTF-8 is not necessarily a single byte as it would be in an ASCII-encoded string. Each byte is called a "code unit" (in UTF-16, code units are 2 bytes; in UTF-32 they are 4 bytes). "Code points" are composed of one or more code units, and combine in "grapheme clusters" which most closely approximate characters.

Thus, even though you may index on bytes in a UTF-8 string, you can't access the `i`th code point or grapheme cluster in constant time. However, if you know at which byte that desired code point or grapheme cluster begins, then you _can_ access it in constant time. Functions including [`str::find()`][str__find] and regex matches return byte indices, facilitating this sort of access.

<h3><a href="#why-are-strings-utf-8" name="why-are-strings-utf-8">
为什么字符串默认为 UTF-8？
</a></h3>

The [`str`][str] type is UTF-8 because we observe more text in the wild in this encoding – particularly in network transmissions, which are endian-agnostic – and we think it's best that the default treatment of I/O not involve having to recode codepoints in each direction.

This does mean that locating a particular Unicode codepoint inside a string is an O(n) operation, although if the starting byte index is already known then they can be accessed in O(1) as expected. On the one hand, this is clearly undesirable; on the other hand, this problem is full of trade-offs and we'd like to point out a few important qualifications:

Scanning a [`str`][str] for ASCII-range codepoints can still be done safely byte-at-a-time. If you use [`.as_bytes()`][str__as_bytes], pulling out a [`u8`][u8] costs only `O(1)` and produces a value that can be cast and compared to an ASCII-range [`char`][char]. So if you're (say) line-breaking on `'\n'`, byte-based treatment still works. UTF-8 was well-designed this way.

Most "character oriented" operations on text only work under very restricted language assumptions such as "ASCII-range codepoints only". Outside ASCII-range, you tend to have to use a complex (non-constant-time) algorithm for determining linguistic-unit (glyph, word, paragraph) boundaries anyway. We recommend using an "honest" linguistically-aware, Unicode-approved algorithm.

The [`char`][char] type is UTF-32. If you are sure you need to do a codepoint-at-a-time algorithm, it's trivial to write a `type wstr = [char]`, and unpack a [`str`][str] into it in a single pass, then work with the `wstr`. In other words: the fact that the language is not "decoding to UTF32 by default" shouldn't stop you from decoding (or re-encoding any other way) if you need to work with that encoding.

For a more in-depth explanation of why UTF-8 is usually preferable over UTF-16 or UTF-32, read the [UTF-8 Everywhere manifesto](http://utf8everywhere.org/).

<h3><a href="#what-string-type-should-i-use" name="what-string-type-should-i-use">
我应该使用哪种字符串类型？
</a></h3>

Rust has four pairs of string types, [each serving a distinct purpose](http://www.suspectsemantics.com/blog/2016/03/27/string-types-in-rust/). In each pair, there is an "owned" string type, and a "slice" string type. The organization looks like this:

|               | "Slice" 类型 | "Owned" 类型 |
|:--------------|:-------------|:-------------|
| UTF-8         | `str`        | `String`     |
| OS 兼容        | `OsStr`      | `OsString`   |
| C 兼容         | `CStr`       | `CString`    |
| 系统路径       | `Path`       | `PathBuf`    |

Rust 的不同字符串类型适用于不同的目的。`String` 和 `str` 为 UTF-8 编码的通用目的字符串。`OsString` 和 `OsStr` 的编码取决于当前平台，在与操作系统交互时 使用。`CString` 和 `CStr` 是 Rust 中与 C 字符串相当的存在，用于 FFI 代码。`PathBuf` 和 `Path` 是 `OsString` 和 `OsStr` 的方便包装，提供特定路径操作的方法。

<h3><a href="#why-are-there-multiple-types-of-strings" name="why-are-there-multiple-types-of-strings">
怎样写一个同时接受 <code>&str</code> 与 <code>String</code> 的函数？
</a></h3>

有几种方法，具体取决于该函数的需求：

- If the function needs an owned string, but wants to accept any type of string, use an `Into<String>` bound.
- If the function needs a string slice, but wants to accept any type of string, use an `AsRef<str>` bound.
- If the function does not care about the string type, and wants to handle the two possibilities uniformly, use `Cow<str>` as the input type.

__使用 `Into<String>`__

在此例中，the function will accept both owned strings and string slices, either doing nothing or converting the input into an owned string within the function body. Note that the conversion needs to be done explicitly, and will not happen otherwise.

```rust
fn accepts_both<S: Into<String>>(s: S) {
    let s = s.into();   // This will convert s into a `String`.
    // ... the rest of the function
}
```

__使用 `AsRef<str>`__

在此例中，the function will accept both owned strings and string slices, either doing nothing or converting the input into a string slice. This can be done automatically by taking the input by reference, like so:

```rust
fn accepts_both<S: AsRef<str>>(s: &S) {
    // ... the body of the function
}
```

__使用 `Cow<str>`__

在此例中，该函数采用一个 `Cow<str>`，它不是通用类型，而是一个容器，包含所需的字符串或字符串切片。

```rust
fn accepts_cow(s: Cow<str>) {
    // ... the body of the function
}
```


<h2 id="collections">集合</h2>

<h3><a href="#can-i-implement-linked-lists-in-rust" name="can-i-implement-linked-lists-in-rust">
可以在 Rust 中有效地实现向量和链表的数据结构吗？
</a></h3>

If your reason for implementing these data structures is to use them for other programs, there's no need, as efficient implementations of these data structures are provided by the standard library.

If, however, [your reason is simply to learn](http://cglab.ca/~abeinges/blah/too-many-lists/book/), then you will likely need to dip into unsafe code. While these data structures _can_ be implemented entirely in safe Rust, the performance is likely to be worse than it would be with the use of unsafe code. The simple reason for this is that data structures like vectors and linked lists rely on pointer and memory operations that are disallowed in safe Rust.

例如，a doubly-linked list requires that there be two mutable references to each node, but this violates Rust's mutable reference aliasing rules. You can solve this using [`Weak<T>`][Weak], but the performance will be poorer than you likely want. With unsafe code you can bypass the mutable reference aliasing rule restriction, but must manually verify that your code introduces no memory safety violations.

<h3><a href="#how-can-i-iterate-over-a-collection-without-consuming-it" name="how-can-i-iterate-over-a-collection-without-consuming-it">
如何在不移动/消费的情况下迭代集合？
</a></h3>

The easiest way is by using the collection's [`IntoIterator`][IntoIterator] implementation. Here is an example for [`&Vec`][Vec]:

```rust
let v = vec![1,2,3,4,5];
for item in &v {
    print!("{} ", item);
}
println!("\nLength: {}", v.len());
```

Rust `for` 循环调用 `into_iter()` (defined on the [`IntoIterator`][IntoIterator] trait) for whatever they're iterating over. Anything implementing the [`IntoIterator`][IntoIterator] trait may be looped over with a `for` loop. [`IntoIterator`][IntoIterator] is implemented for [`&Vec`][Vec] and [`&mut Vec`][Vec], causing the iterator from `into_iter()` to borrow the contents of the collection, rather than moving/consuming them. The same is true for other standard collections as well.

如果需要移动/消费迭代器，在迭代中撰写没有 `&` 或 `&mut` 的 `for`。

如果您需要直接访问一个借用的迭代器，您通常可以调用 `iter()` 方法来获取它。

<h3><a href="#why-do-i-need-to-type-the-array-size-in-the-array-declaration" name="why-do-i-need-to-type-the-array-size-in-the-array-declaration">
为什么需要在数组声明中键入数组大小？
</a></h3>

You don't necessarily have to. If you're declaring an array directly, the size is inferred based on the number of elements. But if you're declaring a function that takes a fixed-size array, the compiler has to know how big that array will be.

One thing to note is that currently Rust doesn't offer generics over arrays of different size. If you'd like to accept a contiguous container of a variable number of values, use a [`Vec`][Vec] or slice (depending on whether you need ownership).

<h2 id="ownership">Ownership</h2>

<h3><a href="#how-can-i-implement-a-data-structure-that-contains-cycles" name="how-can-i-implement-a-data-structure-that-contains-cycles">
如何实现包含循环的图或其他数据结构？
</a></h3>

There are at least four options (discussed at length in [Too Many Linked Lists](http://cglab.ca/~abeinges/blah/too-many-lists/book/)):

- 您可以使用 [`Rc`][Rc] 和 [`Weak`][Weak] 来允许节点共享所有权以实现它。虽然这种方法要支付内存管理的成本。
- You can implement it using `unsafe` code using raw pointers. This will be
more efficient, but bypasses Rust's safety guarantees.
- Using vectors and indices into those vectors. There are [several](http://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/) [available](https://featherweightmusings.blogspot.com/2015/04/graphs-in-rust.html) examples and explanations of this approach.
- Using borrowed references with [`UnsafeCell`][UnsafeCell]. There are [explanations and code](https://github.com/nrc/r4cppp/blob/master/graphs/README.md#node-and-unsafecell) available for this approach.

<h3><a href="#how-can-i-define-a-struct-that-contains-a-reference-to-one-of-its-own-fields" name="how-can-i-define-a-struct-that-contains-a-reference-to-one-of-its-own-fields">
我该如何定义一个包含对其自身字段之一引用的结构体？
</a></h3>

这有可能，但没有用。结构本身是永久借用，因此无法移动。下面用一些代码说明：

```rust
use std::cell::Cell;

#[derive(Debug)]
struct Unmovable<'a> {
    x: u32,
    y: Cell<Option<&'a u32>>,
}


fn main() {
    let test = Unmovable { x: 42, y: Cell::new(None) };
    test.y.set(Some(&test.x));

    println!("{:?}", test);
}
```

<h3><a href="#what-is-the-difference-between-consuming-and-moving" name="what-is-the-difference-between-consuming-and-moving">
按值传递，消费，移动和转让所有权之间有什么区别？
</a></h3>

These are different terms for the same thing. In all cases, it means the value has been moved to another owner, and moved out of the possession of the original owner, who can no longer use it. If a type implements the `Copy` trait, the original owner's value won't be invalidated, and can still be used.

<h3><a href="#why-can-values-of-some-types-by-reused-while-others-are-consumed" name="why-can-values-of-some-types-by-reused-while-others-are-consumed">
为什么某些类型的值在传递到一个函数后可以使用，而另一些类型的值传递后使用会导致错误？
</a></h3>

如果一个类型实现了 [`Copy`][Copy] 特征，则它将在传递到一个函数时被复制。Rust 中的所有数字类型都实现了 [`Copy`][Copy]，但结构类型默认并未实现 [`Copy`][Copy]，因此它们是被移动。这意味着结构体不能在其他地方重用，除非它是通过 return 从函数中移出。

<h3><a href="#how-do-you-deal-with-a-use-of-moved-value-error" name="how-do-you-deal-with-a-use-of-moved-value-error">
如何处理“use of moved value”错误？
</a></h3>

This error means that the value you're trying to use has been moved to a new owner. The first thing to check is whether the move in question was necessary: if it moved into a function, it may be possible to rewrite the function to use a reference, rather than moving. Otherwise if the type being moved implements [`Clone`][Clone], then calling `clone()` on it before moving will move a copy of it, leaving the original still available for further use. Note though that cloning a value should typically be the last resort since cloning can be expensive, causing further allocations.

If the moved value is of your own custom type, consider implementing [`Copy`][Copy] (for implicit copying, rather than moving) or [`Clone`][Clone] (explicit copying). [`Copy`][Copy] is most commonly implemented with `#[derive(Copy, Clone)]` ([`Copy`][Copy] requires [`Clone`][Clone]), and [`Clone`][Clone] with `#[derive(Clone)]`.

If none of these are possible, you may want to modify the function that acquired ownership to return ownership of the value when the function exits.

<h3><a href="#what-are-the-rules-for-different-self-types-in-methods" name="what-are-the-rules-for-different-self-types-in-methods">
在方法声明中使用 <code>self</code>、<code>&amp;self</code> 与 <code>&amp;mut self</code> 的规则是什么？
</a></h3>

- 使用 `self`，当一个函数需要消费该值
- 使用 `&self`，当一个函数只需要该值的只读引用
- 使用 `&mut self`，当一个函数不需消费而要变更该值

<h3><a href="#how-can-i-understand-the-borrow-checker" name="how-can-i-understand-the-borrow-checker">
我该怎么能理解借用检查器？
</a></h3>

借用检查器只使用几条规则，而这可以在 Rust 之书的[借用部分](https://doc.rust-lang.org/stable/book/references-and-borrowing.html#the-rules)找到，当评估（Eval） Rust 代码时。这些规则为：

> First, any borrow must last for a scope no greater than that of the owner. Second, you may have one or the other of these two kinds of borrows, but not both at the same time:
>
> - one or more references (&T) to a resource.
> - exactly one mutable reference (&mut T)

While the rules themselves are simple, following them consistently is not, particularly for those unaccustomed to reasoning about lifetimes and ownership.

The first step in understanding the borrow checker is reading the errors it produces. A lot of work has been put into making sure the borrow checker provides quality assistance in resolving the issues it identifies. When you encounter a borrow checker problem, the first step is to slowly and carefully read the error reported, and to only approach the code after you understand the error being described.

The second step is to become familiar with the ownership and mutability-related container types provided by the Rust standard library, including [`Cell`][Cell], [`RefCell`][RefCell], and [`Cow`][Cow]. These are useful and necessary tools for expressing certain ownership and mutability situations, and have been written to be of minimal performance cost.

The single most important part of understanding the borrow checker is practice. Rust's strong static analyses guarantees are strict and quite different from what many programmers have worked with before. It will take some time to become completely comfortable with everything.

If you find yourself struggling with the borrow checker, or running out of patience, always feel free to reach out to the [Rust community](community.html) for help.

<h3><a href="#when-is-rc-useful" name="when-is-rc-useful">
<code>Rc</code> 在什么时候有用？
</a></h3>

This is covered in the official documentation for [`Rc`][Rc], Rust's non-atomically reference-counted pointer type. In short, [`Rc`][Rc] and its thread-safe cousin [`Arc`][Arc] are useful to express shared ownership, and have the system automatically deallocate the associated memory when no one has access to it.

<h3><a href="#how-do-i-return-a-closure-from-a-function" name="how-do-i-return-a-closure-from-a-function">
如何从函数返回一个闭包？
</a></h3>

To return a closure from a function, it must be a "move closure", meaning that the closure is declared with the `move` keyword. As [explained in the Rust book](https://doc.rust-lang.org/book/closures.html#move-closures), this gives the closure its own copy of the captured variables, independent of its parent stack frame. Otherwise, returning a closure would be unsafe, as it would allow access to variables that are no longer valid; put another way: it would allow reading potentially invalid memory. The closure must also be wrapped in a [`Box`][Box], so that it is allocated on the heap. Read more about this [in the book](https://doc.rust-lang.org/book/closures.html#returning-closures).

<h3><a href="#what-are-deref-coercions" name="what-are-deref-coercions">
什么是强制退还，它是如何工作？
</a></h3>

A [deref coercion](https://doc.rust-lang.org/book/deref-coercions.html) is a handy coercion
that automatically converts references to pointers (e.g., `&Rc<T>` or `&Box<T>`) into references
to their contents (e.g., `&T`). Deref coercions exist to make using Rust more ergonomic, and are implemented via the [`Deref`][Deref] trait.

A Deref implementation indicates that the implementing type may be converted into a target by a call to the `deref` method, which takes an immutable reference to the calling type and returns a reference (of the same lifetime) to the target type. The `*` prefix operator is shorthand for the `deref` method.

They're called "coercions" because of the following rule, quoted here [from the Rust book](https://doc.rust-lang.org/stable/book/deref-coercions.html):

> If you have a type `U`, and it implements `Deref<Target=T>`, values of `&U` will automatically coerce to a `&T`.

For example, if you have a `&Rc<String>`, it will coerce via this rule into a `&String`, which then coerces to a `&str` in the same way. So if a function takes a `&str` parameter, you can pass in a `&Rc<String>` directly, with all coercions handled automatically via the `Deref` trait.

The most common sorts of deref coercions are:

- `&Rc<T>` to `&T`
- `&Box<T>` to `&T`
- `&Arc<T>` to `&T`
- `&Vec<T>` to `&[T]`
- `&String` to `&str`

<h2 id="lifetimes">生命周期</h2>

<h3><a href="#why-lifetimes" name="why-lifetimes">
为什么有生命周期？
</a></h3>

Lifetimes are Rust's answer to the question of memory safety. They allow Rust to ensure memory safety without the performance costs of garbage collection. They are based on a variety of academic work, which can be found in the [Rust book](https://doc.rust-lang.org/stable/book/bibliography.html#type-system).

<h3><a href="#why-is-the-lifetime-syntax-the-way-it-is" name="why-is-the-lifetime-syntax-the-way-it-is">
为什么生命周期语法是这样的？
</a></h3>

The `'a` syntax comes from the ML family of programming languages, where `'a` is used to indicate a generic type parameter. For Rust, the syntax had to be something that was unambiguous, noticeable, and fit nicely in a type declaration right alongside traits and references. Alternative syntaxes have been discussed, but no alternative syntax has been demonstrated to be clearly better.

<h3><a href="#how-do-i-return-a-borrow-to-something-i-created-from-a-function" name="how-do-i-return-a-borrow-to-something-i-created-from-a-function">
如何将一个函数中创建的东西返回为借用？
</a></h3>

You need to ensure that the borrowed item will outlive the function. This can be done by binding the output lifetime to some input lifetime like so:

```rust
type Pool = TypedArena<Thing>;

// (the lifetime below is only written explicitly for
// expository purposes; it can be omitted via the
// elision rules described in a later FAQ entry)
fn create_borrowed<'a>(pool: &'a Pool,
                       x: i32,
                       y: i32) -> &'a Thing {
    pool.alloc(Thing { x: x, y: y })
}
```

An alternative is to eliminate the references entirely by returning an owning type like [`String`][String]:

```rust
fn happy_birthday(name: &str, age: i64) -> String {
    format!("Hello {}! You're {} years old!", name, age)
}
```

这种方法更简单，但往往导致不必要的分配。

<h3><a href="#when-are-lifetimes-required-to-be-explicit" name="when-are-lifetimes-required-to-be-explicit">
为什么某些引用有生命周期，比如 <code>&amp;'a T</code>，而某些没有，比如 <code>&amp;T</code>？
</a></h3>

In fact, *all* reference types have a lifetime, but most of the time you do not have to write
it explicitly. The rules are as follows:

1. Within a function body, you never have to write a lifetime explicitly; the correct value
   should always be inferred.
2. Within a function *signature* (for example, in the types of its
   arguments, or its return type), you *may* have to write a lifetime
   explicitly. Lifetimes there use a simple defaulting scheme called
   ["lifetime elision"](https://doc.rust-lang.org/book/lifetimes.html#lifetime-elision),
   which consists of the following three rules:
  - Each elided lifetime in a function’s arguments becomes a distinct lifetime parameter.
  - If there is exactly one input lifetime, elided or not, that
    lifetime is assigned to all elided lifetimes in the return values
    of that function.
  - If there are multiple input lifetimes, but one of them is &self
    or &mut self, the lifetime of self is assigned to all elided
    output lifetimes.
3. Finally, in a `struct` or `enum` definition, all lifetimes must be explicitly declared.

If these rules result in compilation errors, the Rust compiler will provide an error message indicating the error caused, and suggesting a potential solution based on which step of the inference process caused the error.

<h3><a href="#how-can-rust-guarantee-no-null-pointers" name="how-can-rust-guarantee-no-null-pointers">
Rust 如何保证“无空指针”和“无悬挂指针”？
</a></h3>

The only way to construct a value of type `&Foo` or `&mut Foo` is to specify an existing value of type `Foo` that the reference points to. The reference "borrows" the original value for a given region of code (the lifetime of the reference), and the value being borrowed from cannot be moved or destroyed for the duration of the borrow.

<h3><a href="#how-do-i-express-the-absense-of-a-value-without-null" name="how-do-i-express-the-absense-of-a-value-without-null">
如果没有 <code>null</code>，我该怎么表达缺失值？
</a></h3>

You can do that with the [`Option`][Option] type, which can either be `Some(T)` or `None`. `Some(T)` indicates that a value of type `T` is contained within, while `None` indicates the absence of a value.

<h2 id="generics">泛型</h2>

<h3><a href="#what-is-monomorphisation" name="what-is-monomorphisation">
什么是“单态化”？
</a></h3>

Monomorphisation specializes each use of a generic function (or structure) with specific instance,
based on the parameter types of calls to that function (or uses of the structure).

During monomorphisation a new copy of the generic function is translated for each unique set of types the function is instantiated with. This is the same strategy used by C++. It results in fast code that is specialized for every call-site and statically dispatched, with the tradeoff that functions instantiated with many different types can cause "code bloat", where multiple function instances result in larger binaries than would be created with other translation strategies.

Functions that accept [trait objects](https://doc.rust-lang.org/book/trait-objects.html) instead of type parameters do not undergo monomorphisation. Instead, methods on the trait objects are dispatched dynamically at runtime.

<h3><a href="#whats-the-difference-between-a-function-and-a-closure-that-doesnt-capture" name="whats-the-difference-between-a-function-and-a-closure-that-doesnt-capture">
一个函数和一个没有捕获任何变量的闭包有什么区别？
</a></h3>

Functions and closures are operationally equivalent, but have different runtime representations due to their differing implementations.

Functions are a built-in primitive of the language, while closures are essentially syntactic sugar for one of three traits: [`Fn`][Fn], [`FnMut`][FnMut], and [`FnOnce`][FnOnce]. When you make a closure, the Rust compiler automatically creates a struct implementing the appropriate trait of those three and containing the captured environment variables as members, and makes it so the struct can be called as a function. Bare functions can not capture an environment.

The big difference between these traits is how they take the `self` parameter. [`Fn`][Fn] takes `&self`, [`FnMut`][FnMut] takes `&mut self`, and [`FnOnce`][FnOnce] takes `self`.

Even if a closure does not capture any environment variables, it is represented at runtime as two pointers, the same as any other closure.

<h3><a href="#what-are-higher-kinded-types" name="what-are-higher-kinded-types">
什么是更高级的类型，为什么我想要它们，为什么 Rust 没有它们？
</a></h3>

Higher-kinded types are types with unfilled parameters. Type constructors, like [`Vec`][Vec], [`Result`][Result], and [`HashMap`][HashMap] are all examples of higher-kinded types: each requires some additional type parameters in order to actually denote a specific type, like `Vec<u32>`. Support for higher-kinded types means these "incomplete" types may be used anywhere "complete" types can be used, including as generics for functions.

Any complete type, like [`i32`][i32], [`bool`][bool], or [`char`][char] is of kind `*` (this notation comes from the field of type theory). A type with one parameter, like [`Vec<T>`][Vec] is of kind `* -> *`, meaning that [`Vec<T>`][Vec] takes in a complete type like [`i32`][i32] and returns a complete type `Vec<i32>`. A type with three parameters, like [`HashMap<K, V, S>`][HashMap] is of kind `* -> * -> * -> *`, and takes in three complete types (like [`i32`][i32], [`String`][String], and [`RandomState`][RandomState]) to produce a new complete type `HashMap<i32, String, RandomState>`.

In addition to these examples, type constructors can take *lifetime* arguments, which we'll denote as `Lt`. For example, `slice::Iter` has kind `Lt -> * -> *`, because it must be instantiated like `Iter<'a, u32>`.

The lack of support for higher-kinded types makes it difficult to write certain kinds of generic code. It's particularly problematic for abstracting over concepts like iterators, since iterators are often parameterized over a lifetime at least. That in turn has prevented the creation of traits abstracting over Rust's collections.

Another common example is concepts like functors or monads, both of which are type constructors, rather than single types.

Rust doesn't currently have support for higher-kinded types because it hasn't been a priority compared to other improvements we want to make. Since the design is a major, cross-cutting change, we also want to approach it carefully. But there's no inherent reason for the current lack of support.

<h3><a href="#what-do-named-type-parameters-in-generic-types-mean" name="what-do-named-type-parameters-in-generic-types-mean">
通用类型中 <code>&lt;T=Foo&gt;</code> 这样的命名类型参数是什么意思？
</a></h3>

These are called [associated types](https://doc.rust-lang.org/stable/book/associated-types.html), and they allow for the expression of trait bounds that can't be expressed with a `where` clause. For example, a generic bound `X: Bar<T=Foo>` means "`X` must implement the trait `Bar`, and in that implementation of `Bar`, `X` must choose `Foo` for `Bar`'s associated type, `T`." Examples of where such a constraint cannot be expressed via a `where` clause include trait objects like `Box<Bar<T=Foo>>`.

Associated types exist because generics often involve families of types, where one type determines all of the others in a family. For example, a trait for graphs might have as its `Self` type the graph itself, and have associated types for nodes and for edges. Each graph type uniquely determines the associated types. Using associated types makes it much more concise to work with these families of types, and also provides better type inference in many cases.

<h3><a href="#how-do-i-overload-operators" name="how-do-i-overload-operators">
我可以重载运算符吗？哪些可以，怎么做？
</a></h3>

You can provide custom implementations for a variety of operators using their associated traits: [`Add`][Add] for `+`, [`Mul`][Mul] for `*`, and so on. It looks like this:

```rust
use std::ops::Add;

struct Foo;

impl Add for Foo {
    type Output = Foo;
    fn add(self, rhs: Foo) -> Self::Output {
        println!("Adding!");
        self
    }
}
```

The following operators can be overloaded:

| Operation            | Trait                          |
|:---------------------|:-------------------------------|
| `+`                  | [`Add`][Add]                   |
| `+=`                 | [`AddAssign`][AddAssign]       |
| `binary -`           | [`Sub`][Sub]                   |
| `-=`                 | [`SubAssign`][SubAssign]       |
| `*`                  | [`Mul`][Mul]                   |
| `*=`                 | [`MulAssign`][MulAssign]       |
| `/`                  | [`Div`][Div]                   |
| `/=`                 | [`DivAssign`][DivAssign]       |
| `unary -`            | [`Neg`][Neg]                   |
| `%`                  | [`Rem`][Rem]                   |
| `%=`                 | [`RemAssign`][RemAssign]       |
| `&`                  | [`BitAnd`][BitAnd]             |
| <code>&#124;</code>  | [`BitOr`][BitOr]               |
| <code>&#124;</code>= | [`BitOrAssign`][BitOrAssign]   |
| `^`                  | [`BitXor`][BitXor]             |
| `^=`                 | [`BitXorAssign`][BitXorAssign] |
| `!`                  | [`Not`][Not]                   |
| `<<`                 | [`Shl`][Shl]                   |
| `<<=`                | [`ShlAssign`][ShlAssign]       |
| `>>`                 | [`Shr`][Shr]                   |
| `>>=`                | [`ShrAssign`][ShrAssign]       |
| `*`                  | [`Deref`][Deref]               |
| `mut *`              | [`DerefMut`][DerefMut]         |
| `[]`                 | [`Index`][Index]               |
| `mut []`             | [`IndexMut`][IndexMut]         |

<h3><a href="#why-the-split-between-eq-partialeq-and-ord-partialord" name="why-the-split-between-eq-partialeq-and-ord-partialord">
Why the split between <code>Eq</code>/<code>PartialEq</code> and <code>Ord</code>/<code>PartialOrd</code>?
</a></h3>

There are some types in Rust whose values are only partially ordered, or have only partial equality. Partial ordering means that there may be values of the given type that are neither less than nor greater than each other. Partial equality means that there may be values of the given type that are not equal to themselves.

Floating point types ([`f32`][f32] and [`f64`][f64]) are good examples of each. Any floating point type may have the value `NaN` (meaning "not a number"). `NaN` is not equal to itself (`NaN == NaN` is false), and not less than or greater than any other floating point value. As such, both [`f32`][f32] and [`f64`][f64] implement [`PartialOrd`][PartialOrd] and [`PartialEq`][PartialEq] but not [`Ord`][Ord] and not [`Eq`][Eq].

As explained in [the earlier question on floats](#why-cant-i-compare-floats), these distinctions are important because some collections rely on total orderings/equality in order to give correct results.

<h2 id="input-output">输入 / 输出</h2>

<h3><a href="#how-do-i-read-a-file-into-a-string" name="how-do-i-read-a-file-into-a-string">
如何将文件读入一个 <code>String</code>？
</a></h3>

Using the [`read_to_string()`][read__read_to_string] method, which is defined on the [`Read`][Read] trait in [`std::io`][std-io].

```rust
use std::io::Read;
use std::fs::File;

fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut f = try!(File::open(path));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));  // `s` contains the contents of "foo.txt"
    Ok(s)
}

fn main() {
    match read_file("foo.txt") {
        Ok(_) => println!("Got file contents!"),
        Err(err) => println!("Getting file contents failed with error: {}", err)
    };
}
```

<h3><a href="#how-do-i-read-file-input-efficiently" name="how-do-i-read-file-input-efficiently">
如何有效地读取文件输入？
</a></h3>

The [`File`][File] type implements the [`Read`][Read] trait, which has a variety of functions for reading and writing data, including [`read()`][read__read], [`read_to_end()`][read__read_to_end], [`bytes()`][read__bytes], [`chars()`][read__chars], and [`take()`][read__take]. Each of these functions reads a certain amount of input from a given file. [`read()`][read__read] reads as much input as the underlying system will provide in a single call. [`read_to_end()`][read__read_to_end] reads the entire buffer into a vector, allocating as much space as is needed. [`bytes()`][read__bytes] and [`chars()`][read__chars] allow you to iterate over the bytes and characters of the file, respectively. Finally, [`take()`][read__take] allows you to read up to an arbitrary number of bytes from the file. Collectively, these should allow you to efficiently read in any data you need.

For buffered reads, use the [`BufReader`][BufReader] struct, which helps to reduce the number of system calls when reading.

<h3><a href="#how-do-i-do-asynchronous-input-output-in-rust" name="how-do-i-do-asynchronous-input-output-in-rust">
Rust 中如何进行异步输入/输出？
</a></h3>

There are several libraries providing asynchronous input / output in Rust, including [mioco](https://github.com/dpc/mioco), [coio-rs](https://github.com/zonyitoo/coio-rs), and [rotor](https://github.com/tailhook/rotor).

<h3><a href="#how-do-i-get-command-line-arguments" name="how-do-i-get-command-line-arguments">
如何在 Rust 中获取命令行参数？
</a></h3>

The easiest way is to use [`Args`][Args], which provides an iterator over the input arguments.

If you're looking for something more powerful, there are a [number of options on crates.io](https://crates.io/keywords/argument).

<h2 id="error-handling">错误处理</h2>

<h3><a href="#why-doesnt-rust-have-exceptions" name="why-doesnt-rust-have-exceptions">
为什么 Rust 没有异常？
</a></h3>

Exceptions complicate understanding of control-flow, they express validity/invalidity outside of the type system, and they interoperate poorly with multithreaded code (a major focus of Rust).

Rust prefers a type-based approach to error handling, which is [covered at length in the book](https://doc.rust-lang.org/stable/book/error-handling.html). This fits more nicely with Rust's control flow, concurrency, and everything else.

<h3><a href="#whats-the-deal-with-unwrap" name="whats-the-deal-with-unwrap">
What's the deal with <code>unwrap()</code> everywhere?
</a></h3>

`unwrap()` is a function that extracts the value inside an [`Option`][Option] or [`Result`][Result] and panics if no value is present.

`unwrap()` shouldn't be your default way to handle errors you expect to arise, such as incorrect user input. In production code, it should be treated like an assertion that the value is non-empty, which will crash the program if violated.

It's also useful for quick prototypes where you don't want to handle an error yet, or blog posts where error handling would distract from the main point.

<h3><a href="#why-do-i-get-errors-with-try" name="why-do-i-get-errors-with-try">
为什么我尝试运行使用 <code>try!</code> 宏的示例代码时收到错误？
</a></h3>

It's probably an issue with the function's return type. The [`try!`][TryMacro] macro either extracts the value from a [`Result`][Result], or returns early with the error [`Result`][Result] is carrying. This means that [`try`][TryMacro] only works for functions that return [`Result`][Result] themselves, where the `Err`-constructed type implements `From::from(err)`. In particular, this means that the [`try!`][TryMacro] macro cannot work inside the `main` function.

<h3><a href="#error-handling-without-result" name="error-handling-without-result">
Is there an easier way to do error handling than having <code>Result</code>s everywhere?
</a></h3>

If you're looking for a way to avoid handling [`Result`s][Result] in other people's code, there's always [`unwrap()`][unwrap], but it's probably not what you want. [`Result`][Result] is an indicator that some computation may or may not complete successfully. Requiring you to handle these failures explicitly is one of the ways that Rust encourages robustness. Rust provides tools like the [`try!` macro][TryMacro] to make handling failures ergonomic.

If you really don't want to handle an error, use [`unwrap()`][unwrap], but know that doing so means that the code panics on failure, which usually results in a shutting down the process.

<h2 id="concurrency">并发</h2>

<h3><a href="#can-i-use-static-values-across-threads-without-an-unsafe-block" name="can-i-use-static-values-across-threads-without-an-unsafe-block">
Can I use static values across threads without an <code>unsafe</code> block?
</a></h3>

Mutation is safe if it's synchronized. Mutating a static [`Mutex`][Mutex] (lazily initialized via the [lazy-static](https://crates.io/crates/lazy_static/) crate) does not require an `unsafe` block, nor does mutating a static [`AtomicUsize`][AtomicUsize] (which can be initialized without lazy_static).

More generally, if a type implements [`Sync`][Sync] and does not implement [`Drop`][Drop], it [can be used in a `static`](https://doc.rust-lang.org/book/const-and-static.html#static).

<h2 id="macros">宏</h2>

<h3><a href="#can-i-write-a-macro-to-generate-identifiers" name="can-i-write-a-macro-to-generate-identifiers">
Can I write a macro to generate identifiers?
</a></h3>

Not currently. Rust macros are ["hygienic macros"](https://en.wikipedia.org/wiki/Hygienic_macro), which intentionally avoid capturing or creating identifiers that may cause unexpected collisions with other identifiers. Their capabilities are significantly different than the style of macros commonly associated with the C preprocessor. Macro invocations can only appear in places where they are explicitly supported: items, method declarations, statements, expressions, and patterns. Here, "method declarations" means a blank space where a method can be put. They can't be used to complete a partial method declaration. By the same logic, they can't be used to complete a partial variable declaration.

<h2 id="debugging">调试和工具</h2>

<h3><a href="#how-do-i-debug-rust-programs" name="how-do-i-debug-rust-programs">
如何调试 Rust 程序？
</a></h3>

Rust programs can be debugged using [gdb](https://sourceware.org/gdb/current/onlinedocs/gdb/) or [lldb](http://lldb.llvm.org/tutorial.html), the same as C and C++. In fact, every Rust installation comes with one or both of rust-gdb and rust-lldb (depending on platform support). These are wrappers over gdb and lldb with Rust pretty-printing enabled.

<h3><a href="#how-do-i-locate-a-panic" name="how-do-i-locate-a-panic">
<code>rustc</code> said a panic occurred in standard library code. How do I locate the mistake in my code?
</a></h3>

This error is usually caused by [`unwrap()`ing][unwrap] a `None` or `Err` in client code. Enabling backtraces by setting the environment variable `RUST_BACKTRACE=1` helps with getting more information. Compiling in debug mode (the default for `cargo build`) is also helpful. Using a debugger like the provided `rust-gdb` or `rust-lldb` is also helpful.

<h3><a href="#what-ide-should-i-use" name="what-ide-should-i-use">
我该使用什么 IDE？
</a></h3>

Rust 有许多开发环境可供选择，详见官方的 [IDE 支持页面](https://forge.rust-lang.org/ides.html)。

<h3><a href="#wheres-rustfmt" name="wheres-rustfmt">
<code>gofmt</code> is great. Where's <code>rustfmt</code>?
</a></h3>

`rustfmt` is [right here](https://github.com/rust-lang-nursery/rustfmt), and is being actively developed to make reading Rust code as easy and predictable as possible.

<h2 id="low-level">底层</h2>

<h3><a href="#how-do-i-memcpy-bytes" name="how-do-i-memcpy-bytes">
如何 <code>memcpy</code> 字节？
</a></h3>

如果您想安全地克隆现有切片，可以使用 [`clone_from_slice`][clone_from_slice]。

要复制可能重叠的字节，使用 [`copy`][copy]. 要复制非重叠字节，使用 [`copy_nonoverlapping`][copy_nonoverlapping]。两个特性都是 `unsafe`，因此两者都可以颠覆本语言的安全保障，使用时请小心。

<h3><a href="#does-rust-work-without-the-standard-library" name="does-rust-work-without-the-standard-library">
没有标准库的 Rust 还有适当的功能吗？
</a></h3>

绝对可以。Rust 程序可以使用 `#![no_std]` 属性设置为不加载标准库。使用此属性集后，您可以继续使用 Rust 核心库，那些是与平台无关的原函数。也因此，它不包括 IO、并发、堆分配等。

<h3><a href="#can-i-write-an-operating-system-in-rust" name="can-i-write-an-operating-system-in-rust">
我可以用 Rust 写一个操作系统吗？
</a></h3>

行的！事实上，[有多个项目正在进行中](http://wiki.osdev.org/Rust)。

<h3><a href="#how-can-i-write-endian-independent-values" name="how-can-i-write-endian-independent-values">
如何以大端或小端格式读取与写入一个文件或其他字节流中的是 <code>i32</code> 或 <code>f64</code> 这样的数字类型？
</a></h3>

您应该检出 [byteorder 包装箱](http://burntsushi.net/rustdoc/byteorder/)，它提供了这种实用程序。

<h3><a href="#does-rust-guarantee-data-layout" name="does-rust-guarantee-data-layout">
Rust 是否保证特定的数据布局？
</a></h3>

默认不会。在一般情况下，`enum` 和 `struct` 布局是未定义的。这允许编译器进行潜在优化，例如重新使用 padding for the discriminant, compacting variants of nested `enum`s, reordering fields to remove padding, etc. `enums` which carry no data ("C-like") are eligible to have a defined representation. Such `enums` are easily distinguished in that they are simply a list of names that carry no data:

```rust
enum CLike {
    A,
    B = 32,
    C = 34,
    D
}
```

`#[repr(C)]` 属性可以应用到诸如 `enums` 以提供等同 C 语言中的表示。This allows using Rust `enum`s in FFI code where C `enum`s are also used, for most use cases. The attribute can also be applied to `struct`s to get the same layout as a C `struct` would.

<h2 id="cross-platform">跨平台</h2>

<!--
### How do I build a Windows binary that doesn't display the console window?

TODO: Write this answer.
-->

<!--
### How do I make the console-less binary not crash on panic!?

TODO: Write this answer.
-->

<h3><a href="#how-do-i-express-platform-specific-behavior" name="how-do-i-express-platform-specific-behavior">
在 Rust 中表达特定平台行为的惯用方式是什么？
</a></h3>

特定平台的行为可以使用[条件编译属性](https://doc.rust-lang.org/reference/attributes.html#conditional-compilation)，诸如 `target_os`、`target_family`、`target_endian` 等。

<h3><a href="#can-rust-be-used-for-android-ios-programs" name="can-rust-be-used-for-android-ios-programs">
Rust 可以用于 Android/iOS 编程吗？
</a></h3>

可以！已经有为 [Android](https://github.com/tomaka/android-rs-glue) 和 [iOS](https://www.bignerdranch.com/blog/building-an-ios-app-in-rust-part-1/) 使用 Rust 的例子。它需要一些工作来完成设置，但是 Rust 在这两个平台上都能正常工作。

<h3><a href="#can-i-run-my-rust-program-in-a-web-browser" name="can-i-run-my-rust-program-in-a-web-browser">
我可以在网页浏览器中运行我的 Rust 程序吗？
</a></h3>

目前还不行，但也正在努力使 Rust 能用 [Emscripten](https://kripken.github.io/emscripten-site/) 编译为 Web 版。

<h3><a href="#how-do-i-cross-compile-rust" name="how-do-i-cross-compile-rust">
如何在 Rust 中交叉编译？
</a></h3>

Rust 的交叉编译是可能的，它但需要[一些流程](https://github.com/japaric/rust-cross/blob/master/README.md)完成设置。每个 Rust 编译器都是一个交叉编译器，但库需要针对目标平台进行交叉编译。

Rust 确实为每个受支持平台方法[标准库副本](https://static.rust-lang.org/dist/index.html)，在分发目录上可以找到各构建目录，其中包含的 `rust-std-*` 文件就是它们，但尚没有自动安装它们的方法。

<h2 id="modules-and-crates">模块和包装箱</h2>

<h3><a href="#what-is-the-relationship-between-a-module-and-a-crate" name="what-is-the-relationship-between-a-module-and-a-crate">
模块与包装箱之间的关系是什么？
</a></h3>

- 一个包装箱是一个编译单元，它是 Rust 编译器可操作的最小代码量。
- 一个模块是一个（可能是嵌套的）放在一个包装箱内的代码单元。
- 一个包装箱包含一个隐含的、未命名的顶层模块。
- 递归定义可以跨越模块，但不能跨包装箱。

<h3><a href="#why-cant-the-rust-compiler-find-a-library-im-using" name="why-cant-the-rust-compiler-find-a-library-im-using">
为什么 Rust 编译器找不到我 <code>use</code> 的库？
</a></h3>

There are a number of possible answers, but a common mistake is not realizing that `use` declarations are relative to the crate root. Try rewriting your declarations to use the paths they would use if defined in the root file of your project and see if that fixes the problem.

There are also `self` and `super`, which disambiguate `use` paths as being relative to the current module or parent module, respectively.

For complete information on `use`ing libraries, read the Rust book's chapter ["Crates and Modules"](https://doc.rust-lang.org/stable/book/crates-and-modules.html).

<h3><a href="#why-do-i-have-to-declare-modules-with-mod" name="why-do-i-have-to-declare-modules-with-mod">
为什么我必须在包装箱顶层用 <code>mod</code> 声明模块文件，而不能直接 <code>use</code> 它们？
</a></h3>

有两种方法在 Rust 中声明模块：内链或者另一个文件。下面是两个例子：

```rust
// In main.rs
mod hello {
    pub fn f() {
        println!("hello!");
    }
}

fn main() {
    hello::f();
}
```

```rust
// In main.rs
mod hello;

fn main() {
    hello::f();
}

// In hello.rs
pub fn f() {
    println!("hello!");
}
```

在第一个例子中，模块在其使用的同一个文件中定义。第二个例子中，主文件中的模块声明告诉编译器找到 `hello.rs` 或 `hello/mod.rs`，然后加载该文件。

注意 `mod` 与 `use` 的差异：`mod` 声明一个模块的存在，而 `use` 引用一个在别处声明的模块，将其内容纳入到当前模块的范围。

<h3><a href="#how-do-i-configure-cargo-to-use-a-proxy" name="how-do-i-configure-cargo-to-use-a-proxy">
如何配置 Cargo 使用代理服务器？
</a></h3>

正如 Cargo 的[配置文档](http://doc.crates.io/config.html)所说，您可以设置配置文件中 `[http]` 下的“proxy”变量使 Cargo 使用一个代理服务器。

<h3><a href="#why-cant-the-compile-find-method-implementations" name="why-cant-the-compile-find-method-implementations">
为什么编译器找不到方法实现，即使我已经 <code>use</code> 包装箱？
</a></h3>

For methods defined on a trait, you have to explicitly import the trait declaration. This means it's not enough to import a module where a struct implements the trait, you must also import the trait itself.

<h3><a href="#why-cant-the-compiler-infer-use-statements" name="why-cant-the-compiler-infer-use-statements">
为什么编译器不能为我推断 <code>use</code> 声明？
</a></h3>

It probably could, but you also don't want it to. While in many cases it is likely that the compiler could determine the correct module to import by simply looking for where a given identifier is defined, this may not be the case in general. Any decision rule in `rustc` for choosing between competing options would likely cause surprise and confusion in some cases, and Rust prefers to be explicit about where names are coming from.

For example, the compiler could say that in the case of competing identifier definitions the definition from the earliest imported module is chosen. So if both module `foo` and module `bar` define the identifier `baz`, but `foo` is the first registered module, the compiler would insert `use foo::baz;`.

```rust
mod foo;
mod bar;

// use foo::baz  // to be inserted by the compiler.

fn main() {
  baz();
}
```

If you know this is going to happen, perhaps it saves a small number of keystrokes, but it also greatly increases the possibility for surprising error messages when you actually meant for `baz()` to be `bar::baz()`, and it decreases the readability of the code by making the meaning of a function call dependent on module declaration. These are not tradeoffs we are willing to make.

但在未来，IDE 可以帮助管理声明，这会在两方面带来更好的结果：机器人帮忙拉取名称，并明确声明这些名称来自何处。

<!--
### How do I package and archive crates from [https://crates.io](https://crates.io)?

TODO: Write this answer.
-->

<h3><a href="#how-do-i-do-dynamic-rust-library-loading" name="how-do-i-do-dynamic-rust-library-loading">
如何动态加载 Rust 库？
</a></h3>

在 Rust 中使用 [libloading](https://crates.io/crates/libloading) 导入动态库，它提供了一个用于动态链接的跨平台系统。

<h3><a href="#why-doesnt-crates-io-have-namespaces" name="why-doesnt-crates-io-have-namespaces">
为什么 crates.io 没有名字空间？
</a></h3>

Quoting the [official explanation](https://internals.rust-lang.org/t/crates-io-package-policies/1041) of [https://crates.io](https://crates.io)'s design:

> In the first month with crates.io, a number of people have asked us about the possibility of introducing [namespaced packages](https://github.com/rust-lang/crates.io/issues/58).<br><br>
>
> While namespaced packages allow multiple authors to use a single, generic name, they add complexity to how packages are referenced in Rust code and in human communication about packages. At first glance, they allow multiple authors to claim names like `http`, but that simply means that people will need to refer to those packages as `wycats' http` or `reem's http`, offering little benefit over package names like `wycats-http` or `reem-http`.<br><br>
>
> When we looked at package ecosystems without namespacing, we found that people tended to go with more creative names (like `nokogiri` instead of "tenderlove's libxml2"). These creative names tend to be short and memorable, in part because of the lack of any hierarchy. They make it easier to communicate concisely and unambiguously about packages. They create exciting brands. And we've seen the success of several 10,000+ package ecosystems like NPM and RubyGems whose communities are prospering within a single namespace.<br><br>
>
> In short, we don't think the Cargo ecosystem would be better off if Piston chose a name like `bvssvni/game-engine` (allowing other users to choose `wycats/game-engine`) instead of simply `piston`.<br><br>
>
> Because namespaces are strictly more complicated in a number of ways, and because they can be added compatibly in the future should they become necessary, we're going to stick with a single shared namespace.

<h2 id="libraries">库</h2>

<h3><a href="#how-can-i-make-an-http-request" name="how-can-i-make-an-http-request">
我该怎样发出一个 HTTP 请求？
</a></h3>

标准库不包含 HTTP 的实现，因此您需要用外部的包装箱。 [Hyper](https://github.com/hyperium/hyper) 是最流行的选择，但[也有众多其他可选](https://crates.io/keywords/http)。

<h3><a href="#how-can-i-write-a-gui-application" name="how-can-i-write-a-gui-application">
如何用 Rust 编写 GUI 应用程序？
</a></h3>

有多种方法在 Rust 中编写 GUI 应用程序。查阅我们的[GUI 框架列表](https://github.com/kud1ing/awesome-rust#gui)吧。

<h3><a href="#how-can-i-parse-json-xml" name="how-can-i-parse-json-xml">
如何解析 JSON/XML？
</a></h3>

[Serde](https://github.com/serde-rs/serde) 是推荐的将 Rust 数据序列化与反序列化为多种格式的库。

<h3><a href="#is-there-a-standard-2d-vector-crate" name="is-there-a-standard-2d-vector-crate">
有标准的 2D+ 矢量和形状包装箱吗？
</a></h3>

还没有呢！你能写一个吗？

<h3><a href="#how-do-i-write-an-opengl-app" name="how-do-i-write-an-opengl-app">
如何用 Rust 编写 OpenGL 应用程序？
</a></h3>

[Glium](https://github.com/tomaka/glium) is the major library for OpenGL programming in Rust. [GLFW](https://github.com/bjz/glfw-rs) is also a solid option.

<h3><a href="#can-i-write-a-video-game-in-rust" name="can-i-write-a-video-game-in-rust">
我能用 Rust 编写一个电子游戏吗？
</a></h3>

没问题！Rust 主要的游戏编程库是 [Piston](http://www.piston.rs/)，另有 [Rust 游戏编程的 reddit 板块](https://www.reddit.com/r/rust_gamedev/) 和 IRC 频道（[Mozilla IRC](https://wiki.mozilla.org/IRC) 上的 `#rust-gamedev`)。

<h2 id="design-patterns">设计模式</h2>

<h3><a href="#is-rust-object-oriented" name="is-rust-object-oriented">
Rust 是面向对象吗？
</a></h3>

它是多范式。你在面向对象（OO）语言中做的许多事情也能在 Rust 中做，但并不是所有都可以，并且不一定是你熟悉的方式。

<h3><a href="#how-do-i-map-object-oriented-concepts-to-rust" name="how-do-i-map-object-oriented-concepts-to-rust">
如何将面向对象的概念映射到 Rust？
</a></h3>

That depends. There _are_ ways of translating object-oriented concepts like [multiple inheritance](https://www.reddit.com/r/rust/comments/2sryuw/ideaquestion_about_multiple_inheritence/) to Rust, but as Rust is not object-oriented the result of the translation may look substantially different from its appearance in an OO language.

<h3><a href="#how-do-i-configure-a-struct-with-optional-parameters" name="how-do-i-configure-a-struct-with-optional-parameters">
如何处理有可选参数的结构体的配置？
</a></h3>

The easiest way is to use the [`Option`][Option] type in whatever function you're using to construct instances of the struct (usually `new()`). Another way is to use the [builder pattern](https://aturon.github.io/ownership/builders.html), where only certain functions instantiating member variables must be called before the construction of the built type.

<h3><a href="#how-do-i-do-global-variables" name="how-do-i-do-global-variables">
如何在 Rust 中做全局对象？
</a></h3>

Rust 中可以用 `const` 声明在编译时计算的全局常量，而 `static` 可用于可变的全局变量。请注意，修改一个 `static mut` 变量需要使用 `unsafe`，因为它允许数据竞争（races）， one of the things guaranteed not to happen in safe Rust. `const` 与 `static` 值的一项重要区别是，你可以引用 `static` 值，但不能引用 `const` 值，因为它没有一个特定的内存位置。有关 `const` 与 `static` 的更多信息，请阅读 [Rust 之书](https://doc.rust-lang.org/book/const-and-static.html)。

<h3><a href="#how-can-i-set-compile-time-constants-that-are-defined-procedurally" name="how-can-i-set-compile-time-constants-that-are-defined-procedurally">
如何设置程序定义的编译时的常量？
</a></h3>

Rust 目前对编译时常量的支持有限。您可以使用 `const` 声明（类似 `static`，但它不可变，并且在内存中没有特定位置）定义原函数，以及定义 `const` 函数和固有方法。

To define procedural constants that can't be defined via these mechanisms, use the [`lazy-static`](https://github.com/rust-lang-nursery/lazy-static.rs) crate, which emulates compile-time evaluation by automatically evaluating the constant at first use.

<h3><a href="#can-i-run-code-before-main" name="can-i-run-code-before-main">
我可以在 main 发生前运行初始化代码吗？
</a></h3>

Rust 没有“`main` 之前”的概念。你想找到的最接近的可能是 [`lazy-static`](https://github.com/Kimundi/lazy-static.rs) 包装箱，它在初次使用时通过懒惰初始化静态变量做到类似“在main之前”。

<!--

This answer needs significant work. Let's revise after the initial posting. --aturon

<h3><a href="#why-doesnt-rust-have-inheritance" name="why-doesnt-rust-have-inheritance">
Why doesn't Rust have inheritance?
</a></h3>

There are two meanings for the word "inheritance": _subtyping_, and _interface sharing_. Both purposes are already handled by traits.

For the first, subtyping exists for polymorphism, which traits already provide.

For the second, interface sharing is handled via trait methods, which define a collection of related functions that must be implemented for any implementation of the trait.

Rust has consistently worked to avoid having features with overlapping purposes, preferring to keep features orthogonal. For this reason, and given that the two major purposes are already handled by traits, Rust has opted not to include inheritance.

-->

<h3><a href="#does-rust-allow-non-constant-expression-values-for-globals" name="does-rust-allow-non-constant-expression-values-for-globals">
Rust 是否允许全局的非常量表达式值？
</a></h3>

No. Globals cannot have a non-constant-expression constructor and cannot have a destructor at all. Static constructors are undesirable because portably ensuring a static initialization order is difficult. Life before main is often considered a misfeature, so Rust does not allow it.

See the [C++ FQA](http://yosefk.com/c++fqa/ctors.html#fqa-10.12) about the "static initialization order fiasco", and [Eric Lippert's blog](https://ericlippert.com/2013/02/06/static-constructors-part-one/) for the challenges in C#, which also has this feature.

您可以以 [lazy-static](https://crates.io/crates/lazy_static/) 包装箱使用近似的非常量表达式全局对象。

<h2 id="other-languages">其他语言</h2>

<h3><a href="#how-can-i-use-static-fields" name="how-can-i-use-static-fields">
如何在 Rust 中实现类似 C 的 <code>struct X { static int X; };</code>？
</a></h3>

如上所说，Rust 没有 `static` 字段。不过，您可以在指定的模块中声明一个 `static` 变量，这样它将为该模块私有。

<h3><a href="#how-can-i-convert-a-c-style-enum-to-an-integer" name="how-can-i-convert-a-c-style-enum-to-an-integer">
如何将 C 风格的枚举转换为整数，反之亦然？
</a></h3>

Converting a C-style enum to an integer can be done with an `as` expression, like `e as i64` (where `e` is some enum).

Converting in the other direction can be done with a `match` statement, which maps different numeric values to different potential values for the enum.

<h3><a href="#why-do-rust-programs-use-more-memory-than-c" name="why-do-rust-programs-use-more-memory-than-c">
为什么 Rust 程序使用比 C 更多的内存？
</a></h3>

There are several factors that contribute to Rust programs having, by default, larger binary sizes than functionally-equivalent C programs. In general, Rust's preference is to optimize for the performance of real-world programs, not the size of small programs.

__Monomorphization__

Rust monomorphizes generics, meaning that a new version of a generic function or type is generated for each concrete type it's used with in the program. This is similar to how templates work in C++. For example, in the following program:

```rust
fn foo<T>(t: T) {
    // ... do something
}

fn main() {
    foo(10);       // i32
    foo("hello");  // &str
}
```

Two distinct versions of `foo` will be in the final binary, one specialized to an `i32` input, one specialized to a `&str` input. This enables efficient static dispatch of the generic function, but at the cost of a larger binary.

__调试符号__

Rust programs compile with some debug symbols retained, even when compiling in release mode. These are used for providing backtraces on panics, and can be removed with `strip`, or another debug symbol removal tool. It is also useful to note that compiling in release mode with Cargo is equivalent to setting optimization level 3 with rustc. An alternative optimization level (called `s` or `z`) [has recently landed](https://github.com/rust-lang/rust/pull/32386) and tells the compiler to optimize for size rather than performance.

__Jemalloc__

Rust uses jemalloc as the default allocator, which adds some size to compiled Rust binaries. Jemalloc is chosen because it is a consistent, quality allocator that has preferable performance characteristics compared to a number of common system-provided allocators. There is work being done to [make it easier to use custom allocators](https://github.com/rust-lang/rust/issues/32838), but that work is not yet finished.

__链接时优化__

Rust does not do link-time optimization by default, but can be instructed to do so. This increases the amount of optimization that the Rust compiler can potentially do, and can have a small effect on binary size. This effect is likely larger in combination with the previously mentioned size optimizing mode.

__标准库__

The Rust standard library includes libbacktrace and libunwind, which may be undesirable in some programs. Using `#![no_std]` can thus result in smaller binaries, but will also usually result in substantial changes to the sort of Rust code you're writing. Note that using Rust without the standard library is often functionally closer to the equivalent C code.

作为一个例子，下列 C 程序读入一个名称，并向该名称的人说“Hello”。

```c
#include <stdio.h>

int main(void) {
    printf("What's your name?\n");
    char input[100] = {0};
    scanf("%s", input);
    printf("Hello %s!\n", input);
    return 0;
}
```

将这用 Rust 重写，您可能得到这样的内容：

```rust
use std::io;

fn main() {
    println!("What's your name?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("Hello {}!", input);
}
```

This program, when compiled and compared against the C program, will have a larger binary and use more memory. But this program is not exactly equivalent to the above C code. The equivalent Rust code would instead look something like this:

```rust
#![feature(lang_items)]
#![feature(libc)]
#![feature(no_std)]
#![feature(start)]
#![no_std]

extern crate libc;

extern "C" {
    fn printf(fmt: *const u8, ...) -> i32;
    fn scanf(fmt: *const u8, ...) -> i32;
}

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    unsafe {
        printf(b"What's your name?\n\0".as_ptr());
        let mut input = [0u8; 100];
        scanf(b"%s\0".as_ptr(), &mut input);
        printf(b"Hello %s!\n\0".as_ptr(), &input);
        0
    }
}

#[lang="eh_personality"] extern fn eh_personality() {}
#[lang="panic_fmt"] fn panic_fmt() -> ! { loop {} }
#[lang="stack_exhausted"] extern fn stack_exhausted() {}
```

Which should indeed roughly match C in memory usage, at the expense of more programmer complexity, and a lack of static guarantees usually provided by Rust (avoided here with the use of `unsafe`).

<h3><a href="#why-no-stable-abi" name="why-no-stable-abi">
为什么 Rust 没有像 C 一样稳定的 ABI，以及为什么必须用 extern 来标注？
</a></h3>

Committing to an ABI is a big decision that can limit potentially advantageous language changes in the future. Given that Rust only hit 1.0 in May of 2015, it is still too early to make a commitment as big as a stable ABI. This does not mean that one won't happen in the future, though. (Though C++ has managed to go for many years without specifying a stable ABI.)

The `extern` keyword allows Rust to use specific ABI's, such as the well-defined C ABI, for interop with other languages.

<h3><a href="#can-rust-code-call-c-code" name="can-rust-code-call-c-code">
Rust 代码能调用 C 代码吗？
</a></h3>

可以。从 Rust 调用 C 代码与从 C++ 调用 C 代码一样高效。

<h3><a href="#can-c-code-call-rust-code" name="can-c-code-call-rust-code">
C 代码能调用 Rust 代码吗？
</a></h3>

可以。The Rust code has to be exposed via an `extern` declaration, which makes it C-ABI compatible. Such a function can be passed to C code as a function pointer or, if given the `#[no_mangle]` attribute to disable symbol mangling, can be called directly from C code.

<h3><a href="#why-rust-vs-cxx" name="why-rust-vs-cxx">
我已经写了完美的 C++。 Rust 能给我什么？
</a></h3>

Modern C++ includes many features that make writing safe and correct code less error-prone, but it's not perfect, and it's still easy to introduce unsafety. This is something the C++ core developers are working to overcome, but C++ is limited by a long history that predates a lot of the ideas they are now trying to implement.

Rust was designed from day one to be a safe systems programming language, which means it's not limited by historic design decisions that make getting safety right in C++ so complicated. In C++, safety is achieved by careful personal discipline, and is very easy to get wrong. In Rust, safety is the default. It gives you the ability to work in a team that includes people less perfect than you are, without having to spend your time double-checking their code for safety bugs.

<h3><a href="#how-to-get-cxx-style-template-specialization" name="how-to-get-cxx-style-template-specialization">
如何在 Rust 中做到 C++ 模板专业化 那样？
</a></h3>

Rust 目前还没有完全等同的模板专业化，这[正在研究](https://github.com/rust-lang/rfcs/pull/1210)并有希望尽快加入。不过，可以用[关联类型](https://doc.rust-lang.org/stable/book/associated-types.html)达成类似的效果。

<h3><a href="#how-does-ownership-relate-to-cxx-move-semantics" name="how-does-ownership-relate-to-cxx-move-semantics">
Rust 的所有权系统如何与 C++ 中的语义相关联？
</a></h3>

The underlying concepts are similar, but the two systems work very
differently in practice. In both systems, "moving" a value is a way to
transfer ownership of its underlying resources. For example, moving a
string would transfer the string's buffer rather than copying it.

In Rust, ownership transfer is the default behavior. For example, if I
write a function that takes a `String` as argument, this function will
take ownership of the `String` value supplied by its caller:

```rust
fn process(s: String) { }

fn caller() {
    let s = String::from("Hello, world!");
    process(s); // Transfers ownership of `s` to `process`
    process(s); // Error! ownership already transferred.
}
```

As you can see in the snippet above, in the function `caller`, the
first call to `process` transfers ownership of the variable `s`. The
compiler tracks ownership, so the second call to `process` results in
an error, because it is illegal to give away ownership of the same
value twice. Rust will also prevent you from moving a value if there
is an outstanding reference into that value.

C++ takes a different approach. In C++, the default is to copy a value
(to invoke the copy constructor, more specifically). However, callees
can declare their arguments using an "rvalue reference", like
`string&&`, to indicate that they will take ownership of some of the
resources owned by that argument (in this case, the string's internal
buffer). The caller then must either pass a temporary expression or
make an explicit move using `std::move`. The rough equivalent to the
function `process` above, then, would be:

```
void process(string&& s) { }

void caller() {
    string s("Hello, world!");
    process(std::move(s));
    process(std::move(s));
}
```

C++ 编译器 are not obligated to track moves. For example, the code
above compiles without a warning or error, at least using the default
settings on clang. Moreover, in C++ ownership of the string `s` itself
(if not its internal buffer) remains with `caller`, and so the
destructor for `s` will run when `caller` returns, even though it has
been moved (in Rust, in contrast, moved values are dropped only by
their new owners).

<h3><a href="#how-to-interoperate-with-cxx" name="how-to-interoperate-with-cxx">
如何让 Rust 与 C++ 互操作，反之亦然？
</a></h3>

Rust 与 C++ 可以通过 C 互操作。Rust 和 C++ 都提供一个适用于 C 的[外部函数接口](https://doc.rust-lang.org/book/ffi.html)，我们可以用它来相互沟通。如果编写 C 语言的绑定太麻烦，您随时可以用 [rust-bindgen](https://github.com/crabtw/rust-bindgen) 来帮忙自动生成可执行的 C 绑定。

<h3><a href="#does-rust-have-cxx-style-constructors" name="does-rust-have-cxx-style-constructors">
Rust 有 C++ 风格的构造函数吗？
</a></h3>

没有。Functions serve the same purpose as constructors without adding language complexity. The usual name for the constructor-equivalent function in Rust is `new()`, although this is just a convention rather than a language rule. The `new()` function in fact is just like any other function. An example of it looks like so:

```rust
struct Foo {
    a: i32,
    b: f64,
    c: bool,
}

impl Foo {
    fn new() -> Foo {
        Foo {
            a: 0,
            b: 0.0,
            c: false,
        }
    }
}
```

<h3><a href="#does-rust-have-copy-constructors" name="does-rust-have-copy-constructors">
Rust 有复制构造函数吗？
</a></h3>

不太完全。Types which implement `Copy` will do a standard C-like "shallow copy" with no extra work (similar to "plain old data" in C++). It is impossible to implement `Copy` types that require custom copy behavior. Instead, in Rust "copy constructors" are created by implementing the `Clone` trait, and explicitly calling the `clone` method. Making user-defined copy operators explicit surfaces the underlying complexity, making it easier for the developer to identify potentially expensive operations.

<h3><a href="#does-rust-have-move-constructors" name="does-rust-have-move-constructors">
Rust 有移动构造函数吗？
</a></h3>

没有。Values of all types are moved via `memcpy`. This makes writing generic unsafe code much simpler since assignment, passing and returning are known to never have a side effect like unwinding.

<h3><a href="#compare-go-and-rust" name="compare-go-and-rust">
Go 与 Rust 有何相似，及它们有什么不同？
</a></h3>

Rust 与 Go 有着完全不同的设计目标。下列差异不是全部的差异（它们太多而无法列出），而是较重要的一些：

- Rust is lower level than Go. For example, Rust does not require a garbage collector, whereas Go does. In general, Rust affords a level of control that is comparable to C or C++.
- Rust's focus is on ensuring safety and efficiency while also providing high-level affordances, while Go's is on being a small, simple language which compiles quickly and can work nicely with a variety of tools.
- Rust has strong support for generics, which Go does not.
- Rust has strong influences from the world of functional programming, including a type system which draws from Haskell's typeclasses. Go has a simpler type system, using interfaces for basic generic programming.

<h3><a href="#how-do-rust-traits-compare-to-haskell-typeclasses" name="how-do-rust-traits-compare-to-haskell-typeclasses">
Rust 特征与 Haskell 类型类相较如何？
</a></h3>

Rust traits are similar to Haskell typeclasses, but are currently not as powerful, as Rust cannot express higher-kinded types. Rust's associated types are equivalent to Haskell type families.

Some specific difference between Haskell typeclasses and Rust traits include:

- Rust traits have an implicit first parameter called `Self`. `trait Bar` in Rust corresponds to `class Bar self` in Haskell, and `trait Bar<Foo>` in Rust corresponds to `class Bar foo self` in Haskell.
- "Supertraits" or "superclass constraints" in Rust are written `trait Sub: Super`, compared to `class Super self => Sub self` in Haskell.
- Rust forbids orphan instances, resulting in different coherence rules in Rust compared to Haskell.
- Rust's `impl` resolution considers the relevant `where` clauses and trait bounds when deciding whether two `impl`s overlap, or choosing between potential `impl`s. Haskell only considers the constraints in the `instance` declaration, disregarding any constraints provided elsewhere.
- A subset of Rust's traits (the ["object safe"](https://github.com/rust-lang/rfcs/blob/master/text/0255-object-safety.md) ones) can be used for dynamic dispatch via trait objects. The same feature is available in Haskell via GHC's `ExistentialQuantification`.

<h2 id="documentation">文档</h2>

<h3><a href="#why-are-so-many-rust-answers-on-stackoverflow-wrong" name="why-are-so-many-rust-answers-on-stackoverflow-wrong">
为什么 Stack Overflow（问答网站）上的许多 Rust 回答是错的？
</a></h3>

Rust 语言已存在多年，而它在 2015 年 5 月才达成 1.0 版本。在那达成之前，语言曾经发生过很多变化，而问答网站上的答案可能只适用较旧的语言版本。

随着时间的推移，会有越来越多的答案适用于当前版本，从而改善这一问题。

<h3><a href="#where-do-i-report-issues-in-the-rust-documentation" name="where-do-i-report-issues-in-the-rust-documentation">
我该在哪报告 Rust 文档的问题？
</a></h3>

您可以在 Rust 编译器的[问题跟踪器](https://github.com/rust-lang/rust/issues)中报告 Rust 文档的问题。在报告前，请先阅读[贡献指南](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#writing-documentation)。

<h3><a href="#how-do-i-view-rustdoc-documentation-for-a-library-my-project-depends-on" name="how-do-i-view-rustdoc-documentation-for-a-library-my-project-depends-on">
如何查看我的项目所依赖的库的 rustdoc 文档？
</a></h3>

When you use `cargo doc` to generate documentation for your own project, it also generates docs for the active dependency versions. These are put into the `target/doc` directory of your project. Use `cargo doc --open` to open the docs after building them, or just open up `target/doc/index.html` yourself.

[Vec]: https://doc.rust-lang.org/stable/std/vec/struct.Vec.html
[HashMap]: https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html
[Into]: https://doc.rust-lang.org/stable/std/convert/trait.Into.html
[From]: https://doc.rust-lang.org/stable/std/convert/trait.From.html
[Eq]: https://doc.rust-lang.org/stable/std/cmp/trait.Eq.html
[PartialEq]: https://doc.rust-lang.org/stable/std/cmp/trait.PartialEq.html
[Ord]: https://doc.rust-lang.org/stable/std/cmp/trait.Ord.html
[PartialOrd]: https://doc.rust-lang.org/stable/std/cmp/trait.PartialOrd.html
[f32]: https://doc.rust-lang.org/stable/std/primitive.f32.html
[f64]: https://doc.rust-lang.org/stable/std/primitive.f64.html
[i32]: https://doc.rust-lang.org/stable/std/primitive.i32.html
[i64]: https://doc.rust-lang.org/stable/std/primitive.i64.html
[bool]: https://doc.rust-lang.org/stable/std/primitive.bool.html
[Hash]: https://doc.rust-lang.org/stable/std/hash/trait.Hash.html
[BTreeMap]: https://doc.rust-lang.org/stable/std/collections/struct.BTreeMap.html
[VecMacro]: https://doc.rust-lang.org/stable/std/macro.vec!.html
[String]: https://doc.rust-lang.org/stable/std/string/struct.String.html
[to_string]: https://doc.rust-lang.org/stable/std/string/trait.ToString.html#tymethod.to_string
[str]: https://doc.rust-lang.org/stable/std/primitive.str.html
[str__find]: https://doc.rust-lang.org/stable/std/primitive.str.html#method.find
[str__as_bytes]: https://doc.rust-lang.org/stable/std/primitive.str.html#method.as_bytes
[u8]: https://doc.rust-lang.org/stable/std/primitive.u8.html
[char]: https://doc.rust-lang.org/stable/std/primitive.char.html
[Weak]: https://doc.rust-lang.org/stable/std/rc/struct.Weak.html
[IntoIterator]: https://doc.rust-lang.org/stable/std/iter/trait.IntoIterator.html
[Rc]: https://doc.rust-lang.org/stable/std/rc/struct.Rc.html
[UnsafeCell]: https://doc.rust-lang.org/stable/std/cell/struct.UnsafeCell.html
[Copy]: https://doc.rust-lang.org/stable/std/marker/trait.Copy.html
[Clone]: https://doc.rust-lang.org/stable/std/clone/trait.Clone.html
[Cell]: https://doc.rust-lang.org/stable/std/cell/struct.Cell.html
[RefCell]: https://doc.rust-lang.org/stable/std/cell/struct.RefCell.html
[Cow]: https://doc.rust-lang.org/stable/std/borrow/enum.Cow.html
[Deref]: https://doc.rust-lang.org/stable/std/ops/trait.Deref.html
[Arc]: https://doc.rust-lang.org/stable/std/sync/struct.Arc.html
[Box]: https://doc.rust-lang.org/stable/std/boxed/struct.Box.html
[Option]: https://doc.rust-lang.org/stable/std/option/enum.Option.html
[Fn]: https://doc.rust-lang.org/stable/std/ops/trait.Fn.html
[FnMut]: https://doc.rust-lang.org/stable/std/ops/trait.FnMut.html
[FnOnce]: https://doc.rust-lang.org/stable/std/ops/trait.FnOnce.html
[Result]: https://doc.rust-lang.org/stable/std/result/enum.Result.html
[RandomState]: https://doc.rust-lang.org/stable/std/collections/hash_map/struct.RandomState.html
[Add]: https://doc.rust-lang.org/stable/std/ops/trait.Add.html
[AddAssign]: https://doc.rust-lang.org/stable/std/ops/trait.AddAssign.html
[Sub]: https://doc.rust-lang.org/stable/std/ops/trait.Sub.html
[SubAssign]: https://doc.rust-lang.org/stable/std/ops/trait.SubAssign.html
[Mul]: https://doc.rust-lang.org/stable/std/ops/trait.Mul.html
[MulAssign]: https://doc.rust-lang.org/stable/std/ops/trait.MulAssign.html
[Div]: https://doc.rust-lang.org/stable/std/ops/trait.Div.html
[DivAssign]: https://doc.rust-lang.org/stable/std/ops/trait.DivAssign.html
[Neg]: https://doc.rust-lang.org/stable/std/ops/trait.Neg.html
[Rem]: https://doc.rust-lang.org/stable/std/ops/trait.Rem.html
[RemAssign]: https://doc.rust-lang.org/stable/std/ops/trait.RemAssign.html
[BitAnd]: https://doc.rust-lang.org/stable/std/ops/trait.BitAnd.html
[BitAndAssign]: https://doc.rust-lang.org/stable/std/ops/trait.BitAndAssign.html
[BitOr]: https://doc.rust-lang.org/stable/std/ops/trait.BitOr.html
[BitOrAssign]: https://doc.rust-lang.org/stable/std/ops/trait.BitOrAssign.html
[BitXor]: https://doc.rust-lang.org/stable/std/ops/trait.BitXor.html
[BitXorAssign]: https://doc.rust-lang.org/stable/std/ops/trait.BitXorAssign.html
[Not]: https://doc.rust-lang.org/stable/std/ops/trait.Not.html
[Shl]: https://doc.rust-lang.org/stable/std/ops/trait.Shl.html
[ShlAssign]: https://doc.rust-lang.org/stable/std/ops/trait.ShlAssign.html
[Shr]: https://doc.rust-lang.org/stable/std/ops/trait.Shr.html
[ShrAssign]: https://doc.rust-lang.org/stable/std/ops/trait.ShrAssign.html
[Deref]: https://doc.rust-lang.org/stable/std/ops/trait.Deref.html
[DerefMut]: https://doc.rust-lang.org/stable/std/ops/trait.DerefMut.html
[Index]: https://doc.rust-lang.org/stable/std/ops/trait.Index.html
[IndexMut]: https://doc.rust-lang.org/stable/std/ops/trait.IndexMut.html
[read__read_to_string]: https://doc.rust-lang.org/stable/std/io/trait.Read.html#method.read_to_string
[Read]: https://doc.rust-lang.org/stable/std/io/trait.Read.html
[std-io]: https://doc.rust-lang.org/stable/std/io/index.html
[File]: https://doc.rust-lang.org/stable/std/fs/struct.File.html
[read__read]: https://doc.rust-lang.org/stable/std/io/trait.Read.html#tymethod.read
[read__read_to_end]: https://doc.rust-lang.org/stable/std/io/trait.Read.html#method.read_to_end
[read__bytes]: https://doc.rust-lang.org/stable/std/io/trait.Read.html#method.bytes
[read__chars]: https://doc.rust-lang.org/stable/std/io/trait.Read.html#method.chars
[read__take]: https://doc.rust-lang.org/stable/std/io/trait.Read.html#method.take
[BufReader]: https://doc.rust-lang.org/stable/std/io/struct.BufReader.html
[Args]: https://doc.rust-lang.org/stable/std/env/struct.Args.html
[TryMacro]: https://doc.rust-lang.org/stable/std/macro.try!.html
[unwrap]: https://doc.rust-lang.org/stable/core/option/enum.Option.html#method.unwrap
[Mutex]: https://doc.rust-lang.org/stable/std/sync/struct.Mutex.html
[AtomicUsize]: https://doc.rust-lang.org/stable/std/sync/atomic/struct.AtomicUsize.html
[Sync]: https://doc.rust-lang.org/stable/std/marker/trait.Sync.html
[Drop]: https://doc.rust-lang.org/stable/std/ops/trait.Drop.html
[clone_from_slice]: https://doc.rust-lang.org/stable/std/primitive.slice.html#method.clone_from_slice
[copy]: https://doc.rust-lang.org/stable/std/ptr/fn.copy.html
[copy_nonoverlapping]: https://doc.rust-lang.org/stable/std/ptr/fn.copy_nonoverlapping.html
[clone]: https://doc.rust-lang.org/stable/std/clone/trait.Clone.html#tymethod.clone
