---
layout: ja-JP/default
title: Rustへ貢献する &mdash; ツール、IDE、インフラ &middot; プログラミング言語Rust
---

# Rustへ貢献する &mdash; ツール、IDE、インフラ

ツールは言語が成功するために大きな役割を果しますが、実装すべきことは沢山残されています。
***今のRustの開発の主な焦点は [IDEのユーザ体験を改善することにあります][ides]***。
この作業はコンパイラ自身からあなたの愛用しているIDEまでRustスタックの上から下まで関わっています。
さらなる情報へはリンク先へ飛んで下さい。

パッケージマネージャのCargoとドキュメントジェネレータのrustdocは一通り機能を実装していますしきちんと使えますが開発者不足に苦しんでいます。
Rustdocにはメインレポジトリに[A-rustdoc]ラベルのついたイシューがいくつもあります。
そのほとんどがバグで、貢献するにはバグを直してプルリクエストを送ればよいでしょう。
Cargoは[自身のレポジトリとイシューを持っており][Cargo]、貢献したいと思っている人は[#cargo]に入ると良いでしょう。

Rustはgdbとlldbの両方のデバッガの下である程度動きますが、期待通りに動かないケースは沢山あります。
[A-debuginfo]でこれらの問題を追跡しています。

さらに貢献できるツールのプロジェクトを探したいなら[awesome-rust]を参照して下さい。

適切な人がきて実装してくれるのを待っているような面白いプロジェクトが他にもよくあります。
他のツール好きの人と[#rust-tools]で議論しましょう。


[#cargo]: https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rustc
[#rust-tools]: https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-tools
[A-debuginfo]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-debuginfo
[A-rustdoc]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-rustdoc
[Cargo]: https://github.com/rust-lang/cargo/issues
[awesome-rust]: https://github.com/kud1ing/awesome-rust
[ides]: https://forge.rust-lang.org/ides.html
