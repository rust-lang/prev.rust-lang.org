---
layout: ja-JP/default
title:  Rustへ貢献する &mdash; 言語、コンパイラ、標準ライブラリ &middot; The Rust Programming Language
---

# Rustへ貢献する &mdash; 言語、コンパイラ、標準ライブラリ


コンパイラと標準ライブラリのソースコードはメインレポジトリにあります。
メインレポジトリは主にそれらのメンテナンスのために存在しているので、イシュートラッカー上にあるラベルの多くはこれに関連したものです。
RustからLLVM IRへの変換についての[A-codegen]、デバッガで使うためのメタデータ生成についての[A-debuginfo]、エラー時にコンパイラーが提供するフィードバックについての[A-diagnostics]、標準ライブラリの問題についての[A-libs]、構文拡張に関連する[A-macros]と [A-syntaxext]、型についての[A-typesystem]などが有益なラベルです。

コンパイラのアーキテクチャについてきちんとメンテナンスされているガイドはありませんが[ソースツリーに多少の概要はあります][rustc-guide]。
[コンパイラを構成するクレートのAPIドキュメント][internals-docs]もコードを読むときの指針となるでしょうし、同じくソースコードブラウザの[Rust DXR]も参考になるはずです。
[Rustのテストスィートのガイド][testsuite]はRustのビルドシステムに馴染む方法を教えてくれますし、同様に[make tips][tips]をコマンドラインから走らせても得られるものはあるでしょう。


近々入る予定のコンパイラの大きな変更に、内部でASTを直接操作することから[MIRと呼ばれる中間表現][mir]を使うようにするものがあります。（訳注: 翻訳時点で既に入っています）。
この作業でコンパイラが単純化されるため様々な新たな可能性を開くことが期待されます。
例えばMIRベースの変換パスやMIRベースの最適化、インクリメンタルコンパイルなどです。
このため手助けが必要になります。
この作業に必要なまとまった情報源はありませんが[internals.rust-lang.org]や[#rust-internals]で案内してもらいましょう。

[コンパイラがクラッシュしたらばつが悪いですね][ice] &mdash; とても恐しい「コンパイラ内部のエラー(internal compiler error)」 (ICE)。
[I-ICE]ラベルはこれらをトラックしており、大抵ありあまるほどあります。
これらは通常、とっかかりとしては良いバグです、何故なら直ったかどうかは簡単に分かりますし、比較的それ自身で完結することが多いからです。

RustのコードのパフォーマンスはRustの大きな長所の1つです。そしてRustコンパイラのパフォーマンスはRustの大きな短所の一つです。
実行時、あるいは &mdash; 特に &mdash; コンパイル時のパフォーマンスの改善はどんなものであっても歓迎されます。
[I-slow]や[A-optimization]は実行時のパフォーマンスを、[I-compiletime]はコンパイル時のパフォーマンスを扱います。
多数の負荷に対する[コンパイル時間のパフォーマンスをトラックするサイト][rustc-perf]もあります。
コンパイラフラグの `-Z time-passes` はコンパイラパフォーマンスのデバッグに役立つでしょうし、RustのコードはLinuxの `perf` などの標準的なプロファイラでプロファイルが取れます。

大きな新機能は[Request for Comments (RFC)][rfc]プロセスを通して採用されます。RFCでは設計への合意を取ります。
RFCは全員に開かれていますが、既に何度も協業をしている人と関わり合うプロセスなので徐々に関わることをお勧めします &mdash; 思い付きのRFCを歴史的、技術的、人的背景を理解せずに提出すると簡単に印象が悪くなり、人望を失うでしょう。
先程のREADMEファイルを読んでどういう仕組みかをしっかりと理解しましょう。
Rustの歴史の中で多数のアイディアが議論されてきており、拒絶されたものもあれば将来へ保留になったものありますし、RFCの[イシュートラッカー][rfc-issues]上に言語に導入されようとしているアイディアの一覧があります。
RFCが受理され実装されることになる直前に「最後のコメント期間(final comment period)」と呼ばれるものに入ります。[rust-lang/rfcsのfinal-comment-periodラベル][rfc-fcp]がついているものがそれです。
同じように、安定版コンパイラで機能が有効にされる（「ungating」と呼ばれています）前に[rust-lang/rustでfinal-comment-period][issue-fcp]に入ります。
どちらのFCPも言語の方針へ関わったり意見を言ったりするための重要な期間であり、[internals.rust-lang.org]のサブチームの週報で広報されます。

Rustのコンパイラエンジニアには[#rustc]で、言語設計者には[#rust-lang]で、ライブラリの設計者には[#rust-libs]で会いましょう。

<!--
TODO: guide to compile-time benchmarking
TODO: using the triage bot?
TODO: some of this RFC description could probably go in the RFC readme
-->


[#rust-internals]: https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-internals
[#rust-lang]: https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-lang
[#rust-libs]: https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-libs
[#rustc]: https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rustc
[A-codegen]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-codegen
[A-debuginfo]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-debuginfo
[A-diagnostics]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-diagnostics
[A-libs]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-libs
[A-macros]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-macros
[A-optimization]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-optimization
[A-syntaxext]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-syntaxext
[A-typesystem]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-typesystem
[I-ICE]: https://github.com/rust-lang/rust/labels/I-ICE
[I-compiletime]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AI-compiletime
[I-slow]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AI-slow
[Rust DXR]: https://dxr.mozilla.org/rust/source/src
[ice]: https://users.rust-lang.org/t/glacier-a-big-ol-pile-of-ice/3380
[internals-docs]: https://manishearth.github.io/rust-internals-docs
[internals.rust-lang.org]: https://internals.rust-lang.org/
[issue-fcp]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AB-unstable+label%3Afinal-comment-period
[mir]: https://github.com/rust-lang/rust/issues/27840
[rfc-fcp]: https://github.com/rust-lang/rfcs/pulls?q=is%3Aopen+is%3Apr+label%3Afinal-comment-period
[rfc-issues]: https://github.com/rust-lang/rfcs/issues
[rfc]: https://github.com/rust-lang/rfcs#table-of-contents
[rustc-guide]: https://github.com/rust-lang/rust/blob/master/src/librustc/README.md
[rustc-perf]: http://ncameron.org/perf-rustc/
[testsuite]: https://github.com/rust-lang/rust-wiki-backup/blob/master/Note-testsuite.md
[tips]: https://github.com/rust-lang/rust/blob/3d1f3c9d389d46607ae28c51cc94c1f43d65f3f9/Makefile.in#L48
