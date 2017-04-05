---
layout: ja-JP/default
title: 他のインストール方法 &middot; プログラミング言語Rust
---

# 他のインストール方法

- [どのインストーラを使うべきか](#which)
- [`rustup` をインストールする他の方法](#more-rustup)
- [スタンドアロンなインストーラ](#standalone)
- [ソースコード](#source)

## どのインストーラを使うべきか
<span id="which"></span>

Rustは様々なプラットフォームで動き、多くのインストール方法があります。
最も素直で推奨される方法でインストールしたいならメインの[インストールページ][installation page]の指示に従って下さい。

メインのインストールページでは[`rustup`]を使ったインストール方法が説明されています。
`rustup` はRustがサポートするプラットフォームで複数のRustのツールチェーンを一貫した方法で管理してくれます。
では、どうしてrustupを使わ _ない_ 方法でインストールする必要があるのでしょうか。

- オフラインなインストール。
  `rustup` はコンポーネントを必要に応じてインターネットからダウンロードします。
  インターネットを経由せずRustをインストールしたいのなら、 `rustup` は適さないでしょう。
- パッケージマネージャを使いたいから。
  Linuxでは特に、しかしmacOSでの[Homebrew]やWindowsでの[Chocolatey]でも、プラットフォームのパッケージマネージャでRustをインストールしたい開発者もいるでしょう。
- `curl | sh` を使いたくない。
  Unixでは `rustup` を `curl` 経由でシェルスクリプトを実行してインストールします。
  セキュリティ上の懸念からこの方法を好まず、インストーラ自身をダウンロードしてインストールしたい人もいます。
- 署名を検証したい。
  `rustup` はHTTPSでダウンロードしますが、現行のRustインストーラの署名を検証するにはスタンドアロンインストーラを手でインストールするしかありません。
- WindowsでGUIを使ってインストールし、「プログラムの追加/削除」で管理したい。
  `rustup` はコンソール上で動き、よくあるWindowsのプログラムのようにインストールを登録しません。
  Windowsでよくあるプログラムのようにインストールしたいなら、スタンドアロンの `.msi` インストーラがあります。
  将来、 `rustup` もWindowsではGUIインストーラを追加するはずです。

Rustのプラットフォームサポートは[3階級][three tiers]で定義されてて、ほぼ利用可能なインストール方法に対応します。
一般に、Rustプロジェクトは1級、2級のプラットフォームにバイナリビルドを提供していて、`rustup`からインストール出来ます。
しかし2級のプラットフォームのいくつかは標準ライブラリのみインストール可能で、コンパイラ自身はインストール出来ません。
これは、クロスコンパイルのターゲットでしかないということです。Rustのコードはそのプラットフォームで動きますが、コンパイラ自身は動きません。
クロスコンパイルのターゲットは `rustup target add` コマンドで追加出来ます。


## `rustup` をインストールする他の方法
<span id="rustup"></span>

`rustup` をインストールする方法はプラットフォーム毎に異なります。

* Unixではシェルから `curl https://sh.rustup.rs -sSf | sh` を実行します。
  これは[`rustup-init.sh`]をダウンロードし、実行します。
  このスクリプトはご使用のプラットフォーム向けの適切なバージョンの `rustup-init` をダウンロードし、実行します。
* Windowsでは、[`rustup-init.exe`]をダウンロードし、実行します

`rustup-init`は対話的に設定でき、全てのオプションはコマンドライン引数から制御可能です。
コマンドライン引数はシェルスクリプトから渡せます。
以下のように`rustup-init`に`--help`を渡すと`rustup-init`が受け付ける引数が表示されます。


```
curl https://sh.rustup.rs -sSf | sh -s -- --help
```

シェルスクリプトを使いたくないなら、それぞれのプラットフォーム向けの`rustup-init`を自身で選んで直接ダウンロード出来ます。


<div class="rustup-init-table">
  {% for column in site.data.platforms.rustup %}
  <div>
    {% for target in column %}
    {% if target contains 'windows' %}
    <a href="https://static.rust-lang.org/rustup/dist/{{ target }}/rustup-init.exe">
      {{ target }}
    </a>
    {% else %}
    <a href="https://static.rust-lang.org/rustup/dist/{{ target }}/rustup-init">
      {{ target }}
    </a>
    {% endif %}
    {% endfor %}
  </div>
  {% endfor %}
</div>

## スタンドアロンなインストーラ
<span id="standalone"></span>

公式のRustのスタンドアロンなインストーラは単一のRustのリリースを内包していて、オフラインでのインストールに適しています。
3種類の形式があります。どんなUnixライクな環境でも動くtarball(`.tar.gz` 拡張子)、Windowsインストーラ(`.msi`)、ししてMacインストーラ(`.pkg`)。
これらのインストーラは`rustc`、`cargo`、`rustdoc`、標準ライブラリ、標準ドキュメントを提供しますが、`rustup`のように追加のクロスコンパイルターゲットは提供しません。

これらを使うよくある理由は

- オフラインのインストール
- Windowsプラットフォームに統合された、視覚的なインストーラの方が良い

これらのバイナリは[Rustの署名鍵][Rust signing key]でRustのビルド基盤で[GPG]で署名されていてます。
この鍵は[keybase.ioから入手可能][available on keybase.io]です。
以下のテーブルでは `asc` ファイルは署名です。

過去のリリースは[アーカイブ][the archives]にあります。

{% for channel in site.channels %}

### {{ channel.name | capitalize }} ({{ channel.vers }})
<span id="{{ channel.name }}"></span>

<div class="installer-table {{ channel.name }}">
  {% for column in site.data.platforms[channel.name] %}
  <div>
    {% for target in column %}
    <div>
      <span>{{ target }}</span>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-{{ target }}.tar.gz">.tar.gz</a>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-{{ target }}.tar.gz.asc">.asc</a>
    </div>
    {% if target contains 'windows' %}
    <div>
      <span>{{ target }}</span>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-{{ target }}.msi">.msi</a>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-{{ target }}.msi.asc">.asc</a>
    </div>
    {% elsif target contains 'darwin' %}
    <div>
      <span>{{ target }}</span>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-{{ target }}.pkg">.pkg</a>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-{{ target }}.pkg.asc">.asc</a>
    </div>
    {% endif %}
    {% endfor %}
  </div>
  {% endfor %}
</div>

{% endfor %}

## ソースコード
<span id="source"></span>

<div class="installer-table">
  <div>
    <div>
      <span>Stable</span>
      <a href="https://static.rust-lang.org/dist/rustc-{{ site.stable }}-src.tar.gz">.tar.gz</a>
      <a href="https://static.rust-lang.org/dist/rustc-{{ site.stable }}-src.tar.gz.asc">.asc</a>
    </div>
  </div>
  <div>
    <div>
      <span>Beta</span>
      <a href="https://static.rust-lang.org/dist/rustc-beta-src.tar.gz">.tar.gz</a>
      <a href="https://static.rust-lang.org/dist/rustc-beta-src.gz.asc">.asc</a>
    </div>
  </div>
  <div>
    <div>
      <span>Nightly</span>
      <a href="https://static.rust-lang.org/dist/rustc-nightly-src.tar.gz">.tar.gz</a>
      <a href="https://static.rust-lang.org/dist/rustc-nightly-src.tar.gz.asc">.asc</a>
    </div>
  </div>
</div>

[installation page]: install.html
[`rustup`]: https://github.com/rust-lang-nursery/rustup.rs
[other-rustup]: https://github.com/rust-lang-nursery/rustup.rs#other-installation-methods
[`rustup-init.exe`]: https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe
[`rustup-init.sh`]: https://static.rust-lang.org/rustup/rustup-init.sh
[Homebrew]: http://brew.sh/
[Chocolatey]: http://chocolatey.org/
[three tiers]: https://forge.rust-lang.org/platform-support.html
[Rust signing key]: https://static.rust-lang.org/rust-key.gpg.ascii
[GPG]: https://gnupg.org/
[available on keybase.io]: https://keybase.io/rust
[the archives]: https://static.rust-lang.org/dist/index.html
