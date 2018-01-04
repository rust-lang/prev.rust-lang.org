---
layout: zh-CN/faq
title: 常见问题解答 &middot; Rust 程序设计语言
---

# 常见问题解答

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
            <li><a href="#modules-and-crates">模块和包</a></li>
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

不是。Rust 始于 2006 年 Graydon Hoare 的兼职项目，并持续了三年。Mozilla 在语言成熟到足以运行基本测试并展示其核心概念时于 2009 年开始参与。虽然它由 Mozilla 资助，但 Rust 是由来自世界各地的多样化的发烧友开发的。[Rust 团队](https://www.rust-lang.org/team.html)由 Mozilla 和非 Mozilla 成员组成，`rustc`（Rust 的编译器）至今也有超过[1,900 名独立贡献者](https://github.com/rust-lang/rust/)。

就[项目治理](https://github.com/rust-lang/rfcs/blob/master/text/1068-rust-governance.md)而言，
Rust 由一个从全局角度出发，为项目设定愿景和优先事项的核心团队管理。
我们还有小组指导和促进特定兴趣领域的发展，包括核心语言、编译器、Rust 库、Rust 工具和官方 Rust 社区的管理。这些领域中的设计都先进地经过一个 [RFC 流程](https://github.com/rust-lang/rfcs)。对于不需要 RFC 的更改，决定是通过 [`rustc` 代码库](https://github.com/rust-lang/rust)上的拉取请求决定。

<h3><a href="#what-are-some-non-goals" name="what-are-some-non-goals">
哪些不是 Rust 的目标？
</a></h3>

1. 我们不使用任何特别尖端的技术。旧的、成熟的技术更好。
2. 我们不把表现力、极简或优雅看的比其他目标更重要。这些特性值得拥有，但这不是主要目标。
3. 我们不打算覆盖 C++ 或任何其他语言的完整功能集。Rust 应提供适用于大部分场景的功能。
4. 我们不打算做到 100% 静态， 100% 安全， 100% 反射或任何其他意义上的教条主义。存在权衡。
5. 我们不强求 Rust 能在「每个可能的平台」上运行。它必须没有不必要的妥协，最终适用于广泛使用的硬件和软件平台。

<h3><a href="#how-does-mozilla-use-rust" name="how-does-mozilla-use-rust">
Mozilla 的哪些项目使用 Rust？
</a></h3>

主要项目是 [Servo](https://github.com/servo/servo)，它是 Mozilla 正在雕琢的一个实验性的浏览器引擎。Mozilla 还在努力[整合 Rust 组件](https://bugzilla.mozilla.org/show_bug.cgi?id=1135640)到 Firefox。

<h3><a href="#what-examples-are-there-of-large-rust-projects" name="what-examples-are-there-of-large-rust-projects">
有哪些大型的 Rust 项目？
</a></h3>

目前两个最大的 Rust 开源项目是 [Servo](https://github.com/servo/servo) 和 [Rust 编译器](https://github.com/rust-lang/rust)本身。

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

[playpen](https://play.rust-lang.org/) 是尝试 Rust 的最简单方法，它是一个可编写和运行 Rust 代码的在线应用。如果想在你的系统上尝试 Rust，[安装](https://www.rust-lang.org/install.html)并尝试[猜谜游戏](https://doc.rust-lang.org/stable/book/second-edition/ch02-00-guessing-game-tutorial.html)教程。

<h3><a href="#how-do-i-get-help-with-rust-issues" name="how-do-i-get-help-with-rust-issues">
如何就 Rust 问题取得帮助？
</a></h3>

那有很多种方式。你可以尝试：

- 在 [users.rust-lang.org](https://users.rust-lang.org/) 发帖，这是官方的 Rust 用户论坛
- 在官方的 [Rust IRC 频道](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust)提问（irc.mozilla.org 上的 #rust）
- 在 [Stack Overflow](https://stackoverflow.com/questions/tagged/rust) 提问，加上「rust」tag
- 在 [/r/rust](https://www.reddit.com/r/rust) 发帖，这是非官方的 Rust 板块

<h3><a href="#why-has-rust-changed-so-much" name="why-has-rust-changed-so-much">
为什么 Rust 随时间推移在大幅变化？
</a></h3>

Rust 起步的目标是创建一个安全且合用的系统编程语言。为了追求这个目标，它探索了很多想法，其中一些（生命周期、trait）被保留，而其他则被丢弃（类型体系系统、绿色线程）。此外，由于早期的设计被升级以最好地使用 Rust 的特性，并提供高质量，一致的跨平台 API，在 2.0 之前，许多标准库被重写。现在的 Rust 已达到了 1.0，语言保证为「稳定」（stable）；虽然它可能继续演变，但运行于目前 Rust 上的代码，在未来的发行版本上应该能继续工作。

<h3><a href="#how-does-rust-language-versioning-work" name="how-does-rust-language-versioning-work">
Rust 语言的版本控制是怎样的？
</a></h3>

Rust 的语言版本遵循 [SemVer](http://semver.org/)，如果更改修复了编译器错误、补丁安全漏洞，或更改调度或类型推断需要附加注明，则稳定 API 的向后不兼容更改允许在次要版本中出现。次要版本更改的更详细指南可以在[语言](https://github.com/rust-lang/rfcs/blob/master/text/1122-language-semver.md)和[标准库](https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md)的已批准 RFC 中找到。

Rust 维护三个「发行频道」：稳定版（stable）、测试版（beta）和开发版（nightly）。稳定版和测试版每六周更新一次，而在那时的开发版会变为新的测试版，测试版变为新的稳定版。标记为不稳定或者隐藏在特性门控后的语言和标准库特性只能在开发版上使用，新特性最初会被标记为不稳定，一旦被核心团队和相关的子团队批准的话就变成「通过门控的（ungated）」，这种方法允许实验性变更，并同时为稳定频道提供强有力的向后兼容保证。

就相关的其他详细信息，请阅读 Rust 博客 [「Stability as a Deliverable」](http://blog.rust-lang.org/2014/10/30/Stability.html)。

<h3><a href="#can-i-use-unstable-features-on-the-beta-or-stable-channel" name="can-i-use-unstable-features-on-the-beta-or-stable-channel">
我可以在 beta 或 stable 频道上使用不稳定的功能吗？
</a></h3>

并不能。Rust 努力保证测试版和稳定版的特性稳定性。当某些特性不稳定时，意味着我们还不能提供这种保证，不希望开发者依赖它。这给了我们在开发版上适时尝试改变的机会，同时继续维护开发者寻求的稳定性。

测试和稳定版每六周更新一次是固定的，偶然有测试版的修正被及时接受，你不想使用开发版而等待希望的某个特性的话，你可以通过在 issue tracker 上查看 [`B-unstable`](https://github.com/rust-lang/rust/issues?q=is%3Aissue+is%3Aopen+tracking+label%3AB-unstable) tag 跟踪尚存的问题。


<h3><a href="#what-are-feature-gates" name="what-are-feature-gates">
什么是「特性门控」（Feature Gates）？
</a></h3>

「特性门控」（Feature Gates）是 Rust 用来稳定编译器、语言和标准库特性的机制。一个受「门控」的特性只能在 nightly 发布渠道才能使用，且必须显式指定 `#[feature]` 属性或者命令行参数 `-Z unstable-options`。当一个特性稳定了，它才能在稳定版上可用，不需要显式启用。此时，这个特性被认为是通过门控的（ungated）。特性门控允许开发者在稳定版提供之前，在开发中测试试验性的功能。

<h3><a href="#why-a-dual-mit-asl2-license" name="why-a-dual-mit-asl2-license">
为什么采用 MIT/ASL2 双许可证？
</a></h3>

Apache 许可证包含对专利侵权的重要保护，但它与 GPL 第二版不兼容。为避免 Rust 使用 GPL2 会遇到的问题，因而同时采用 MIT 许可证。

<h3><a href="#why-a-permissive-license" name="why-a-permissive-license">
为什么是 BSD 风格的许可证，而不是 MPL 或三个许可证？
</a></h3>

这一部分是由于原始开发者（Graydon）的偏好，另一部分原因在于，语言倾向于拥有更广泛的受众群体，以及将其更多样的嵌入和提供到终端，例如网页浏览器。我们希望呼吁尽可能多地潜在贡献者。

<h2 id="performance">性能</h2>

<h3><a href="#how-fast-is-rust" name="how-fast-is-rust">
Rust 有多快？
</a></h3>

飞快！Rust 在一些基准测试中（例如 [Benchmarks Game](https://benchmarksgame.alioth.debian.org/u64q/compare.php?lang=rust&lang2=gpp)、[等等](https://github.com/kostya/benchmarks)）已经有与惯用的 C 和 C++ 竞争的能力。

与 C++ 类似，Rust 采用[零成本抽象](http://blog.rust-lang.org/2015/05/11/traits.html)作为其核心原则之一：Rust 的抽象都没有施加全局性能损失，也不会有任何传统意义上的运行时系统的开销。

鉴于 Rust 建立在 LLVM 上，以及力求从 LLVM 的角度来看 C 语言，任何 LLVM 的性能改进也都有助于 Rust。从长远来看，Rust 的类型系统中更丰富的信息也应该能够实现对于 C / C++ 代码来说困难或不可能的优化。

<h3><a href="#is-rust-garbage-collected" name="is-rust-garbage-collected">
Rust 会垃圾收集吗？
</a></h3>

不会。Rust 的重要创新之一就是保证内存安全（无段错误 segfaults），*无需*垃圾收集。

通过避免垃圾收集（GC），Rust 可以提供许多益处：可预测的资源清理，降低内存管理开销，基本上没有运行时系统。这些特征使 Rust 很容易嵌入到任意上下文，并使其更容易[集成 Rust 代码到有 GC 的语言](http://calculist.org/blog/2015/12/23/neon-node-rust/)。

Rust 通过其所有权和借用（borrowing）系统避免了垃圾收集的需求，而同一系统也有助于解决许多其他问题，包括[一般的资源管理](http://blog.skylight.io/rust-means-never-having-to-close-a-socket/)和[并发](http://blog.rust-lang.org/2015/04/10/Fearless-Concurrency.html)。

如果单一的所有权不够，Rust 程序依赖标准的引用计数智能指针类型，[`Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html)，及其线程安全版的 [`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html)，而不是 GC。

不过，我们也在调查*可选*的垃圾收集作为一项未来扩展。
其目标旨在顺利与有垃圾收集的运行时整合，
例如由
[Spidermonkey](https://developer.mozilla.org/en-US/docs/Mozilla/Projects/SpiderMonkey)
和 [V8](https://developers.google.com/v8/?hl=en) JavaScript 引擎所提供的那些。 最后，有人已调研了无编译器支持实现的[纯 Rust 垃圾收集器](https://manishearth.github.io/blog/2015/09/01/designing-a-gc-in-rust/)
。

<h3><a href="#why-is-my-program-slow" name="why-is-my-program-slow">
为什么我的程序很慢？
</a></h3>

如非要求，Rust 编译器不会优化编译，[因为优化会编译速度，且在开发过程中通常是不合需要](https://users.rust-lang.org/t/why-does-cargo-build-not-optimise-by-default/4150/3)。

如果你使用 `cargo` 编译，请使用 `--release` 标志。如果你直接使用 `rustc`，使用 `-O` 标志。两者都将优化编译。

<h3><a href="#why-is-rustc-slow" name="why-is-rustc-slow">
Rust 编译似乎很慢。这是为什么？
</a></h3>

代码转换和优化。Rust 提供了高级抽象，它可以编译成高效的机器代码，而这些翻译需要时间来运行，特别是优化。

但 Rust 的编译时间并不像看起来那么糟糕，并有理由相信会有所改善。当比较 C++ 与 Rust 的类似大小的项目时，整个项目的编译时间一般被认为是可比的。Rust 编译缓慢的一般认识很大程度上是由于 C++ 与 Rust 的*编译模型*的差异：C++ 的编译单元是文件，而 Rust 则是包（crate），它由很多文件组成。因此，在开发过程中，修改单个 C++ 文件导致的重新编译比 Rust 更少。正在进行的重大工作将重构编译器来引入[增量编译](https://github.com/rust-lang/rfcs/blob/master/text/1298-incremental-compilation.md)，这将使 Rust 的编译时间变得比 C++ 的模型更有优势。

除了编译模型外，Rust 的语言设计和编译器实现的其他几个方面也影响了编译时性能。

首先，Rust 有中等复杂类型的系统，并且必须花费不可忽略的编译时间来强制在运行时使 Rust 安全的约束。

其次，Rust 编译器遭受着长期的技术债务，特别是生成质量差的 LLVM IR，而 LLVM 必须花时间「修复」。 在 Rust 编译器内部增加了一个名为 [MIR](https://github.com/rust-lang/rfcs/blob/master/text/1211-mir.md) 的新的内部表示方式，为执行更多优化和提升 LLVM IR 的生成质量提供了潜在的可能，然而这项工作尚未展开。

第三，Rust 使用的 LLVM 代码生成是一把双刃剑：虽然它能够使 Rust 具有世界一流的运行时性能，但 LLVM 是一个不重视编译时性能的大型框架，特别是在使用较差输入质量时。

最后，虽然 Rust 的单态（monomorphising）泛型（C ++  风格                                                          ）的首选策略产生快速代码，但它需要比其他翻译策略产生更多的代码。Rust 程序员可以使用 trait 对象通过动态调度来抵消这个代码膨胀。

<h3><a href="#why-are-rusts-hashmaps-slow" name="why-are-rusts-hashmaps-slow">
为什么 Rust 的 <code>HashMap</code> 很慢？
</a></h3>

默认情况下，Rust 的 [`HashMap`][HashMap] 使用 [SipHash](https://131002.net/siphash/) 哈希算法，其旨在防止[哈希表碰撞攻击](http://programmingisterrible.com/post/40620375793/hash-table-denial-of-service-attacks-revisited)，同时[在各种工作负载上提供合理的性能](https://www.reddit.com/r/rust/comments/3hw9zf/rust_hasher_comparisons/cub4oh6)。

虽然 SipHash 在许多情况下[表现出竞争优势](http://cglab.ca/%7Eabeinges/blah/hash-rs/)，但其中一个比其它哈希算法要慢的情况是使用短键，例如整数。这就是为什么 Rust 程序员经常观察到 [`HashMap`][HashMap] 表现不佳的原因。在这些情况下，经常推荐 [FNV 哈希](https://crates.io/crates/fnv)，但请注意，它不具备与 SipHash 相同的防碰撞性。

<h3><a href="#why-is-there-no-integrated-benchmarking" name="why-is-there-no-integrated-benchmarking">
为什么没有集成的基准测试基础设施？
</a></h3>

有，但它只在 nightly 发行频道上可用。我们最终计划为集成的基准建立一个可插拔系统，但同时，目前的系统[被认为是不稳定的](https://github.com/rust-lang/rust/issues/29553)。

<h3><a href="#does-rust-do-tail-call-optimization" name="does-rust-do-tail-call-optimization">
Rust 是否有做尾部调用优化？
</a></h3>

一般来说，没有。尾部调用优化可能在[有限的情况](http://llvm.org/docs/CodeGenerator.html#sibling-call-optimization)下进行，但是[不能保证](https://mail.mozilla.org/pipermail/rust-dev/2013-April/003557.html)。由于该功能总归是需要的， Rust 保留了一个关键字（`become`），尽管目前还不清楚它在技术上是否可行，以及它是否会被实现。有一个[提议的扩展](https://github.com/rust-lang/rfcs/pull/81)，将允许在某些情况下消除尾部调用，但目前被推迟了。

<h3><a href="#does-rust-have-a-runtime" name="does-rust-have-a-runtime">
Rust 有运行时吗？
</a></h3>

没有像 Java 语言这样典型意义上的运行时。但 Rust 标准库的一部分可以被认为是一个「运行时」，它提供了堆（heap），回溯（backtraces），展开（unwinding）和栈保护（stack guards）。在用户的 `main` 函数之前只运行[很少的初始化代码](https://github.com/rust-lang/rust/blob/33916307780495fe311fe9c080b330d266f35bfb/src/libstd/rt.rs#L43)。Rust 标准库还链接到 C 标准库，执行类似的[运行时初始化](http://www.embecosm.com/appnotes/ean9/html/ch05s02.html)。 Rust 代码可以在没有标准库的情况下编译，在这种情况下，运行时大概相当于 C。

<h2 id="syntax">语法</h2>

<h3><a href="#why-curly-braces" name="why-curly-braces">
为什么用花括号？为什么 Rust 的语法不能像 Haskell 或者 Python 那样？
</a></h3>

使用花括号表示块是各种编程语言中常见的设计选择，Rust 与此一致，对于那些已经熟悉这种风格的人员来说很有用。

对程序员和编译器中的更简单的解析器来说，花括号允许更加灵活的语法。

<h3><a href="#why-brackets-around-blocks" name="why-brackets-around-blocks">
我可以在 <code>if</code> 条件中省略括号，那为什么我还必须在单行块上面加括号？为什么不允许 C 的风格？
</a></h3>

鉴于 C 强制要求 `if` 条件语句加括号，但花括号可选。 Rust 在 `if` 表达式中作出了相反的选择。这使得条件明确地与主体分离，并避免了可选花括号可能导致的在重构时易漏的错误，就像 Apple 的 [goto 故障](https://gotofail.com/) bug。

<h3><a href="#why-no-literal-syntax-for-dictionaries" name="why-no-literal-syntax-for-dictionaries">
为什么字典类型没有字面语法？
</a></h3>

Rust 的整体设计偏好是限制*语言*的大小，同时支持强大的*库*。Rust 确实为数组和字符串字面值提供了初始化语法，这些是该语言中仅有的内建集合类型。其它由库定义的类型，包括无处不在的 [`Vec`][Vec] 集合类型，使用像 [`vec!`][VecMacro] 这样的宏来进行初始化。

这种使用 Rust 宏机制来初始化集合的设计选择，在将来很可能会被普遍地扩展到其它集合类型，不仅可以简化 [`HashMap`][HashMap] 和 [`Vec`][Vec] 的初始化，还可以用于其它类型如 [`BTreeMap`][BTreeMap]。同时，如果你想要一个更加方便的语法来初始化集合，你可以通过[创建你自己的宏](https://stackoverflow.com/questions/27582739/how-do-i-create-a-hashmap-literal)来提供。

<h3><a href="#when-should-i-use-an-implicit-return" name="when-should-i-use-an-implicit-return">
我什么时候应该使用隐式返回？
</a></h3>

Rust 是一个非常面向表达式的语言，「隐式返回」是设计的一部分。 像 `if`，`match` 的结构和正常的块，在 Rust 中都是表达式。例如，以下代码检查一个 [`i64`][i64] 是否是奇数，通过简单地将将其作为值来返回结果：

```rust
fn is_odd(x: i64) -> bool {
    if x % 2 != 0 { true } else { false }
}
```

然而，它还可以更进一步简化为：

```rust
fn is_odd(x: i64) -> bool {
    x % 2 != 0
}
```

在每个示例中，函数的最后一行是该函数的返回值。重要的是要注意，如果一个函数以分号结尾，它的返回类型将是 `()` ，表示没有返回值。隐式返回必须省略分号才能起作用。

只有在隐式返回不可用的时候才使用显式返回，因为你在函数体结尾之前返回。虽然上述每个函数都可以写成 `return` 关键字加上分号，但这样做是不必要的冗余，而且与 Rust 的惯例不一致。

<h3><a href="#why-arent-function-signatures-inferred" name="why-arent-function-signatures-inferred">
为什么函数签名不作推导？
</a></h3>

在 Rust 中，声明倾向于使用显式类型，而实际代码则用类型推导。这种设计有几个原因：

- 强制性声明有助于在模块和包（crate）级别上实现接口的稳定性。
- 签名便于提高程序员对代码的理解，消除 IDE 在整个包上运行类型推导算法来猜测一个函数参数类型的必要；它总是明确和就近的。
- 实现上，它简化了类型推导算法，因为推导只需要一次查看一个函数。

<h3><a href="#why-does-match-have-to-be-exhaustive" name="why-does-match-have-to-be-exhaustive">
为什么 <code>match</code> 必须是穷举的？
</a></h3>

有助于重构和明确性。

首先，如果 `match` 涵盖了所有的可能性，将来在 `enum` 中添加变量将导致编译失败，而不是运行时错误。这种类型的编译器辅助使得在 Rust 中大胆地重构成为可能。

其次，穷举检查使得默认情况下的语义很明确：通常，在一个非穷举的 `match` 中如果没有匹配到任何项，唯一安全的做法是 panic 当前的线程。早期版本的 Rust 并不要求 `match` 项需要穷举，发现这是一个很大的 bug 的来源。

使用通配符 `_` 可以很容易地忽略所有未指定的情况：

```rust
match val.do_something() {
    Cat(a) => { /* ... */ }
    _      => { /* ... */ }
}
```

<h2 id="numerics">数字</h2>

<h3><a href="#which-type-of-float-should-i-use" name="which-type-of-float-should-i-use">
在浮点运算中我该用 <code>f32</code> 还是 <code>f64</code>？
</a></h3>

对于这两者的选择取决于程序的目的。

如果你对浮点数的最大精度感兴趣，那么更倾向于选择 [`f64`][f64]。如果你更注重于保持数值占用更小的空间，或者更高的效率，而不关心由于每个数值具有较少的位而失去一些精确性，则选择 [`f32`][f32] 会更好。在 [`f32`][f32] 上的操作通常更快，即使在 64 位的硬件上也是如此。作为一个常见的例子，图形编程通常使用 [`f32`][f32] 因为它需要更高的性能，而且 32 位浮点数足以表示屏幕上的像素。

如果有疑问，请选择 [`f64`][f64] 以获得更高的精度。

<h3><a href="#why-cant-i-compare-floats" name="why-cant-i-compare-floats">
为什么我不能比较浮点数，或者把它们作为 <code>HashMap</code> 或 <code>BTreeMap</code> 的键？
</a></h3>

浮点数可以用操作符 `==`，`!=`，`<`，`<=`，`>`，`>=`，以及函数 `partial_cmp()` 进行比较。`==` 和 `!=` 是 [`PartialEq`][PartialEq] trait 的一部分，而 `<`，`<=`，`>`，`>=`，以及 `partial_cmp()` 是 [`PartialOrd`][PartialOrd] trait 的一部分。

浮点数不能用 `cmp()` 函数进行比较，它是 [`Ord`][Ord] trait 的一部分，因为浮点数不是全序的。此外，浮点数也不是完全等价的，所以它们没有实现 [`Eq`][Eq] trait。

浮点数不是全序或等价的，因为浮点值 [`NaN`](https://en.wikipedia.org/wiki/NaN) 既不小于，也不大于，或者等于其它任何浮点数或者自身。

因为浮点数没有实现 [`Eq`][Eq] 或 [`Ord`][Ord]，它们不能用于 trait 约束要求是这两种 trait 的地方，例如 [`BTreeMap`][BTreeMap] 或 [`HashMap`][HashMap]。这很重要，因为这些类型*假设*它们的键提供一个全序或完全等价的关系，否则将失败。

有一个[包](https://crates.io/crates/ordered-float)提供了对 [`f32`][f32] 和 [`f64`][f64] 的 [`Ord`][Ord] 和 [`Eq`][Eq] 的实现的封装，可能在某些情况下会有用。

<h3><a href="#how-can-i-convert-between-numeric-types" name="how-can-i-convert-between-numeric-types">
如何在数字类型间进行转换？
</a></h3>

有两种方法：一种是 `as` 关键字，它用于原生类型的简单转换，另一种是 [`Into`][Into] 和 [`From`][From] trait，用于实现多种类型的转换（也可以用于你自己定义的类型）。[`Into`][Into] 和 [`From`][From] trait 仅用于无损转换的情况，所以例如 `f64::from(0f32)` 可以成功编译而 `f32::from(0f64)` 则不行。另一方面，`as` 可以用于任意两种原生类型的转换，并根据需要进行截断。

<h3><a href="#why-doesnt-rust-have-increment-and-decrement-operators" name="why-doesnt-rust-have-increment-and-decrement-operators">
为什么 Rust 没有递增或递减操作符？
</a></h3>

先增和后增（以及相对应的减法）虽然方便，但也相当复杂。它们都需要求值顺序的知识，而且经常导致 C 和 C++ 中的细节错误和未定义的行为。 `x = x + 1` 或 `x += 1` 只是略长一点，但更加明确。
<h2 id="strings">字符串</h2>

<h3><a href="#how-to-convert-string-or-vec-to-slice" name="how-to-convert-string-or-vec-to-slice">
如何将 <code>String</code> 或 <code>Vec&lt;T&gt;</code> 转换成切片（<code>&amp;str</code> 和 <code>&amp;[T]</code>）？
</a></h3>

通常，你可以在需要切片的地方传入 `String` 或 `Vec<T>`。使用[强制解引（Deref）](https://doc.rust-lang.org/stable/book/second-edition/ch15-02-deref.html)，在用 `&` 或 `&mut` 传递引用时，[`String`][String] 和 [`Vec`][Vec] 会自动强制转换为对应的切片。

在 `&str` 和 `&[T]` 上实现的方法可以直接在 `String` 和 `Vec<T>` 上访问。例如，即使 `trim` 是 `&str` 上的方法，而 `some_string` 是一个 `String`，`some_string.trim()` 也可以运行。

在某些情况下，例如泛型代码，需要手动转换。可以用切片操作符实现手动转换，如：`&my_vec[..]`。

<h3><a href="#how-to-convert-between-str-and-string" name="how-to-convert-between-str-and-string">
如何将 <code>&amp;str</code> 转换为 <code>String</code>，或者反向转换？
</a></h3>

[`to_string()`][to_string] 方法将 [`&str`][str] 转换为 [`String`][String]，当你借用一个 [`String`][String] 的引用时，它会自动转换成 [`&str`][str]。两者都在以下示例中演示：

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

[`String`][String] 是一个被拥有（owned）的在堆上分配的 UTF-8 的字节缓冲区。可变 [`String`][String] 可以被修改，根据需要增加其容量。[`&str`][str] 是一个指向分配在某处的 [`String`][String] 的一个固定容量的「视图」。如果切片是在从 [`String`][String] 解引而来的，则通常是指向在堆上，如果是字符串字面值，则指向静态内存。

[`&str`][str] 是一个由 Rust 语言实现的原生类型，而 [`String`][String] 则是由标准库实现的。

<h3><a href="#how-do-i-do-o1-character-access-in-a-string" name="how-do-i-do-o1-character-access-in-a-string">
如何以 O(1) 复杂度访问一个 <code>String</code> 中的字符？
</a></h3>

做不到。至少在缺少对你所说的「字符（character）」充分的理解，并且缺少用于查找所需字符的索引而对字符串的预处理的情况下，无法做到。

Rust 字符串是用 UTF-8 编码的。UTF-8 中的单个可见字符不一定是单个字节，除非它是用 ASCII 编码的字符串。每个字节被称为「码元（code unit）」（在 UTF-16 中，码元是 2 字节，在 UTF-32 中码元是 4 字节）。「码位（code point）」由一个或多个码元组成，并由此结合成「grapheme cluster」，这是最接近字符的概念了。

因此，即使你可以在 UTF-8 字符串中索引字节，你也不能在常数时间里访问第 `i` 个码元或 grapheme cluster。但是，如果你知道所需的 code point 或 grapheme cluster 开始的字节的位置，那么你 _可以_ 在常数时间时访问它。包括 [`str::find()`][str__find] 和正则匹配函数都返回字节索引，以便于此类的访问。

<h3><a href="#why-are-strings-utf-8" name="why-are-strings-utf-8">
为什么字符串默认为 UTF-8？
</a></h3>

[`str`][str] 类型之所以是 UTF-8，是因为我们观察了许多实现中这种编码的文本 —— 尤其是在与字节顺序无关的网络传输中 —— 我们认为 I/O 的默认处理无需牵涉到对码位的双向重编码是最好的选择。

这意味着在一个字符串中的定位一个特定的 Unicode 码位是一个 O(n) 的操作，但是如果已经知道起始字节的索引，则可以按预期以 O(1) 访问。一方面，这显然是不可取的；另一方面，这个问题充满了权衡，我们要指出一些重要的限定：

扫描一个 ASCII 范围的 [`str`][str] 仍然可以安全地依字节进行。如果你使用 [`.as_bytes()`][str__as_bytes]，取出一个 [`u8`][u8] 只需要消耗 `O(1)`，并生产一个可以与 ASCII 范围内的 [`char`][char] 相互转换和比较的值。所以如果你要在 `'\n'` 处换行，基于字节的处理依然可以运行。UTF-8 是用这种方式精心设计的。

大多数在文本上「面向字符」的操作只能在非常受限的语言假设前提下工作，例如「ASCII 范围内的码位」。在 ASCII 范围之外，你往往必须使用复杂的（非常数时间）算法来确定语言单位（字形，单词，段落）的边界。我们建议使用一个「可靠的」有语言感知的，经 Unicode 验证过的算法。

[`char`][char] 类型是 UTF-32 的。如果你确定需要一个依码位的算法，写一个 `type wstr = [char]`，将一个 [`str`][str] 放入其中，然后使用 `wstr`，这是很简单的。换句话说：实际上如果你需要用到该编码，编程语言不会「默认解码为 UTF32」的事实不应该阻止你解码（或用其它任何方式重新编码）。

要更深入了解为什么 UTF-8 通常比 UTF-16 或 UTF-32 更好的解释，请阅读 [UTF-8 遍地开花宣言](http://utf8everywhere.org/)。

<h3><a href="#what-string-type-should-i-use" name="what-string-type-should-i-use">
我应该使用哪种字符串类型？
</a></h3>

Rust 拥有四种字符串类型，[每种都用于不同的目的](http://www.suspectsemantics.com/blog/2016/03/27/string-types-in-rust/)。在每一对中，都有一个「Owned」的字符串类型，和一个「切片」的字符串类型。组织起来像这样：

|               | 「Slice」类型 | 「Owned」类型 |
|:--------------|:--------------|:--------------|
| UTF-8         | `str`         | `String`      |
| OS 兼容       | `OsStr`       | `OsString`    |
| C 兼容        | `CStr`        | `CString`     |
| 系统路径      | `Path`        | `PathBuf`     |

Rust 的不同字符串类型适用于不同的目的。`String` 和 `str` 为 UTF-8 编码的通用目的字符串。`OsString` 和 `OsStr` 的编码取决于当前平台，在与操作系统交互时使用。`CString` 和 `CStr` 是 Rust 中与 C 字符串相当的存在，用于 FFI 代码。`PathBuf` 和 `Path` 是 `OsString` 和 `OsStr` 的方便封装，提供路径操作的特定方法。

<h3><a href="#why-are-there-multiple-types-of-strings" name="why-are-there-multiple-types-of-strings">
怎样写一个同时接受 <code>&str</code> 与 <code>String</code> 的函数？
</a></h3>

有几种方法，具体取决于该函数的需求：

- 如果函数需要拥有一个字符串，又希望接受任何类型的字符串，请使用 `Into<String>` 约束。
- 如果函数需要一个字符串切片，又希望接受任何类型的字符串，请使用 `AsRef<str>` 约束。
- 如果函数不关心字符串类型，希望用统一的方式处理两种可能性，请使用 `Cow<str>` 作为输入类型。

__使用 `Into<String>`__

在此例中，该函数可以接受 owned 字符串和字符串切片作为参数，它要么不作任何处理，要么将输入转换为函数体内部拥有的字符串。请注意，转换需要显式调用，否则不会生效。

```rust
fn accepts_both<S: Into<String>>(s: S) {
    let s = s.into();   // This will convert s into a `String`.
    // ... the rest of the function
}
```

__使用 `AsRef<str>`__

在此例中，该函数可以接受 owned 字符串和字符串切片作为参数，它要么不做任何处理，要么将输入转换为一个字符串切片。这里可以通过引用输入参数来自动完成，如下所示：

```rust
fn accepts_both<S: AsRef<str>>(s: &S) {
    // ... the body of the function
}
```

__使用 `Cow<str>`__

在此例中，该函数接受一个 `Cow<str>`，它不是泛型类型，而是一个容器，包含一个 owned 字符串或字符串切片。

```rust
fn accepts_cow(s: Cow<str>) {
    // ... the body of the function
}
```


<h2 id="collections">集合</h2>

<h3><a href="#can-i-implement-linked-lists-in-rust" name="can-i-implement-linked-lists-in-rust">
可以在 Rust 中高效地实现向量和链表的数据结构吗？
</a></h3>

如果实现这些数据结构的原因是要将它们用于其它程序，则不需要，因为标准库已经提供了这些数据结构的高效实现。

但是，如果[你的目的只是为了学习](http://cglab.ca/~abeinges/blah/too-many-lists/book/)，那么你可能需要深入学习 unsafe 代码。虽然这些数据结构可以完全在安全的 Rust 中实现，但性能可能会比使用 unsafe 代码更差。简单来说原因是如向量和链表这样的数据结构依赖于指针和内存操作，而这些在安全的 Rust 中是不允许的。

例如，双向链表需要每个节点有两个可变引用，但这违反了 Rust 的可变引用规则。你可以用 [`Weak<T>`][Weak] 解决这个问题，但性能会比你想的要差。使用 unsafe 代码，你可以绕过可变引用规则的限制，但必须自行验证你引入的代码不会违反内存安全。

<h3><a href="#how-can-i-iterate-over-a-collection-without-consuming-it" name="how-can-i-iterate-over-a-collection-without-consuming-it">
如何在不移动/消费的情况下迭代集合？
</a></h3>

最简单的方法是使用集合的 [`IntoIterator`][IntoIterator] 实现。这里有一个 [`&Vec`][Vec] 的例子：

```rust
let v = vec![1,2,3,4,5];
for item in &v {
    print!("{} ", item);
}
println!("\nLength: {}", v.len());
```

Rust 的 `for` 循环对于任何迭代都调用 `into_iter()`（在 [`IntoIterator`][IntoIterator] trait 中定义）。任何实现了 [`IntoIterator`][IntoIterator] trait 的类型都可以用在 `for` 循环中。 [`IntoIterator`][IntoIterator] 是对针 [`&Vec`][Vec] 和 [`&mut Vec`][Vec] 实现的，所以 `into_iter()` 返回的迭代器只是借用集合的内容，而不是移动/消费它们。其它标准集合也是如此。

如果需要移动/消费迭代器，在迭代中用没有 `&` 或 `&mut` 的 `for`。

如果你需要直接访问一个借用的迭代器，你通常可以调用 `iter()` 方法来获取它。

<h3><a href="#why-do-i-need-to-type-the-array-size-in-the-array-declaration" name="why-do-i-need-to-type-the-array-size-in-the-array-declaration">
为什么需要在数组声明中输入数组大小？
</a></h3>

不一定要。如果你直接声明数组，数组大小会根据元素的数量来推断。但是如果你声明一个固定大小的数组，那么编译器就需要知道该数组的大小。

需要注意的是，目前 Rust 并没有提供针对不同大小数组的泛型。如果你想接受可变数值的连续容器，请使用 [`Vec`][Vec] 或切片（视是否需要所有权而定）。

<h2 id="ownership">所有权</h2>

<h3><a href="#how-can-i-implement-a-data-structure-that-contains-cycles" name="how-can-i-implement-a-data-structure-that-contains-cycles">
如何实现包含循环的图或其他数据结构？
</a></h3>

至少有四种选择（在 [Too Many Linked Lists](http://cglab.ca/~abeinges/blah/too-many-lists/book/) 里有详细的讨论）：

- 你可以使用 [`Rc`][Rc] 和 [`Weak`][Weak] 来实现，以允许节点共享所有权以实现它。然而这种方法要付出内存管理的成本。
- 你可以通过使用包含裸指针的 `unsafe` 代码来实现。这会更高效，但绕过了 Rust 的安全保障机制。
- 在这些向量中使用向量和索引。有[几种](http://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/)[可用](https://featherweightmusings.blogspot.com/2015/04/graphs-in-rust.html)的例子和这种方法的解释。
- 使用借用的引用和 [`UnsafeCell`][UnsafeCell]。这种方法有[解释和代码](https://github.com/nrc/r4cppp/blob/master/graphs/README.md#node-and-unsafecell)。

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

它们是同一个东西的不同术语。所有的情况下，这意味着该值已经从原始所有者移动到另一个所有者，原始所有者不能再使用它。如果一个类型实现了 `Copy` trait，原始所有者的值不会无效，仍然可以使用。

<h3><a href="#why-can-values-of-some-types-by-reused-while-others-are-consumed" name="why-can-values-of-some-types-by-reused-while-others-are-consumed">
为什么某些类型的值在传递到一个函数后可以使用，而另一些类型的值传递后使用会导致错误？
</a></h3>

如果一个类型实现了 [`Copy`][Copy] trait，则它将在传递到一个函数时被复制。Rust 中的所有数字类型都实现了 [`Copy`][Copy]，但结构类型默认并未实现 [`Copy`][Copy]，因此它们是被移动。这意味着结构体不能在其他地方重用，除非它是通过 return 从函数中移出。

<h3><a href="#how-do-you-deal-with-a-use-of-moved-value-error" name="how-do-you-deal-with-a-use-of-moved-value-error">
如何处理「use of moved value」错误？
</a></h3>

此错误表示你尝试使用的值已经被移动给新的拥有者。首先要检查的是这个移动是否有必要：如果移动到一个函数中，可能可以尝试重写函数，改为使用引用而不是移动。另外如果如果被移动的类型实现了 [`Clone`][Clone]，那么在移动之前调用它的 `clone()` 方法，将移动它的一个拷贝，原始的值可以在之后继续使用。注意，然后克隆一个值通常应该是最后考虑的手段，因为克隆可能会是昂贵的，会导致更多的内存分配。

如果移动的值是你自定义的类型，请考虑实现 [`Copy`][Copy]（用于隐式拷贝而不是移动）或者 [`Clone`][Clone]（用于显式拷贝）。[`Copy`][Copy] 经常是用 `#[derive(Copy, Clone)]` 来实现的（[`Copy`][Copy] 的实现要求要实现 [`Clone`][Clone]，[`Clone`][Clone] 也是用 `#[derive(Clone)]` 来实现的。

如果这些方式都不行，你可能需要修改这个要求获取所有权的函数，改为在函数退出的时候返回这个值的所有权。

<h3><a href="#what-are-the-rules-for-different-self-types-in-methods" name="what-are-the-rules-for-different-self-types-in-methods">
在方法声明中使用 <code>self</code>、<code>&amp;self</code> 与 <code>&amp;mut self</code> 的规则是什么？
</a></h3>

- 如果一个函数需要消费该值，用 `self`
- 如果一个函数只需要该值的只读引用，用 `&self`
- 如果一个函数不需消费而要变更该值，用 `&mut self`

<h3><a href="#how-can-i-understand-the-borrow-checker" name="how-can-i-understand-the-borrow-checker">
我该怎么能理解借用检查器？
</a></h3>

借用检查器在对 Rust 代码进行求值的时候只使用几条规则，可以在 Rust 之书的[借用章节](https://doc.rust-lang.org/stable/book/second-edition/ch04-02-references-and-borrowing.html)找到。这些规则为：

> 首先，任何借用所持续的作用范围不得超过它的拥有者。其次，你可以使用以下两种借用形式的其中任何一种，但不能同时使用：
>
> - 某个资源的一个或者多个引用（&T）。
> - 该资源的一个可变引用（&mut T）。

虽然规则本身很简单，但是要遵循它们却并不容易，尤其是对于那些不习惯于推导生命周期和所有权的人。

理解借用检查器的第一步是阅读它产生的错误信息。为了确保借用检查器能够产生高质量的帮助信息来解决发现的问题，已经投入了大量的工作。当你遇到一个借用检查器的问题，首先要做的是慢慢扡仔细地阅读错误报告，只有在理解了所描述的错误之后才着手处理相关的代码。

第二步是熟悉 Rust 标准库所提供的与所有权和可变性相关的容器类型，包括 [`Cell`][Cell]，[`RefCell`][RefCell]，以及 [`Cow`][Cow]。这些都是用于表达某些涉及所有权和可变性的情况时，非常有用有且必需的工具，而且是具有最低的性能损耗的实现。

理解借用检查器是重要的环节是实践。Rust 的强静态分析保证与许多程序员以往接触过的有天壤之别。需要花些时间来适应这些。

如果你发现你在借用检查器中遇到困难，或者失去耐心，可以随时联系 [Rust 社区](community.thml)寻求帮助。

<h3><a href="#when-is-rc-useful" name="when-is-rc-useful">
<code>Rc</code> 在什么时候有用？
</a></h3>

Rust 的非原子引用计数指针类型，在官方文档中涵盖了 [`Rc`][Rc] 的描述。简而言之，[`Rc`][Rc] 和它的线程安全版本 [`Arc`][Arc] 在表示共享所有权的时候是有用的，当没有人拥有它的访问权限的时候，系统会自动释放相应的内存。

<h3><a href="#how-do-i-return-a-closure-from-a-function" name="how-do-i-return-a-closure-from-a-function">
如何从函数返回一个闭包？
</a></h3>

要从一个函数中返回闭包，这个闭包必须是一个「move 闭包」，意思是这个闭包是使用关键字 `move` 来定义的。正如
[Rust 之书中的解释](https://doc.rust-lang.org/book/closures.html#move-closures)，这让这个闭包拥有了它所在环境里的变量的拷贝，独立于它的父级栈帧。另外，返回一个闭包会是不安全的，因为它会被允许访问那些失效的变量；换句话说，它会允许访问潜在的无效内存。返回的闭包也必须用 [`Box`][Box] 包装，这样它才会在堆上分配空间。[在这本书](https://doc.rust-lang.org/book/closures.html#returning-closures)里阅读更多相关的信息。

<h3><a href="#what-are-deref-coercions" name="what-are-deref-coercions">
什么是强制解引（deref coercion），它是如何工作的？
</a></h3>

[强制解引](https://doc.rust-lang.org/stable/book/second-edition/ch15-02-deref.html) 是自动将指向指针的引用（例如
`&Rc<T>` 或者 `&Box<T>`）转换为指向内容的引用（例如 `&T`）这种强制转换的一种便捷方式。
强制解引的存在使得 Rust 更加符合人体工程学，它是通过 [`Deref`][Deref] trait 来实现的。

一个 Deref 的实现表示该实现类型可以通过调用 `deref` 方法来转换成一个目标类型，该方法接受该调用类型的一个不可变引用，并返回一个指向目标的引用（相同的生命周期）。前缀操作符 `*` 是 `deref` 方法的缩写方式。

之所以被称为「强制 coercions」是因为以下的规则，这里引用自 [Rust 之书](https://doc.rust-lang.org/stable/book/deref-coercions.html)：

> 假设你有一个类型 `U`，并且它实现了 `Deref<Target=T>`，那么 `&U` 将会自动强制转换为 `&T`。

例如，假设你有一个 `&Rc<String>`，它会被此规则强制转换为 `&String`，然后再被用同样的方式强制转换为 `&str`。
那么对于一个接收 `&str` 参数的函数，你可以直接传入一个 `&Rc<String>`，所有的强制转换都通过 `Deref` trait 自动处理。

以下是最常见的一些强制解引：

- `&Rc<T>` 到 `&T`
- `&Box<T>` 到 `&T`
- `&Arc<T>` 到 `&T`
- `&Vec<T>` 到 `&[T]`
- `&String` 到 `&str`

<h2 id="lifetimes">生命周期</h2>

<h3><a href="#why-lifetimes" name="why-lifetimes">
为什么有生命周期？
</a></h3>

生命周期是 Rust 以于内存安全问题的解答。它允许 Rust 确保内存安全，而无需付出垃圾收集的性能代价。这是基于多种学术成果的，这些参考文献可以从 [Rust 之书](https://doc.rust-lang.org/stable/book/first-edition/bibliography.html#type-system)中找到。

<h3><a href="#why-is-the-lifetime-syntax-the-way-it-is" name="why-is-the-lifetime-syntax-the-way-it-is">
为什么生命周期语法是这样的？
</a></h3>

`'a` 用于表示一个通用类型参数，这种语法来自于 ML 系列编程语言。对于 Rust 来说，这种语法必须是无歧义的，显而易见的，并且在类型声明中适合于与 trait 和引用放在一起。一些替代语法也被讨论过，但没有一种替代语法会表现得更好。

<h3><a href="#how-do-i-return-a-borrow-to-something-i-created-from-a-function" name="how-do-i-return-a-borrow-to-something-i-created-from-a-function">
我该如何从一个函数中返回我创建的某个东西的借用？
</a></h3>

你必须确保这个借用项的生命周期要比这个函数更长。将输出的生命周期和某些输入的生命周期绑定在一起，这样就可以做到，如下所示：

```rust
type Pool = TypedArena<Thing>;

// （以下的生命周期只是为了说明而显式写出来；
//  可以通过稍后 FAQ 条目中所描述的省略规则来省略它）
fn create_borrowed<'a>(pool: &'a Pool,
                       x: i32,
                       y: i32) -> &'a Thing {
    pool.alloc(Thing { x: x, y: y })
}
```

另一个替代的方式是通过返回一个拥有的类型（owning type）来彻底消除引用，如 [`String`][String]：

```rust
fn happy_birthday(name: &str, age: i64) -> String {
    format!("Hello {}! You're {} years old!", name, age)
}
```

这种方法更简单，但往往导致不必要的分配。

<h3><a href="#when-are-lifetimes-required-to-be-explicit" name="when-are-lifetimes-required-to-be-explicit">
为什么有些引用有生命周期，如 <code>&amp;'a T</code>，而有些没有，如 <code>&amp;T</code>？
</a></h3>

实际上，*所有的*引用类型都有一个生命周期，但在大多数时候你无需将它显式写出来。规定如下：

1. 在函数体内，你无需显式写出生命周期；正确的值总会被推导出来。
2. 在函数*签名*中（例如，其参数类型或其返回值类型），你*可能*要显式写出生命周期。生命周期有一个简单的默认机制，称作[「生命周期省略（lifetime elision）」](https://doc.rust-lang.org/book/lifetimes.html#lifetime-elision)，其中包含了以下有三条规则：
  - 函数的参数中，每个省略的生命周期都变成独立的生命周期参数。
  - 如果只有一个输入的生命周期，无论省略与否，此生命周期将被赋给函数返回值中所有省略的生命周期。
  - 如果有多个输入的生命周期，但其中一个是 &self 或者 &mut self，那这个 self 的生命周期将被赋给输出中所有省略的生命周期。
3. 最后，在 `struct` 或者 `enum` 定义中，所有的生命周期都必须明确声明。

如果这些规则导致编译错误， Rust 编译器会给出一个错误信息，指出错误的原因，并根据推导过程中出错的步骤给出潜在的解决方案。

<h3><a href="#how-can-rust-guarantee-no-null-pointers" name="how-can-rust-guarantee-no-null-pointers">
Rust 如何保证「无空指针」和「无悬挂指针」？
</a></h3>

构造一个类型为 `&Foo` 或者 `&mut Foo` 的值的唯一方式是指定该引用指向的类型 `Foo` 的一个已存在的值。该引用在给定的代码区域中（该引用的生命周期）「借用」了原始值，而被借用的值在借用期间无法被移动或者被销毁。

<h3><a href="#how-do-i-express-the-absense-of-a-value-without-null" name="how-do-i-express-the-absense-of-a-value-without-null">
如果没有 <code>null</code>，我该怎么表达缺失值？
</a></h3>

你可以使用 [`Option`][Option] 类型，可以是 `Some(T)` 或者是 `None`。`Some(T)` 表示其包含类型为 `T` 的值，而 `None` 则表示没有值。

<h2 id="generics">泛型</h2>

<h3><a href="#what-is-monomorphisation" name="what-is-monomorphisation">
什么是「单态化（monomorphisation）」？
</a></h3>

单态化是基于函数调用（或结构的使用）的参数类型，对每个泛型函数（或结构）的特化。

在单态化期间，该泛型函数实例化的每个单独的类型集被翻译成一个新的拷贝。这和 C++ 使用的策略相同。它产生专门针对每次调用和静态调度的快速代码，比起使用其它翻译策略所产生的二进制文件，这种函数被多个不同的类型实例化的权衡可能会导致「代码膨胀」，其中多个函数实例导致了更大的二进制文件。

相比接收不同类型的参数，接受 [trait 对象](https://doc.rust-lang.org/book/trait-objects.html)的函数不会发生单态化。相反，trait 对象的方法在运行时是动态调度的。

<h3><a href="#whats-the-difference-between-a-function-and-a-closure-that-doesnt-capture" name="whats-the-difference-between-a-function-and-a-closure-that-doesnt-capture">
一个函数和一个没有捕获任何变量的闭包有什么区别？
</a></h3>

函数和闭包在操作上是等效的，但是由于它们的不同实现而具有不同的运行时表现。

函数是语言内置的原生类型，而闭包基本上是三种 trait 之一的语法糖：[`Fn`][Fn]，[`FnMut`][FnMut]，和 [`FnOnce`][FnOnce]。当你创建一个闭包时，Rust 编译器会自动创建一个实现了这三种 trait 之中相应的 trait 的结构，并将捕获到的环境变量作为成员，并使此结构能够作为函数一样调用。裸函数无法捕获它的环境。

这些 trait 之间的差异是它们如何使用 `self` 参数。[`Fn`][Fn] 使用 `&self`，[`FnMut`][FnMut] 使用 `&mut self`，而 [`FnOnce`][FnOnce] 则使用 `self`。

即使闭包没有捕获任何环境变量，它在运行时也表现为两个指针，与其它任何闭包相同。

<h3><a href="#what-are-higher-kinded-types" name="what-are-higher-kinded-types">
什么是更高级的类型，为什么我想要它们，为什么 Rust 没有它们？
</a></h3>

高级类型具有未知参数。类型构造器，如 [`Vec`][Vec]，[`Result`][Result]，以及 [`HashMap`][HashMap] 都是高级类型的范例：每个都需要一些额外的类型参数，以便于实际显示一个特定的类型，如 `Vec<u32>`。对高级类型的支持意味着这些「不完全」的类型可以用于任何「完全」类型可以使用的地方，包括函数的泛型。

任意的完全类型，如 [`i32`][i32]，[`bool`][bool]，或 [`char`][char] 都是一种 `*` 类型（这种符号来自类型理论领域）。具有一个参数的类型，如 [`Vec<T>`][Vec] 是一种 `* -> *`，意思是 [`Vec<T>`][Vec] 接受一个如 [`i32`][i32] 的完全类型，并返回一个完全类型 `Vec<i32>`。一个具有三个参数的类型，如 [`HashMap<K, V, S>`][HashMap] 是 `* -> * -> * -> *` 类型，它接受三个完全类型（如 [`i32`][i32]，[`String`][String]，以及 [`RandomState`][RandomState]），产生一个新的完全类型 `HashMap<i32, String, RandomState>`。

除了这些例子，类型构造器还可以接受 *生命周期（lifetime）* 参数，我们将其表示为 `Lt`。例如，`slice::Iter` 具有 `Lt -> * -> *` 类型，因为它必须像 `Iter<'a, u32>` 这样实例化。

缺少对高级类型的支持，使得很验证编写某些类型的泛型代码。尤其是对于像迭代器这样的概念进行抽象会有问题，因为迭代器通常为至少需要用一个生命周期才能进行参数化。这反过来又阻止了对 Rust 的集合类型上的 trait 抽象。

另一个常见的例子是像 function 或 monad 这样的概念，它们都是类型构造器，而不是单一的类型。

Rust 目前不支持更高级的类型，因为与我们想要进行的其它改进相比，这不是一个高优先的事项。由于这种设计涉及了主要和交叉的改动，我们也想仔细考虑。但目前缺乏支持并没有固有的内在原因。

<h3><a href="#what-do-named-type-parameters-in-generic-types-mean" name="what-do-named-type-parameters-in-generic-types-mean">
泛型类型中 <code>&lt;T=Foo&gt;</code> 这样的命名类型参数是什么意思？
</a></h3>

这些被称为[关联类型](https://doc.rust-lang.org/stable/book/second-edition/ch19-03-advanced-traits.html)，允许表达不能用 `where` 子句表达的 trait 约束。例如，一个泛型约束 `X: Bar<T=Foo>` 的意思是「`X` 必须实现 `Bar` 这个 trait，而且在 `Bar` 的实现中，`X` 必须将 `Foo` 作为 `Bar` 的关联类型 `T`」。例子中这样的约束不能通过一个 `where` 子句表示，也不能用像 `Box<Bar<T=Foo>>` 这样的 trait 对象表示。

关联类型的存在，是由于泛型通常牵涉类型家族，其中某个类型决定了家族中的所有其它类型。例如，一个用于图形的 trait 可能具有图形自己的 `Self` 类型，而且具有节点和边缘的关联类型。每个图形类型唯一确定其关联类型。使用关联类型可以使这些类型的家族更加简洁，并且在许多情况下还能提供更好的类型推断。

<h3><a href="#how-do-i-overload-operators" name="how-do-i-overload-operators">
我可以重载运算符吗？哪些可以，怎么做？
</a></h3>

你可以通过实现相应的 trait 来为多种操作符提供自定义的实现：[`Add`][Add] 对应于 `+`，[`Mul`][Mul] 对应于 `*`，等等。类似这样：

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

以下的操作符可以被重载：

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
为什么要把 <code>Eq</code>/<code>PartialEq</code> 及 <code>Ord</code>/<code>PartialOrd</code> 分开？
</a></h3>

Rust 中有一些类型的值只是部分有序（偏序），或者只有部分等价。部分有序的意思对于给定的类型，可能存在既不小于也不大于彼此的值。部分等价的意思是对于给定的类型，可能存在与自身不相等的值。

浮点类型（[`f32`][f32] 和 [`f64`][f64]）对于这两种类型是个好例子。任何浮点类型可能具有值 `NaN`（意思是「非数字」）。`NaN` 不等于自身（`NaN == NaN` 为假），也不小于或者大于任何其它浮点值。这样，[`f32`][f32] 和 [`f64`][f64] 都实现了 [`PartialOrd`][PartialOrd] 和 [`PartialEq`][PartialEq]， 而非 [`Ord`][Ord] 也非 [`Eq`][Eq]。

正如[先前关于浮点类型问题](#why-cant-i-compare-floats)的解答，这些区别很重要，因为一些集合依赖于全序/完全等价以得出正确的结果。

<h2 id="input-output">输入 / 输出</h2>

<h3><a href="#how-do-i-read-a-file-into-a-string" name="how-do-i-read-a-file-into-a-string">
如何将文件读入一个 <code>String</code>？
</a></h3>

使用 [`read_to_string()`][read__read_to_string] 方法，它定义在 [`std::io`][std-io] 的 [`Read`][Read] trait 中。

```rust
use std::io::Read;
use std::fs::File;

fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut s = String::new();
    let _ = File::open(path)?.read_to_string(&mut s);  // `s` contains the contents of "foo.txt"
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

[`File`][File] 类型实现了 [`Read`][Read] trait，它具有读写数据的各种函数，包括[`read()`][read__read]，[`read_to_end()`][read__read_to_end]，[`bytes()`][read__bytes]，[`chars()`][read__chars]，和 [`take()`][read__take]。这些函数中每个都从给定的文件中读取一定量的输入。[`read()`][read__read] 在单次调用中读取与底层系统一样多的输入。[`bytes_to_end()`][read__read_to_end] 将整个缓冲区读进一个向量，分配所需的空间。[`bytes()`][read__bytes] 和 [`chars()`][read__chars] 可以分别对文件的字节和字符进行迭代。最后，[`take()`][read__take] 允许你从文件中读取任意数量的字节。总而言之，这些都允许你有效地读取所需的任何数据。

对于缓冲区的读取，使用 [`BufReader`][BufReader] 结构体，有助于读取时减少系统调用的次数。

<h3><a href="#how-do-i-do-asynchronous-input-output-in-rust" name="how-do-i-do-asynchronous-input-output-in-rust">
Rust 中如何进行异步输入 / 输出？
</a></h3>

有几个 Rust 的库提供了异步输入 / 输出，包括 [mio](https://github.com/carllerche/mio)，[tokio](https://github.com/tokio-rs/tokio-core)，[mioco](https://github.com/dpc/mioco)，[coio-rs](https://github.com/zonyitoo/coio-rs) 和 [rotor](https://github.com/tailhook/rotor)。

<h3><a href="#how-do-i-get-command-line-arguments" name="how-do-i-get-command-line-arguments">
如何在 Rust 中获取命令行参数？
</a></h3>

最简单的方法是使用 [`Args`][Args]，它为输入参数提供了迭代器。

如果你正在寻找一些更强大的工具，那么[在 crate.io 上有很多选择](https://crates.io/keywords/argument)。

<h2 id="error-handling">错误处理</h2>

<h3><a href="#why-doesnt-rust-have-exceptions" name="why-doesnt-rust-have-exceptions">
为什么 Rust 没有异常？
</a></h3>

异常使对于控制流的理解复杂化，它们表示了类型系统之外的有效性/无效性，而且它们与多线程代码（Rust 的主要关注点）交互并不佳。

Rust 更倾向于使用基于类型的错误处理方式，在[这本书中有详细的介绍](https://doc.rust-lang.org/stable/book/second-edition/ch09-00-error-handling.html)。这会更适合 Rust 的控制流，并发和其它所有的特性。

<h3><a href="#whats-the-deal-with-unwrap" name="whats-the-deal-with-unwrap">
到处都有 <code>unwrap()</code> 是怎么回事？
</a></h3>

`unwrap()` 是一个函数，它提取一个 [`Option`][Option] 或 [`Result`][Result] 中的值，如果值不存在，则引发 panic。

`unwrap()` 不应该是你处理预料会出现的错误的默认方式，例如用户的错误输入。在生产环境的代码中，它被视为一个断言，断言该值非空，否则会导致程序崩溃。

它也适用于快速原型中，你暂时不想处理错误，或者博客文章中，错误处理会喧宾夺主。

<h3><a href="#why-do-i-get-errors-with-try" name="why-do-i-get-errors-with-try">
为什么我尝试运行使用 <code>try!</code> 宏的示例代码时会得到错误？
</a></h3>

这可能是函数返回类型的一个问题。[`try!`][TryMacro] 宏从 [`Result`][Result] 中提取值，或者捕获 [`Result`][Result] 中的错误并提早返回此错误。这意味着 [`try!`][TryMacro] 只适用于返回 [`Result`][Result] 的函数，其中的 `Err` 构造类型实现了 `From::from(err)`。在实践中，这意味着 [`try!`][TryMacro] 宏不能在 main 函数中执行。

<h3><a href="#error-handling-without-result" name="error-handling-without-result">
比起无处不在的 <code>Result</code>，有没有一种更简单的错误处理方式？
</a></h3>

如果你在寻找一种方法，避免处理从别人代码中返回的 [`Result`][Result] ，总是可以使用 [`unwrap()`][unwrap]，但它可能不是你想要的。 [`Result`][Result] 是一种用于指示某些运算可能成功或者失败的指示器。要求你显式处理这些失败，是 Rust 鼓励的鲁棒性的方法之一。 Rust 提供了像 [`try!`][TryMacro] 这样的宏，使得处理失败更便捷。

如果你真的不想处理错误，可以用 [`unwrap()`][unwrap]，但请注意，这么做意味着在失败的时候代码会 panic，这通常会导致进程关闭。

<h2 id="concurrency">并发</h2>

<h3><a href="#can-i-use-static-values-across-threads-without-an-unsafe-block" name="can-i-use-static-values-across-threads-without-an-unsafe-block">
我可以不用 <code>unsafe</code> 块而跨线程使用静态（static）值吗？
</a></h3>

在同步的情况下，修改是安全的。修改一个静态的 [`Mutex`][Mutex] （通过 [lazy-static](https://crates.io/crates/lazy_static/) 包进行延迟初始化）不需要用 `unsafe` 块，修改一个静态的 [`AtomicUsize`][AtomicUsize] （可以不用 lazy_staic 初始化）也是一样。

更一般来说，如果一个类型实现了 [`Sync`][Sync] 而且不实现 [`Drop`][Drop]，它就[可以被用作 `static`](https://doc.rust-lang.org/book/const-and-static.html#static)

<h2 id="macros">宏</h2>

<h3><a href="#can-i-write-a-macro-to-generate-identifiers" name="can-i-write-a-macro-to-generate-identifiers">
我可以编写一个宏来生成标识符吗？
</a></h3>

目前还不行。 Rust 宏是 [「卫生宏（hygienic macros）」](https://en.wikipedia.org/wiki/Hygienic_macro)，它有意避免捕获或者创建可能导致与其它标识符产生意外冲突的标识符。它们的功能与 C 预处理器相关的宏的风格有着显著的不同。宏调用只能在显式支持的地方出现：项（item），方法声明，表达式，以及模式。这里，「方法声明」是指可以放置方法的位置。它们不能用于完成部分方法声明。由于相同的逻辑，它们也不能用于完成部分变量声明。

<h2 id="debugging">调试和工具</h2>

<h3><a href="#how-do-i-debug-rust-programs" name="how-do-i-debug-rust-programs">
如何调试 Rust 程序？
</a></h3>

与 C 和 C++ 相同，Rust 程序可以用 [gdb](https://sourceware.org/gdb/current/onlinedocs/gdb/) 或 [lldb](http://lldb.llvm.org/tutorial.html) 进行调试。实际上，每个 Rust 安装都带了 rust-gdb 和 rust-lldb 两者或其中一个（取决于平台支持）。这些是对 gdb 和 lldb 的封装，增加了对 Rust 的显示优化。

<h3><a href="#how-do-i-locate-a-panic" name="how-do-i-locate-a-panic">
<code>rustc</code> 说标准库代码中发生了一个 panic。我如何才能在我的代码中定位错误？
</a></h3>

这个错误通常是由于用户代码中对一个 `None` 或者 `Err` 值执行 [`unwrap()`][unwrap] 操作引起的。设置环境变量 `RUST_BACKTRACE=1` 启用回溯或能，在助于获取更多的信息。在调试模式下编译（`cargo build` 的默认模式）也是有帮助的。使用 rust 提供的 `rust-gdb` 或 `rust-lldb` 这样的调试器也是有帮助的。

<h3><a href="#what-ide-should-i-use" name="what-ide-should-i-use">
我该使用什么 IDE？
</a></h3>

Rust 有许多开发环境可供选择，详见官方的 [IDE 支持页面](https://forge.rust-lang.org/ides.html)。

<h3><a href="#wheres-rustfmt" name="wheres-rustfmt">
<code>gofmt</code> 很棒。<code>rustfmt</code>在哪？
</a></h3>

`rustfmt` [就在这里](https://github.com/rust-lang-nursery/rustfmt)，正在活跃地开发中，以使得阅读 Rust 代码尽可能更容易和可预测。

<h2 id="low-level">底层</h2>

<h3><a href="#how-do-i-memcpy-bytes" name="how-do-i-memcpy-bytes">
如何用 <code>memcpy</code> 在内存中拷贝字节？
</a></h3>

如果你想安全地克隆一个现有切片，可以使用 [`clone_from_slice`][clone_from_slice]。

用 [`copy`][copy] 拷贝可能重叠（overlapping）的字节。用 [`copy_nonoverlapping`][copy_nonoverlapping] 拷贝非重叠字节。两个特性都是 `unsafe` 的，因此两者都会破坏本语言的安全保障，使用时请小心。

<h3><a href="#does-rust-work-without-the-standard-library" name="does-rust-work-without-the-standard-library">
没有标准库的 Rust 还能正确运行吗？
</a></h3>

绝对可以。Rust 程序可以使用 `#![no_std]` 属性设置为不加载标准库。使用此属性集后，你可以继续使用 Rust 核心库，那些是与平台无关的原函数。也因此，它不包括 IO、并发、堆分配等。

<h3><a href="#can-i-write-an-operating-system-in-rust" name="can-i-write-an-operating-system-in-rust">
我可以用 Rust 写一个操作系统吗？
</a></h3>

行的！事实上，[有多个项目正在进行中](http://wiki.osdev.org/Rust)。

<h3><a href="#how-can-i-write-endian-independent-values" name="how-can-i-write-endian-independent-values">
如何以大端或小端格式读取或写入像 <code>i32</code> 或 <code>f64</code> 这样的数字类型到一个文件或其他字节流中？
</a></h3>

你应该检出 [byteorder 包](http://burntsushi.net/rustdoc/byteorder/)，它正是为这种情况提供了各种工具。

<h3><a href="#does-rust-guarantee-data-layout" name="does-rust-guarantee-data-layout">
Rust 是否保证特定的数据布局？
</a></h3>

默认不会。在一般情况下，`enum` 和 `struct` 布局是未定义的。这允许编译器进行潜在优化，如根据差别式重新对齐，压缩嵌套的 `enum`，重排字段以消除填充，等。不带数据的 `enum` （类 C）符合按定义表示的条件。这样的 `enum` 很容易区分，因为它们只是一个不包含数据的名字列表：

```rust
enum CLike {
    A,
    B = 32,
    C = 34,
    D
}
```

`#[repr(C)]` 属性可以应用到诸如 `enums` 以提供等同 C 语言中的表示。对于大多数情况来说，这允许在 FFI 代码中使用 Rust 的 `enum` 和 C 的 `enum`。这个属性也可以应用到 `strut` 上，来获得与 C `struct` 相同的布局。

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

有可能。Rust 已经拥有了对于 [asm.js] 和 [WebAssembly] 的[实验性支持][wasm]。

[wasm]: https://davidmcneil.gitbooks.io/the-rusty-web/
[asm.js]: http://asmjs.org/
[WebAssembly]: http://webassembly.org/

<h3><a href="#how-do-i-cross-compile-rust" name="how-do-i-cross-compile-rust">
如何在 Rust 中交叉编译？
</a></h3>

Rust 的交叉编译是可能的，它但需要[一些流程](https://github.com/japaric/rust-cross/blob/master/README.md)完成设置。每个 Rust 编译器都是一个交叉编译器，但库需要针对目标平台进行交叉编译。

Rust 确实为每个受支持平台方法[标准库副本](https://static.rust-lang.org/dist/index.html)，在分发目录上可以找到各构建目录，其中包含的 `rust-std-*` 文件就是它们，但尚没有自动安装它们的方法。

<h2 id="modules-and-crates">模块（module）和包（crate）</h2>

<h3><a href="#what-is-the-relationship-between-a-module-and-a-crate" name="what-is-the-relationship-between-a-module-and-a-crate">
模块与包之间的关系是什么？
</a></h3>

- 一个包是一个编译单元，它是 Rust 编译器可操作的最小代码量。
- 一个模块是一个（可能是嵌套的）放在一个包内的代码单元。
- 一个包包含一个隐含的、未命名的顶层模块。
- 递归定义可以跨越模块，但不能跨包。

<h3><a href="#why-cant-the-rust-compiler-find-a-library-im-using" name="why-cant-the-rust-compiler-find-a-library-im-using">
为什么 Rust 编译器找不到我 <code>use</code> 的库？
</a></h3>

可能有多种原因，但一个常见的错误是没有意识到 `use` 声明是相对于包的根层级。如果在项目的根文件中定义，请尝试重写声明以使用它们要使用的路径，并查看是否可以解决问题。

还有 `self` 和 `super`，它们分别使用相对于当前模块或者父模块的路径。

有关 `use` 库的完整信息，请阅读 Rust 之书的[「包和模块」](https://doc.rust-lang.org/stable/book/second-edition/ch07-00-modules.html) 一章。

<h3><a href="#why-do-i-have-to-declare-modules-with-mod" name="why-do-i-have-to-declare-modules-with-mod">
为什么我必须在包顶层用 <code>mod</code> 声明模块文件，而不能直接 <code>use</code> 它们？
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

正如 Cargo 的[配置文档](http://doc.crates.io/config.html)所说，你可以设置配置文件中 `[http]` 下的「proxy」变量使 Cargo 使用一个代理服务器。

<h3><a href="#why-cant-the-compile-find-method-implementations" name="why-cant-the-compile-find-method-implementations">
为什么编译器找不到方法实现，即使我已经 <code>use</code> 了相应的包？
</a></h3>

对于在 trait 上定义的方法，必须显式导入 trait 的声明。这意味着导入一个实现了 trait 的 struct 模块是不够的，还必须导入这个 trait 本身。

<h3><a href="#why-cant-the-compiler-infer-use-statements" name="why-cant-the-compiler-infer-use-statements">
为什么编译器不能为我推断 <code>use</code> 声明？
</a></h3>

它可以做到，但你也许也不希望它这么做。尽管在许多情况下，编译器可以通过简单地查找给定的标识符定义的位置来确定要导入的正确模块，但这可能不是一般的情况。在某些情况下， `rustc` 中任何用于在竞争项之间进行选择的规则都有可能引起意外和混淆，而 Rust 更倾向于显式标明名字来源于哪里。

例如，编译器可以说在竞争标识符定义的情况下，选择最早导入的模块。那么如果两个模块 `foo` 和模块 `bar` 都定义了标识符 `baz`，但是 `foo` 是首先注册的模块，编译器会插入 `use foo::baz;`。

```rust
mod foo;
mod bar;

// use foo::baz  // 会由编译器插入。

fn main() {
  baz();
}
```

假如你知道这将会发生，或许这样可以节省少量的按键次数，但当你实际需要用到 `bar::baz()` 中的 `baz()` 时，会大大增加意外的错误信息的可能性，而且通过模块声明来决定一个函数调用的含义，也降低了代码的可读性。这些不是我们愿意做的折衷。

然而，在将来，IDE 可以帮助管理声明，这会在两方面带来更好的结果：机器帮助引入名字，但显式声明这些名字来自何处。

<!--
### How do I package and archive crates from [https://crates.io](https://crates.io)?

TODO: Write this answer.
-->

<h3><a href="#how-do-i-do-dynamic-rust-library-loading" name="how-do-i-do-dynamic-rust-library-loading">
如何动态加载 Rust 库？
</a></h3>

在 Rust 中使用 [libloading](https://crates.io/crates/libloading) 导入动态库，它提供了一个用于动态链接的跨平台系统。

<h3><a href="#why-doesnt-crates-io-have-namespaces" name="why-doesnt-crates-io-have-namespaces">
为什么 crates.io 没有命名空间？
</a></h3>

引用 [https://crates.io](https://crates.io) 设计的[官方解释](https://internals.rust-lang.org/t/crates-io-package-policies/1041)：

> 在 crates.io 的第一个月里，有一些人就问我们关于引入[基于命名空间的包](https://github.com/rust-lang/crates.io/issues/58)的可能性。<br><br>
>
> 尽管基于命名空间的包允许多个作者使用一个单一的，通用的名字，但这也增加了 Rust 代码中和人际沟通中如何引用包的复杂度。乍看之下，这允许多个作者占有像 `http` 这样的名字，但这只意味着人们必须用 `wycats' http` 或者 `reem's http` 来指定所引用的包，相对于像 `wycats-http` 或者 `reem-http` 这样的名字，前者几乎没什么好处。<br><br>
>
> 当我们在没有命名空间的情况下观察包生态系统时，发现人们更倾向于使用更多的富有创意的名字（如 `nologiri` 而不是 `tenderlove's libxml2`）。这些创意名字往往是简短且易于记忆的，部分原因在于减少了层级。这使得关于包的信息更容易被简洁明确地交流。它能创造令人兴奋的品牌。而且，我们已经看到了多个拥有超过 10,000+ 个包的生态系统的成功，如 NPM 和 RubyGems，其社区在单一的命名空间中蓬勃发展。<br><br>
>
> 简而言之，假如 Piston 选择一个如 `bvssvni/game-engine` （允许其它用户选择 `wycats/game-engine`）的名字而不是简单的 `piston`，我们不认为这样对 Cargo 生态系统会更好。<br><br>
>
> 因为命名空间在许多方面有更严格的复杂度，而且因为在将来如果有必要可以添加兼容性，我们坚持使用一个共享的命名空间。

<h2 id="libraries">库</h2>

<h3><a href="#how-can-i-make-an-http-request" name="how-can-i-make-an-http-request">
我该怎样发出一个 HTTP 请求？
</a></h3>

标准库不包含 HTTP 的实现，因此你需要用外部的包。 [Hyper](https://github.com/hyperium/hyper) 是最流行的选择，但[也有众多其他可选](https://crates.io/keywords/http)。

<h3><a href="#how-can-i-write-a-gui-application" name="how-can-i-write-a-gui-application">
如何用 Rust 编写 GUI 应用程序？
</a></h3>

有多种方法在 Rust 中编写 GUI 应用程序。查阅我们的 [GUI 框架列表](https://github.com/kud1ing/awesome-rust#gui)吧。

<h3><a href="#how-can-i-parse-json-xml" name="how-can-i-parse-json-xml">
如何解析 JSON/XML？
</a></h3>

[Serde](https://github.com/serde-rs/serde) 是推荐的将 Rust 数据序列化与反序列化为多种格式的库。

<h3><a href="#is-there-a-standard-2d-vector-crate" name="is-there-a-standard-2d-vector-crate">
有标准的 2D+ 矢量和形状包吗？
</a></h3>

还没有呢！你能写一个吗？

<h3><a href="#how-do-i-write-an-opengl-app" name="how-do-i-write-an-opengl-app">
如何用 Rust 编写 OpenGL 应用程序？
</a></h3>

[Glium](https://github.com/tomaka/glium) 是 Rust 中 OpenGL 编程的主要的库。[GLFW](https://github.com/bjz/glfw-rs) 也是一个可靠的选择。

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

看情况。将面向对象的概念转化到 Rust 有许多种 _方式_，例如[多继承](https://www.reddit.com/r/rust/comments/2sryuw/ideaquestion_about_multiple_inheritence/)，但由于 Rust 不是面向对象的，所以转化的结果可能与 OO 语言看起来有很大的不同。

<h3><a href="#how-do-i-configure-a-struct-with-optional-parameters" name="how-do-i-configure-a-struct-with-optional-parameters">
如何处理有可选参数的结构体的配置？
</a></h3>

最简单的方法是在你用来构造结构体实现的任何函数（ 通常是 `new`）中使用 [`Option`][Option] 类型。另一种方法是使用 [构造器模式](https://aturon.github.io/ownership/builders.html)，在构造类型之前，一些明确实例化成员变量的函数必须被调用。

<h3><a href="#how-do-i-do-global-variables" name="how-do-i-do-global-variables">
如何在 Rust 中做全局对象？
</a></h3>

Rust 中可以用 `const` 声明在编译时计算的全局常量，而 `static` 可用于可变的全局变量。请注意，修改一个 `static mut` 变量需要使用 `unsafe`，因为它允许数据竞争（races），这在安全的 Rust 中保证不会发生的事之一。`const` 与 `static` 值的一项重要区别是，你可以引用 `static` 值，但不能引用 `const` 值，因为它没有一个特定的内存位置。有关 `const` 与 `static` 的更多信息，请阅读 [Rust 之书](https://doc.rust-lang.org/book/const-and-static.html)。

<h3><a href="#how-can-i-set-compile-time-constants-that-are-defined-procedurally" name="how-can-i-set-compile-time-constants-that-are-defined-procedurally">
如何设置程序定义的编译时的常量？
</a></h3>

Rust 目前对编译时常量的支持有限。你可以使用 `const` 声明（类似 `static`，但它不可变，并且在内存中没有特定位置）定义原生类型，以及定义 `const` 函数和固有方法。

要定义那些无法通过这种机制定义的过程常量，使用 [`lazy-static`](https://github.com/rust-lang-nursery/lazy-static.rs) crate，它通过第一次使用的时候自动求值来模拟编译时求值。

<h3><a href="#can-i-run-code-before-main" name="can-i-run-code-before-main">
我可以在 main 发生前运行初始化代码吗？
</a></h3>

Rust 没有「`main` 之前」的概念。你想找到的最接近的可能是 [`lazy-static`](https://github.com/Kimundi/lazy-static.rs) 包，它在初次使用时通过懒惰初始化静态变量做到类似「在main之前」。

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

不允许。全局不能有一个非常量表达式的构造器和解构器。静态构造器也不允许，因为确保一个静态初始化顺序很难。main 之前的生命周期经常被认为是不当的功能，所以 Rust 不允许。

参考 [C++ FQA](http://yosefk.com/c++fqa/ctors.html#fqa-10.12) 中关于「static initialization order fiasco」的问题，以及 [Eric Lippert's blog](https://ericlippert.com/2013/02/06/static-constructors-part-one/) 中针对 C# 中此功能的挑战。

你可以以 [lazy-static](https://crates.io/crates/lazy_static/) 包使用近似的非常量表达式全局对象。

<h2 id="other-languages">其他语言</h2>

<h3><a href="#how-can-i-use-static-fields" name="how-can-i-use-static-fields">
如何在 Rust 中实现类似 C 的 <code>struct X { static int X; };</code>？
</a></h3>

如上所说，Rust 没有 `static` 字段。不过，你可以在指定的模块中声明一个 `static` 变量，这样它将为该模块私有。

<h3><a href="#how-can-i-convert-a-c-style-enum-to-an-integer" name="how-can-i-convert-a-c-style-enum-to-an-integer">
如何将 C 风格的枚举转换为整数，反之亦然？
</a></h3>

把 C 风格的枚举转换成一个整型，可以通过 `as` 表达式，如 `e as i64`（这里 `e` 是某个枚举）。

反方向转换，可以通过 `match` 语句，将不同的数值映射到枚举的不同潜在值。

<h3><a href="#why-do-rust-programs-use-more-memory-than-c" name="why-do-rust-programs-use-more-memory-than-c">
为什么 Rust 程序的二进制文件比 C 程序要大？
</a></h3>

在默认情况下，Rust 程序比功能相当的 C 程序有更大的二进制大小，有几个因素。一般来说， Rust 倾向于为实现世界的性能做优化，而不是程序的大小。

__单态化（Monomorphization）__

Rust 的单态化泛型，意味着为程序中使用的每个具体类型生成一个泛型函数或者类型的新版本。这与 C++ 中模板的工作方式类似。例如，在以下程序中：

```rust
fn foo<T>(t: T) {
    // ... do something
}

fn main() {
    foo(10);       // i32
    foo("hello");  // &str
}
```

在最终的二进制文件中会有两个不同版本的 `foo`，一个专门用于接收 `i32` 的输入，一个专门用于接收 `&str` 的输入。这样实现了通用函数的高效静态调度（dispatch），但代价是会产生较大的二进制文件。

__调试符号__

Rust 程序在编译的时候保留了一些调试符号，即使在 release 模式下编译也是如此。这是用于在 panic 的时候提供回溯（backtrace）的功能，可以用 `strip` 或者其它调试符号删除工具进行删除。还有一点值得注意的是，在 Cargo 的 release 模式下进行编译等同于用 rustc 设置优化级别为 3。一个替代的优化级别（称为 `s` 或 `z`）[最近被引入](https://github.com/rust-lang/rust/pull/32386)，用于告诉编译器针对大小而不是性能进行优化。

__Jemalloc__

Rust 使用 jemalloc 作为默认内存分配器，这会增加一些编译后的二进制文件的大小。选择 jemalloc 是因为它是一个一致的，优质的内存分配器，与许多常见的系统提供的分配器相比具有优越的性能特征。[为了更容易地使用自定义的分配器](https://github.com/rust-lang/rust/issues/32838)，有一项工作正在进行，但尚未完成。

__链接时（link-time）优化__

Rust 在默认情况下不提供链接时优化，但可以指定这么做。这增加了 Rust 编译器可能会执行的优化量，对二进制文件产生很小的影响。与之前提到的文件大小优化模式相结合，效果会更好。

__标准库__

Rust 标准库包括 libbacktrace 和 libunwind，这在某些程序中可能用不到。因此，使用 `#![no_std]` 可以产生更小的二进制文件，但通常也会导致你编写的代码需要大量的修改。请注意，使用不带标准库的 Rust 通常在功能上更接近等效的 C 代码。

例如，以下 C 程序读入一个名字，然后输出向该名字的人的问好 「hello」。

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

用 Rust 重写，你可能得到如下的内容：

```rust
use std::io;

fn main() {
    println!("What's your name?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("Hello {}!", input);
}
```

该程序编译之后与 C 程序相比，具有更大的二进制文件且使用更多的内存。但是该程序并不完全等同于上面的 C 代码。相反，等效的 Rust 代码看起来像这样：

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

这样在内存的使用上就应该大致与 C 相当了，但代价是需要程序员更多的编程技巧，而且缺少了 Rust 通常提供的静态保证（由于这里 `unsafe` 的使用）。

<h3><a href="#why-no-stable-abi" name="why-no-stable-abi">
为什么 Rust 没有像 C 一样稳定的 ABI，以及为什么必须用 extern 来标注？
</a></h3>

承诺一个 ABI 是一个很重大的决定，这会限制语言在将来潜在的有利改进。鉴于 Rust 在 2015 年 5 月才达到 1.0，拟作出一个像稳定 ABI 这样的大承诺还为时过早。但这并不意味着将来不会有。（虽然 C++ 已经设法发展了很多年，但并没有指定一个稳定的 ABI。）

`extern` 关键字允许 Rust 使用一些特定的 ABI，例如明确定义的 C ABI，来与其它语言交互。

<h3><a href="#can-rust-code-call-c-code" name="can-rust-code-call-c-code">
Rust 代码能调用 C 代码吗？
</a></h3>

可以。从 Rust 调用 C 代码与从 C++ 调用 C 代码一样高效。

<h3><a href="#can-c-code-call-rust-code" name="can-c-code-call-rust-code">
C 代码能调用 Rust 代码吗？
</a></h3>

可以。 Rust 代码必须通过一个 `extern` 声明来导出，这和 C-ABI 兼容。这样的函数可以作为函数指针传递给 C 代码，或者通过标识 `#[no_mangle]` 来禁用符号调整（symbol mangling），可以直接从 C 代码中调用。

<h3><a href="#why-rust-vs-cxx" name="why-rust-vs-cxx">
我已经写了完美的 C++。 Rust 能给我什么？
</a></h3>

现代 C++ 包含了许多功能，使得写出安全且正确的代码更不容易出错，但并不完美，仍然很容易引入不安全。这是 C++ 核心开发人员正在努力解决的问题，但他们现在正想尝试实现的许多想法被 C++ 被之前悠久的历史包袱限制住了。

Rust 是从第一天开始就被设计成一种安全的系统编程语言，这意味着它不受限于那些让 C++ 变得如此复杂的历史设计决策。在 C++ 中，安全是通过谨慎的个人纪律来达到的，很容易出错。在 Rust 中，安全是默认的。即使你的团队中包含了不如你完美的人，你仍然能够工作，而无需花费时间仔细检查其代码的安全性漏洞。

<h3><a href="#how-to-get-cxx-style-template-specialization" name="how-to-get-cxx-style-template-specialization">
如何在 Rust 中做到相当于 C++ 模板特化？
</a></h3>

Rust 目前还没有完全等同的模板特化，这[正在研究](https://github.com/rust-lang/rfcs/pull/1210)并有希望尽快加入。不过，可以用[关联类型](https://doc.rust-lang.org/stable/book/second-edition/ch19-03-advanced-traits.html)达成类似的效果。

<h3><a href="#how-does-ownership-relate-to-cxx-move-semantics" name="how-does-ownership-relate-to-cxx-move-semantics">
Rust 的所有权系统如何与 C++ 中的 move 语义相关联？
</a></h3>

基本概念是相似的，但两个系统在实践中的工作方式非常不同。在这两个系统中，
「移动」一个值是转移其底层资源所有权的一种方式。例如，移动一个字符串，
将传递这个字符串的缓冲区，而不是复制它。

在Rust 中，所有权转移是默认的行为。例如，假设我写一个接收 `String`
作为参数的函数，该函数将获得其调用者提供的这个 `String` 的所有权：

```rust
fn process(s: String) { }

fn caller() {
    let s = String::from("Hello, world!");
    process(s); // Transfers ownership of `s` to `process`
    process(s); // Error! ownership already transferred.
}
```

从上面的代码片断可以看出，在 `caller` 函数中，第一次调用 `process`
转移了变量 `s` 的所有权。编译器跟踪所有权，所以第二次调用 `process`
会导致错误，因为同一个值所有权被放弃两次是非法的。如果这个值被借出未归还，
Rust 也会阻止你移动它。

C++ 采用不同的方式。在 C++ 中，默认是复制一个值（更具体的是，调用拷贝
构造器）。然而调用者可以使用一个 「rvalue 引用」来定义他们的参数，如
`string&&`，表示它们将获得这些参数拥有的某些资源的所有权（在本例中为字符串的内部缓冲区）。
然后调用者必须要么传递一个临时表达式或者使用 `std::move` 来显式移动。
那么与上面的 `process` 函数大致相当的代码：

```
void process(string&& s) { }

void caller() {
    string s("Hello, world!");
    process(std::move(s));
    process(std::move(s));
}
```

C++ 编译器没有义务跟踪移动。例如，上面的代码在编译时没有警告或错误，
至少在 clang 上使用默认设置的时候如此。此外，在 C++ 中字符串 `s`
本身的所有权（假设不是它内部的缓冲区）还是在 `caller`，因此当 `caller`
返回时解构器会运行，即使它被移动了（在 Rust 中则相反，被移动的值只会被它的新所有者丢弃）。

<h3><a href="#how-to-interoperate-with-cxx" name="how-to-interoperate-with-cxx">
如何让 Rust 与 C++ 互操作，反之亦然？
</a></h3>

Rust 与 C++ 可以通过 C 互操作。Rust 和 C++ 都提供一个适用于 C 的[外部函数接口](https://doc.rust-lang.org/book/ffi.html)，我们可以用它来相互沟通。如果编写 C 语言的绑定太麻烦，你随时可以用 [rust-bindgen](https://github.com/servo/rust-bindgen) 来帮忙自动生成可执行的 C 绑定。

<h3><a href="#does-rust-have-cxx-style-constructors" name="does-rust-have-cxx-style-constructors">
Rust 有 C++ 风格的构造器吗？
</a></h3>

没有。函数与构造器的目的相同，但不增加语言的复杂度。 Rust 中与构造器等效的函数名字通常是 `new()`，但这只是一个约定而不是一个语言规则。实际上 `new()` 函数与其它任何函数一样。如下例子所示：

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
Rust 有复制构造器吗？
</a></h3>

不太完全。实现了 `Copy` 的类型会执行标准的类 C 「浅拷贝」，无需额外的工作（类似于 C++ 中的 trivially copyable 类型）。要实现自定义复制行为的 `Copy` 类型是不可能的。作为替代的是，在 Rust 中，「复制构造器」是通过实现 `Clone` trait 并显式调用 `clone` 方法来创建的。使用户定义的复制操作清晰地表达了底层的复杂性，使开发人员更加容易识别潜在的大开销操作。

<h3><a href="#does-rust-have-move-constructors" name="does-rust-have-move-constructors">
Rust 有移动构造器吗？
</a></h3>

没有。所有类型的值都通过 `memcpy` 来移动。这使得编写通用的 unsafe 代码更加简单，因为分配，传递和返回都是已知的，不像展开（unwinding）那样具有副作用。

<h3><a href="#compare-go-and-rust" name="compare-go-and-rust">
Go 与 Rust 有何相似，及它们有什么不同？
</a></h3>

Rust 与 Go 有着完全不同的设计目标。下列差异不是全部的差异（它们太多而无法列出），而是较重要的一些：

- Rust 比 Go 处理更低的级别。例如 Rust 不需要垃圾收集，而 Go 需要。一般来说，Rust 提供了与 C 或 C++ 相当的控制级别。
- Rust 专注于确保安全和高效，同时也提供了高级别的可用性，而 Go 则至力于成为一种简单易用的语言，可以快速编译，并可以使用各种工具进行良好的工作。
- Rust 受到来自函数编译世界的强烈影响，包括从 Haskell 类型类中抽取出来的类型系统。Go 有一个简单的类型系统，使用接口进行基本的通用编译。

<h3><a href="#how-do-rust-traits-compare-to-haskell-typeclasses" name="how-do-rust-traits-compare-to-haskell-typeclasses">
Rust 的 trait 与 Haskell 类型类（typeclasses）相较如何？
</a></h3>

Rust 的 trait 与 Haskell 的类型类相似，但目前并没有那么强大，因为 Rust 不能表示更高级的类型。
Rust 的关联类型相当于 Haskell 的类型族（type families）。

Haskell 的类型类和 Rust 的 trait 之前的一些具体区别包括：

- Rust 的 trait 有一个隐含的第一个参数 `Self`。Rust 中的 `trait Bar` 相当于 Haskell 中的 `class Bar self`，Rust 中的 `trait Bar<Foo>` 相当于 Haskell 中的 `class Bar foo self`。
- Rust 中的「父 trait」或者「父类约束」写作 `trait Sub: Super`，与 Haskell 中的 `class Super self => Sub self` 区别。
- Rust 禁止孤儿实例，导致与 Rust 与 Haskell 相比有不同的相关性规则。
- Rust 的 `impl` 解决方案在决定两个 `impl` 是否重叠或者在潜在的 `impl` 之间进行选择时，会考虑相关的 `where` 子句和 trait 绑定。Haskell 仅考虑 `instance` 声明中的约束，则不考虑其它地方提供的约束。
- Rust 中的 trait（[「对象安全」](https://github.com/rust-lang/rfcs/blob/master/text/0255-object-safety.md)）的一个子集可以通过 trait 对象用于动态调度。 Haskell 中则是通过 GHC 的`ExistentialQuantification` 来获得相同的功能。

<h2 id="documentation">文档</h2>

<h3><a href="#why-are-so-many-rust-answers-on-stackoverflow-wrong" name="why-are-so-many-rust-answers-on-stackoverflow-wrong">
为什么 Stack Overflow（问答网站）上的许多 Rust 回答是错的？
</a></h3>

Rust 语言已存在多年，而它在 2015 年 5 月才达成 1.0 版本。在那达成之前，语言曾经发生过很多变化，而问答网站上的答案可能只适用较旧的语言版本。

随着时间的推移，会有越来越多的答案适用于当前版本，从而改善这一问题。

<h3><a href="#where-do-i-report-issues-in-the-rust-documentation" name="where-do-i-report-issues-in-the-rust-documentation">
我该在哪报告 Rust 文档的问题？
</a></h3>

你可以在 Rust 编译器的[问题跟踪器](https://github.com/rust-lang/rust/issues)中报告 Rust 文档的问题。在报告前，请先阅读[贡献指南](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#writing-documentation)。

<h3><a href="#how-do-i-view-rustdoc-documentation-for-a-library-my-project-depends-on" name="how-do-i-view-rustdoc-documentation-for-a-library-my-project-depends-on">
如何查看我的项目所依赖的库的 rustdoc 文档？
</a></h3>

当你用 `cargo doc` 来为你的项目生成文档的时候，它也会生成当前依赖的库的版本的文档。这些文档被放在项目的 `target/doc` 目录中。在构建完之后，使用 `cargo doc --open` 来打开文档，或者你可以自己打开 `target/doc/index.html` 文件。

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
