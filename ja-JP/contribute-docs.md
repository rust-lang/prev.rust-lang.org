---
layout: ja-JP/default
title: Rustへ貢献する &mdash; ドキュメント &middot; プログラミング言語Rust
---

# Rustへ貢献する &mdash; ドキュメント

ドキュメントの質も量もこれで十分ということは決してありません。
Rustのドキュメントを様々な側面から改善するのに深い知識は必要としません。
逆にドキュメントを執筆したりレビューしたり編集したりするのはRustを学ぶ良い手段です。
さらにドキュメントの改善点はすぐに見付かりますし、いくらでもあります。
何か書いてあることに不満がありますか？ドキュメントに書いていない情報を見付けましたか？あなたのプルリクエストをお待ちしています。

***あなたの書ける最も価値のあるドキュメントは [Rustのエコシステムを形成するクレートのドキュメント][crate_docs]です。***
ソースツリー内にあるドキュメントは比較的カバー率は高いですが日々Rustプログラマが使っている[多数の有名なクレートやツール][awesome-rust]のドキュメントはそうでもありません。
人気のRustプロジェクトへAPIドキュメントを書いてあげるとメンテナから親愛の情を得られるでしょう。

[プログラミング言語Rust][The Book]はRustの主要なドキュメントでRustのメインレポジトリでメンテナンスされています。
独自の[A-book]ラベルを持っていて、継続的に改善されています。
メインレポジトリにある他のドキュメントには[Rustリファレンス][The Rust Reference]、[標準ライブラリAPIドキュメント][std]、[Rustonomicon][The Rustonomicon]（`unsafe`を正しく使うためのガイド）があります。
[Rustスタイルガイドライン][Rust Style Guidelines]はまだ未完成で、表立ってはリンクが張られていません。
意欲的なコントリビュータはここを狙うとよい成果が得られるでしょう。
[エラーインデックス][err]はコンパイラによって生成されたエラーの詳解を提供します。
新たなエラーが追加されたらこのドキュメントも[メンテナンスされないといけません][err-issue]し、インデックスに反映されていないエラーが常に追加されていきます。
ほとんどのソースツリー内のドキュメントは[src/doc]ディレクトリにあります。イシュートラッカー上では全て[A-docs]ラベルで管理されています。
最後に、このドキュメントと他のWebのページは[RustのWebサイトのGitレポジトリ][Rust website Git repository]で管理されています。
コントリビュートするには編集してプルリクエストを投げさえすれば大丈夫です。

Rustの重要なドキュメントのうち、かなりの量がメインレポジトリになかったり、Rustプロジェクトによって管理されていなかったりしますが、それでもこれらはRustの成功にとって非常に重要です。
アクティブに開発されていてコントリビュータを必要としているRustの素晴しいドキュメントの例として、[Rust By Example]、[Rust Design Patterns]そして[rust-rosetta]などがあります。
貢献すべき他の既存のドキュメントプロジェクトについては[rust-learning]を参照して下さい。

他のRustのドキュメンターに[#rust-docs]で会いましょう。

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
[The Rust Reference]: https://doc.rust-lang.org/reference
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
