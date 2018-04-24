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

Rust 能在许多平台上运行，并且有多种方法来安装 Rust。
如果您想要安装 Rust 最简单、最推荐的方法，
请参照主要[安装页面][installation page]上的操作说明。

本页面会介绍通过 [`rustup`] 安装，它是一个管理多个
Rust 工具链的工具，从而为多个平台提供一致的 Rust 支持。
您还想试试其他的安装方式？见下文。

- 官方安装程序。`rustup` 按需从互联网下载组件。如果您需要离线安装 Rust，那么 `rustup`
  并不合适。
- 系统的软件包管理器。在 Linux 上尤为如此，不过
  macOS 上也有 [Homebrew]，Windows 也有 [Chocolatey]，
  开发者有时喜欢用他们平台的软件包管理器来安装 Rust。
- `curl | sh`。在 Unix 上，我们通常通过 `curl` 运行一个 shell 脚本来安装 `rustup`。有些人可能担心这样做的安全性，而更愿意手动下载和运行安装程序。
- 验证签名。尽管 `rustup` 通过 HTTPS 完成下载，但目前验证 Rust 安装程序的签名的唯一方式是手动验证独立安装包。
- Windows 上的图形化「添加/删除程序」。`rustup` 在控制台中运行，并且不像典型的 Windows 程序那样注册其安装。如果您偏好典型的 GUI 安装，我们有独立的 `.msi` 安装包。在未来，
  `rustup` 也将有 Windows 上的 GUI 安装程序。

Rust 的平台支持定义共有[三层]，这与可用的安装方法密切相关：
通常来说，Rust 项目
为所有一级和二级平台提供二进制版本，并且它们可
通过 `rustup` 安装。部分二级平台只有标准库可用，
而没有编译器，即它们只是交叉编译的目标；
Rust 代码可以在这些平台上运行，但并不在这些平台上运行编译器。
此类目标可以用 `rustup target add` 命令来安装。

## 安装 `rustup` 的其他方法
<span id="rustup"></span>

安装 `rustup` 的方式因平台而异：

* 在 Unix 上，在您的终端运行 `curl https://sh.rustup.rs -sSf | sh` 。
  它将会下载并运行 [`rustup-init.sh`]，它为您的平台下载并运行
  `rustup-init` 可执行文件的正确版本。
* 在 Windows 上，下载并运行 [`rustup-init.exe`].

`rustup-init` 可以交互式配置，所有选项都可以由命令行参数控制，
可以通过 shell 脚本传递。 像下面这样将`--help`传递给`rustup-init`，
会显示出 `rustup-init` 接受的参数：

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

官方的 Rust 独立安装程序其中包含一个 Rust 发布版本，它适合离线安装。它们也有三种形式：适用于任何类 Unix 环境的 tarballs（扩展名 `.tar.gz`）， Windows 安装包（`.msi`），以及 Mac 安装包（`.pkg`）。这些安装程序附有 
`rustc`、`cargo`、`rustdoc`、标准库，以及标准文档，但不提供 `rustup` 那样的其他跨目标访问。

使用独立安装程序的最常见原因是：

- 离线安装
- 热衷在 Windows 上使用更多平台集成的图形安装程序

这些二进制文件每个均使用了 [Rust signing key]来签名（由 Rust 构建基础设施使用 [GPG]），该密钥也在 
[keybase.io] 上提供。在下面的表格中，`.asc` 文件即该签名。

过去发布的版本可在[此处存档][the archives]中找到。

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
