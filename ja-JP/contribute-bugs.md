---
layout: ja-JP/default
title: Rustへ貢献する &mdash; 問題を捜し、仕分け、修正する &middot; プログラミング言語Rust
---

# Rustへ貢献する &mdash; 問題を捜し、仕分け、修正する

日々のプロジェクトのメンテナンスはRustの[イシュートラッカー][issue tracker]、[プルリクエスト][PR]行ったり来たりしていて、もっと助けが必要です。
Rustへ貢献する最も基本的な方法は[E-easy]または[E-mentor]ラベルを見ることです。
これらは新しいRustプログラマでも手が出ることを意味しています。

`E-mentor`では玄人Rust開発者がイシューを解決し[修正をGitHubのプルリクエストで提出する][pull]までアドバイスをくれます。
イシューについてはイシュートラッカー上でメンターの名前に[@メンション][@mentioning]するか、あるいはIRC、E-Mailで連絡を取って下さい。
Rust開発者は多数の通知を受けるため、見逃す可能性があることを覚えておいて下さい。
どんな必要な手段でも彼らを捜すのに遠慮は不要です。

他のRustプロジェクトも同様の入門向けのタスクがあります。
例えばWebブラウザの[Servo]やHTTPライブラリの[hyper]、ソースフォーマッタの[rustfmt]、Unixライブラリバインディングの[nix]、lint集の[clippy]など。

Rustには[広範囲に渡るテストスート][test]がありますが、大抵の場合さらにテストする必要があります。
[E-needstest]ラベルは修正はされたと思われるがテストが必要なイシューを示します。
テストケースを書くのは新たなプロジェクトを理解して貢献を始める良いきっかけです。

Rustは常にイシューの[仕分け][triage]をしてくれる人を必要としています。バグの再現、テストケースの最小化、ラベル付け、解決済みのイシューの終了などのタスクです。
ラベルを付けるのにはGitHubのパーミッションが必要になるかと思いますが、プロジェクトにある程度経験のある人なら権限を得ることはそこまで難しくはないです。
[チームメンバー][team]に尋ねてみて下さい。

プロジェクトに関わるようになってある領域で何度かプルリクエストを作ったら、他の人のプルリクエストをレビューすることを検討してみましょう。
レビュー出来る人というのはそう多くないので常に歓迎されます。
特に権限は必要ありません &mdash; 興味のあるプルリクエストに対して建設的で礼儀正しいコメントをつけるだけです。
コードレビューの作法を身につけたいなら[このガイドを読むとよいでしょう][reviews]。

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
