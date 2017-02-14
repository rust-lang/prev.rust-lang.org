---
layout: ko-KR/default
title: 다른 설치 방법 &middot; Rust 프로그래밍 언어

localized-channels:
  stable: 안정
  beta: 베타
  nightly: 나이틀리
---

# 다른 Rust 설치 방법

- [무슨 인스톨러를 써야 합니까?](#which)
- [`rustup`을 설치하는 다른 방법](#more-rustup)
- [자기 완결 인스톨러](#standalone)
- [소스 코드](#source)

## 무슨 인스톨러를 써야 합니까?
<span id="which"></span>

Rust는 많은 플랫폼에서 동작하며, Rust를 설치하는 방법 또한 많이 있습니다.
만약 Rust를 가장 쉽고 권장되는 방법으로 설치하려면
기본 [설치 페이지][installation page]의 방법을 따라 주세요.

이 페이지에서는 [`rustup`]으로 설치하는 방법을 설명하는데,
이 도구는 Rust가 지원하는 모든 플랫폼에서 Rust 툴체인을 일관된 방법으로 다룰 수 있게 해 줍니다.
이 방법대로 설치를 하지 _않아야_ 할 이유로는 뭐가 있을까요?

- 오프라인 설치. `rustup`은 필요에 따라 인터넷에서 구성 요소를 내려 받습니다.
  인터넷 접속이 없이 Rust를 설치해야 한다면 `rustup`은 적합하지 않습니다.

- 시스템 패키지 관리자를 쓰고자 할 경우. 특히 리눅스에서 그렇습니다만,
  macOS에서 [Homebrew]를 쓰는 경우나, 윈도에서 [Chocolatey]를 쓰는 경우에도 해당됩니다.
  이런 경우 개발자들이 종종 해당 플랫폼의 패키지 관리자로 Rust를 설치하길 원할 수 있습니다.

- `curl | sh`를 쓰지 않고자 할 경우. 유닉스에서 `rustup`은 보통 `curl`을 통해
  셸 스크립트를 실행하여 설치합니다. 이 접근의 보안성에 문제를 제기하는 사람들은
  인스톨러를 직접 내려 받아 실행하길 원할 수 있습니다.

- 서명을 검증하고자 할 경우. `rustup`에서 내려받기는 HTTPS를 통해 이루어지지만,
  현재로선 Rust 인스톨러의 서명을 검증하는 유일한 방법은
  자기 완결 인스톨러를 받아서 수동으로 하는 수 밖에 없습니다.

- 윈도에서 "프로그램 설치/삭제"와 연동하여 GUI 설치를 하고자 할 경우.
  `rustup`은 콘솔에서 실행되며 일반적인 윈도 프로그램처럼 설치를 등록하지 않습니다.
  윈도에서 좀 더 일반적인 GUI 설치를 원한다면 자기 완결 `.msi` 인스톨러도 제공됩니다.
  미래에는 `rustup` 자체적으로 윈도에서 GUI 인스톨러를 제공할 수도 있습니다.

Rust의 플랫폼 지원은 [세 단계][three tiers]로 구성되어 있는데
이는 어떤 설치 방법을 쓸 수 있느냐와 밀접하게 연관되어 있습니다.
일반적으로 Rust 프로젝트는 모든 1단계 및 2단계 플랫폼에서 바이너리 빌드를 제공하며,
`rustup`으로 설치할 수 있습니다.
단 일부 2단계 플랫폼에서는 컴파일러는 제공하지 않으며 표준 라이브러리만 제공하는데,
이 경우 크로스플랫폼 타겟으로만 쓰일 수 있기 때문에
Rust 코드를 실행할 수는 있지만 컴파일러를 실행할 수는 없게 됩니다.
이러한 타겟은 `rustup target add` 명령으로 설치할 수 있습니다.

## `rustup`을 설치하는 다른 방법
<span id="rustup"></span>

`rustup`을 설치하는 방법은 플랫폼마다 다릅니다:

* 유닉스에서는 셸에서 `curl https://sh.rustup.rs -sSf | sh`을 입력합니다.
  이렇게 하면 [`rustup-init.sh`]가 받아지고 실행되는데,
  이 스크립트는 다시 현재 플랫폼에 알맞은 `rustup-init` 실행파일을 내려받아 실행합니다.
* 윈도에서는 [`rustup-init.exe`]를 내려받아 실행합니다.

`rustup-init`은 사용자 문답으로 설정할 수도 있으며,
명령줄 인자를 통해 모든 인자를 제어할 수도 있는데 셸 스크립트를 통해서도 전달할 수 있습니다.
`rustup-init`에 `--help`를 다음과 같이 전달하면 `rustup-init`이 받는 인자들을 볼 수 있습니다:

```
curl https://sh.rustup.rs -sSf | sh -s -- --help
```

셸 스크립트를 쓰지 않고자 한다면 원하는 플랫폼의 `rustup-init`을 직접 받을 수 있습니다:

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

## 자기 완결 인스톨러
<span id="standalone"></span>

공식 Rust 자기 완결(스탠드얼론) 인스톨러는 Rust의 한 릴리스를 담고 있으며 오프라인 설치에 적합합니다.
세 가지 형태가 있는데,
타르볼(확장자 `.tar.gz`)은 아무 유닉스 계열 환경에서나 동작하며,
윈도 인스톨러(`.msi`)와 맥 인스톨러(`.pkg`)도 있습니다.
이 인스톨러들은 `rustc`, `cargo`, `rustdoc`, 표준 라이브러리와 표준 문서를 담고 있지만,
`rustup` 같이 크로스플랫폼 타겟을 더 제공하진 않습니다.

이들을 써야 하는 흔한 이유로는:

- 오프라인 설치가 필요하거나,
- 윈도에서 좀 더 플랫폼에 연동된 그래픽 인스톨러를 선호할 경우가 있습니다.

각 바이너리는 Rust 빌드 설비를 통해,
[keybase.io][available on keybase.io]에도 있는
[Rust 서명 키][Rust signing key]로, [GPG]를 통해 서명되어 있습니다.
아래 표에서 `.asc` 파일들이 서명입니다.

이전 릴리스는 [보관소][the archives]에서 찾을 수 있습니다.

{% for channel in site.channels %}

### {{ page.localized-channels[channel.name] | capitalize }} ({{ channel.vers }})
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

## 소스 코드
<span id="source"></span>

<div class="installer-table">
  <div>
    <div>
      <span>안정</span>
      <a href="https://static.rust-lang.org/dist/rustc-{{ site.stable }}-src.tar.gz">.tar.gz</a>
      <a href="https://static.rust-lang.org/dist/rustc-{{ site.stable }}-src.tar.gz.asc">.asc</a>
    </div>
  </div>
  <div>    
    <div>
      <span>베타</span>
      <a href="https://static.rust-lang.org/dist/rustc-beta-src.tar.gz">.tar.gz</a>
      <a href="https://static.rust-lang.org/dist/rustc-beta-src.gz.asc">.asc</a>
    </div>
  </div>
  <div>    
    <div>
      <span>나이틀리</span>
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
