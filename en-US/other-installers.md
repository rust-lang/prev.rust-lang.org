---
layout: default
title: Other Installation Methods &middot; The Rust Programming Language
---

# Other Rust Installation Methods

- [Which installer should you use?](#which)
- [Other ways to install via `rustup`](#more-rustup)
- [Standalone installers](#standalone)
- [The `rustup.sh` script](#rustup-sh)
- [Obtaining Rust via other package managers](#other)
- [Source code](#source)

## Which installer should you use?
<span id="which"></span>

Rust runs on many platforms, and there are many ways to install Rust. If you
want to install Rust in the most straightforward, recommended way, then follow
the instructions on the main [installation page].

That page describes installation via [`rustup`], a tool that manages multiple
Rust toolchains in a consistent way across all platforms Rust supports. Why
might one _not_ want to install using those instructions?

- Offline installation. `rustup` downloads components from the internet on
  demand. If you need to install Rust without access to the internet, `rustup`
  is not suitable.
- Preference for the system package manager. In Linux in particular, but also on
  macOS with [Homebrew], developers sometimes prefer to install Rust with their
  platform's package manager.
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
available, not the compiler itself; that is, they are cross-compilation targets,
Rust code can run on those platforms, but they do not run the compiler itself.
Such targets can be installed with the `rustup target add` command.

## Other ways to install via `rustup`
<span id="rustup"></span>

The way to install `rustup` differs by platform:

* On Unix, run `curl https://sh.rustup.rs -sSf | sh` in your
  shell. This downloads and runs [`rustup-init.sh`], which in turn
  downloads and runs the correct version of the `rustup-init`
  executable for your platform.
* On Windows, download and run [`rustup-init.exe`].

`rustup-init` can be configured interactively, and all options can additionally
be controlled by command-line arguments, which can be passed through the shell
script. Pass `--help` to `rustup-init` as follows to display the arguments
`rustup-init` accepts:

```
curl https://sh.rustup.rs -sSf | sh -s -- --help
```

If you prefer not to use the shell script, you may directly download
`rustup-init` for the platform of your choice:

<div class="rustup-init-table">
  <div>
    <a href="https://static.rust-lang.org/rustup/dist/aarch64-unknown-linux-gnu/rustup-init">
      aarch64-unknown-linux-gnu
    </a>
    <a href="https://static.rust-lang.org/rustup/dist/arm-unknown-linux-gnueabi/rustup-init">
      arm-unknown-linux-gnueabi
    </a>
    <a href="https://static.rust-lang.org/rustup/dist/arm-unknown-linux-gnueabihf/rustup-init">
      arm-unknown-linux-gnueabihf
    </a>
    <a href="https://static.rust-lang.org/rustup/dist/i686-apple-darwin/rustup-init">
      i686-apple-darwin
    </a>
    <a href="https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe">
      i686-pc-windows-gnu
    </a>
    <a href="https://static.rust-lang.org/rustup/dist/i686-pc-windows-msvc/rustup-init.exe">
      i686-pc-windows-msvc
    </a>
    <a href="https://static.rust-lang.org/rustup/dist/i686-unknown-linux-gnu/rustup-init">
      i686-unknown-linux-gnu
    </a>
    <a href="https://static.rust-lang.org/rustup/dist/mips-unknown-linux-gnu/rustup-init">
      mips-unknown-linux-gnu
    </a>
    <a href="https://static.rust-lang.org/rustup/dist/mipsel-unknown-linux-gnu/rustup-init.exe">
      mipsel-unknown-linux-gnu
    </a>
    <a href="https://static.rust-lang.org/rustup/dist/mips64-unknown-linux-gnuabi64/rustup-init.exe">
      mips64-unknown-linux-gnuabi64
    </a>
    <a href="https://static.rust-lang.org/rustup/dist/mips64el-unknown-linux-gnuabi64/rustup-init">
      mips64el-unknown-linux-gnuabi64
    </a>
  </div>
  <div>
    <a href="https://static.rust-lang.org/rustup/dist/powerpc-unknown-linux-gnu/rustup-init">
      powerpc-unknown-linux-gnu
    </a>
    <a href="https://static.rust-lang.org/rustup/dist/powerpc64-unknown-linux-gnu/rustup-init">
      powerpc64-unknown-linux-gnu
    </a>
    <a href="https://static.rust-lang.org/rustup/dist/powerpc64le-unknown-linux-gnu/rustup-init">
      powerpc64le-unknown-linux-gnu
    </a>
    <a href="https://static.rust-lang.org/rustup/dist/s390x-unknown-linux-gnu/rustup-init">
      s390x-unknown-linux-gnu
    </a>
    <a href="https://static.rust-lang.org/rustup/dist/x86_64-apple-darwin/rustup-init">
      x86_64-apple-darwin
    </a>
    <a href="https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-gnu/rustup-init.exe">
      x86_64-pc-windows-gnu
    </a>
    <a href="https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe">
      x86_64-pc-windows-msvc
    </a>
    <a href="https://static.rust-lang.org/rustup/dist/x86_64-unknown-freebsd/rustup-init">
      x86_64-unknown-freebsd
    </a>
    <a href="https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init">
      x86_64-unknown-linux-gnu
    </a>
    <a href="https://static.rust-lang.org/rustup/dist/x86_64-unknown-netbsd/rustup-init">
      x86_64-unknown-netbsd
    </a>
  </div>
</div>

Note that all builds of Rust can target all supported architectures, and on on
Windows, it is always recommended to install using the `i686-pc-windows-gnu`
build of `rustup`, regardless of whether you are targetting the MSVC or GNU ABI,
i686 or x86_64 architecture. Either Windows build of `rustup` will install the
MSVC build of Rust if it detects that MSVC is installed. All four Windows builds
of `rustup` are functionally equivalent, but the MSVC builds have a confusing
(and temporary) limitation: they are dynamically linked to the MSVC runtime,
which may not be installed, causing `rustup` to fail; whereas the GNU builds of
`rustup` are statically linked. On Windows, use the `i686-pc-windows-gnu` build
of `rustup`.

## Standalone installers
<span id="standalone"></span>

The official Rust standalone installers contain a single release of Rust, and
are suitable for offline installation. They come in three forms: tarballs
(extension `.tar.gz`), that work in any Unix-like environment, Windows
installers (`.msi`), and Mac installers (`.pkg`). These installers come with
`rustc`, `cargo`, `rustdoc`, the standard library, and the standard
documentation, but do not provide access to additional cross-targets like
`rustup` does.

The most common reasons to use these are:

- Offline installation
- Prefering a more platform-integrated, graphical installer on Windows

Each of these binaries is signed with the [Rust signing key], which is
[available on keybase.io], by the Rust build infrastructure, with
[GPG]. Validating their signatures provides strong guarantees about their
origins. In the tables below, the `.asc` files are the signatures.

{% for channel in site.channels %}

### {{ channel.name | capitalize }} ({{ channel.vers }})
<!-- TODO: parameterize id's and tarball names -->
<span id="stable"></span>

<div class="installer-table {{ channel.name }}">
  <div>

    <div>
      <span>aarch64-unknown-linux-gnu</span>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-aarch64-unknown-linux-gnu.tar.gz">.tar.gz</a>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-aarch64-unknown-linux-gnu.tar.gz.asc">.asc</a>
    </div>
    <div>
      <span>arm-unknown-linux-gnueabi</span>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-arm-unknown-linux-gnueabi.tar.gz">.tar.gz</a>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-arm-unknown-linux-gnueabi.tar.gz.asc">.asc</a>
    </div>
    <div>
      <span>arm-unknown-linux-gnueabihf</span>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-arm-unknown-linux-gnueabihf.tar.gz">.tar.gz</a>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-arm-unknown-linux-gnueabihf.tar.gz.asc">.asc</a>
    </div>
    <div>
      <span>i686-apple-darwin</span>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-i686-apple-darwin.tar.gz">.tar.gz</a>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-i686-apple-darwin.tar.gz.asc">.asc</a>
    </div>
    <div>
      <span>i686-apple-darwin</span>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-i686-apple-darwin.pkg">.pkg</a>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-i686-apple-darwin.pkg.asc">.asc</a>
    </div>
    <div>
      <span>i686-pc-windows-gnu</span>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-i686-pc-windows-gnu.tar.gz">.tar.gz</a>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-i686-pc-windows-gnu.tar.gz.asc">.asc</a>
    </div>
    <div>
      <span>i686-pc-windows-gnu</span>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-i686-pc-windows-gnu.msi">.msi</a>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-i686-pc-windows-gnu.msi.asc">.asc</a>
    </div>
    <div>
      <span>i686-pc-windows-msvc</span>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-i686-pc-windows-msvc.tar.gz">.tar.gz</a>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-i686-pc-windows-msvc.tar.gz.asc">.asc</a>
    </div>
    <div>
      <span>i686-pc-windows-msvc</span>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-i686-pc-windows-msvc.msi">.tar.gz</a>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-i686-pc-windows-msvc.msi.asc">.asc</a>
    </div>
    <div>
      <span>i686-unknown-linux-gnu</span>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-i686-unknown-linux-gnu.msi">.tar.gz</a>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-i686-unknown-linux-gnu.msi.asc">.asc</a>
    </div>
    <div class="no-stable">
      <span>mips-unknown-linux-gnu</span>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-mips-unknown-linux-gnu.tar.gz">.tar.gz</a>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-mips-unknown-linux-gnu.tar.gz.asc">.asc</a>
    </div>
    <div class="no-stable">
      <span>mipsel-unknown-linux-gnu</span>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-mipsel-unknown-linux-gnu.msi">.msi</a>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-mipsel-unknown-linux-gnu.msi.asc">.asc</a>
    </div>
    <div class="no-stable">
      <span>mips64-unknown-linux-gnuabi64</span>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-mips64-unknown-linux-gnuabi64.tar.gz">.tar.gz</a>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-mips64-unknown-linux-gnuabi64.tar.gz.asc">.asc</a>
    </div>
    <div class="no-stable">
      <span>mips64-unknown-linux-gnuabi64</span>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-mips64-unknown-linux-gnuabi64.msi">.tar.gz</a>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-mips64-unknown-linux-gnuabi64.msi.asc">.asc</a>
    </div>

  </div>

  <div>

    <div class="no-stable">
      <span>powerpc-unknown-linux-gnu</span>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-powerpc-unknown-linux-gnu.tar.gz">.tar.gz</a>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-powerpc-unknown-linux-gnu.tar.gz.asc">.asc</a>
    </div>
    <div class="no-stable">
      <span>powerpc64-unknown-linux-gnu</span>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-powerpc64-unknown-linux-gnu.pkg">.pkg</a>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-powerpc64-unknown-linux-gnu.pkg.asc">.asc</a>
    </div>
    <div class="no-stable">
      <span>powerpc64le-unknown-linux-gnu</span>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-powerpc64le-unknown-linux-gnu.tar.gz">.tar.gz</a>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-powerpc64le-unknown-linux-gnu.tar.gz.asc">.asc</a>
    </div>
    <div class="no-stable">
      <span>s390x-unknown-linux-gnu</span>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-s390x-unknown-linux-gnu.msi">.msi</a>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-s390x-unknown-linux-gnu.msi.asc">.asc</a>
    </div>
    <div>
      <span>x86_64-apple-darwin</span>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-x86_64-apple-darwin.tar.gz">.tar.gz</a>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-x86_64-apple-darwin.tar.gz.asc">.asc</a>
    </div>
    <div>
      <span>x86_64-apple-darwin</span>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-x86_64-apple-darwin.pkg">.pkg</a>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-x86_64-apple-darwin.pkg.asc">.asc</a>
    </div>
    <div>
      <span>x86_64-pc-windows-gnu</span>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-x86_64-pc-windows-gnu.tar.gz">.tar.gz</a>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-x86_64-pc-windows-gnu.tar.gz.asc">.asc</a>
    </div>
    <div>
      <span>x86_64-pc-windows-gnu</span>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-x86_64-pc-windows-gnu.msi">.msi</a>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-x86_64-pc-windows-gnu.msi.asc">.asc</a>
    </div>
    <div>
      <span>x86_64-pc-windows-msvc</span>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-x86_64-pc-windows-msvc.tar.gz">.tar.gz</a>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-x86_64-pc-windows-msvc.tar.gz.asc">.asc</a>
    </div>
    <div>
      <span>x86_64-pc-windows-msvc</span>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-x86_64-pc-windows-msvc.msi">.msi</a>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-x86_64-pc-windows-msvc.msi.asc">.asc</a>
    </div>
    <div>
      <span>x86_64-unknown-freebsd</span>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-x86_64-unknown-freebsd.tar.gz">.tar.gz</a>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-x86_64-unknown-freebsd.tar.gz.asc">.asc</a>
    </div>
    <div>
      <span>x86_64-unknown-linux-gnu</span>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-x86_64-unknown-linux-gnu.tar.gz">.tar.gz</a>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-x86_64-unknown-linux-gnu.tar.gz.asc">.asc</a>
    </div>
    <div>
      <span>x86_64-unknown-netbsd</span>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-x86_64-unknown-netbsd.tar.gz">.tar.gz</a>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-x86_64-unknown-netbsd.tar.gz.asc">.asc</a>
    </div>

  </div>

</div>

{% endfor %}

## The `rustup.sh` script
<span id="rustup-sh"></span>

On Unixes, there's a script available for installing the standalone installers,
`rustup.sh` (sometimes confused with `rustup`), which can be conveniently combined
with curl to install Rust from any release channel.

To install stable Rust:

```
curl -sSf https://static.rust-lang.org/rustup.sh | sh
```

To install nightly Rust:

```
curl -sSf https://static.rust-lang.org/rustup.sh | sh -s -- --channel=nightly
```

## Obtaining Rust via other package managers
<span id="other"></span>

Rust is packaged by third-parties as well, including Homebrew, Debian, Fedora,
and Arch Linux. The maturity and completeness of third-party packages is
unknown, and Rust makes releases often, so for the most consistent and
up-to-date Rust experience we recommend using the official Rust packages.

## Source code
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
[three tiers]: https://forge.rust-lang.org/platform-support.html
[Rust signing key]: https://static.rust-lang.org/rust-key.gpg.ascii
[GPG]: https://gnupg.org/
[available on keybase.io]: https://keybase.io/rust