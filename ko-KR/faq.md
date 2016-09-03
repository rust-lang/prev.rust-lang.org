---
layout: ko-KR/faq
title: 자주 묻는 질문들 &middot; Rust 프로그래밍 언어
---

# 자주 묻는 질문들

<p class="faq-intro">
이 문서는 Rust 프로그래밍 언어에 대한 흔한 질문들을 답하기 위해 존재합니다. 이 문서는 언어에 대한 완전한 안내서도 아니고, 언어를 가르치는 도구도 아닙니다. 이 문서는 Rust 커뮤니티에서 사람들이 되풀이하여 맞닥뜨리는 질문들을 답하고, Rust의 일부 설계가 왜 그렇게 결정되었는지를 밝히기 위한 참조서입니다.
</p>

<p class="faq-intro">
여기에서 답하지 않았지만 흔하거나 중요한 질문이 누락되어 있다고 생각하신다면 저희가 고칠 수 있도록 <a href="https://github.com/rust-lang/rust-www/blob/master/CONTRIBUTING.md">부담 없이 도와주세요</a>.
</p>

<div id="toc">
    <h2>목차</h2><a href="#toggle-toc"></a>
    <div class="contents">
        <ol id="toc-contents">
            <li><a href="#project">Rust 프로젝트</a></li>
            <li><a href="#performance">성능</a></li>
            <li><a href="#syntax">문법</a></li>
            <li><a href="#numerics">숫자</a></li>
            <li><a href="#strings">문자열</a></li>
            <li><a href="#collections">컬렉션</a></li>
            <li><a href="#ownership">소유권</a></li>
            <li><a href="#lifetimes">수명</a></li>
            <li><a href="#generics">일반화 (제너릭)</a></li>
            <li><a href="#input-output">입출력</a></li>
            <li><a href="#error-handling">오류 처리</a></li>
            <li><a href="#concurrency">동시성</a></li>
            <li><a href="#macros">매크로</a></li>
            <li><a href="#debugging">디버깅 및 도구</a></li>
            <li><a href="#low-level">저수준</a></li>
            <li><a href="#cross-platform">다중 플랫폼</a></li>
            <li><a href="#modules-and-crates">모듈 및 크레이트</a></li>
            <li><a href="#libraries">라이브러리</a></li>
            <li><a href="#design-patterns">디자인 패턴</a></li>
            <li><a href="#other-languages">다른 언어들</a></li>
            <li><a href="#documentation">문서</a></li>
        </ol>
    </div>
</div>


<h2 id="project">Rust 프로젝트</h2>

<h3><a href="#what-is-this-projects-goal" name="what-is-this-projects-goal">
이 프로젝트의 목표는 무엇입니까?
</a></h3>

안전하고, 동시적이며, 실용적인 시스템 언어를 설계하고 구현하기 위함입니다.

Rust는 이 수준의 추상화와 효율을 추구하는 다른 언어들이 만족스럽지 못 하기에 존재합니다. 특히:

1. 안전성이 너무 덜 주목되어 있습니다.
2. 동시성 지원이 부족합니다.
3. 실용적으로 쓰기가 힘듭니다.
4. 자원에 대한 제어가 제한적입니다.

Rust는 효율적인 코드와 편안한 수준의 추상화를 제공하며, 동시에 위 4가지를 모두 개선하는 대안으로 만들어졌습니다.

<h3><a href="#is-this-project-controlled-by-mozilla" name="is-this-project-controlled-by-mozilla">
이 프로젝트를 Mozilla가 제어하나요?
</a></h3>

