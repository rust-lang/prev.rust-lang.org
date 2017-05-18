---
layout: zh-CN/default
title: 其他安装方法 &middot; Rust 程序设计语言
---

# 其他安装 Rust 的方法

- [您应该使用哪个安装程序？](#which)
- [安装 `rustup` 的其他方法](#more-rustup)
- [独立安装程序](#standalone)
- [源代码](#source)

## 您应该使用哪个安装程序？
<span id="which"></span>

Rust 在许多平台上运行，并且有很多方法来安装 Rust 。
如果您想要安装 Rust 最简单，推荐的方法，
然后参照主要的[安装页面][installation page]上的说明。

That page describes installation via [`rustup`], a tool that manages multiple
Rust toolchains in a consistent way across all platforms Rust supports. Why
might one _not_ want to install using those instructions?

- Offline installation. `rustup` downloads components from the internet on
  demand. If you need to install Rust without access to the internet, `rustup`
  is not suitable.
- Preference for the system package manager. On Linux in particular, but also on
  macOS with [Homebrew], and Windows with [Chocolatey], developers sometimes
  prefer to install Rust with their platform's package manager.
- Preference against `curl | sh`. On Unix, we usually install `rustup` by
  running a shell script via `curl`. Some have concerns about the security of
  this arrangement and would prefer to download and run the installer
  themselves.
- Validating signatures. Although `rustup` performs its downloads over HTTPS,
  the only way to verify the signatures of Rust installers today is to do so
  manually with the standalone installers.
- GUI installation and integration with "Add/Remove Programs" on
  Windows. `rustup` runs in the console and does not register its installation
  like typical Windows programs. If you prefer a more typical GUI installation
  on Windows there are standalone `.msi` installers. In the future
  `rustup` will also have a GUI installer on Windows.

Rust's platform support is defined in [three tiers], which correspond closely
with the installation methods available: in general, the Rust project provides
binary builds for all tier 1 and tier 2 platforms, and they are all installable
via `rustup`. Some tier 2 platforms though have only the standard library
available, not the compiler itself; that is, they are cross-compilation targets
only; Rust code can run on those platforms, but they do not run the compiler
itself. Such targets can be installed with the `rustup target add` command.

## 安装 `rustup` 的其他方法
<span id="rustup"></span>

安装 `rustup` 的方式因平台而异：

* 在 Unix 上，在您的终端运行 `curl https://sh.rustup.rs -sSf | sh` 。
  它将会下载并运行 [`rustup-init.sh`]，它为您的平台下载并运行
  `rustup-init` 可执行文件的正确版本。
* 在 Windows 上，下载并运行 [`rustup-init.exe`].

Pass `--help` to `rustup-init` as follows to display the arguments
`rustup-init` accepts:

`rustup-init` 可以交互式配置，所有选项都可以由命令行参数控制，
可以通过 shell 脚本传递。 将`--help`传递给`rustup-init`，
以下是 `rustup-init` 接受的参数：

```
curl https://sh.rustup.rs -sSf | sh -s -- --help
```

如果您不想使用 shell 脚本，可以选择合适您使用平台的 `rustup-init` 直接下载：

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

## 独立安装程序
<span id="standalone"></span>

The official Rust standalone installers contain a single release of Rust, and
are suitable for offline installation. They come in three forms: tarballs
(extension `.tar.gz`), that work in any Unix-like environment, Windows
installers (`.msi`), and Mac installers (`.pkg`). These installers come with
`rustc`, `cargo`, `rustdoc`, the standard library, and the standard
documentation, but do not provide access to additional cross-targets like
`rustup` does.

使用独立安装程序的最常见原因是：

- 离线安装
- 热衷在 Windows 上使用更多平台集成的图形安装程序

Each of these binaries is signed with the [Rust signing key], which is
[available on keybase.io], by the Rust build infrastructure, with
[GPG]. In the tables below, the `.asc` files are the signatures.

Past releases can be found in [the archives].

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

## 源代码
<span id="source"></span>

<div class="installer-table">
  <div>
    <div>
      <span>稳定版（Stable）</span>
      <a href="https://static.rust-lang.org/dist/rustc-{{ site.stable }}-src.tar.gz">.tar.gz</a>
      <a href="https://static.rust-lang.org/dist/rustc-{{ site.stable }}-src.tar.gz.asc">.asc</a>
    </div>
  </div>
  <div>    
    <div>
      <span>测试版（Beta）</span>
      <a href="https://static.rust-lang.org/dist/rustc-beta-src.tar.gz">.tar.gz</a>
      <a href="https://static.rust-lang.org/dist/rustc-beta-src.gz.asc">.asc</a>
    </div>
  </div>
  <div>    
    <div>
      <span>开发版（Nightly）</span>
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
