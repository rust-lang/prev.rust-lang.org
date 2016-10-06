---
layout: ko-KR/default
title: Rust에 기여하기 &mdash; 도구, IDE 및 인프라 &middot; Rust 프로그래밍 언어
---

# Rust에 기여하기 &mdash; 도구, IDE 및 인프라

도구는 언어의 성공에 큰 역할을 하며, 구현해야 할 것들도 참 많습니다.
***현재 Rust 개발의 주안점은 [IDE 경험을 향상시키는 것][ides]입니다***.
여기에는 컴파일러 자체부터 즐겨 쓰는 IDE까지 Rust 스택을 아우르는 작업이 들어갑니다.
자세한 정보는 링크를 따라가 주세요.

Rust의 패키지 관리자인 Cargo와 문서 생성기인 rustdoc 둘 다,
완전한 기능을 제공하고 잘 동작하긴 하지만 개발자들이 부족합니다.
Rustdoc에는 주 저장소의 [A-rustdoc] 라벨 아래 이슈가 많이 열려 있습니다.
이 이슈는 대부분 버그이며 기여를 하려면 버그를 고쳐서
풀 요청(pull request)을 보내는 것으로 충분합니다.
Cargo는 [별도의 저장소와 이슈][Cargo]를 가지고 있으며,
기여에 관심 있는 분들께서는 [#cargo]에 자기 소개를 하실 수 있습니다.

Rust를 gdb와 lldb 디버거로 어느 정도까지는 성공적으로 실행할 수 있지만,
아직 디버깅이 예상대로 동작하지 않는 경우가 많이 있습니다.
[A-debuginfo] 이슈가 이런 문제를 추적합니다.

기여할 만한 다른 도구 프로젝트에 대한 아이디어에 대해서는 [awesome-rust]를 보세요.

종종 어떤 도구 프로젝트는 관심사이기는 하나
올바른 사람들이 찾아 와서 구현될 때까지 기다리고 있기도 합니다.
[#rust-tools]에서 도구에 관심이 있는 다른 분들과 상의해 보세요.

[#cargo]: https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rustc
[#rust-tools]: https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-tools
[A-debuginfo]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-debuginfo
[A-rustdoc]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-rustdoc
[Cargo]: https://github.com/rust-lang/cargo/issues
[awesome-rust]: https://github.com/kud1ing/awesome-rust
[ides]: https://forge.rust-lang.org/ides.html
