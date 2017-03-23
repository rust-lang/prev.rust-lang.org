---
layout: ja-JP/default
title: Rustへ貢献する &mdash; ライブラリ &middot; The Rust Programming Language
---

# Rustへ貢献する &mdash; ライブラリ

Rustのコードをたくさん書いてRustに貢献したいと思うのなら、ライブラリが適任です。
Rustは若い言語なので、いまだ存在しない、あるいは不完全なライブラリというものが多数存在していて、改善や生存競争を必要としています。

何を書けばインパクトが合って楽しいか、それを決めるのは一般に難しいものですので、ここにいくつかアイデアを挙げます。

* [/r/rust]や[users.rust-lang.org]の"what's everyone working on this week"スレッドを読んで参加してください。他のRustプログラマが協力者を捜しているという告知もあります。
* [awesome-rust]や[libs.rs]でRustのライブラリに親しんでみてください。
* Webブラウザの[Servo]、HTTPライブラリの[hyper]、ソースコードフォーマッタの[rustfmt]、Unixライブラリのバインディングである[nix]、Lint集の[clippy]などの大きなプロジェクトは新しいコントリビュータのためにイシューに「easy」とラベルをつけています。
* [PistonDevelopers]、[servo]、[redox-os]、[iron]、[contain-rs]、[hyperium]などのアクティブなRustのGitHubオーガニゼーションに関わってみてください。
  これらのサブコミュニティはrust-langオーガニゼーションよりも人手を必要としていて、案内してくれる経験豊富なRust開発者であふれているので簡単に居場所を見付けられることが多いです。
* [rust-lang-nursery]のライブラリがそこからから正式にrust-langに組み入れられるよう手引きするのを手伝ってみてください。ただ、残念ながらそれを実現するのに必要とされるドキュメントがあまり揃っていません。[#rust-libs]で尋ねてみて下さい。
* RFCのイシュートラッカーで[コミュニティライブラリの要求][requested]を捜して、作ってみてください。
* [Github Trending][trending]をウォッチして、現在アクティブなRustプロジェクトを捜してみてください。

ライブラリ作者として[Rustライブラリのベストプラクティス][lib-prac]を読むと良いでしょう。

Rustライブラリの設計者に[#rust-libs]で会いましょう。

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
