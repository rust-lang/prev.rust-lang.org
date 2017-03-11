---
layout: ja-JP/default
title: Rustへ貢献する &mdash; ライブラリ &middot; The Rust Programming Language
---

# Rustへ貢献する &mdash; ライブラリ

Rustのコードをたくさん書いてRustに貢献したいと思うのなら、ライブラリが適任です。
Rustは若い言語なので、まだ存在していない、あるいは不完全なライブラリというのが多種類存在しており、改善、競争が必要です。

意味があって楽しい何を書くかを決めるのは一般に難しいものです。
ここにいくつかアイディアがあります。

* [/r/rust]や[users.rust-lang.org]の"what's everyone working on this week"スレッドを読んで参加する。他のRustプログラマが協力者を捜しているという告知もあります。
* [awesome-rust]や[libs.rs]でRustのライブラリに親しむ
* Webブラウザの[Servo]、HTTPライブラリの[hyper]、ソースコードフォーマッタの[rustfmt]、Unixライブラリのバインディングである[nix]、Lint集の[clippy]などの大きなプロジェクトは新しいコントリビュータのためにイシューに「easy」とラベルをつけています。
* [PistonDevelopers]、[servo]、[redox-os]、[iron]、[contain-rs]、[hyperium]などのアクティブなRustのGitHubオーガニゼーションに関わる。
  これらのサブコミュニティはrust-langオーガニゼーションよりも人手を必要としていて、手引きしてくれる経験豊富なRust開発者で満ちているので簡単に居場所を見付けられることが多いです。
* [rust-lang-nursery]から正式にrust-langに入れるよう導くのを手伝う。
  残念ながら必要とされてるほどのドキュメントがありません。
  [#rust-libs]で尋いてみて下さい。
* RFCのイシュートラッカーで[コミュニティライブラリの要求][requested]を捜して、作る。
* [Github Trending][trending]をウォッチして、今アクティブなRustプロジェクトを捜す。

ライブラリ作者として[Rustライブラリのベストプラクティス][lib-prac]を読むと良いでしょう。

[#rust-libs]でRustライブラリの設計者に会いましょう。

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