아니오. Rust는 2006년에 그레이던 호어(Graydon Hoare)가 시간을 쪼개서 하던 사이드 프로젝트로 시작하여 3년간 개발되었습니다.
2009년에 언어가 기본 테스트를 실행하고 핵심 개념들을 시연할 수 있을 정도로 성숙하자 Mozilla가 관여하기 시작했습니다.
Mozilla는 여전히 Rust를 지원하고 있습니다만, Rust는 전 세계의 많은 장소에 퍼져 있는 열정적인 사람들로 이루어진 커뮤니티가 개발하고 있습니다.
[Rust 팀](https://www.rust-lang.org/team.html)은 Mozilla 직원들과 아닌 사람들 둘 다를 포함하고, `rustc`(Rust의 컴파일러)에는 [1,000명 이상의 서로 다른 기여자](https://github.com/rust-lang/rust/)가 참여해 왔습니다.

[프로젝트 거버넌스](https://github.com/rust-lang/rfcs/blob/master/text/1068-rust-governance.md)를 따라, Rust는 프로젝트의 비전과 우선 순위를 설정하는 코어 팀에 의해 관리되며 이 코어 팀이 전체적인 관점에서 프로젝트를 인도합니다.
또한 개별 관심 분야의 개발을 인도하고 장려하기 위한 서브팀들이 있으며, 핵심 언어, 컴파일러, Rust 라이브러리, Rust 도구, 그리고 공식 Rust 커뮤니티의 중재 등이 여기에 포함됩니다.
이들 각 분야 안에서의 설계는 [RFC 과정](https://github.com/rust-lang/rfcs)을 통해 심화됩니다.
RFC를 필요로 하지 않는 수정은 [`rustc` 저장소](https://github.com/rust-lang/rust)의 풀 요청(pull request)을 통해 결정이 내려집니다.

<h3><a href="#what-are-some-non-goals" name="what-are-some-non-goals">
Rust의 목표가 아닌 것은 무엇이 있나요?
</a></h3>

1. We do not employ any particularly cutting-edge technologies. Old, established techniques are better.
2. We do not prize expressiveness, minimalism or elegance above other goals. These are desirable but subordinate goals.
3. We do not intend to cover the complete feature-set of C++, or any other language. Rust should provide majority-case features.
4. We do not intend to be 100% static, 100% safe, 100% reflective, or too dogmatic in any other sense. Trade-offs exist.
5. We do not demand that Rust run on "every possible platform". It must eventually work without unnecessary compromises on widely-used hardware and software platforms.

<h3><a href="#how-does-mozilla-use-rust" name="how-does-mozilla-use-rust">
Mozilla에서 Rust를 사용하는 프로젝트가 무엇인가요?
</a></h3>

주된 프로젝트는 Mozilla가 만들고 있는 실험적인 브라우저 엔진인 [Servo](https://github.com/servo/servo)가 있습니다.
또한 파이어폭스에 [Rust 컴포넌트를 통합](https://bugzilla.mozilla.org/show_bug.cgi?id=1135640)하는 작업도 진행 중입니다.

<h3><a href="#what-examples-are-there-of-large-rust-projects" name="what-examples-are-there-of-large-rust-projects">
대규모 Rust 프로젝트의 예제가 있나요?
</a></h3>

현재 가장 큰 오픈소스 Rust 프로젝트로는 [Servo](https://github.com/servo/servo)와 [Rust 컴파일러](https://github.com/rust-lang/rust) 자신이 있습니다.

<h3><a href="#who-else-is-using-rust" name="who-else-is-using-rust">
Rust를 쓰는 다른 곳이 있나요?
</a></h3>

[계속 늘어나고 있습니다!](friends.html)

<!--
### What projects are good examples of idiomatic Rust code?

TODO: Write this answer.
-->

<h3><a href="#how-can-i-try-rust-easily" name="how-can-i-try-rust-easily">
Rust를 쉽게 시도해 보려면 어떻게 해야 하나요?
</a></h3>

The easiest way to try Rust is through the [playpen](https://play.rust-lang.org/), an online app for writing and running Rust code. If you want to try Rust on your system, [install it](https://www.rust-lang.org/install.html) and go through the [Guessing Game](https://doc.rust-lang.org/stable/book/guessing-game.html) tutorial in the book.

<h3><a href="#how-do-i-get-help-with-rust-issues" name="how-do-i-get-help-with-rust-issues">
Rust 문제들에 도움을 받으려면 어떻게 해야 하나요?
</a></h3>

There are several ways. You can:

- Post in [users.rust-lang.org](https://users.rust-lang.org/), the official Rust users forum
- Ask in the official [Rust IRC channel](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust) (#rust on irc.mozilla.org)
- Ask on [Stack Overflow](https://stackoverflow.com/questions/tagged/rust) with the "rust" tag
- Post in [/r/rust](https://www.reddit.com/r/rust), the unofficial Rust subreddit

<h3><a href="#why-has-rust-changed-so-much" name="why-has-rust-changed-so-much">
Rust가 그동안 왜 그렇게 많이 바뀌었나요?
</a></h3>

Rust started with a goal of creating a safe but usable systems programming language. In pursuit of this goal it explored a lot of ideas, some of which it kept (lifetimes, traits) while others were discarded (the typestate system, green threading). Also, in the run up to 1.0 a lot of the standard library was rewritten as early designs were updated to best use Rust's features and provide quality, consistent cross-platform APIs. Now that Rust has reached 1.0, the language is guaranteed to be "stable"; and while it may continue to evolve, code which works on current Rust should continue to work on future releases.

<h3><a href="#how-does-rust-language-versioning-work" name="how-does-rust-language-versioning-work">
Rust 언어의 버전은 어떻게 작동하나요?
</a></h3>

Rust's language versioning follows [SemVer](http://semver.org/), with backwards incompatible changes of stable APIs only allowed in minor versions if those changes fix compiler bugs, patch safety holes, or change dispatch or type inference to require additional annotation. More detailed guidelines for minor version changes are available as approved RFCs for both the [language](https://github.com/rust-lang/rfcs/blob/master/text/1122-language-semver.md) and [standard library](https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md).

Rust maintains three "release channels": stable, beta, and nightly. Stable and beta are updated every six weeks, with the current nightly becoming the new beta, and the current beta becoming the new stable. Language and standard library features marked unstable or hidden behind feature gates may only be used on the nightly release channel. New features land as unstable, and are "ungated" once approved by the core team and relevant subteams. This approach allows for experimentation while providing strong backwards-compatibility guarantees for the stable channel.

For additional details, read the Rust blog post ["Stability as a Deliverable."](http://blog.rust-lang.org/2014/10/30/Stability.html)

<h3><a href="#can-i-use-unstable-features-on-the-beta-or-stable-channel" name="can-i-use-unstable-features-on-the-beta-or-stable-channel">
불안정한 기능을 베타나 안정 버전에서 사용할 수 있나요?
</a></h3>

No, you cannot. Rust works hard to provide strong guarantees about the stability of the features provided on the beta and stable channels. When something is unstable, it means that we can't provide those guarantees for it yet, and don't want people relying on it staying the same. This gives us the opportunity to try changes in the wild on the nightly release channel, while still maintaining strong guarantees for people seeking stability.

Things stabilize all the time, and the beta and stable channels update every six weeks, with occasional fixes accepted into beta at other times. If you're waiting for a feature to be available without using the nightly release channel, you can locate its tracking issue by checking the [`B-unstable`](https://github.com/rust-lang/rust/issues?q=is%3Aissue+is%3Aopen+tracking+label%3AB-unstable) tag on the issue tracker.

<h3><a href="#what-are-feature-gates" name="what-are-feature-gates">
"기능 게이트"가 무엇인가요?
</a></h3>

"Feature gates" are the mechanism Rust uses to stabilize features of the compiler, language, and standard library. A feature that is "gated" is accessible only on the nightly release channel, and then only when it has been explicitly enabled through `#[feature]` attributes or the `-Z unstable-options` command line argument. When a feature is stabilized it becomes available on the stable release channel, and does not need to be explicitly enabled. At that point the features is considered "ungated". Feature gates allow developers to test experimental features while they are under development, before they are available in the stable language.

<h3><a href="#why-a-dual-mit-asl2-license" name="why-a-dual-mit-asl2-license">
MIT/ASL2 이중 라이선스를 쓰는 이유는 무엇인가요?
</a></h3>

아파치 라이선스(ASL)는 특허권의 침해를 방지하는 중요한 장치가 있지만 GPL 버전 2와 호환되지 않습니다.
GPL2와 Rust를 함께 쓰는 데 문제가 없게 하기 위해 MIT로도 함께 라이선스되어 있습니다.

<h3><a href="#why-a-permissive-license" name="why-a-permissive-license">
MPL이나 삼중 라이선스가 아니라 BSD 계열의 관대한(permissive) 라이선스를 쓰는 이유가 있나요?
</a></h3>

어느 정도는 원 개발자(그레이던)가 그걸 좋아해서였고, 또 어느 정도는 언어들은 더 넓은 대중을 가지며 웹 브라우저 같은 제품과는 달리 더 다양한 곳에 포함되고 사용될 수 있기 때문도 있습니다.
우리는 최대한 많은 잠재 기여자들에게 어필하려고 합니다.

<h2 id="performance">성능</h2>

<h3><a href="#how-fast-is-rust" name="how-fast-is-rust">
Rust는 얼마나 빠른가요?
</a></h3>

Fast! Rust is already competitive with idiomatic C and C++ in a number of benchmarks (like the [Benchmarks Game](https://benchmarksgame.alioth.debian.org/u64q/compare.php?lang=rust&lang2=gpp) and [others](https://github.com/kostya/benchmarks)).

Like C++, Rust takes [zero-cost abstractions](http://blog.rust-lang.org/2015/05/11/traits.html) as one of its core principles: none of Rust's abstractions impose a global performance penalty, nor is there overhead from any runtime system.

Given that Rust is built on LLVM and strives to resemble Clang from LLVM's perspective, any LLVM performance improvements also help Rust. In the long run, the richer information in Rust's type system should also enable optimizations that are difficult or impossible for C/C++ code.

<h3><a href="#is-rust-garbage-collected" name="is-rust-garbage-collected">
Rust는 쓰레기 수거(garbage collection)를 하나요?
</a></h3>

No. One of Rust's key innovations is guaranteeing memory safety (no segfaults) *without* requiring garbage collection.

By avoiding GC, Rust can offer numerous benefits: predictable cleanup of resources, lower overhead for memory management, and essentially no runtime system. All of these traits make Rust lean and easy to embed into arbitrary contexts, and make it much easier to [integrate Rust code with languages that have a GC](http://calculist.org/blog/2015/12/23/neon-node-rust/).

Rust avoids the need for GC through its system of ownership and borrowing, but that same system helps with a host of other problems, including
[resource management in general](http://blog.skylight.io/rust-means-never-having-to-close-a-socket/) and [concurrency](http://blog.rust-lang.org/2015/04/10/Fearless-Concurrency.html).

For when single ownership does not suffice, Rust programs rely on the standard reference-counting smart pointer type, [`Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html), and its thread-safe counterpart, [`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html), instead of GC.

We are however investigating *optional* garbage collection as a future
extension. The goal is to enable smooth integration with
garbage-collected runtimes, such as those offered by the
[Spidermonkey](https://developer.mozilla.org/en-US/docs/Mozilla/Projects/SpiderMonkey)
and [V8](https://developers.google.com/v8/?hl=en) JavaScript engines.
Finally, some people have investigated implementing
[pure Rust garbage collectors](https://manishearth.github.io/blog/2015/09/01/designing-a-gc-in-rust/)
without compiler support.

<h3><a href="#why-is-my-program-slow" name="why-is-my-program-slow">
제 프로그램이 왜 느린 거죠?
</a></h3>

The Rust compiler doesn't compile with optimizations unless asked to, [as optimizations slow down compilation and are usually undesirable during development](https://users.rust-lang.org/t/why-does-cargo-build-not-optimise-by-default/4150/3).

If you compile with `cargo`, use the `--release` flag. If you compile with `rustc` directly, use the `-O` flag. Either of these will turn on optimizations.

<h3><a href="#why-is-rustc-slow" name="why-is-rustc-slow">
Rust 컴파일이 느린 것 같습니다. 왜 그런 건가요?
</a></h3>

Code translation and optimizations. Rust provides high level abstractions that compile down into efficient machine code, and those translations take time to run, especially when optimizing.

But Rust's compilation time is not as bad as it may seem, and there is reason to believe it will improve. When comparing projects of similar size between C++ and Rust, compilation time of the entire project is generally believed to be comparable. The common perception that Rust compilation is slow is in large part due to the differences in the *compilation model* between C++ and Rust: C++'s compilation unit is the file, while Rust's is the crate, composed of many files. Thus, during development, modifying a single C++ file can result in much less recompilation than in Rust. There is a major effort underway to refactor the compiler to introduce [incremental compilation](https://github.com/rust-lang/rfcs/blob/master/text/1298-incremental-compilation.md), which will provide Rust the compile time benefits of C++'s model.

Aside from the compilation model, there are several other aspects of Rust's language design and compiler implementation that affect compile-time performance.

First, Rust has a moderately-complex type system, and must spend a non-negligible amount of compile time enforcing the constraints that make Rust safe at runtime.

Secondly, the Rust compiler suffers from long-standing technical debt, and notably generates poor-quality LLVM IR which LLVM must spend time "fixing". There is hope that future [MIR-based](https://github.com/rust-lang/rfcs/blob/master/text/1211-mir.md) optimization and translation passes will ease the burden the Rust compiler places on LLVM.

Thirdly, Rust's use of LLVM for code generation is a double-edged sword: while it enables Rust to have world-class runtime performance, LLVM is a large framework that is not focused on compile-time performance, particularly when working with poor-quality inputs.

Finally, while Rust's preferred strategy of monomorphising generics (ala C++) produces fast code, it demands that significantly more code be generated than other translation strategies. Rust programmers can use trait objects to trade away this code bloat by using dynamic dispatch instead.

<h3><a href="#why-are-rusts-hashmaps-slow" name="why-are-rusts-hashmaps-slow">
Rust의 <code>HashMap</code>은 왜 느린가요?
</a></h3>

By default, Rust's [`HashMap`][HashMap] uses the [SipHash](https://131002.net/siphash/) hashing algorithm, which is designed to prevent [hash table collision attacks](http://programmingisterrible.com/post/40620375793/hash-table-denial-of-service-attacks-revisited) while providing [reasonable performance on a variety of workloads](https://www.reddit.com/r/rust/comments/3hw9zf/rust_hasher_comparisons/cub4oh6).

While SipHash [demonstrates competitive performance](http://cglab.ca/%7Eabeinges/blah/hash-rs/) in many cases, one case where it is notably slower than other hashing algorithms is with short keys, such as integers. This is why Rust programmers often observe slow performance with [`HashMap`][HashMap]. The [FNV hasher](https://crates.io/crates/fnv) is frequently recommended for these cases, but be aware that it does not have the same collision-resistance properties as SipHash.

<h3><a href="#why-is-there-no-integrated-benchmarking" name="why-is-there-no-integrated-benchmarking">
왜 통합된 성능 측정 인프라가 없는 건가요?
</a></h3>

There is, but it's only available on the nightly release channel. We ultimately plan to build a pluggable system for integrated benchmarks, but in the meantime, the current system is [considered unstable](https://github.com/rust-lang/rust/issues/29553).

<h3><a href="#does-rust-do-tail-call-optimization" name="does-rust-do-tail-call-optimization">
Rust는 꼬리 재귀(tail-call) 최적화를 하나요?
</a></h3>

Not generally, no. Tail-call optimization may be done in [limited circumstances](http://llvm.org/docs/CodeGenerator.html#sibling-call-optimization), but is [not guaranteed](https://mail.mozilla.org/pipermail/rust-dev/2013-April/003557.html). As the feature has always been desired, Rust has a keyword (`become`) reserved, though it is not clear yet whether it is technically possible, nor whether it will be implemented. There was a [proposed extension](https://github.com/rust-lang/rfcs/pull/81) that would allow tail-call elimination in certain contexts, but it is currently postponed.

<h3><a href="#does-rust-have-a-runtime" name="does-rust-have-a-runtime">
Rust에는 런타임이 있나요?
</a></h3>

Not in the typical sense used by languages such as Java, but parts of the Rust standard library can be considered a "runtime", providing a heap, backtraces, unwinding, and stack guards. There is a [small amount of initialization code](https://github.com/rust-lang/rust/blob/33916307780495fe311fe9c080b330d266f35bfb/src/libstd/rt.rs#L43) that runs before the user's `main` function. The Rust standard library additionally links to the C standard library, which does similar [runtime initialization](http://www.embecosm.com/appnotes/ean9/html/ch05s02.html). Rust code can be compiled without the standard library, in which case the runtime is roughly equivalent to C's.

<h2 id="syntax">문법</h2>

<h3><a href="#why-curly-braces" name="why-curly-braces">
왜 중괄호인가요? Rust의 문법이 하스켈이나 파이썬 같지 않은 이유가 있나요?
</a></h3>

Use of curly braces to denote blocks is a common design choice in a variety of programming languages, and Rust's consistency is useful for people already familiar with the style.

Curly braces also allow for more flexible syntax for the programmer and a simpler parser in the compiler.

<h3><a href="#why-brackets-around-blocks" name="why-brackets-around-blocks">
<code>if</code> 조건에서 소괄호를 생략할 수 있는데, 그럼 한 줄짜리 블럭에는 왜 중괄호를 넣어야 하나요? C 같은 문법이 안 되는 이유가 있나요?
</a></h3>

Whereas C requires mandatory parentheses for `if`-statement conditionals but leaves brackets optional, Rust makes the opposite choice for its `if`-expressions. This keeps the conditional clearly separate from the body and avoids the hazard of optional brackets, which can lead to easy-to-miss errors during refactoring, like Apple's [goto fail](https://gotofail.com/) bug.

<h3><a href="#why-no-literal-syntax-for-dictionaries" name="why-no-literal-syntax-for-dictionaries">
연관 배열의 리터럴 문법이 없는 이유는 무엇인가요?
</a></h3>

Rust's overall design preference is for limiting the size of the *language* while enabling powerful *libraries*. While Rust does provide initialization syntax for arrays and string literals, these are the only collection types built into the language. Other library-defined types, including the ubiquitous [`Vec`][Vec] collection type, use macros for initialization like the [`vec!`][VecMacro] macro.

This design choice of using Rust's macro facilities to initialize collections will likely be extended generically to other collections in the future, enabling simple initialization of not only [`HashMap`][HashMap] and [`Vec`][Vec], but also other collection types such as [`BTreeMap`][BTreeMap]. In the meantime, if you want a more convenient syntax for initializing collections, you can [create your own macro](https://stackoverflow.com/questions/27582739/how-do-i-create-a-hashmap-literal) to provide it.

<h3><a href="#when-should-i-use-an-implicit-return" name="when-should-i-use-an-implicit-return">
언제 암묵적인 반환을 써야 하나요?
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
왜 함수의 인자들은 추론되지 않는 거죠?
</a></h3>

In Rust, declarations tend to come with explicit types, while actual code has its types inferred. There are several reasons for this design:

- Mandatory declaration signatures help enforce interface stability at both the module and crate level.
- Signatures improve code comprehension for the programmer, eliminating the need for an IDE running an inference algorithm across an entire crate to be able to guess at a function's argument types; it's always explicit and nearby.
- Mechanically, it simplifies the inference algorithm, as inference only requires looking at one function at a time.

<h3><a href="#why-does-match-have-to-be-exhaustive" name="why-does-match-have-to-be-exhaustive">
왜 <code>match</code>에는 모든 조건들이 들어 있어야 하나요?
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

<h2 id="numerics">숫자</h2>

<h3><a href="#which-type-of-float-should-i-use" name="which-type-of-float-should-i-use">
부동 소숫점 계산을 할 때 <code>f32</code>와 <code>f64</code> 중 어느 쪽을 선호해야 하나요?
</a></h3>

프로그램의 목적에 따라 어느 쪽을 쓸지가 달라집니다.

만약 부동 소숫점 숫자가 최대한 정밀해야 한다면 [`f64`][f64]를 우선시하세요.
만약 크기를 작게 유지하거나 최대한의 성능을 얻고 싶고, 그에 따라 줄어드는 정밀도를 신경쓰지 않겠다면 [`f32`][f32]가 낫습니다.
64비트 하드웨어에서도 [`f32`][f32]가 보통 더 빠릅니다.
예를 들어 그래픽 프로그래밍에서는 높은 성능이 필요하고 화면 상의 픽셀을 표현하는 데는 32비트 부동 소숫점으로 충분하기 때문에 보통 [`f32`][f32]를 씁니다.

잘 모르겠으면 정밀도를 우선시해서 [`f64`][f64]를 선택하세요.

<h3><a href="#why-cant-i-compare-floats" name="why-cant-i-compare-floats">
실수들을 비교하거나 <code>HashMap</code> 및 <code>BTreeMap</code>의 키로 쓸 수 없는 이유는 뭔가요?
</a></h3>

Floats can be compared with the `==`, `!=`, `<`, `<=`, `>`, and `>=` operators, and with the `partial_cmp()` function. `==` and `!=` are part of the [`PartialEq`][PartialEq] trait, while `<`, `<=`, `>`, `>=`, and `partial_cmp()` are part of the [`PartialOrd`][PartialOrd] trait.

Floats cannot be compared with the `cmp()` function, which is part of the [`Ord`][Ord] trait, as there is no total ordering for floats. Furthermore, there is no total equality relation for floats, and so they also do not implement the [`Eq`][Eq] trait.

There is no total ordering or equality on floats because the floating-point value [`NaN`](https://en.wikipedia.org/wiki/NaN) is not less than, greater than, or equal to any other floating-point value or itself.

Because floats do not implement [`Eq`][Eq] or [`Ord`][Ord], they may not be used in types whose trait bounds require those traits, such as [`BTreeMap`][BTreeMap] or [`HashMap`][HashMap]. This is important because these types *assume* their keys provide a total ordering or total equality relation, and will malfunction otherwise.

There [is a crate](https://crates.io/crates/ordered-float) that wraps [`f32`][f32] and [`f64`][f64] to provide [`Ord`][Ord] and [`Eq`][Eq] implementations, which may be useful in certain cases.

<h3><a href="#how-can-i-convert-between-numeric-types" name="how-can-i-convert-between-numeric-types">
수치형들 사이에 변환을 하려면 어떻게 하나요?
</a></h3>

There are two ways: the `as` keyword, which does simple casting for primitive types, and the [`Into`][Into] and [`From`][From] traits, which are implemented for a number of type conversions (and which you can implement for your own types). The [`Into`][Into] and [`From`][From] traits are only implemented in cases where conversions are lossless, so for example, `f64::from(0f32)` will compile while `f32::from(0f64)` will not. On the other hand, `as` will convert between any two primitive types, truncating values as necessary.


<h3><a href="#why-doesnt-rust-have-increment-and-decrement-operators" name="why-doesnt-rust-have-increment-and-decrement-operators">
왜 Rust에는 증감 연산자가 없나요?
</a></h3>

전위 및 후위 증감 연산자는 편하긴 하지만 꽤 복잡합니다.
이들 연산자를 쓰려면 연산 순서를 알아야 하고, C나 C++에서 이로 인한 미묘한 버그나 정의되지 않은 동작이 흔히 발생하지요.
`x = x + 1`이나 `x += 1`은 살짝 더 길 뿐이지만 모호하지 않습니다.

<h2 id="strings">문자열</h2>

<h3><a href="#how-to-convert-string-or-vec-to-slice" name="how-to-convert-string-or-vec-to-slice">
<code>String</code>이나 <code>Vec&lt;T&gt;</code>를 슬라이스(<code>&amp;str</code> 및 <code>&amp;[T]</code>)로 어떻게 바꾸나요?
</a></h3>

Usually, you can pass a reference to a `String` or `Vec<T>` wherever a slice is expected.
Using [Deref coercions](https://doc.rust-lang.org/stable/book/deref-coercions.html), [`String`s][String] and [`Vec`s][Vec] will automatically coerce to their respective slices when passed by reference with `&` or `& mut`.

Methods implemented on `&str` and `&[T]` can be accessed directly on `String` and `Vec<T>`. For example, `some_string.char_at(0)` will work even though `char_at` is a method on `&str` and `some_string` is a `String`.

In some cases, such as generic code, it's necessary to convert manually. Manual conversions can be achieved using the slicing operator, like so: `&my_vec[..]`.

<h3><a href="#how-to-convert-between-str-and-string" name="how-to-convert-between-str-and-string">
<code>&amp;str</code>를 <code>String</code>로 바꾸거나 반대로 하려면 어떻게 하나요?
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
두 개의 다른 문자열 타입에 어떤 차이가 있나요?
</a></h3>

[`String`][String] is an owned buffer of UTF-8 bytes allocated on the heap. Mutable [`String`s][String] can be modified, growing their capacity as needed. [`&str`][str] is a fixed-capacity "view" into a [`String`][String] allocated elsewhere, commonly on the heap, in the case of slices dereferenced from [`String`s][String], or in static memory, in the case of string literals.

[`&str`][str] is a primitive type implemented by the Rust language, while [`String`][String] is implemented in the standard library.

<h3><a href="#how-do-i-do-o1-character-access-in-a-string" name="how-do-i-do-o1-character-access-in-a-string">
<code>String</code>의 각 문자를 O(1), 즉 상수 시간에 접근하려면 어떻게 해야 하나요?
</a></h3>

You cannot. At least not without a firm understanding of what you mean by "character", and preprocessing the string to find the index of the desired character.

Rust strings are UTF-8 encoded. A single visual character in UTF-8 is not necessarily a single byte as it would be in an ASCII-encoded string. Each byte is called a "code unit" (in UTF-16, code units are 2 bytes; in UTF-32 they are 4 bytes). "Code points" are composed of one or more code units, and combine in "grapheme clusters" which most closely approximate characters.

Thus, even though you may index on bytes in a UTF-8 string, you can't access the `i`th code point or grapheme cluster in constant time. However, if you know at which byte that desired code point or grapheme cluster begins, then you _can_ access it in constant time. Functions including [`str::find()`][str__find] and regex matches return byte indices, facilitating this sort of access.

<h3><a href="#why-are-strings-utf-8" name="why-are-strings-utf-8">
왜 문자열이 기본적으로 UTF-8인가요?
</a></h3>

The [`str`][str] type is UTF-8 because we observe more text in the wild in this encoding – particularly in network transmissions, which are endian-agnostic – and we think it's best that the default treatment of I/O not involve having to recode codepoints in each direction.

This does mean that locating a particular Unicode codepoint inside a string is an O(n) operation, although if the starting byte index is already known then they can be accessed in O(1) as expected. On the one hand, this is clearly undesirable; on the other hand, this problem is full of trade-offs and we'd like to point out a few important qualifications:

Scanning a [`str`][str] for ASCII-range codepoints can still be done safely byte-at-a-time. If you use [`.as_bytes()`][str__as_bytes], pulling out a [`u8`][u8] costs only `O(1)` and produces a value that can be cast and compared to an ASCII-range [`char`][char]. So if you're (say) line-breaking on `'\n'`, byte-based treatment still works. UTF-8 was well-designed this way.

Most "character oriented" operations on text only work under very restricted language assumptions such as "ASCII-range codepoints only". Outside ASCII-range, you tend to have to use a complex (non-constant-time) algorithm for determining linguistic-unit (glyph, word, paragraph) boundaries anyway. We recommend using an "honest" linguistically-aware, Unicode-approved algorithm.

The [`char`][char] type is UTF-32. If you are sure you need to do a codepoint-at-a-time algorithm, it's trivial to write a `type wstr = [char]`, and unpack a [`str`][str] into it in a single pass, then work with the `wstr`. In other words: the fact that the language is not "decoding to UTF32 by default" shouldn't stop you from decoding (or re-encoding any other way) if you need to work with that encoding.

For a more in-depth explanation of why UTF-8 is usually preferable over UTF-16 or UTF-32, read the [UTF-8 Everywhere manifesto](http://utf8everywhere.org/).

<h3><a href="#what-string-type-should-i-use" name="what-string-type-should-i-use">
어떤 문자열 타입을 써야 하죠?
</a></h3>

Rust has four pairs of string types, [each serving a distinct purpose](http://www.suspectsemantics.com/blog/2016/03/27/string-types-in-rust/). In each pair, there is an "owned" string type, and a "slice" string type. The organization looks like this:

|               | "Slice" type | "Owned" type |
|:--------------|:-------------|:-------------|
| UTF-8         | `str`        | `String`     |
| OS-compatible | `OsStr`      | `OsString`   |
| C-compatible  | `CStr`       | `CString`    |
| System path   | `Path`       | `PathBuf`    |

Rust's different string types serve different purposes. `String` and `str` are UTF-8 encoded general-purpose strings. `OsString` and `OsStr` are encoded according to the current platform, and are used when interacting with the operating system. `CString` and `CStr` are the Rust equivalent of strings in C, and are used in FFI code, and `PathBuf` and `Path` are convenience wrappers around `OsString` and `OsStr`, providing methods specific to path manipulation.

<h3><a href="#why-are-there-multiple-types-of-strings" name="why-are-there-multiple-types-of-strings">
<code>&str</code>와 <code>String</code>을 동시에 받는 함수를 어떻게 짤 수 있나요?
</a></h3>

There are several options, depending on the needs of the function:

- If the function needs an owned string, but wants to accept any type of string, use an `Into<String>` bound.
- If the function needs a string slice, but wants to accept any type of string, use an `AsRef<str>` bound.
- If the function does not care about the string type, and wants to handle the two possibilities uniformly, use `Cow<str>` as the input type.

__Using `Into<String>`__

In this example, the function will accept both owned strings and string slices, either doing nothing or converting the input into an owned string within the function body. Note that the conversion needs to be done explicitly, and will not happen otherwise.

```rust
fn accepts_both<S: Into<String>>(s: S) {
    let s = s.into();   // This will convert s into a `String`.
    // ... the rest of the function
}
```

__Using `AsRef<str>`__

In this example, the function will accept both owned strings and string slices, either doing nothing or converting the input into a string slice. This can be done automatically by taking the input by reference, like so:

```rust
fn accepts_both<S: AsRef<str>>(s: &S) {
    // ... the body of the function
}
```

__Using `Cow<str>`__

In this example, the function takes in a `Cow<str>`, which is not a generic type but a container, containing either an owned string or string slice as needed.

```rust
fn accepts_cow(s: Cow<str>) {
    // ... the body of the function
}
```


<h2 id="collections">컬렉션</h2>

<h3><a href="#can-i-implement-linked-lists-in-rust" name="can-i-implement-linked-lists-in-rust">
Rust로 벡터나 연결 리스트 같은 자료 구조를 효율적으로 구현할 수 있나요?
</a></h3>

If your reason for implementing these data structures is to use them for other programs, there's no need, as efficient implementations of these data structures are provided by the standard library.

If, however, [your reason is simply to learn](http://cglab.ca/~abeinges/blah/too-many-lists/book/), then you will likely need to dip into unsafe code. While these data structures _can_ be implemented entirely in safe Rust, the performance is likely to be worse than it would be with the use of unsafe code. The simple reason for this is that data structures like vectors and linked lists rely on pointer and memory operations that are disallowed in safe Rust.

For example, a doubly-linked list requires that there be two mutable references to each node, but this violates Rust's mutable reference aliasing rules. You can solve this using [`Weak<T>`][Weak], but the performance will be poorer than you likely want. With unsafe code you can bypass the mutable reference aliasing rule restriction, but must manually verify that your code introduces no memory safety violations.

<h3><a href="#how-can-i-iterate-over-a-collection-without-consuming-it" name="how-can-i-iterate-over-a-collection-without-consuming-it">
컬렉션을 움직이거나 소모(consume)하지 않고 각 원소에 대해 반복하려면 어떻게 하나요?
</a></h3>

The easiest way is by using the collection's [`IntoIterator`][IntoIterator] implementation. Here is an example for [`&Vec`][Vec]:

```rust
let v = vec![1,2,3,4,5];
for item in &v {
    print!("{} ", item);
}
println!("\nLength: {}", v.len());
```

Rust `for` loops call `into_iter()` (defined on the [`IntoIterator`][IntoIterator] trait) for whatever they're iterating over. Anything implementing the [`IntoIterator`][IntoIterator] trait may be looped over with a `for` loop. [`IntoIterator`][IntoIterator] is implemented for [`&Vec`][Vec] and [`&mut Vec`][Vec], causing the iterator from `into_iter()` to borrow the contents of the collection, rather than moving/consuming them. The same is true for other standard collections as well.

If a moving/consuming iterator is desired, write the `for` loop without `&` or `&mut` in the iteration.

If you need direct access to a borrowing iterator, you can usually get it by calling the `iter()` method.

<h3><a href="#why-do-i-need-to-type-the-array-size-in-the-array-declaration" name="why-do-i-need-to-type-the-array-size-in-the-array-declaration">
배열 선언에 배열 크기를 넣어야 하는 이유가 무엇인가요?
</a></h3>

You don't necessarily have to. If you're declaring an array directly, the size is inferred based on the number of elements. But if you're declaring a function that takes a fixed-size array, the compiler has to know how big that array will be.

One thing to note is that currently Rust doesn't offer generics over arrays of different size. If you'd like to accept a contiguous container of a variable number of values, use a [`Vec`][Vec] or slice (depending on whether you need ownership).

<h2 id="ownership">소유권</h2>

<h3><a href="#how-can-i-implement-a-data-structure-that-contains-cycles" name="how-can-i-implement-a-data-structure-that-contains-cycles">
그래프 같이 사이클을 포함하는 자료 구조를 구현하려면 어떻게 해야 하나요?
</a></h3>

There are at least four options (discussed at length in [Too Many Linked Lists](http://cglab.ca/~abeinges/blah/too-many-lists/book/)):

- You can implement it using [`Rc`][Rc] and [`Weak`][Weak] to allow shared ownership of nodes,
although this approach pays the cost of memory management.
- You can implement it using `unsafe` code using raw pointers. This will be
more efficient, but bypasses Rust's safety guarantees.
- Using vectors and indices into those vectors. There are [several](http://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/) [available](https://featherweightmusings.blogspot.com/2015/04/graphs-in-rust.html) examples and explanations of this approach.
- Using borrowed references with [`UnsafeCell`][UnsafeCell]. There are [explanations and code](https://github.com/nrc/r4cppp/blob/master/graphs/README.md#node-and-unsafecell) available for this approach.

<h3><a href="#how-can-i-define-a-struct-that-contains-a-reference-to-one-of-its-own-fields" name="how-can-i-define-a-struct-that-contains-a-reference-to-one-of-its-own-fields">
자기 자신의 필드를 가리키는 참조를 가진 구조체는 어떻게 선언해야 하나요?
</a></h3>

It's possible, but useless to do so. The struct becomes permanently borrowed by itself and therefore can't be moved. Here is some code illustrating this:

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
값으로 호출하기, 소모(consume)하기, 움직이기, 그리고 소유권을 넘기기에 서로 무슨 차이가 있나요?
</a></h3>

다 같은 뜻입니다.
네 가지 경우에서 모두, 값이 새 소유자에게 옮겨가고, 원 소유자가 소유를 잃어버려 더 이상 쓸 수 없게 됩니다.
만약 타입이 `Copy` 트레이트를 구현한다면 원 소유자의 값은 무효화되지 않아 계속 쓸 수 있습니다.

<h3><a href="#why-can-values-of-some-types-by-reused-while-others-are-consumed" name="why-can-values-of-some-types-by-reused-while-others-are-consumed">
왜 어떤 타입은 함수에 넘긴 뒤에도 재사용할 수 있지만 다른 타입은 그렇지 않나요?
</a></h3>

타입이 [`Copy`][Copy] 트레이트를 구현하면 함수에 전달될 때 복사됩니다.
Rust의 모든 수치형은 [`Copy`][Copy]를 구현하지만, 구조체는 기본적으로 [`Copy`][Copy]를 구현하지 않기 때문에 대신 이동이 일어납니다.
즉 구조체는 함수에서 다시 반환되거나 하지 않는 한 더 이상 다른 데서 사용할 수 없게 됩니다.

<h3><a href="#how-do-you-deal-with-a-use-of-moved-value-error" name="how-do-you-deal-with-a-use-of-moved-value-error">
"움직인 값을 사용"(use of moved value)했다는 오류를 어떻게 다뤄야 하나요?
</a></h3>

This error means that the value you're trying to use has been moved to a new owner. The first thing to check is whether the move in question was necessary: if it moved into a function, it may be possible to rewrite the function to use a reference, rather than moving. Otherwise if the type being moved implements [`Clone`][Clone], then calling `clone()` on it before moving will move a copy of it, leaving the original still available for further use. Note though that cloning a value should typically be the last resort since cloning can be expensive, causing further allocations.

If the moved value is of your own custom type, consider implementing [`Copy`][Copy] (for implicit copying, rather than moving) or [`Clone`][Clone] (explicit copying). [`Copy`][Copy] is most commonly implemented with `#[derive(Copy, Clone)]` ([`Copy`][Copy] requires [`Clone`][Clone]), and [`Clone`][Clone] with `#[derive(Clone)]`.

If none of these are possible, you may want to modify the function that acquired ownership to return ownership of the value when the function exits.

<h3><a href="#what-are-the-rules-for-different-self-types-in-methods" name="what-are-the-rules-for-different-self-types-in-methods">
메소드 선언에서 언제 <code>self</code>, <code>&amp;self</code>, 또는 <code>&amp;mut self</code>를 쓰는지 규칙이 있나요?
</a></h3>

- Use `self` when a function needs to consume the value
- Use `&self` when a function only needs a read-only reference to the value
- Use `&mut self` when a function needs to mutate the value without consuming it

<h3><a href="#how-can-i-understand-the-borrow-checker" name="how-can-i-understand-the-borrow-checker">
빌림 체커(borrow checker)를 이해하는 방법은 무엇인가요?
</a></h3>

The borrow checker applies only a few rules, which can be found in the Rust book's [section on borrowing](https://doc.rust-lang.org/stable/book/references-and-borrowing.html#the-rules), when evaluating Rust code. These rules are:

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
<code>Rc</code>가 유용한 때는 언제인가요?
</a></h3>

This is covered in the official documentation for [`Rc`][Rc], Rust's non-atomically reference-counted pointer type. In short, [`Rc`][Rc] and its thread-safe cousin [`Arc`][Arc] are useful to express shared ownership, and have the system automatically deallocate the associated memory when no one has access to it.

<h3><a href="#how-do-i-return-a-closure-from-a-function" name="how-do-i-return-a-closure-from-a-function">
함수에서 어떻게 클로저를 반환하나요?
</a></h3>

To return a closure from a function, it must be a "move closure", meaning that the closure is declared with the `move` keyword. As [explained in the Rust book](https://doc.rust-lang.org/book/closures.html#move-closures), this gives the closure its own copy of the captured variables, independent of its parent stack frame. Otherwise, returning a closure would be unsafe, as it would allow access to variables that are no longer valid; put another way: it would allow reading potentially invalid memory. The closure must also be wrapped in a [`Box`][Box], so that it is allocated on the heap. Read more about this [in the book](https://doc.rust-lang.org/book/closures.html#returning-closures).

<h3><a href="#what-are-deref-coercions" name="what-are-deref-coercions">
Deref 변환(coercion)이 무엇이고 어떻게 동작하나요?
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

<h2 id="lifetimes">수명</h2>

<h3><a href="#why-lifetimes" name="why-lifetimes">
수명(lifetime)을 왜 쓰나요?
</a></h3>

수명은 메모리 안전성에 대한 Rust의 해답입니다.
Rust는 수명을 사용해서 쓰레기 수거(garbage collection)의 성능 비용 없이 메모리 안전성을 보장합니다.
수명 개념은 다양한 학술 연구에 기반해 있으며, 《[Rust 프로그래밍 언어](https://doc.rust-lang.org/stable/book/bibliography.html#type-system)》에서 참조를 확인할 수 있습니다.

<h3><a href="#why-is-the-lifetime-syntax-the-way-it-is" name="why-is-the-lifetime-syntax-the-way-it-is">
왜 수명 문법이 지금과 같은가요?
</a></h3>

`'a` 문법은 ML 계열의 프로그래밍 언어에서 따 왔는데, 여기서 `'a` 문법은 일반화된 타입 인자를 나타내는 데 사용됩니다.
Rust의 경우 수명 문법은 모호하지 않고, 눈에 띄어야 했으며 타입 선언에서 트레이트와 참조와 함께 쓰기 좋아야 했습니다.
다른 문법도 의논되었으나 이보다 확실히 더 좋은 문법이 제시되진 않았습니다.

<h3><a href="#how-do-i-return-a-borrow-to-something-i-created-from-a-function" name="how-do-i-return-a-borrow-to-something-i-created-from-a-function">
함수 안에서 만든 무언가를 빌려서 반환하려면 어떻게 하나요?
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

This approach is simpler, but often results in unnecessary allocations.

<h3><a href="#when-are-lifetimes-required-to-be-explicit" name="when-are-lifetimes-required-to-be-explicit">
왜 어떤 참조에는 <code>&amp;'a T</code>같이 수명이 있고 다른 참조에는 <code>&amp;T</code>같이 없는 건가요?
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
Rust는 어떻게 "널 포인터가 없다"는 것과 "유령 포인터(dangling pointer)가 없다"는 것을 보장하나요?
</a></h3>

`&Foo`와 `&mut Foo` 타입의 값을 만드는 유일한 방법은 이미 존재하는 `Foo` 타입의 값을 참조가 가리키는 값으로 명시하는 것 뿐입니다.
참조는 주어진 코드 영역(즉, 참조의 수명) 안에서 원래 값을 "빌리며", 참조가 값을 빌린 동안에는 원래 값을 옮기거나 소멸시킬 수 없습니다.

<h3><a href="#how-do-i-express-the-absense-of-a-value-without-null" name="how-do-i-express-the-absense-of-a-value-without-null">
<code>null</code> 없이 값이 없다는 걸 어떻게 표현하나요?
</a></h3>

[`Option`][Option] 타입을 씁니다.
이 타입은 `Some(T)`이거나 `None`일 수 있는데, `Some(T)`은 `T` 타입의 값이 들어 있다는 걸 나타내고, `None`은 값이 없다는 걸 나타냅니다.

<h2 id="generics">일반화 (제너릭)</h2>

<h3><a href="#what-is-monomorphisation" name="what-is-monomorphisation">
"단형화"(monomorphisation)가 무엇인가요?
</a></h3>

Monomorphisation specializes each use of a generic function (or structure) with specific instance,
based on the parameter types of calls to that function (or uses of the structure).

During monomorphisisation a new copy of the generic function is translated for each unique set of types the function is instantiated with. This is the same strategy used by C++. It results in fast code that is specialized for every call-site and statically dispatched, with the tradeoff that functions instantiated with many different types can cause "code bloat", where multiple function instances result in larger binaries than would be created with other translation strategies.

Functions that accept [trait objects](https://doc.rust-lang.org/book/trait-objects.html) instead of type parameters do not undergo monomorphisation. Instead, methods on the trait objects are dispatched dynamically at runtime.

<h3><a href="#whats-the-difference-between-a-function-and-a-closure-that-doesnt-capture" name="whats-the-difference-between-a-function-and-a-closure-that-doesnt-capture">
함수와 아무 변수도 갈무리하지 않는 클로저 사이에 어떤 차이가 있나요?
</a></h3>

Functions and closures are operationally equivalent, but have different runtime representations due to their differing implementations.

Functions are a built-in primitive of the language, while closures are essentially syntactic sugar for one of three traits: [`Fn`][Fn], [`FnMut`][FnMut], and [`FnOnce`][FnOnce]. When you make a closure, the Rust compiler automatically creates a struct implementing the appropriate trait of those three and containing the captured environment variables as members, and makes it so the struct can be called as a function. Bare functions can not capture an environment.

The big difference between these traits is how they take the `self` parameter. [`Fn`][Fn] takes `&self`, [`FnMut`][FnMut] takes `&mut self`, and [`FnOnce`][FnOnce] takes `self`.

Even if a closure does not capture any environment variables, it is represented at runtime as two pointers, the same as any other closure.

<h3><a href="#what-are-higher-kinded-types" name="what-are-higher-kinded-types">
상류(higher-kinded) 타입이 무엇인가요? 왜 제가 그걸 필요로 할 수 있나요? Rust에 상류 타입이 없는 이유는 무엇인가요?
<!-- 역주: higher-order 고차, higher-rank 상위, higher-kind 상류. -->
</a></h3>

Higher-kinded types are types with unfilled parameters. Type constructors, like [`Vec`][Vec], [`Result`][Result], and [`HashMap`][HashMap] are all examples of higher-kinded types: each requires some additional type parameters in order to actually denote a specific type, like `Vec<u32>`. Support for higher-kinded types means these "incomplete" types may be used anywhere "complete" types can be used, including as generics for functions.

Any complete type, like [`i32`][i32], [`bool`][bool], or [`char`][char] is of kind `*` (this notation comes from the field of type theory). A type with one parameter, like [`Vec<T>`][Vec] is of kind `* -> *`, meaning that [`Vec<T>`][Vec] takes in a complete type like [`i32`][i32] and returns a complete type `Vec<i32>`. A type with three parameters, like [`HashMap<K, V, S>`][HashMap] is of kind `* -> * -> * -> *`, and takes in three complete types (like [`i32`][i32], [`String`][String], and [`RandomState`][RandomState]) to produce a new complete type `HashMap<i32, String, RandomState>`.

In addition to these examples, type constructors can take *lifetime* arguments, which we'll denote as `Lt`. For example, `slice::Iter` has kind `Lt -> * -> *`, because it must be instantiated like `Iter<'a, u32>`.

The lack of support for higher-kinded types makes it difficult to write certain kinds of generic code. It's particularly problematic for abstracting over concepts like iterators, since iterators are often parameterized over a lifetime at least. That in turn has prevented the creation of traits abstracting over Rust's collections.

Another common example is concepts like functors or monads, both of which are type constructors, rather than single types.

Rust doesn't currently have support for higher-kinded types because it hasn't been a priority compared to other improvements we want to make. Since the design is a major, cross-cutting change, we also want to approach it carefully. But there's no inherent reason for the current lack of support.

<h3><a href="#what-do-named-type-parameters-in-generic-types-mean" name="what-do-named-type-parameters-in-generic-types-mean">
일반화 타입에 <code>&lt;T=Foo&gt;</code> 같은 이름이 붙은 타입 인자는 무엇인가요?
</a></h3>

These are called [associated types](https://doc.rust-lang.org/stable/book/associated-types.html), and they allow for the expression of trait bounds that can't be expressed with a `where` clause. For example, a generic bound `X: Bar<T=Foo>` means "`X` must implement the trait `Bar`, and in that implementation of `Bar`, `X` must choose `Foo` for `Bar`'s associated type, `T`." Examples of where such a constraint cannot be expressed via a `where` clause include trait objects like `Box<Bar<T=Foo>>`.

Associated types exist because generics often involve families of types, where one type determines all of the others in a family. For example, a trait for graphs might have as its `Self` type the graph itself, and have associated types for nodes and for edges. Each graph type uniquely determines the associated types. Using associated types makes it much more concise to work with these families of types, and also provides better type inference in many cases.

<h3><a href="#how-do-i-overload-operators" name="how-do-i-overload-operators">
연산자를 오버로드할 수 있나요? 어떤 게 가능하고 어떻게 하나요?
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
<code>Eq</code>/<code>PartialEq</code>과 <code>Ord</code>/<code>PartialOrd</code>가 나뉜 이유는 무엇인가요?
</a></h3>

There are some types in Rust whose values are only partially ordered, or have only partial equality. Partial ordering means that there may be values of the given type that are neither less than nor greater than each other. Partial equality means that there may be values of the given type that are not equal to themselves.

Floating point types ([`f32`][f32] and [`f64`][f64]) are good examples of each. Any floating point type may have the value `NaN` (meaning "not a number"). `NaN` is not equal to itself (`NaN == NaN` is false), and not less than or greater than any other floating point value. As such, both [`f32`][f32] and [`f64`][f64] implement [`PartialOrd`][PartialOrd] and [`PartialEq`][PartialEq] but not [`Ord`][Ord] and not [`Eq`][Eq].

As explained in [the earlier question on floats](#why-cant-i-compare-floats), these distinctions are important because some collections rely on total orderings/equality in order to give correct results.

<h2 id="input-output">입출력</h2>

<h3><a href="#how-do-i-read-a-file-into-a-string" name="how-do-i-read-a-file-into-a-string">
파일을 <code>String</code>로 읽으려면 어떻게 하나요?
</a></h3>

[`std::io`][std-io]의 [`Read`][Read] 트레이트에 있는 [`read_to_string()`][read__read_to_string] 메소드를 씁니다.

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
파일을 효율적으로 읽으려면 어떻게 하나요?
</a></h3>

The [`File`][File] type implements the [`Read`][Read] trait, which has a variety of functions for reading and writing data, including [`read()`][read__read], [`read_to_end()`][read__read_to_end], [`bytes()`][read__bytes], [`chars()`][read__chars], and [`take()`][read__take]. Each of these functions reads a certain amount of input from a given file. [`read()`][read__read] reads as much input as the underlying system will provide in a single call. [`read_to_end()`][read__read_to_end] reads the entire buffer into a vector, allocating as much space as is needed. [`bytes()`][read__bytes] and [`chars()`][read__chars] allow you to iterate over the bytes and characters of the file, respectively. Finally, [`take()`][read__take] allows you to read up to an arbitrary number of bytes from the file. Collectively, these should allow you to efficiently read in any data you need.

For buffered reads, use the [`BufReader`][BufReader] struct, which helps to reduce the number of system calls when reading.

<h3><a href="#how-do-i-do-asynchronous-input-output-in-rust" name="how-do-i-do-asynchronous-input-output-in-rust">
Rust에서 어떻게 비동기 입출력을 하나요?
</a></h3>

There are several libraries providing asynchronous input / output in Rust, including [mioco](https://github.com/dpc/mioco), [coio-rs](https://github.com/zonyitoo/coio-rs), and [rotor](https://github.com/tailhook/rotor).

<h3><a href="#how-do-i-get-command-line-arguments" name="how-do-i-get-command-line-arguments">
Rust에서 명령행 인자를 받는 방법은 무엇인가요?
</a></h3>

The easiest way is to use [`Args`][Args], which provides an iterator over the input arguments.

If you're looking for something more powerful, there are a [number of options on crates.io](https://crates.io/keywords/argument).

<h2 id="error-handling">오류 처리</h2>

<h3><a href="#why-doesnt-rust-have-exceptions" name="why-doesnt-rust-have-exceptions">
Rust에는 왜 예외(exception)가 없나요?
</a></h3>

Exceptions complicate understanding of control-flow, they express validity/invalidity outside of the type system, and they interoperate poorly with multithreaded code (a major focus of Rust).

Rust prefers a type-based approach to error handling, which is [covered at length in the book](https://doc.rust-lang.org/stable/book/error-handling.html). This fits more nicely with Rust's control flow, concurrency, and everything else.

<h3><a href="#whats-the-deal-with-unwrap" name="whats-the-deal-with-unwrap">
여기 저기 보이는 <code>unwrap()</code>를 어떻게 할 수 없나요?
</a></h3>

`unwrap()` is a function that extracts the value inside an [`Option`][Option] or [`Result`][Result] and panics if no value is present.

`unwrap()` shouldn't be your default way to handle errors you expect to arise, such as incorrect user input. In production code, it should be treated like an assertion that the value is non-empty, which will crash the program if violated.

It's also useful for quick prototypes where you don't want to handle an error yet, or blog posts where error handling would distract from the main point.

<h3><a href="#why-do-i-get-errors-with-try" name="why-do-i-get-errors-with-try">
<code>try!</code> 매크로를 쓰는 예제 코드를 실행하려고 할 때 오류가 나는 이유는 뭔가요?
</a></h3>

It's probably an issue with the function's return type. The [`try!`][TryMacro] macro either extracts the value from a [`Result`][Result], or returns early with the error [`Result`][Result] is carrying. This means that [`try`][TryMacro] only works for functions that return [`Result`][Result] themselves, where the `Err`-constructed type implements `From::from(err)`. In particular, this means that the [`try!`][TryMacro] macro cannot work inside the `main` function.

<h3><a href="#error-handling-without-result" name="error-handling-without-result">
모든 곳에 <code>Result</code>를 쓰는 것 말고 더 쉽게 오류를 처리할 방법이 없나요?
</a></h3>

If you're looking for a way to avoid handling [`Result`s][Result] in other people's code, there's always [`unwrap()`][unwrap], but it's probably not what you want. [`Result`][Result] is an indicator that some computation may or may not complete successfully. Requiring you to handle these failures explicitly is one of the ways that Rust encourages robustness. Rust provides tools like the [`try!` macro][TryMacro] to make handling failures ergonomic.

If you really don't want to handle an error, use [`unwrap()`][unwrap], but know that doing so means that the code panics on failure, which usually results in a shutting down the process.

<h2 id="concurrency">동시성</h2>

<h3><a href="#can-i-use-static-values-across-threads-without-an-unsafe-block" name="can-i-use-static-values-across-threads-without-an-unsafe-block">
<code>unsafe</code> 블록 없이 여러 스레드에서 정적인 값을 사용할 수는 없을까요?
</a></h3>

Mutation is safe if it's synchronized. Mutating a static [`Mutex`][Mutex] (lazily initialized via the [lazy-static](https://crates.io/crates/lazy_static/) crate) does not require an `unsafe` block, nor does mutating a static [`AtomicUsize`][AtomicUsize] (which can be initialized without lazy_static).

More generally, if a type implements [`Sync`][Sync] and does not implement [`Drop`][Drop], it [can be used in a `static`](https://doc.rust-lang.org/book/const-and-static.html#static).

<h2 id="macros">매크로</h2>

<h3><a href="#can-i-write-a-macro-to-generate-identifiers" name="can-i-write-a-macro-to-generate-identifiers">
식별자를 생성하는 매크로를 짤 수 있나요?
</a></h3>

Not currently. Rust macros are ["hygienic macros"](https://en.wikipedia.org/wiki/Hygienic_macro), which intentionally avoid capturing or creating identifiers that may cause unexpected collisions with other identifiers. Their capabilities are significantly different than the style of macros commonly associated with the C preprocessor. Macro invocations can only appear in places where they are explicitly supported: items, method declarations, statements, expressions, and patterns. Here, "method declarations" means a blank space where a method can be put. They can't be used to complete a partial method declaration. By the same logic, they can't be used to complete a partial variable declaration.

<h2 id="debugging">디버깅 및 도구</h2>

<h3><a href="#how-do-i-debug-rust-programs" name="how-do-i-debug-rust-programs">
Rust 프로그램은 어떻게 디버깅하나요?
</a></h3>

Rust programs can be debugged using [gdb](https://sourceware.org/gdb/current/onlinedocs/gdb/) or [lldb](http://lldb.llvm.org/tutorial.html), the same as C and C++. In fact, every Rust installation comes with one or both of rust-gdb and rust-lldb (depending on platform support). These are wrappers over gdb and lldb with Rust pretty-printing enabled.

<h3><a href="#how-do-i-locate-a-panic" name="how-do-i-locate-a-panic">
<code>rustc</code>가 표준 라이브러리 코드에서 패닉(panic)이 일어났다고 하는데, 제 코드의 실수를 어떻게 찾을 수 있을까요?
</a></h3>

This error is usually caused by [`unwrap()`ing][unwrap] a `None` or `Err` in client code. Enabling backtraces by setting the environment variable `RUST_BACKTRACE=1` helps with getting more information. Compiling in debug mode (the default for `cargo build`) is also helpful. Using a debugger like the provided `rust-gdb` or `rust-lldb` is also helpful.

<h3><a href="#what-ide-should-i-use" name="what-ide-should-i-use">
무슨 IDE를 써야 하나요?
</a></h3>

There are a number of options for development environment with Rust, all of which are detailed on the official [IDE support page](https://forge.rust-lang.org/ides.html).

<h3><a href="#wheres-rustfmt" name="wheres-rustfmt">
<code>gofmt</code>은 멋져요. <code>rustfmt</code> 같은 건 없나요?
</a></h3>

`rustfmt` is [right here](https://github.com/rust-lang-nursery/rustfmt), and is being actively developed to make reading Rust code as easy and predictable as possible.

<h2 id="low-level">저수준</h2>

<h3><a href="#how-do-i-memcpy-bytes" name="how-do-i-memcpy-bytes">
<code>memcpy</code>같이 바이트를 복사하려면 어떻게 하나요?
</a></h3>

If you want to clone an existing slice safely, you can use [`clone_from_slice`][clone_from_slice].

To copy potentially overlapping bytes, use [`copy`][copy]. To copy nonoverlapping bytes, use [`copy_nonoverlapping`][copy_nonoverlapping]. Both of these functions are `unsafe`, as both can be used to subvert the language's safety guarantees. Take care when using them.

<h3><a href="#does-rust-work-without-the-standard-library" name="does-rust-work-without-the-standard-library">
표준 라이브러리 없이 Rust를 사용하는 건 할 만한가요?
</a></h3>

Absolutely. Rust programs can be set to not load the standard library using the `#![no_std]` attribute. With this attribute set, you can continue to use the Rust core library, which is nothing but the platform-agnostic primitives. As such, it doesn't include IO, concurrency, heap allocation, etc.

<h3><a href="#can-i-write-an-operating-system-in-rust" name="can-i-write-an-operating-system-in-rust">
Rust로 운영체제를 작성할 수 있나요?
</a></h3>

Yes! In fact there are [several projects underway doing just that](http://wiki.osdev.org/Rust).

<h3><a href="#how-can-i-write-endian-independent-values" name="how-can-i-write-endian-independent-values">
<code>i32</code>나 <code>f64</code> 같은 수치형을 빅 엔디안이나 리틀 엔디안 형식으로 파일 및 다른 바이트 스트림에 읽고 쓰려면 어떻게 하나요?
</a></h3>

You should check out the [byteorder crate](http://burntsushi.net/rustdoc/byteorder/), which provides utilities for exactly that.

<h3><a href="#does-rust-guarantee-data-layout" name="does-rust-guarantee-data-layout">
Rust가 메모리 상에 값이 어떻게 배치될 지가 고정되어 있나요?
</a></h3>

Not by default. In the general case, `enum` and `struct` layouts are undefined. This allows the compiler to potentially do optimizations like re-using padding for the discriminant, compacting variants of nested `enum`s, reordering fields to remove padding, etc. `enums` which carry no data ("C-like") are eligible to have a defined representation. Such `enums` are easily distinguished in that they are simply a list of names that carry no data:

```rust
enum CLike {
    A,
    B = 32,
    C = 34,
    D
}
```

The `#[repr(C)]` attribute can be applied to such `enums` to give them the same representation they would have in equivalent C code. This allows using Rust `enum`s in FFI code where C `enum`s are also used, for most use cases. The attribute can also be applied to `struct`s to get the same layout as a C `struct` would.

<h2 id="cross-platform">다중 플랫폼</h2>

<!--
### How do I build a Windows binary that doesn't display the console window?

TODO: Write this answer.
-->

<!--
### How do I make the console-less binary not crash on panic!?

TODO: Write this answer.
-->

<h3><a href="#how-do-i-express-platform-specific-behavior" name="how-do-i-express-platform-specific-behavior">
Rust에서 플랫폼 의존적인 동작을 표현하는 일반적인 방법은 무엇인가요?
</a></h3>

Platform-specific behavior can be expressed using [conditional compilation attributes](https://doc.rust-lang.org/reference.html#conditional-compilation) such as `target_os`, `target_family`, `target_endian`, etc.

<h3><a href="#can-rust-be-used-for-android-ios-programs" name="can-rust-be-used-for-android-ios-programs">
Rust를 안드로이드 및 iOS 프로그래밍에 쓸 수 있나요?
</a></h3>

Yes it can! There are already examples of using Rust for both [Android](https://github.com/tomaka/android-rs-glue) and [iOS](https://www.bignerdranch.com/blog/building-an-ios-app-in-rust-part-1/). It does require a bit of work to set up, but Rust functions fine on both platforms.

<h3><a href="#can-i-run-my-rust-program-in-a-web-browser" name="can-i-run-my-rust-program-in-a-web-browser">
제 Rust 프로그램을 웹 브라우저에서 실행할 수 있나요?
</a></h3>

Not yet, but there are efforts underway to make Rust compile to the web with [Emscripten](https://kripken.github.io/emscripten-site/).

<h3><a href="#how-do-i-cross-compile-rust" name="how-do-i-cross-compile-rust">
Rust에서 크로스 컴파일은 어떻게 하나요?
</a></h3>

Cross compilation is possible in Rust, but it requires [a bit of work](https://github.com/japaric/rust-cross/blob/master/README.md) to set up. Every Rust compiler is a cross-compiler, but libraries need to be cross-compiled for the target platform.

Rust does distribute [copies of the standard library](https://static.rust-lang.org/dist/) for each of the supported platforms, which are contained in the `rust-std-*` files for each of the build directories found on the distribution page, but there are not yet automated ways to install them.

<h2 id="modules-and-crates">모듈 및 크레이트</h2>

<h3><a href="#what-is-the-relationship-between-a-module-and-a-crate" name="what-is-the-relationship-between-a-module-and-a-crate">
모듈과 크레이트 사이에 어떤 관계가 있나요?
</a></h3>

- A crate is a compilation unit, which is the smallest amount of code that the Rust compiler can operate on.
- A module is a (possibly nested) unit of code organization inside a crate.
- A crate contains an implicit, un-named top-level module.
- Recursive definitions can span modules, but not crates.

<h3><a href="#why-cant-the-rust-compiler-find-a-library-im-using" name="why-cant-the-rust-compiler-find-a-library-im-using">
Rust 컴파일러가 제가 <code>use</code>한 라이브러리를 찾지 못 하는 이유는 뭔가요?
</a></h3>

There are a number of possible answers, but a common mistake is not realizing that `use` declarations are relative to the crate root. Try rewriting your declarations to use the paths they would use if defined in the root file of your project and see if that fixes the problem.

There are also `self` and `super`, which disambiguate `use` paths as being relative to the current module or parent module, respectively.

For complete information on `use`ing libraries, read the Rust book's chapter ["Crates and Modules"](https://doc.rust-lang.org/stable/book/crates-and-modules.html).

<h3><a href="#why-do-i-have-to-declare-modules-with-mod" name="why-do-i-have-to-declare-modules-with-mod">
왜 모듈 파일을 정의하기 위해 크레이트 최상위에 <code>mod</code>를 넣어야 하나요? 그냥 <code>use</code>로 지정하면 안 되나요?
</a></h3>

There are two ways to declare modules in Rust, inline or in another file. Here is an example of each:

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

In the first example, the module is defined in the same file it's used. In the second example, the module declaration in the main file tells the compiler to look for either `hello.rs` or `hello/mod.rs`, and to load that file.

Note the difference between `mod` and `use`: `mod` declares that a module exists, whereas `use` references a module declared elsewhere, bringing its contents into scope within the current module.

<h3><a href="#how-do-i-configure-cargo-to-use-a-proxy" name="how-do-i-configure-cargo-to-use-a-proxy">
Cargo가 프록시를 사용하도록 설정하려면 어떻게 하나요?
</a></h3>

As explained on the Cargo [configuration documentation](http://doc.crates.io/config.html), you can set Cargo to use a proxy by setting the "proxy" variable under `[http]` in the configuration file.

<h3><a href="#why-cant-the-compile-find-method-implementations" name="why-cant-the-compile-find-method-implementations">
이미 크레이트를 <code>use</code>했는데도 왜 컴파일러가 메소드 구현을 찾지 못 하는 걸까요?
</a></h3>

For methods defined on a trait, you have to explicitly import the trait declaration. This means it's not enough to import a module where a struct implements the trait, you must also import the trait itself.

<h3><a href="#why-cant-the-compiler-infer-use-statements" name="why-cant-the-compiler-infer-use-statements">
왜 컴파일러가 <code>use</code> 선언을 자동으로 추론하지 못 하나요?
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

However, in the future, an IDE could help manage declarations, which gives you the best of both worlds: machine assistance for pulling in names, but explicit declarations about where those names are coming from.

<!--
### How do I package and archive crates from [https://crates.io](https://crates.io)?

TODO: Write this answer.
-->

<h3><a href="#how-do-i-do-dynamic-rust-library-loading" name="how-do-i-do-dynamic-rust-library-loading">
Rust 라이브러리를 어떻게 동적으로 읽어들이나요?
</a></h3>

Import dynamic libraries in Rust with [libloading](https://crates.io/crates/libloading), which provides a cross-platform system for dynamic linking.

<h3><a href="#why-doesnt-crates-io-have-namespaces" name="why-doesnt-crates-io-have-namespaces">
왜 crates.io에는 이름 공간이 없나요?
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

<h2 id="libraries">라이브러리</h2>

<h3><a href="#how-can-i-make-an-http-request" name="how-can-i-make-an-http-request">
HTTP 요청을 어떻게 보내나요?
</a></h3>

표준 라이브러리에는 HTTP 구현이 포함되어 있지 않으므로 외부 크레이트를 사용해야 합니다.
[Hyper](https://github.com/hyperium/hyper)가 가장 널리 쓰이지만 [다른 라이브러리들도 여럿 있습니다](https://crates.io/keywords/http).

<h3><a href="#how-can-i-write-a-gui-application" name="how-can-i-write-a-gui-application">
Rust로 GUI 애플리케이션을 작성하려면 어떻게 해야 하나요?
</a></h3>

Rust로 GUI 애플리케이션을 만드는 방법은 여럿 있습니다. [GUI 프레임워크들의 목록](https://github.com/kud1ing/awesome-rust#gui)을 참고하세요.

<h3><a href="#how-can-i-parse-json-xml" name="how-can-i-parse-json-xml">
JSON/XML을 파싱하는 방법은 무엇인가요?
</a></h3>

[Serde](https://github.com/serde-rs/serde)는 Rust 데이터를 다른 여러 포맷으로 직렬화(serialize)하고 역직렬화(deserialize)하는데 추천하는 라이브러리입니다.

<h3><a href="#is-there-a-standard-2d-vector-crate" name="is-there-a-standard-2d-vector-crate">
표준 2차원/3차원/... 벡터 및 도형 크레이트가 있나요?
</a></h3>

아직요! 만들어 보실래요?

<h3><a href="#how-do-i-write-an-opengl-app" name="how-do-i-write-an-opengl-app">
Rust로 OpenGL 앱을 작성하려면 어떻게 해야 하죠?
</a></h3>

[Glium](https://github.com/tomaka/glium)는 Rust에서 OpenGL 프로그래밍에 쓰이는 주요 라이브러리입니다.
[GLFW](https://github.com/bjz/glfw-rs) 또한 괜찮은 대안입니다.

<h3><a href="#can-i-write-a-video-game-in-rust" name="can-i-write-a-video-game-in-rust">
Rust로 비디오 게임을 만들 수 있나요?
</a></h3>

가능합니다!
Rust로 만들어진 주요 게임 프로그래밍 라이브러리는 [Piston](http://www.piston.rs/)이 있으며, [Rust 게임 프로그래밍을 위한 서브레딧](https://www.reddit.com/r/rust_gamedev/)과 IRC 채널([Mozilla IRC](https://wiki.mozilla.org/IRC)의 `#rust-gamedev` 채널)도 있습니다.

<h2 id="design-patterns">디자인 패턴</h2>

<h3><a href="#is-rust-object-oriented" name="is-rust-object-oriented">
Rust는 객체 지향적(object-oriented)인가요?
</a></h3>

Rust는 여러 패러다임을 지원합니다.
객체 지향 언어에서 할 수 있는 많은 것들은 Rust에서도 할 수 있지만, 전부 가능한 건 아니고, 여러분에게 익숙한 추상화를 사용하지 않을 수도 있습니다.

<h3><a href="#how-do-i-map-object-oriented-concepts-to-rust" name="how-do-i-map-object-oriented-concepts-to-rust">
객체 지향적인 개념을 Rust에 어떻게 대응시키죠?
</a></h3>

상황에 따라 다릅니다.
[다중 상속](https://www.reddit.com/r/rust/comments/2sryuw/ideaquestion_about_multiple_inheritence/)과 같은 객체 지향 개념을 Rust로 옮기는 방법은 _여럿_ 있습니다만, Rust는 객체 지향이 아니기에 객체 지향 언어들과는 상당히 다르게 보일 수 있습니다.

<h3><a href="#how-do-i-configure-a-struct-with-optional-parameters" name="how-do-i-configure-a-struct-with-optional-parameters">
선택 인자가 있는 구조체를 설정하는 인터페이스를 어떻게 만들어야 할까요?
</a></h3>

The easiest way is to use the [`Option`][Option] type in whatever function you're using to construct instances of the struct (usually `new()`). Another way is to use the [builder pattern](https://aturon.github.io/ownership/builders.html), where only certain functions instantiating member variables must be called before the construction of the built type.

<h3><a href="#how-do-i-do-global-variables" name="how-do-i-do-global-variables">
Rust에서 전역 변수를 쓰려면 어떻게 하죠?
</a></h3>

Globals in Rust can be done using `const` declarations for compile-time computed global constants, while `static` can be used for mutable globals. Note that modifying a `static mut` variable requires the use of `unsafe`, as it allows for data races, one of the things guaranteed not to happen in safe Rust. One important distinction between `const` and `static` values is that you can take references to `static` values, but not references to `const` values, which don't have a specified memory location. For more information on `const` vs. `static`, read [the Rust book](https://doc.rust-lang.org/book/const-and-static.html).

<h3><a href="#how-can-i-set-compile-time-constants-that-are-defined-procedurally" name="how-can-i-set-compile-time-constants-that-are-defined-procedurally">
절차적으로 정의되는 컴파일 시간 상수는 어떻게 설정하나요?
</a></h3>

Rust currently has limited support for compile time constants. You can define primitives using `const` declarations (similar to `static`, but immutable and without a specified location in memory) as well as define `const` functions and inherent methods.

To define procedural constants that can't be defined via these mechanisms, use the [`lazy-static`](https://github.com/rust-lang-nursery/lazy-static.rs) crate, which emulates compile-time evaluation by automatically evaluating the constant at first use.

<h3><a href="#can-i-run-code-before-main" name="can-i-run-code-before-main">
`main` 이전에 실행되는 초기화 코드를 만들 수 있나요?
</a></h3>

Rust has no concept of "life before `main`". The closest you'll see can be done through the [`lazy-static`](https://github.com/Kimundi/lazy-static.rs) crate, which simulates a "before main" by lazily initializing static variables at their first usage.

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
Rust에서 상수 수식이 아닌 값을 전역에 넣을 수 있나요?
</a></h3>

No. Globals cannot have a non-constant-expression constructor and cannot have a destructor at all. Static constructors are undesirable because portably ensuring a static initialization order is difficult. Life before main is often considered a misfeature, so Rust does not allow it.

See the [C++ FQA](http://yosefk.com/c++fqa/ctors.html#fqa-10.12) about the "static initialization order fiasco", and [Eric Lippert's blog](https://ericlippert.com/2013/02/06/static-constructors-part-one/) for the challenges in C#, which also has this feature.

You can approximate non-constant-expression globals with the [lazy-static](https://crates.io/crates/lazy_static/) crate.

<h2 id="other-languages">다른 언어들</h2>

<h3><a href="#how-can-i-use-static-fields" name="how-can-i-use-static-fields">
C의 <code>struct X { static int X; };</code> 같은 코드를 Rust에서는 어떻게 만드나요?
</a></h3>

Rust does not have `static` fields as shown in the code snippet above. Instead, you can declare a `static` variable in a given module, which is kept private to that module.

<h3><a href="#how-can-i-convert-a-c-style-enum-to-an-integer" name="how-can-i-convert-a-c-style-enum-to-an-integer">
C 스타일의 열거형을 정수로 바꾸거나 반대로 하려면 어떻게 하나요?
</a></h3>

Converting a C-style enum to an integer can be done with an `as` expression, like `e as i64` (where `e` is some enum).

Converting in the other direction can be done with a `match` statement, which maps different numeric values to different potential values for the enum.

<h3><a href="#why-do-rust-programs-use-more-memory-than-c" name="why-do-rust-programs-use-more-memory-than-c">
왜 Rust 프로그램이 C보다 많은 메모리를 사용하는 거죠?
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

__Debug symbols__

Rust programs compile with some debug symbols retained, even when compiling in release mode. These are used for providing backtraces on panics, and can be removed with `strip`, or another debug symbol removal tool. It is also useful to note that compiling in release mode with Cargo is equivalent to setting optimization level 3 with rustc. An alternative optimization level (called `s` or `z`) [has recently landed](https://github.com/rust-lang/rust/pull/32386) and tells the compiler to optimize for size rather than performance.

__Jemalloc__

Rust uses jemalloc as the default allocator, which adds some size to compiled Rust binaries. Jemalloc is chosen because it is a consistent, quality allocator that has preferable performance characteristics compared to a number of common system-provided allocators. There is work being done to [make it easier to use custom allocators](https://github.com/rust-lang/rust/issues/32838), but that work is not yet finished.

__Link-time optimization__

Rust does not do link-time optimization by default, but can be instructed to do so. This increases the amount of optimization that the Rust compiler can potentially do, and can have a small effect on binary size. This effect is likely larger in combination with the previously mentioned size optimizing mode.

__Standard library__

The Rust standard library includes libbacktrace and libunwind, which may be undesirable in some programs. Using `#![no_std]` can thus result in smaller binaries, but will also usually result in substantial changes to the sort of Rust code you're writing. Note that using Rust without the standard library is often functionally closer to the equivalent C code.

As an example, the following C program reads in a name and says "hello" to the person with that name.

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

Rewriting this in Rust, you may get something like the following:

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
왜 Rust는 C 같이 안정화된 ABI가 없는 건가요? 그리고 왜 `extern`을 달아야 하는 거죠?
</a></h3>

Committing to an ABI is a big decision that can limit potentially advantageous language changes in the future. Given that Rust only hit 1.0 in May of 2015, it is still too early to make a commitment as big as a stable ABI. This does not mean that one won't happen in the future, though. (Though C++ has managed to go for many years without specifying a stable ABI.)

The `extern` keyword allows Rust to use specific ABI's, such as the well-defined C ABI, for interop with other languages.

<h3><a href="#can-rust-code-call-c-code" name="can-rust-code-call-c-code">
Rust 코드가 C 코드를 호출할 수 있나요?
</a></h3>

Yes. Calling C code from Rust is designed to be as efficient as calling C code from C++.

<h3><a href="#can-c-code-call-rust-code" name="can-c-code-call-rust-code">
C 코드가 Rust 코드를 호출할 수 있나요?
</a></h3>

Yes. The Rust code has to be exposed via an `extern` declaration, which makes it C-ABI compatible. Such a function can be passed to C code as a function pointer or, if given the `#[no_mangle]` attribute to disable symbol mangling, can be called directly from C code.

<h3><a href="#why-rust-vs-cxx" name="why-rust-vs-cxx">
전 이미 C++를 완벽하게 짤 수 있는데, Rust가 어떤 잇점이 있나요?
</a></h3>

Modern C++ includes many features that make writing safe and correct code less error-prone, but it's not perfect, and it's still easy to introduce unsafety. This is something the C++ core developers are working to overcome, but C++ is limited by a long history that predates a lot of the ideas they are now trying to implement.

Rust was designed from day one to be a safe systems programming language, which means it's not limited by historic design decisions that make getting safety right in C++ so complicated. In C++, safety is achieved by careful personal discipline, and is very easy to get wrong. In Rust, safety is the default. It gives you the ability to work in a team that includes people less perfect than you are, without having to spend your time double-checking their code for safety bugs.

<h3><a href="#how-to-get-cxx-style-template-specialization" name="how-to-get-cxx-style-template-specialization">
C++의 템플릿 특수화 같은 걸 Rust에서는 어떻게 할 수 있을까요?
</a></h3>

Rust doesn't currently have an exact equivalent to template specialization, but it is [being worked on](https://github.com/rust-lang/rfcs/pull/1210) and will hopefully be added soon. However, similar effects can be achieved via [associated types](https://doc.rust-lang.org/stable/book/associated-types.html).

<h3><a href="#how-does-ownership-relate-to-cxx-move-semantics" name="how-does-ownership-relate-to-cxx-move-semantics">
Rust의 소유권 시스템이 C++의 "이동" 의미론과 어떻게 연관되나요?
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

C++ compilers are not obligated to track moves. For example, the code
above compiles without a warning or error, at least using the default
settings on clang. Moreover, in C++ ownership of the string `s` itself
(if not its internal buffer) remains with `caller`, and so the
destructor for `s` will run when `caller` returns, even though it has
been moved (in Rust, in contrast, moved values are dropped only by
their new owners).

<h3><a href="#how-to-interoperate-with-cxx" name="how-to-interoperate-with-cxx">
C++에서 Rust와 상호작용하거나, Rust에서 C++와 상호작용하려면 어떻게 하나요?
</a></h3>

Rust and C++ can interoperate through C. Both Rust and C++ provide a [foreign function interface](https://doc.rust-lang.org/book/ffi.html) for C, and can use that to communicate between each other. If writing C bindings is too tedious, you can always use [rust-bindgen](https://github.com/crabtw/rust-bindgen) to help automatically generate workable C bindings.

<h3><a href="#does-rust-have-cxx-style-constructors" name="does-rust-have-cxx-style-constructors">
Rust에는 C++ 같은 생성자가 있나요?
</a></h3>

No. Functions serve the same purpose as constructors without adding language complexity. The usual name for the constructor-equivalent function in Rust is `new()`, although this is just a convention rather than a language rule. The `new()` function in fact is just like any other function. An example of it looks like so:

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
Rust에는 복사 생성자가 있나요?
</a></h3>

Not exactly. Types which implement `Copy` will do a standard C-like "shallow copy" with no extra work (similar to "plain old data" in C++). It is impossible to implement `Copy` types that require custom copy behavior. Instead, in Rust "copy constructors" are created by implementing the `Clone` trait, and explicitly calling the `clone` method. Making user-defined copy operators explicit surfaces the underlying complexity, making it easier for the developer to identify potentially expensive operations.

<h3><a href="#does-rust-have-move-constructors" name="does-rust-have-move-constructors">
Rust에는 이동 생성자가 있나요?
</a></h3>

No. Values of all types are moved via `memcpy`. This makes writing generic unsafe code much simpler since assignment, passing and returning are known to never have a side effect like unwinding.

<h3><a href="#compare-go-and-rust" name="compare-go-and-rust">
Go와 Rust가 비슷한 점은 무엇이고 다른 점은 무엇인가요?
</a></h3>

Rust and Go have substantially different design goals. The following differences are not the only ones (which are too numerous to list), but are a few of the more important ones:

- Rust is lower level than Go. For example, Rust does not require a garbage collector, whereas Go does. In general, Rust affords a level of control that is comparable to C or C++.
- Rust's focus is on ensuring safety and efficiency while also providing high-level affordances, while Go's is on being a small, simple language which compiles quickly and can work nicely with a variety of tools.
- Rust has strong support for generics, which Go does not.
- Rust has strong influences from the world of functional programming, including a type system which draws from Haskell's typeclasses. Go has a simpler type system, using interfaces for basic generic programming.

<h3><a href="#how-do-rust-traits-compare-to-haskell-typeclasses" name="how-do-rust-traits-compare-to-haskell-typeclasses">
Rust 트레이트를 하스켈 타입 클래스와 비교하면 어떤가요?
</a></h3>

Rust traits are similar to Haskell typeclasses, but are currently not as powerful, as Rust cannot express higher-kinded types. Rust's associated types are equivalent to Haskell type families.

Some specific difference between Haskell typeclasses and Rust traits include:

- Rust traits have an implicit first parameter called `Self`. `trait Bar` in Rust corresponds to `class Bar self` in Haskell, and `trait Bar<Foo>` in Rust corresponds to `class Bar foo self` in Haskell.
- "Supertraits" or "superclass constraints" in Rust are written `trait Sub: Super`, compared to `class Super self => Sub self` in Haskell.
- Rust forbids orphan instances, resulting in different coherence rules in Rust compared to Haskell.
- Rust's `impl` resolution considers the relevant `where` clauses and trait bounds when deciding whether two `impl`s overlap, or choosing between potential `impl`s. Haskell only considers the constraints in the `instance` declaration, disregarding any constraints provided elsewhere.
- A subset of Rust's traits (the ["object safe"](https://github.com/rust-lang/rfcs/blob/master/text/0255-object-safety.md) ones) can be used for dynamic dispatch via trait objects. The same feature is available in Haskell via GHC's `ExistentialQuantification`.

<h2 id="documentation">문서</h2>

<h3><a href="#why-are-so-many-rust-answers-on-stackoverflow-wrong" name="why-are-so-many-rust-answers-on-stackoverflow-wrong">
왜 Stack Overflow의 Rust 답변 중에는 틀린 게 많은가요?
</a></h3>

Rust 언어는 여러 해 동안 개발되어 왔으며, 2015년 5월에서야 1.0 버전에 도달했습니다.
그 이전에는 언어가 상당히 많이 바뀌었고 Stack Overflow의 답변 중 많은 것들은 언어가 옛날 버전일 때 작성된 것입니다.

시간이 지날수록 더 많은 답변이 현재 버전을 기준으로 작성되고, 따라서 오래된 답변의 비율이 줄어들 것이므로 이 문제는 자연히 개선될 것입니다.

<h3><a href="#where-do-i-report-issues-in-the-rust-documentation" name="where-do-i-report-issues-in-the-rust-documentation">
Rust 문서에 문제를 보고하려면 어디에 하나요?
</a></h3>

You can report issues in the Rust documentation on the Rust compiler [issue tracker](https://github.com/rust-lang/rust/issues). Make sure to read the [contributing guidelines](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#writing-documentation) first.

<h3><a href="#how-do-i-view-rustdoc-documentation-for-a-library-my-project-depends-on" name="how-do-i-view-rustdoc-documentation-for-a-library-my-project-depends-on">
제 프로젝트가 의존하는 라이브러리의 rustdoc 문서를 어떻게 볼 수 있나요?
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
