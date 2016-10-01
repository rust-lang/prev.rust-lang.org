---
layout: ko-KR/default
title: Rust에 기여하기 &mdash; 언어, 컴파일러 및 표준 라이브러리 &middot; Rust 프로그래밍 언어
---

# Rust에 기여하기 &mdash; 언어, 컴파일러 및 표준 라이브러리

컴파일러와 표준 라이브러리의 소스 코드는 주 저장소에 있고,
그걸 관리하는 게 해당 저장소의 주 목적이기 때문에
이슈 트래커의 수많은 라벨들은 관리에 연관됩니다.
가장 많이 붙는 라벨을 몇 개 꼽자면,
Rust를 LLVM IR로 번역하는 걸 다루는 [A-codegen],
디버거가 사용하는 메타데이터의 생성을 다루는 [A-debuginfo],
컴파일러가 오류 시 제공하는 피드백을 다루는 [A-diagnostics],
표준 라이브러리의 문제를 다루는 [A-libs],
문법 확장에 연관된 [A-macros]와 [A-syntaxext],
그리고 타입 관련된 주제가 올라오는 [A-typesystem] 등이 있습니다.

컴파일러의 구조에 대해 잘 관리되는 안내는 없습니다만
[저장소 내에 짧은 개관이 있습니다][rustc-guide].
[컴파일러를 구성하는 크레이트들의 API 문서][internals-docs]나
[Rust DXR] 소스 코드 브라우저가 코드를 돌아다니는 데 도움이 될 수 있습니다.
[Rust 테스트 수트 안내][testsuite]와 명령줄에서 [`make tips`][tips]를 실행하는 걸로
어떻게 Rust 빌드 시스템을 효과적으로 써 먹을지 배울 수 있습니다.

근시일 내에 Rust 컴파일러 개발이 나아가는 큰 방향 중 하나로
AST를 직접 다루지 않고 [MIR라 불리는 중간 표현][mir]을 다루도록
내부를 바꾸는 것이 있습니다.
이 작업은 컴파일러를 단순화하여 더 많은 가능성을 열 것으로 예상되며,
MIR 기반의 번역 단계를 만들고, MIR 기반 최적화를 추가하며,
증분 컴파일을 구현하는 등의 작업에서 도움이 필요합니다.
아직 여기에서 필요한 작업들의 정보를 한 곳에서 볼 수는 없지만,
인도가 필요하면 [internals.rust-lang.org]이나 [#rust-internals]에서 물어 보세요.

[컴파일러가 깨지면 부끄럽지요][ice].
악명 높은 '내부 컴파일러 오류'(ICE)입니다.
[I-ICE] 라벨로 추적하며, 종종 많이 있습니다.
이들은 보통 시작하기 좋은 버그인데 고쳤는지의 여부를 쉽게 알 수 있고
때때로 비교적 자기 완결적이기 때문입니다.

Rust 코드의 성능은 훌륭한 이득 중 하나이며,
Rust 컴파일러의 성능은 훌륭한 약점 중 하나입니다.
실행 시간이나, 특히, 컴파일 시간 성능에서의 개선은 널리 찬양받습니다.
[I-slow]와 [A-optimization] 라벨은 실행 시간 성능을 다루고,
[I-compiletime]은 컴파일 시간 성능을 다룹니다.
여러 작업물에 대한 [컴파일 시간 성능을 추적하는 사이트][rustc-perf]가 있습니다.
`-Z time-passes` 컴파일 플래그가 컴파일러 성능을 디버깅하는 데 도움이 될 수 있으며,
Rust 코드는 Linux의 `perf` 같은 표준적인 프로파일러로도 프로파일할 수 있습니다.

새로운 주요 기능은 [의견 요청][rfc](Request for Comments, RFC) 과정에 따라 설계가 합의됩니다.
이 과정은 모두에게 열려 있긴 하지만,
기존에 함께 작업하는 경험에 차이가 있는 개발자들 사이에서의 사회적인 과정이기에
느리게 뛰어 들길 권장합니다.
역사적, 기술적 또는 사회적 맥락을 이해하지 않고 RFC를 서둘러 제출했다가는
나쁜 인상을 받고 실망하여 돌아서게 만들기 십상입니다.
앞에 링크된 읽어보기 파일을 읽고 이 모든 게 어떻게 동작하는지 최대한 이해하세요.
Rust의 역사 동안 많은 아이디어들에 대해 논쟁이 있었는데,
어떤 건 기각되었고, 어떤 건 미래로 미루어졌으며,
RFC [이슈 트래커][rfc-issues]에는 아직 언어에 들어갈 자리를 못 만든
희망 사항인 아이디어들을 나열하고 있습니다.
RFC가 구현되기로 승낙되기 약간 전에 RFC는 '마지막 의견 기간'에 들어가며,
이는 [rust-lang/rfcs 저장소의 final-comment-period 라벨][rfc-fcp]로 표시됩니다.
마찬가지로 안정 버전의 컴파일러에서 기능이 켜지기 전에는 ('게이트가 풀린다'고 합니다)
해당 이슈가 [rust-lang/rust 저장소의 final-comment-period 라벨][issue-fcp]로 들어갑니다.
두 종류의 FCP 모두 언어의 방향성에 대해 참여하고 의견을 행사할 수 있는 중요한 순간이기에,
[internals.rust-lang.org]의 주간 서브팀 보고에서 홍보됩니다.

Rust 컴파일러 기술자들은 [#rustc]에서,
언어 설계자들은 [#rust-lang]에서, 그리고 라이브러리 설계자들은 [#rust-libs]에서 만나 보세요.

<!--
TODO: guide to compile-time benchmarking
TODO: using the triage bot?
TODO: some of this RFC description could probably go in the RFC readme
-->


[#rust-internals]: https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-internals
[#rust-lang]: https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-lang
[#rust-libs]: https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-libs
[#rustc]: https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rustc
[A-codegen]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-codegen
[A-debuginfo]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-debuginfo
[A-diagnostics]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-diagnostics
[A-libs]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-libs
[A-macros]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-macros
[A-optimization]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-optimization
[A-syntaxext]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-syntaxext
[A-typesystem]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-typesystem
[I-ICE]: https://github.com/rust-lang/rust/labels/I-ICE
[I-compiletime]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AI-compiletime
[I-slow]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AI-slow
[Rust DXR]: https://dxr.mozilla.org/rust/source/src
[ice]: https://users.rust-lang.org/t/glacier-a-big-ol-pile-of-ice/3380
[internals-docs]: https://manishearth.github.io/rust-internals-docs
[internals.rust-lang.org]: https://internals.rust-lang.org/
[issue-fcp]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AB-unstable+label%3Afinal-comment-period
[mir]: https://github.com/rust-lang/rust/issues/27840
[rfc-fcp]: https://github.com/rust-lang/rfcs/pulls?q=is%3Aopen+is%3Apr+label%3Afinal-comment-period
[rfc-issues]: https://github.com/rust-lang/rfcs/issues
[rfc]: https://github.com/rust-lang/rfcs#table-of-contents
[rustc-guide]: https://github.com/rust-lang/rust/blob/master/src/librustc/README.md
[rustc-perf]: http://ncameron.org/perf-rustc/
[testsuite]: https://github.com/rust-lang/rust-wiki-backup/blob/master/Note-testsuite.md
[tips]: https://github.com/rust-lang/rust/blob/3d1f3c9d389d46607ae28c51cc94c1f43d65f3f9/Makefile.in#L48
