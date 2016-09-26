---
layout: ko-KR/faq
title: 자주 묻는 질문들 &middot; Rust 프로그래밍 언어
---

# 자주 묻는 질문들

<p class="faq-intro">
이 문서는 Rust 프로그래밍 언어에 대한 흔한 질문들을 답하기 위해 존재합니다. 이 문서는 언어에 대한 완전한 안내서도 아니고, 언어를 가르치는 도구도 아닙니다. 이 문서는 Rust 커뮤니티에서 사람들이 되풀이하여 맞닥뜨리는 질문들을 답하고, Rust의 일부 설계가 왜 그렇게 결정되었는지를 밝히기 위한 참조서입니다.
</p>

<p class="faq-intro">
여기에서 답하지 않았지만 흔하거나 중요한 질문이 누락되어 있다고 생각하신다면 저희가 고칠 수 있도록 <a href="https://github.com/rust-lang/rust-www/blob/master/CONTRIBUTING.md">부담 없이 도와주세요</a>.
</p>

<div id="toc">
    <h2>목차</h2><a href="#toggle-toc"></a>
    <div class="contents">
        <ol id="toc-contents">
            <li><a href="#project">Rust 프로젝트</a></li>
            <li><a href="#performance">성능</a></li>
            <li><a href="#syntax">문법</a></li>
            <li><a href="#numerics">숫자</a></li>
            <li><a href="#strings">문자열</a></li>
            <li><a href="#collections">컬렉션</a></li>
            <li><a href="#ownership">소유권</a></li>
            <li><a href="#lifetimes">수명</a></li>
            <li><a href="#generics">일반화 (제너릭)</a></li>
            <li><a href="#input-output">입출력</a></li>
            <li><a href="#error-handling">오류 처리</a></li>
            <li><a href="#concurrency">동시성</a></li>
            <li><a href="#macros">매크로</a></li>
            <li><a href="#debugging">디버깅 및 도구</a></li>
            <li><a href="#low-level">저수준</a></li>
            <li><a href="#cross-platform">다중 플랫폼</a></li>
            <li><a href="#modules-and-crates">모듈 및 크레이트</a></li>
            <li><a href="#libraries">라이브러리</a></li>
            <li><a href="#design-patterns">디자인 패턴</a></li>
            <li><a href="#other-languages">다른 언어들</a></li>
            <li><a href="#documentation">문서</a></li>
        </ol>
    </div>
</div>


<h2 id="project">Rust 프로젝트</h2>

<h3><a href="#what-is-this-projects-goal" name="what-is-this-projects-goal">
이 프로젝트의 목표는 무엇입니까?
</a></h3>

안전하고, 동시적이며, 실용적인 시스템 언어를 설계하고 구현하기 위함입니다.

Rust는 이 수준의 추상화와 효율을 추구하는 다른 언어들이 만족스럽지 못 하기에 존재합니다. 특히:

1. 안전성이 너무 덜 주목되어 있습니다.
2. 동시성 지원이 부족합니다.
3. 실용적으로 쓰기가 힘듭니다.
4. 자원에 대한 제어가 제한적입니다.

Rust는 효율적인 코드와 편안한 수준의 추상화를 제공하며, 동시에 위 4가지를 모두 개선하는 대안으로 만들어졌습니다.

<h3><a href="#is-this-project-controlled-by-mozilla" name="is-this-project-controlled-by-mozilla">
이 프로젝트를 Mozilla가 제어하나요?
</a></h3>

아니오. Rust는 2006년에 그레이던 호어(Graydon Hoare)가 시간을 쪼개서 하던 사이드 프로젝트로 시작하여 3년간 개발되었습니다.
2009년에 언어가 기본 테스트를 실행하고 핵심 개념들을 시연할 수 있을 정도로 성숙하자 Mozilla가 관여하기 시작했습니다.
Mozilla는 여전히 Rust를 지원하고 있습니다만, Rust는 전 세계의 많은 장소에 퍼져 있는 열정적인 사람들로 이루어진 커뮤니티가 개발하고 있습니다.
[Rust 팀](https://www.rust-lang.org/team.html)은 Mozilla 직원들과 아닌 사람들 둘 다를 포함하고, `rustc`(Rust의 컴파일러)에는 [1,000명 이상의 서로 다른 기여자](https://github.com/rust-lang/rust/)가 참여해 왔습니다.

[프로젝트 거버넌스](https://github.com/rust-lang/rfcs/blob/master/text/1068-rust-governance.md)를 따라, Rust는 프로젝트의 비전과 우선 순위를 설정하는 코어 팀에 의해 관리되며 이 코어 팀이 전체적인 관점에서 프로젝트를 인도합니다.
또한 개별 관심 분야의 개발을 인도하고 장려하기 위한 서브팀들이 있으며, 핵심 언어, 컴파일러, Rust 라이브러리, Rust 도구, 그리고 공식 Rust 커뮤니티의 중재 등이 여기에 포함됩니다.
이들 각 분야 안에서의 설계는 [RFC 과정](https://github.com/rust-lang/rfcs)을 통해 심화됩니다.
RFC를 필요로 하지 않는 수정은 [`rustc` 저장소](https://github.com/rust-lang/rust)의 풀 요청(pull request)을 통해 결정이 내려집니다.

<h3><a href="#what-are-some-non-goals" name="what-are-some-non-goals">
Rust의 목표가 아닌 것은 무엇이 있나요?
</a></h3>

1. 우리는 특별히 최신의 기술을 도입하지 않습니다. 오래되고 자리 잡힌 기술이 더 좋습니다.
2. 우리는 표현력, 최소주의 또는 우아함을 다른 목표에 우선하지 않습니다. 이들은 바람직하긴 하지만 부수적인 목표입니다.
3. 우리는 C++나 기타 다른 언어의 모든 기능 집합을 커버하려 하지 않습니다. Rust는 자주 쓰이는 기능들을 제공할 것입니다.
4. 우리는 100% 정적이거나, 100% 안전하거나, 100% 반영적(reflective)이거나, 기타 어떤 의미에서도 너무 교조적이려 하지 않습니다. 트레이드 오프는 존재합니다.
5. 우리는 Rust가 "가능한 모든 플랫폼"에서 동작할 걸 요구하지 않습니다. 언젠가 Rust는 널리 쓰이는 하드웨어와 소프트웨어 플랫폼에서 불필요한 타협 없이 동작할 것입니다.

<h3><a href="#how-does-mozilla-use-rust" name="how-does-mozilla-use-rust">
Mozilla에서 Rust를 사용하는 프로젝트가 무엇인가요?
</a></h3>

주된 프로젝트는 Mozilla가 만들고 있는 실험적인 브라우저 엔진인 [Servo](https://github.com/servo/servo)가 있습니다.
또한 파이어폭스에 [Rust 컴포넌트를 통합](https://bugzilla.mozilla.org/show_bug.cgi?id=1135640)하는 작업도 진행 중입니다.

<h3><a href="#what-examples-are-there-of-large-rust-projects" name="what-examples-are-there-of-large-rust-projects">
대규모 Rust 프로젝트의 예제가 있나요?
</a></h3>

현재 가장 큰 오픈소스 Rust 프로젝트로는 [Servo](https://github.com/servo/servo)와 [Rust 컴파일러](https://github.com/rust-lang/rust) 자신이 있습니다.

<h3><a href="#who-else-is-using-rust" name="who-else-is-using-rust">
Rust를 쓰는 다른 곳이 있나요?
</a></h3>

[계속 늘어나고 있습니다!](friends.html)

<!--
### What projects are good examples of idiomatic Rust code?

TODO: Write this answer.
-->

<h3><a href="#how-can-i-try-rust-easily" name="how-can-i-try-rust-easily">
Rust를 쉽게 시도해 보려면 어떻게 해야 하나요?
</a></h3>

Rust를 시도해 보는 가장 쉬운 방법은 Rust 코드를 작성하고 실행할 수 있는 온라인 앱인 [플레이펜](https://play.rust-lang.org/)을 통하는 것입니다.
여러분의 시스템에서 Rust를 시도해 보고 싶다면, [설치](https://www.rust-lang.org/install.html) 후 《Rust 프로그래밍 언어》의 [숫자 맞추기 게임](https://doc.rust-lang.org/stable/book/guessing-game.html) 지도서를 따라가세요.

<h3><a href="#how-do-i-get-help-with-rust-issues" name="how-do-i-get-help-with-rust-issues">
Rust 문제들에 도움을 받으려면 어떻게 해야 하나요?
</a></h3>

여러 방법이 있습니다:

- 공식 Rust 사용자 포럼인 [users.rust-lang.org](https://users.rust-lang.org/)에 글을 올려 보세요.
- 공식 [Rust IRC 채널](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust)(#rust on irc.mozilla.org)에서 질문해 보세요.
- [Stack Overflow](https://stackoverflow.com/questions/tagged/rust)에 "rust" 태그를 붙이고 질문해 보세요.
- 비공식 Rust 서브레딧인 [/r/rust](https://www.reddit.com/r/rust)에 글을 올려 보세요.

<h3><a href="#why-has-rust-changed-so-much" name="why-has-rust-changed-so-much">
Rust가 그동안 왜 그렇게 많이 바뀌었나요?
</a></h3>

Rust는 안전하지만 쓸만한 시스템 프로그래밍 언어를 만들기 위한 목표로 출발했습니다.
이 목표를 달성하기 위해 수많은 아이디어들을 탐색했고, 그 중 일부는 채택되었지만(수명\[lifetime], 트레이트) 다른 것들은 기각되었습니다(타입스테이트\[type state] 시스템, 그린 스레딩).
또한 버전이 1.0에 근접하면서, Rust의 기능을 잘 사용하면서 질 좋고 일관된 크로스 플랫폼 API를 제공하기 위하여 표준 라이브러리의 많은 부분이 초기 설계로부터 재작성되었습니다.
이제 Rust가 1.0이 되면서 언어는 "안정화"되었다고 보장할 수 있고, 언어는 여전히 진화하겠지만 현재 Rust에서 작동하는 코드는 앞으로의 버전에서도 계속 작동할 것입니다.

<h3><a href="#how-does-rust-language-versioning-work" name="how-does-rust-language-versioning-work">
Rust 언어의 버전은 어떻게 작동하나요?
</a></h3>

Rust의 언어 버전은 [유의적 버전(semantic versioning)](http://semver.org/)을 따릅니다.
부(部) 버전에서 안정화된 API의 호환성을 깨뜨리는 수정은, 그 수정이 컴파일러 버그를 고치거나, 안전성 구멍을 메꾸거나, 추가적인 타입 정보를 요구하는 방향으로 디스패치 및 타입 추론을 고칠 경우에만 허용됩니다.
부 버전 변경에 대한 더 자세한 가이드라인은 [언어](https://github.com/rust-lang/rfcs/blob/master/text/1122-language-semver.md) 및 [표준 라이브러리](https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md)에 대한 승낙된 RFC들에서 확인할 수 있습니다.

Rust는 세 개의 "릴리스 채널", 즉 안정, 베타 및 나이틀리를 관리합니다.
안정 및 베타 버전은 매 6주마다 갱신되며, 기존의 나이틀리는 새 베타가 되고 기존의 베타는 새 안정 버전이 됩니다.
불안정하다고 표시되어 있거나 기능 게이트로 숨겨져 있는 언어 및 표준 라이브러리 기능은 나이틀리 버전에서만 사용할 수 있습니다.
새 기능은 불안정 상태로 도입되고, 코어 팀 및 유관한 서브팀이 재가하면 게이트가 "풀립니다".
이런 접근을 통해 안정 버전들 사이에서는 강한 하위 호환성을 제공하면서 실험을 할 수 있습니다.

더 자세한 설명은 Rust 블로그의 ["Stability as a Deliverable"](http://blog.rust-lang.org/2014/10/30/Stability.html) 글을 읽어 보세요.

<h3><a href="#can-i-use-unstable-features-on-the-beta-or-stable-channel" name="can-i-use-unstable-features-on-the-beta-or-stable-channel">
불안정한 기능을 베타나 안정 버전에서 사용할 수 있나요?
</a></h3>

아뇨, 사용할 수 없습니다.
Rust는 베타 및 안정 버전에 있는 기능들의 안정성에 강한 보장을 하려 애씁니다.
무언가가 불안정하다는 말은 우리가 아직 그 기능을 보장할 수 없다는 뜻이며, 사람들이 그게 똑같이 유지된다고 생각하며 의존하도록 하고 싶지 않다는 얘깁니다.
이렇게 함으로써 나이틀리 버전에서 야생의 수정을 시도할 기회를 얻는 동시에, 안정성을 원하는 사람들에게는 강한 보장이 여전히 제공됩니다.

매 버전마다 여러 기능들이 안정화되며, 베타 및 안정 버전은 매 6주마다 갱신됩니다.
종종 베타에는 수정 사항들이 반영되기도 합니다.
원하는 기능이 나이틀리 없이 사용 가능해지는 걸 기다리고 있다면, 이슈 트래커의 [`B-unstable`](https://github.com/rust-lang/rust/issues?q=is%3Aissue+is%3Aopen+tracking+label%3AB-unstable) 태그를 확인해서 해당 기능을 추적하는 이슈를 찾아 볼 수 있습니다.

<h3><a href="#what-are-feature-gates" name="what-are-feature-gates">
"기능 게이트"가 무엇인가요?
</a></h3>

"기능 게이트"는 Rust가 컴파일러, 언어 및 표준 라이브러리 기능을 안정화하는 데 사용하는 기작입니다.
게이트에 "막혀 있는" 기능은 나이틀리 버전으로만 접근할 수 있고, 더불어 `#[feature]` 속성이나 `-Z unstable-options` 명령줄 인자로 명시적으로 켜져야만 합니다.
기능이 안정화되면 안정 버전에서 사용 가능해지며 명시적으로 켤 필요가 없습니다.
이 시점에서 기능은 게이트가 "풀렸다"고 봅니다.
기능 게이트를 쓰면 안정화된 언어에서 사용 가능해지기 전에 개발 도중의 실험적인 기능을 실험해 볼 수 있습니다.

<h3><a href="#why-a-dual-mit-asl2-license" name="why-a-dual-mit-asl2-license">
MIT/ASL2 이중 라이선스를 쓰는 이유는 무엇인가요?
</a></h3>

아파치 라이선스(ASL)는 특허권의 침해를 방지하는 중요한 장치가 있지만 GPL 버전 2와 호환되지 않습니다.
GPL2와 Rust를 함께 쓰는 데 문제가 없게 하기 위해 MIT로도 함께 라이선스되어 있습니다.

<h3><a href="#why-a-permissive-license" name="why-a-permissive-license">
MPL이나 삼중 라이선스가 아니라 BSD 계열의 관대한(permissive) 라이선스를 쓰는 이유가 있나요?
</a></h3>

어느 정도는 원 개발자(그레이던)가 그걸 좋아해서였고, 또 어느 정도는 언어들은 더 넓은 대중을 가지며 웹 브라우저 같은 제품과는 달리 더 다양한 곳에 포함되고 사용될 수 있기 때문이기도 합니다.
우리는 최대한 많은 잠재 기여자들에게 어필하려고 합니다.

<h2 id="performance">성능</h2>

<h3><a href="#how-fast-is-rust" name="how-fast-is-rust">
Rust는 얼마나 빠른가요?
</a></h3>

빠릅니다!
Rust는 이미 여러 벤치마크(이를테면 [Benchmarks Game](https://benchmarksgame.alioth.debian.org/u64q/compare.php?lang=rust&lang2=gpp)이나 [기타 이것 저것](https://github.com/kostya/benchmarks))에서 관용적인 C 및 C++ 코드에 경쟁력이 있습니다.

C++와 동일하게 Rust는 [비용 없는 추상화](http://blog.rust-lang.org/2015/05/11/traits.html)를 주요 원칙으로 삼습니다.
Rust에는 전역으로 성능을 떨어뜨리는 추상화가 존재하지 않으며, 런타임 시스템에서 부하가 발생하지도 않습니다.

Rust가 LLVM에 기반해 있고 LLVM이 보기에 Clang과 비슷하게 보이려 한다는 걸 생각해 보면, LLVM에서 성능 개선이 일어난다면 Rust도 도움을 받게 됩니다.
장기적으로는 Rust 타입 시스템의 더 풍부한 정보로 C/C++ 코드에서는 어렵거나 불가능한 최적화도 가능해질 것입니다.

<h3><a href="#is-rust-garbage-collected" name="is-rust-garbage-collected">
Rust는 쓰레기 수거(garbage collection, GC)를 하나요?
</a></h3>

아니요.
Rust의 중요 혁신 중 하나는 쓰레기 수거 *없이* 메모리 안전성을 보장한다는 것입니다(즉, 세그폴트가 나지 않습니다).

Rust는 GC를 피한 덕에 여러 장점을 제공할 수 있었습니다.
자원들을 예측 가능하게 해제할 수 있고, 메모리 관리 오버헤드가 낮으며, 사실상 런타임 시스템이 없습니다.
이 모든 특징들 때문에 Rust는 아무 맥락에나 깔끔하게 포함(embed)하기 쉬우며, [이미 GC를 가지고 있는 언어에 Rust 코드를 통합](http://calculist.org/blog/2015/12/23/neon-node-rust/)하기에도 훨씬 쉽습니다.

Rust의 소유권 및 빌림(borrowing) 시스템은 GC를 쓸 필요를 없애지만, 같은 시스템으로 다른 문제들, 이를테면 [일반적인 자원 관리](http://blog.skylight.io/rust-means-never-having-to-close-a-socket/)나 [동시성](http://blog.rust-lang.org/2015/04/10/Fearless-Concurrency.html) 같은 것들에서도 도움을 받을 수 있습니다.

단일 소유권만으로 부족한 경우, Rust 프로그램은 GC 대신 표준적인 참조 카운팅 스마트 포인터 타입인 [`Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html)와 이 타입의 스레드 안전한 버전인 [`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html)에 의존합니다.

하지만 우리는 추후 확장으로 *선택 가능한* 쓰레기 수거를 조사하고 있습니다.
목표는 [Spidermonkey](https://developer.mozilla.org/en-US/docs/Mozilla/Projects/SpiderMonkey)나 [V8](https://developers.google.com/v8/?hl=en) 자바스크립트 엔진 같은 곳에서 제공하는 쓰레기 수거 런타임에 매끄럽게 통합할 수 있게 하는 것입니다.
또한 몇몇 사람들은 컴파일러 지원 없이 [순수 Rust만으로 쓰레기 수거기를 만드는 시도](https://manishearth.github.io/blog/2015/09/01/designing-a-gc-in-rust/)를 하기도 했습니다.

<h3><a href="#why-is-my-program-slow" name="why-is-my-program-slow">
제 프로그램이 왜 느린 거죠?
</a></h3>

Rust 컴파일러는 요청이 없다면 최적화 없이 컴파일을 하는데, 이는 [최적화를 하면 컴파일이 느려지고 개발 과정에서는 보통 바람직하지 않기 때문입니다](https://users.rust-lang.org/t/why-does-cargo-build-not-optimise-by-default/4150/3).

`cargo`로 컴파일을 한다면 `--release` 플래그를 쓰세요.
`rustc`를 직접 써서 컴파일을 한다면 `-O` 플래그를 쓰세요.
어느 쪽이나 최적화를 켜는 역할을 합니다.

<h3><a href="#why-is-rustc-slow" name="why-is-rustc-slow">
Rust 컴파일이 느린 것 같습니다. 왜 그런 건가요?
</a></h3>

코드를 기계어로 번역하고 최적화를 하기 때문입니다.
Rust는 효율적인 기계어로 컴파일되는 고수준 추상화를 제공하고, 이 번역 과정은 특히 최적화를 할 경우 시간이 걸리게 마련입니다.

그러나 Rust의 컴파일 시간은 생각보다는 나쁜 편은 아니며, 앞으로 더 개선될 거라고 믿을 이유가 있습니다.
C++와 Rust로 비슷한 크기의 프로젝트를 비교해 보면 전체 프로젝트를 컴파일하는 시간은 일반적으로 비슷하다고 봅니다.
Rust 컴파일이 느리다고 느끼는 주된 원인은 C++와 Rust가 *컴파일 모델*이 다르다는 점, 즉 C++의 컴파일 단위는 한 파일이지만 Rust는 여러 파일로 이루어진 크레이트라는 것 때문입니다.
따라서 개발 도중에 C++ 파일 하나를 고치면 Rust에 비해 컴파일 시간이 훨씬 줄어들 수 있습니다.
현재 Rust 컴파일러를 리팩토링해서 [증분 컴파일](https://github.com/rust-lang/rfcs/blob/master/text/1298-incremental-compilation.md)을 가능하게 하려는 대형 작업이 진행 중이며, 완료되면 Rust에서도 C++ 모델과 같이 컴파일 시간이 개선될 것입니다.

컴파일 모델과는 별개로, Rust의 언어 설계에는 컴파일 시간에 영향을 미치는 요소가 여럿 있습니다.

먼저 Rust는 비교적 복잡한 타입 시스템을 가지고 있고, 실행 시간에 Rust를 안전하게 만들기 위한 제약 사항을 강제하는 데 무시할 수 없는 컴파일 시간을 사용해야 합니다.

두번째로 Rust 컴파일러에는 오래된 기술 부채가 있으며, 특히 생성되는 LLVM IR의 품질이 좋지 못하기 때문에 LLVM이 시간을 들여 이를 "고쳐야" 합니다.
미래에는 [MIR 기반](https://github.com/rust-lang/rfcs/blob/master/text/1211-mir.md) 최적화 및 번역 단계가 Rust 컴파일러가 LLVM에 가하는 부하를 줄여 줄지도 모릅니다.

세번째로 Rust가 코드 생성에 LLVM을 쓰는 것은 양날의 검이라는 점입니다.
LLVM 덕분에 Rust는 세계구급 런타임 성능을 보여 주지만, LLVM은 컴파일 시간에 촛점을 맞추지 않은 거대한 프레임워크이며 특히 품질이 낮은 입력에 취약합니다.

마지막으로 Rust가 일반화(제너릭) 타입을 C++와 비슷하게 단형화(monomorphise)하는 전략은 빠른 코드를 생성하지만, 다른 번역 전략에 비해 상당히 많은 코드를 생성해야 한다는 문제가 있습니다.
이 코드 팽창은 트레이트 객체를 써서 동적 디스패치와 장단을 교환할 수 있습니다.

<h3><a href="#why-are-rusts-hashmaps-slow" name="why-are-rusts-hashmaps-slow">
Rust의 <code>HashMap</code>은 왜 느린가요?
</a></h3>

Rust의 [`HashMap`][HashMap]은 기본적으로 [SipHash](https://131002.net/siphash/) 해시 알고리즘을 사용합니다.
이 알고리즘은 [해시 테이블 충돌 공격](http://programmingisterrible.com/post/40620375793/hash-table-denial-of-service-attacks-revisited)을 막으면서 [여러 종류의 입력에 대해 적절한 성능을 내도록](https://www.reddit.com/r/rust/comments/3hw9zf/rust_hasher_comparisons/cub4oh6) 설계되었습니다.

SipHash가 많은 경우 [경쟁력 있는 성능](http://cglab.ca/%7Eabeinges/blah/hash-rs/)을 보여 주긴 하지만, SipHash는 정수 같이 키가 짧을 경우 다른 해시 알고리즘에 비해 현저히 느립니다.
이 때문에 종종 [`HashMap`][HashMap]의 성능이 낮은 걸 볼 수 있습니다.
이런 경우에는 보통 [FNV 해시](https://crates.io/crates/fnv)를 추천하지만, 이 알고리즘이 충돌 공격에서 SipHash와 다른 특성을 보인다는 점은 염두에 두어야 합니다.

<h3><a href="#why-is-there-no-integrated-benchmarking" name="why-is-there-no-integrated-benchmarking">
왜 통합된 성능 측정 인프라가 없는 건가요?
</a></h3>

있긴 한데요, 나이틀리 버전에만 있습니다.
궁극적으로는 착탈 가능한 통합 성능 측정 시스템을 만들 예정입니다만, 일단 현재 시스템은 [불안정하다고 여겨집니다](https://github.com/rust-lang/rust/issues/29553).

<h3><a href="#does-rust-do-tail-call-optimization" name="does-rust-do-tail-call-optimization">
Rust는 꼬리 재귀(tail-call) 최적화를 하나요?
</a></h3>

일반적으로는 아닙니다.
[제한적으로](http://llvm.org/docs/CodeGenerator.html#sibling-call-optimization) 꼬리 재귀 최적화를 하긴 하지만 [보장되지는 않습니다](https://mail.mozilla.org/pipermail/rust-dev/2013-April/003557.html).
이 기능은 언제나 요청되어 왔기 때문에 Rust에는 이를 위해 예약어(`become`)가 예약되어 있습니다만, 이 기능이 기술적으로 가능한지, 그리고 가능하다면 구현이 될 것인지는 아직 불투명합니다.
특정한 맥락에서 꼬리 재귀 최적화를 하는 [확장이 제안](https://github.com/rust-lang/rfcs/pull/81)되었지만 현재 보류된 상태입니다.

<h3><a href="#does-rust-have-a-runtime" name="does-rust-have-a-runtime">
Rust에는 런타임이 있나요?
</a></h3>

Java 같은 언어들에서 말하는 그런 통상의 런타임은 없습니다만, Rust 표준 라이브러리의 일부분은 힙(heap), 스택 추적(backtrace), 되감기(unwinding) 및 보호(guard)를 제공하는 "런타임"이라고 볼 수 있습니다.
사용자의 `main` 함수가 실행되기 전에는 [소량의 초기화 코드](https://github.com/rust-lang/rust/blob/33916307780495fe311fe9c080b330d266f35bfb/src/libstd/rt.rs#L43)가 실행됩니다.
또한 Rust 표준 라이브러리는 C 표준 라이브러리를 링크하는데 여기에서도 비슷한 [런타임 초기화](http://www.embecosm.com/appnotes/ean9/html/ch05s02.html)가 일어납니다.
Rust 코드는 표준 라이브러리 없이 컴파일될 수 있으며 이 경우 런타임은 대략 C와 비슷해집니다.

<h2 id="syntax">문법</h2>

<h3><a href="#why-curly-braces" name="why-curly-braces">
왜 중괄호인가요? Rust의 문법이 하스켈이나 파이썬 같지 않은 이유가 있나요?
</a></h3>

중괄호를 블록에 사용하는 건 여러 프로그래밍 언어에서 흔히 쓰이는 설계이며, 이 스타일에 이미 익숙한 사람들한테는 Rust가 이를 따르는 쪽이 편리합니다.

또한 중괄호는 프로그래머 입장에서는 더 유연한 문법을 제공하고 컴파일러 입장에서는 더 간단한 파서를 가능하게 합니다.

<h3><a href="#why-brackets-around-blocks" name="why-brackets-around-blocks">
<code>if</code> 조건에서 소괄호를 생략할 수 있는데, 그럼 한 줄짜리 블럭에는 왜 중괄호를 넣어야 하나요? C 같은 문법이 안 되는 이유가 있나요?
</a></h3>

C에서는 `if` 조건문에서 괄호가 필수이고 중괄호가 선택이지만, Rust에서는 반대로 합니다.
이렇게 해서 조건문 몸체와 조건을 명확하게 구분할 수 있고, 중괄호가 선택이라서 벌어지는 위험도 막을 수 있는데, 이는 Apple의 [goto fail](https://gotofail.com/) 버그와 같이 리팩토링 과정에서 흔히 생기고 잡기 어려운 오류들을 유발할 수 있습니다.

<h3><a href="#why-no-literal-syntax-for-dictionaries" name="why-no-literal-syntax-for-dictionaries">
연관 배열의 리터럴 문법이 없는 이유는 무엇인가요?
</a></h3>

Rust의 전반적인 설계는 *언어*의 크기를 제한하되 강력한 *라이브러리*를 만들 수 있게 하는 쪽을 선호합니다.
Rust는 배열과 문자열 리터럴을 초기화하는 문법을 가지고 있지만 언어에 내장된 컬렉션 타입은 이걸로 전부입니다.
매우 널리 쓰이는 [`Vec`][Vec] 컬렉션 타입 같이, 라이브러리에서 정의하는 다른 타입들은 [`vec!`][VecMacro] 같은 매크로를 사용하여 초기화를 합니다.

나중에는 Rust가 매크로를 써서 컬렉션을 초기화하는 설계가 다른 타입에도 일반적으로 사용할 수 있도록 확장될 수 있고, 그렇게 되면 [`HashMap`][HashMap]이나 [`Vec`][Vec] 같은 것 뿐만이 아니라 [`BTreeMap`][BTreeMap] 같은 다른 타입들도 간단하게 초기화할 수 있게 될 것입니다.
그 전에 컬렉션을 더 간단한 문법으로 초기화하고 싶다면 [직접 매크로를 만들 수 있습니다](https://stackoverflow.com/questions/27582739/how-do-i-create-a-hashmap-literal).

<h3><a href="#when-should-i-use-an-implicit-return" name="when-should-i-use-an-implicit-return">
언제 암묵적인 반환을 써야 하나요?
</a></h3>

Rust는 매우 수식 지향적인 언어이며 "암묵적인 반환"은 이 설계의 한 부분입니다.
`if`, `match`나 일반 블록들은 Rust에서는 다 수식입니다.
예를 들어 다음 코드는 [`i64`][i64]가 홀수인지 확인하고 결과를 단순히 값으로 내서 결과를 반환합니다:

```rust
fn is_odd(x: i64) -> bool {
    if x % 2 != 0 { true } else { false }
}
```

물론 더 간단하게는 이렇게 쓰겠지만요:

```rust
fn is_odd(x: i64) -> bool {
    x % 2 != 0
}
```

두 예제에서 함수의 마지막 줄은 그 함수의 반환값입니다.
중요한 것은 함수가 세미콜론으로 끝난다면 그 반환값은 `()`이고, 이는 반환값이 없다는 뜻이라는 점입니다.
암묵적으로 반환하려면 세미콜론이 없어야 합니다.

명시적인 반환은 함수 몸체의 맨 끄트머리보다 이전에 반환을 해야 해서 암묵적인 반환이 불가능할 때만 쓰입니다.
물론 위 함수들도 `return` 예약어와 세미콜론을 쓸 수는 있지만 불필요하게 번잡하고 Rust 코드의 규약에 어긋날 것입니다.

<h3><a href="#why-arent-function-signatures-inferred" name="why-arent-function-signatures-inferred">
왜 함수의 타입 서명(signature)들은 추론되지 않는 거죠?
</a></h3>

Rust에서 선언은 타입을 명시적으로 쓰는 편이며 실제 코드는 타입을 추론하는 편입니다.
이 설계에는 몇 가지 이유가 있습니다:

- 선언의 서명을 명시적으로 쓰면 모듈 및 크레이트 수준에서 인터페이스 안정성을 강제하는 데 도움이 됩니다.
- 서명은 프로그래머가 코드를 더 잘 이해할 수 있게 하므로, IDE가 함수의 인자 타입들을 추측하려고 전체 크레이트에 추론 알고리즘을 돌릴 필요가 사라집니다. 언제나 명시적이고 바로 옆에 있기 때문이죠.
- 기계적으로는 추론 과정에서 한 번에 한 함수만 보면 되므로 추론 알고리즘이 간단해집니다.

<h3><a href="#why-does-match-have-to-be-exhaustive" name="why-does-match-have-to-be-exhaustive">
왜 <code>match</code>에는 모든 조건들이 들어 있어야 하나요?
</a></h3>

리팩토링을 돕고 코드를 명료하게 하기 위함입니다.

먼저, `match`가 모든 가능성을 커버하고 있다면 `enum`에 새 변종(variant)을 넣을 때 실행 시간에 오류가 나는 게 아니라 컴파일이 실패하게 됩니다.
Rust에서 이런 종류의 컴파일러 도움은 두려움 없이 리팩토링을 가능하게 합니다.

두 번째로, 이러한 체크는 기본 선택지를 명시적으로 만듭니다.
일반적으로 모든 가능성을 커버하지 않는 `match`를 안전하게 만드는 방법은 아무 선택지도 선택되지 않았을 때 스레드를 패닉하게 만드는 것 뿐입니다.
Rust의 옛 버전에서는 `match`가 모든 가능성을 커버하지 않아도 되게 했는데 수많은 버그의 온상이 되었습니다.

기술되지 않은 선택지는 `_` 와일드 카드로 간단하게 무시할 수 있습니다:

```rust
match val.do_something() {
    Cat(a) => { /* ... */ }
    _      => { /* ... */ }
}
```

<h2 id="numerics">숫자</h2>

<h3><a href="#which-type-of-float-should-i-use" name="which-type-of-float-should-i-use">
부동 소숫점 계산을 할 때 <code>f32</code>와 <code>f64</code> 중 어느 쪽을 선호해야 하나요?
</a></h3>

프로그램의 목적에 따라 어느 쪽을 쓸지가 달라집니다.

만약 부동 소숫점 숫자가 최대한 정밀해야 한다면 [`f64`][f64]를 우선시하세요.
만약 크기를 작게 유지하거나 최대한의 성능을 얻고 싶고, 그에 따라 줄어드는 정밀도를 신경쓰지 않겠다면 [`f32`][f32]가 낫습니다.
64비트 하드웨어에서도 [`f32`][f32]가 보통 더 빠릅니다.
예를 들어 그래픽 프로그래밍에서는 높은 성능이 필요하고 화면 상의 픽셀을 표현하는 데는 32비트 부동 소숫점으로 충분하기 때문에 보통 [`f32`][f32]를 씁니다.

잘 모르겠으면 정밀도를 우선시해서 [`f64`][f64]를 선택하세요.

<h3><a href="#why-cant-i-compare-floats" name="why-cant-i-compare-floats">
실수들을 비교하거나 <code>HashMap</code> 및 <code>BTreeMap</code>의 키로 쓸 수 없는 이유는 뭔가요?
</a></h3>

실수들은 `==`, `!=`, `<`, `<=`, `>` 및 `>=` 연산자나 `partial_cmp()` 함수로 비교할 수 있습니다.
`==`와 `!=`는 [`PartialEq`][PartialEq] 트레이트의 일부이고, `<`, `<=`, `>`, `>=` 및 `partial_cmp()`는 [`PartialOrd`][PartialOrd] 트레이트의 일부입니다.

실수들은 [`Ord`][Ord] 트레이트에 있는 `cmp()` 함수로는 비교할 수 없는데 실수에는 전순서(total order)가 없기 때문입니다.
덧붙여 실수에는 전등치(total equality) 관계도 없으므로 [`Eq`][Eq] 트레이트도 구현되어 있지 않습니다.

실수에 전순서나 전등치가 없는 이유는 부동 소숫점 실수에 있는 [`NaN`](https://en.wikipedia.org/wiki/NaN) 값은 다른 어떤 값이나 자기 자신보다 작지도, 크지도, 같지도 않기 때문입니다.

실수가 [`Eq`][Eq]나 [`Ord`][Ord]를 구현하지 않으므로 이들 트레이트를 요구하는 타입에서는 사용 불가능하는데, 여기에는 [`BTreeMap`][BTreeMap]이나 [`HashMap`][HashMap]도 들어갑니다.
이들 타입은 그 키가 전순서나 전등치를 가지고 있다고 *가정*하고, 그렇지 않다면 오동작할 것이므로 이 조건은 중요합니다.

다만, [`f32`][f32]와 [`f64`][f64]를 감싸서 [`Ord`][Ord]와 [`Eq`][Eq] 구현을 제공하는 [크레이트는 존재](https://crates.io/crates/ordered-float)하며 몇몇 상황에서 유용할 수 있습니다.

<h3><a href="#how-can-i-convert-between-numeric-types" name="how-can-i-convert-between-numeric-types">
수치형들 사이에 변환을 하려면 어떻게 하나요?
</a></h3>

두 가지 방법이 있는데, 하나는 `as` 예약어로 원시 타입 사이에서 간단한 변환을 하는 것이고, 다른 하나는 [`Into`][Into]와 [`From`][From] 트레이트를 써서 임의의 타입 변환을 하는 것입니다(트레이트를 직접 구현해서 변환을 추가할 수도 있습니다).
[`Into`][Into]와 [`From`][From] 트레이트는 변환에서 손실이 일어나지 않을 때만 구현되어 있며, 이를테면 `f64::from(0f32)`는 컴파일이 되지만 `f32::from(0f64)`는 아닙니다.
한편 `as`는 원시 타입들 사이에서는 모두 변환이 가능하며 필요하다면 값을 잘라냅니다.

<h3><a href="#why-doesnt-rust-have-increment-and-decrement-operators" name="why-doesnt-rust-have-increment-and-decrement-operators">
왜 Rust에는 증감 연산자가 없나요?
</a></h3>

전위 및 후위 증감 연산자는 편하긴 하지만 꽤 복잡합니다.
이들 연산자를 쓰려면 연산 순서를 알아야 하고, C나 C++에서 이로 인한 미묘한 버그나 정의되지 않은 동작이 흔히 발생하지요.
`x = x + 1`이나 `x += 1`은 살짝 더 길 뿐이지만 모호하지 않습니다.

<h2 id="strings">문자열</h2>

<h3><a href="#how-to-convert-string-or-vec-to-slice" name="how-to-convert-string-or-vec-to-slice">
<code>String</code>이나 <code>Vec&lt;T&gt;</code>를 슬라이스(<code>&amp;str</code> 및 <code>&amp;[T]</code>)로 어떻게 바꾸나요?
</a></h3>

보통 슬라이스를 예상하는 곳에서는 `String`이나 `Vec<T>`의 참조를 넘길 수 있습니다.
[`String`][String]과 [`Vec`][Vec]이 `&`나 `& mut` 참조로 넘겨질 때는 [deref 변환(coercion)](https://doc.rust-lang.org/stable/book/deref-coercions.html)을 통해 각자 대응되는 슬라이스로 자동으로 변환됩니다.

`&str`과 `&[T]`에 구현된 메소드들은 `String`과 `Vec<T>`에서 바로 접근할 수 있습니다.
예를 들어 `char_at`은 `&str`의 메소드이고 `some_string`이 `String`이라 하더라도 `some_string.char_at(0)`은 동작할 것입니다.

일반화된 코드 같이 몇몇 상황에서는 수동으로 변환해야 할 필요가 있습니다.
수동 변환은 슬라이스 연산자를 써서 `&my_vec[..]`과 같이 할 수 있습니다.

<h3><a href="#how-to-convert-between-str-and-string" name="how-to-convert-between-str-and-string">
<code>&amp;str</code>를 <code>String</code>로 바꾸거나 반대로 하려면 어떻게 하나요?
</a></h3>

[`to_string()`][to_string] 메소드는 [`&str`][str]를 [`String`][String]로 변환하고, [`String`][String]에서 참조를 빌리면 [`&str`][str]로 자동으로 변환됩니다.
아래 예제는 두 가지 방향을 모두 시연합니다:

```rust
fn main() {
    let s = "Jane Doe".to_string();
    say_hello(&s);
}

fn say_hello(name: &str) {
    println!("Hello {}!", name);
}
```

<h3><a href="#what-are-the-differences-between-str-and-string" name="what-are-the-differences-between-str-and-string">
두 개의 다른 문자열 타입에 어떤 차이가 있나요?
</a></h3>

[`String`][String]은 힙에 할당된 UTF-8 바이트를 소유하는 버퍼입니다.
변경 가능한 [`String`][String]은 수정할 수 있고 필요에 따라 그 용량(capacity)을 늘릴 수 있습니다.
[`&str`][str]은 다른 데 (보통 힙에) 할당되어 있는 [`String`][String]으로부터 참조된 슬라이스나, 문자열 리터럴의 경우 정적 메모리를 가리키는, 용량이 고정된 "창"입니다.
<!-- TODO: the original text states that string literals are "allocated in static memory"? -->

[`&str`][str]은 Rust 언어가 구현하는 원시 타입이지만 [`String`][String]은 표준 라이브러리에 구현되어 있습니다.

<h3><a href="#how-do-i-do-o1-character-access-in-a-string" name="how-do-i-do-o1-character-access-in-a-string">
<code>String</code>의 각 문자를 O(1), 즉 상수 시간에 접근하려면 어떻게 해야 하나요?
</a></h3>

불가능합니다.
적어도 "문자"가 무슨 의미인지 제대로 이해하고 있지 않거나, 원하는 문자의 인덱스를 찾으려 문자열을 전처리하지 않는다면 말이지요.

Rust 문자열은 UTF-8로 인코딩되어 있습니다.
보기에 하나의 문자는 UTF-8에서는 ASCII 문자열는 달리 꼭 한 바이트인 건 아닙니다.
각 바이트는 "코드 단위"라고 불립니다(UTF-16에서는 코드 단위가 2바이트이고 UTF-32에서는 4바이트이지요).
"코드포인트"는 하나 이상의 코드 단위로 구성되어 있고, 문자를 가장 가까이 근사한다고 할 수 있는 "자소(grapheme) 클러스터"는 여러 개의 코드포인트로 구성되어 있습니다.

따라서 UTF-8 문자열에서 바이트를 인덱싱할 수 있다 하더라도 상수 시간에 `i`번째 코드포인트나 자소 클러스터를 얻어낼 수는 없습니다.
하지만 원하는 코드포인트나 자소 클러스터가 어느 바이트에서 시작하는지 안다면 _그건_ 상수 시간에 접근할 수 있습니다.
[`str::find()`][str__find]나 정규식 검색 결과는 바이트 인덱스를 반환하므로 이 방법으로 접근하는 게 가능합니다.

<h3><a href="#why-are-strings-utf-8" name="why-are-strings-utf-8">
왜 문자열이 기본적으로 UTF-8인가요?
</a></h3>

[`str`][str] 타입이 UTF-8인 것은 현실에서, 특히 엔디안이 정해져 있지 않은 네트워크 전송에서 이 인코딩이 널리 쓰이기 때문이고, I/O를 할 때 어느 방향에서도 코드포인트를 다시 변환할 필요가 없는 것이 최선이라고 생각하기 때문입니다.

물론 이는 문자열 안의 특정 유니코드 코드포인트의 위치를 찾는데 O(n) 연산이 필요하다는 뜻이긴 합니다.
이미 시작하는 바이트 인덱스를 알고 있을 경우에는 예상대로 O(1) 시간이 걸리겠지만요.
어떻게 보면 바람직하지 않을 수도 있지만, 어떻게 보면 이 문제 자체가 트레이드오프로 가득 차 있기에 다음 중요한 점들을 지적할 필요가 있겠습니다:

[`str`][str]에서 ASCII 영역의 코드포인트를 훑는 건 바이트 단위로 안전하게 할 수 있습니다.
[`.as_bytes()`][str__as_bytes]를 쓸 경우 [`u8`][u8]을 얻는 건 `O(1)` 연산이며 이 값은 ASCII 범위의 [`char`][char]로 변환하거나 비교할 수 있습니다.
그러니까 이를테면 `'\n'`로 줄 바꿈을 찾는다면 바이트 단위로 검색해도 됩니다.
UTF-8은 원래부터 이렇게 설계되었거든요.

대부분의 "문자 기반" 텍스트 연산들은 "ASCII 범위의 코드포인트 한정" 같이 매우 제약된 언어 가정이 있어야만 동작합니다.
ASCII 범위를 벗어나면 언어학적인 단위들(글리프, 낱말, 문단)의 경계를 찾기 위해 (상수 시간이 아닌) 복잡한 알고리즘을 써야 하기 마련입니다.
저희는 "솔직한", 언어학적으로 올바르며 유니코드에서 인증한 알고리즘을 권장합니다.

[`char`][char] 타입은 UTF-32입니다.
한 번에 한 코드포인트를 들여다 보는 알고리즘이 정말로 필요하다고 생각한다면 `type wstr = [char]`을 정의하여 [`str`][str]로부터 한번에 읽어들인 뒤 `wstr`에서 연산을 하면 됩니다.
다르게 말하면, 언어가 "기본적으로 UTF-32로 디코딩하지 않는다"고 해서 UTF-32로 디코딩하거나 다시 인코딩하는 것 자체가 불가능한 건 아니라는 말입니다.

왜 UTF-8이 UTF-16이나 UTF-32보다 보통 더 선호되는지 자세한 설명을 원한다면 [UTF-8 Everywhere manifesto](http://utf8everywhere.org/)를 읽어 보시길 바랍니다.

<h3><a href="#what-string-type-should-i-use" name="what-string-type-should-i-use">
어떤 문자열 타입을 써야 하죠?
</a></h3>

Rust는 네 쌍의 문자열 타입이 있고 [각각 다른 역할을 합니다](http://www.suspectsemantics.com/blog/2016/03/27/string-types-in-rust/).
각 쌍마다 "소유된" 문자열 타입과 "슬라이스" 문자열 타입이 따로 있고, 다음과 같이 구성되어 있습니다:

|               | "슬라이스" 타입 | "소유된" 타입 |
|:--------------|:----------------|:-------------|
| UTF-8         | `str`           | `String`     |
| OS 호환용     | `OsStr`         | `OsString`   |
| C 호환용      | `CStr`          | `CString`    |
| 시스템 경로   | `Path`          | `PathBuf`    |

Rust의 서로 다른 문자열 타입은 각자 다른 목적을 가집니다.
`String`과 `str`은 UTF-8로 인코딩된 일반 목적의 문자열입니다.
`OsString`과 `OsStr`은 현재 플랫폼에 맞춰 인코딩되어 있고 운영체제와 상호작용할 때 쓰입니다.
`CString`과 `CStr`은 C 문자열의 Rust 버전으로 FFI 코드에 사용되고, `PathBuf`와 `Path`는 `OsString`과 `OsStr`에 편의를 위해 경로 조작을 위한 메소드들을 추가한 것입니다.

<h3><a href="#why-are-there-multiple-types-of-strings" name="why-are-there-multiple-types-of-strings">
<code>&str</code>와 <code>String</code>을 동시에 받는 함수를 어떻게 짤 수 있나요?
</a></h3>

함수의 요구 사항에 따라 여러 선택이 있습니다:

- 함수가 소유된 문자열을 필요로 하지만 아무 문자열 타입이나 받고 싶다면, `Into<String>` 제약을 쓰세요.
- 함수가 문자열 슬라이스를 필요로 하지만 아무 문자열 타입이나 받고 싶다면, `AsRef<str>` 제약을 쓰세요.
- 함수가 문자열 타입에 대해 신경쓰지 않고 두 가능성을 일관되게 처리하고 싶다면, 입력 타입으로 `Cow<str>`을 쓰세요.

__`Into<String>`의 사용__

이 예제에서 함수는 소유된 문자열과 문자열 슬라이스를 둘 다 받으며, 어느 쪽인지에 따라 함수 몸체 안에서 아무 일도 하지 않거나 입력을 소유된 문자열로 변환합니다.
참고로 변환은 명시적으로 해야 하며 안 그러면 변환되지 않을 것입니다.

```rust
fn accepts_both<S: Into<String>>(s: S) {
    let s = s.into();   // s를 `String`으로 변환합니다.
    // ... 함수의 나머지 내용
}
```

__`AsRef<str>`의 사용__

이 예제에서 함수는 소유된 문자열과 문자열 슬라이스를 둘 다 받으며, 어느 쪽인지에 따라 아무 일도 하지 않거나 입력을 문자열 슬라이스로 변환합니다.
이는 입력을 참조로 받아서 다음과 같이 자동으로 일어나게 할 수 있습니다:

```rust
fn accepts_both<S: AsRef<str>>(s: &S) {
    // ... 함수의 몸체
}
```

__`Cow<str>`의 사용__

이 예제에서 함수는 `Cow<str>`을 받는데, 이는 일반화된 타입이 아니라 컨테이너로서 필요에 따라 소유된 문자열이나 문자열 슬라이스를 담을 수 있습니다.

```rust
fn accepts_cow(s: Cow<str>) {
    // ... 함수의 몸체
}
```


<h2 id="collections">컬렉션</h2>

<h3><a href="#can-i-implement-linked-lists-in-rust" name="can-i-implement-linked-lists-in-rust">
Rust로 벡터나 연결 리스트 같은 자료 구조를 효율적으로 구현할 수 있나요?
</a></h3>

만약 이들 자료 구조를 구현하려는 이유가 다른 프로그램에서 그걸 사용하려는 거라면 표준 라이브러리에 효율적인 구현들이 존재하므로 그럴 필요는 없습니다.

하지만, 만약 [단순히 공부를 위해서라면](http://cglab.ca/~abeinges/blah/too-many-lists/book/) 안전하지 않은 코드에 발을 들일 필요가 있을 겁니다.
이들 자료 구조들이 안전한 Rust만으로도 구현할 수*는* 있지만, 안전하지 않은 코드를 사용하는 것보다 성능이 떨어질 가능성이 높습니다.
간단하게 이유를 말하자면 벡터나 연결 리스트 같은 자료 구조들은 안전한 Rust에서 허용되지 않는 포인터와 메모리 연산에 의존하기 때문입니다.

예를 들어 양방향 연결 리스트에서는 각 노드마다 두 개의 변경 가능한 참조를 가져야 하는데 이는 Rust에서 변경 가능한 참조가 별명(alias)을 가질 수 없다는 규칙을 위배합니다.
[`Weak<T>`][Weak]로 이 문제를 해결할 수 있지만 보통 원하는 것보다 성능이 떨어질 것입니다.
안전하지 않은 코드를 쓰면 이 별명 규칙을 우회할 수 있지만, 메모리 안전성을 위배하는 코드가 없다는 걸 수동으로 검증해야 합니다.

<h3><a href="#how-can-i-iterate-over-a-collection-without-consuming-it" name="how-can-i-iterate-over-a-collection-without-consuming-it">
컬렉션을 움직이거나 소모(consume)하지 않고 각 원소에 대해 반복하려면 어떻게 하나요?
</a></h3>

가장 쉬운 방법은 컬렉션의 [`IntoIterator`][IntoIterator] 구현을 사용하는 겁니다.
다음은 [`&Vec`][Vec]을 쓰는 예제입니다:

```rust
let v = vec![1,2,3,4,5];
for item in &v {
    print!("{} ", item);
}
println!("\nLength: {}", v.len());
```

Rust의 `for` 반복문은 반복하고자 하는 대상에 대해 ([`IntoIterator`][IntoIterator] 트레이트에 정의된) `into_iter()`를 호출합니다.
[`IntoIterator`][IntoIterator] 트레이트를 구현하는 아무 값이나 `for` 반복문에서 사용할 수 있습니다.
[`IntoIterator`][IntoIterator]는 [`&Vec`][Vec]과 [`&mut Vec`][Vec]에 구현되어 있으며, 이 경우 `into_iter()`가 컬렉션을 옮기거나 소모하는 것이 아니라 그 내용물을 빌리도록 합니다.
다른 표준 컬렉션에 대해서도 똑같은 관계가 성립합니다.

만약 옮기거나 소모하는 반복자가 필요하다면 `for` 반복문에서 반복할 때 `&`나 `&mut` 없이 쓰세요.

빌리는 반복자를 직접 접근하고 싶다면 보통 `iter()` 메소드를 써서 얻을 수 있습니다.

<h3><a href="#why-do-i-need-to-type-the-array-size-in-the-array-declaration" name="why-do-i-need-to-type-the-array-size-in-the-array-declaration">
배열 선언에 배열 크기를 넣어야 하는 이유가 무엇인가요?
</a></h3>

꼭 그럴 필요는 없습니다.
배열을 직접 선언한다면 원소의 갯수로부터 크기가 추론됩니다.
하지만 고정된 크기의 배열을 받는 함수를 선언한다면 컴파일러가 배열이 얼마나 클 지를 알아야 합니다.

하나 짚고 넘어가야 하는 게 있는데, Rust는 현재 서로 다른 크기의 배열에 대해 일반화를 지원하지 않습니다.
만약 갯수가 바뀔 수 있는 값들의 연속된 컨테이너를 받고자 한다면 (소유권이 필요하냐 마냐에 따라) [`Vec`][Vec]이나 슬라이스를 사용하세요.

<h2 id="ownership">소유권</h2>

<h3><a href="#how-can-i-implement-a-data-structure-that-contains-cycles" name="how-can-i-implement-a-data-structure-that-contains-cycles">
그래프 같이 사이클을 포함하는 자료 구조를 구현하려면 어떻게 해야 하나요?
</a></h3>

적어도 네 가지 선택이 있습니다([Too Many Linked Lists](http://cglab.ca/~abeinges/blah/too-many-lists/book/)에서 더 길게 설명합니다):

- [`Rc`][Rc]와 [`Weak`][Weak]을 써서 노드가 공유된 소유권을 가질 수 있게 하여 구현할 수 있습니다. 다만 이 접근은 메모리 관리 비용을 치뤄야 합니다.
- `unsafe` 코드에서 생 포인터를 사용해서 구현할 수 있습니다. 더 효율적이지만 Rust의 안전성 보장을 우회합니다.
- 벡터와 그 벡터에 대한 인덱스를 사용합니다. 이 접근에 대한 [여러](http://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/) [가지](https://featherweightmusings.blogspot.com/2015/04/graphs-in-rust.html) 예제와 설명이 있습니다.
- [`UnsafeCell`][UnsafeCell]와 함께 빌린 참조를 사용합니다. 이 접근에 대해서 [설명과 코드](https://github.com/nrc/r4cppp/blob/master/graphs/README.md#node-and-unsafecell)가 준비되어 있습니다.

<h3><a href="#how-can-i-define-a-struct-that-contains-a-reference-to-one-of-its-own-fields" name="how-can-i-define-a-struct-that-contains-a-reference-to-one-of-its-own-fields">
자기 자신의 필드를 가리키는 참조를 가진 구조체는 어떻게 선언해야 하나요?
</a></h3>

가능하지만 의미가 없습니다.
구조체는 자기 자신을 영구히 빌리게 되며 따라서 더 이상 움직일 수 없게 됩니다.
다음 코드가 이런 상황을 보여 줍니다:

```rust
use std::cell::Cell;

#[derive(Debug)]
struct Unmovable<'a> {
    x: u32,
    y: Cell<Option<&'a u32>>,
}


fn main() {
    let test = Unmovable { x: 42, y: Cell::new(None) };
    test.y.set(Some(&test.x));

    println!("{:?}", test);
}
```

<h3><a href="#what-is-the-difference-between-consuming-and-moving" name="what-is-the-difference-between-consuming-and-moving">
값으로 호출하기, 소모(consume)하기, 움직이기, 그리고 소유권을 넘기기에 서로 무슨 차이가 있나요?
</a></h3>

다 같은 뜻입니다.
네 가지 경우에서 모두, 값이 새 소유자에게 옮겨가고, 원 소유자가 소유를 잃어버려 더 이상 쓸 수 없게 됩니다.
만약 타입이 `Copy` 트레이트를 구현한다면 원 소유자의 값은 무효화되지 않아 계속 쓸 수 있습니다.

<h3><a href="#why-can-values-of-some-types-by-reused-while-others-are-consumed" name="why-can-values-of-some-types-by-reused-while-others-are-consumed">
왜 어떤 타입은 함수에 넘긴 뒤에도 재사용할 수 있지만 다른 타입은 그렇지 않나요?
</a></h3>

타입이 [`Copy`][Copy] 트레이트를 구현하면 함수에 전달될 때 복사됩니다.
Rust의 모든 수치형은 [`Copy`][Copy]를 구현하지만, 구조체는 기본적으로 [`Copy`][Copy]를 구현하지 않기 때문에 대신 이동이 일어납니다.
즉 구조체는 함수에서 다시 반환되거나 하지 않는 한 더 이상 다른 데서 사용할 수 없게 됩니다.

<h3><a href="#how-do-you-deal-with-a-use-of-moved-value-error" name="how-do-you-deal-with-a-use-of-moved-value-error">
"움직인 값을 사용"(use of moved value)했다는 오류를 어떻게 다뤄야 하나요?
</a></h3>

이 오류는 사용하려는 값이 새 소유권자에게 옮겨갔다는 뜻입니다.
먼저 문제의 옮김이 정말로 필요한 것이었는가를 확인해 보세요.
값이 함수 안으로 옮겨갔다면 함수가 대신 참조를 쓰도록 재작성해야 할 수도 있습니다.
그게 아니고, 만약 옮겨가는 타입이 [`Clone`][Clone]을 구현할 경우, 옮겨가기 전에 `clone()`을 호출하면 값의 복사본이 옮겨가고 원래 값은 계속 쓸 수 있게 됩니다.
다만 값을 복제하는 건 일반적으로 최후의 수단이어야 하는데, 복제가 추가로 할당을 일으켜 비쌀 수 있기 때문입니다.

옮겨가는 값의 타입이 직접 만든 것이라면, (옮겨가는 대신 암묵적으로 복사를 할 경우) [`Copy`][Copy]나 (명시적으로 복사할 경우) [`Clone`][Clone]을 구현하는 걸 생각해 보세요.
[`Copy`][Copy]은 대부분 `#[derive(Copy, Clone)]`로 구현되고([`Copy`][Copy]는 [`Clone`][Clone]를 필요로 합니다), [`Clone`][Clone]은 `#[derive(Clone)]`로 구현합니다.

어느 쪽도 가능하지 않다면 함수를 고쳐서, 소유권을 얻은 함수에서 나갈 때 그 값의 소유권을 반환하도록 해야 할 수 있습니다.

<h3><a href="#what-are-the-rules-for-different-self-types-in-methods" name="what-are-the-rules-for-different-self-types-in-methods">
메소드 선언에서 언제 <code>self</code>, <code>&amp;self</code>, 또는 <code>&amp;mut self</code>를 쓰는지 규칙이 있나요?
</a></h3>

- 함수가 값을 소모해야 한다면 `self`를 쓰세요.
- 함수가 값에 대한 읽기 전용 참조만 필요하다면 `&self`를 쓰세요.
- 함수가 값을 소모하지 않으면서 값을 변경해야 한다면 `&mut self`를 쓰세요.

<h3><a href="#how-can-i-understand-the-borrow-checker" name="how-can-i-understand-the-borrow-checker">
빌림 체커(borrow checker)를 이해하는 방법은 무엇인가요?
</a></h3>

빌림 체커는 Rust 코드를 평가하는 과정에서 오로지 몇 가지 규칙만 적용하는데, 《Rust 프로그래밍 언어》의 [빌림(borrowing) 장](https://doc.rust-lang.org/stable/book/references-and-borrowing.html#the-rules)에서 확인할 수 있습니다.
이 규칙은 다음과 같습니다:

> 첫째, 모든 빌림은 소유권자의 그것보다 작거나 같은 범위(scope)동안 지속되어야 합니다.
> 둘째, 다음 두 종류의 빌림 중 하나를 가질 수 있지만 둘을 동시에 가질 수는 없습니다:
>
> - 자원에 대한 하나 이상의 참조 (&T)
> - 정확히 하나의 변경 가능한 참조 (&mut T)

규칙들 자체는 간단하지만 이를 일관되게 지키는 건, 특히 수명과 소유권에 대해 생각하는 습관이 들지 않았을 경우 간단하지 않습니다.

빌림 체커를 이해하는 첫 단계는 산출된 오류를 읽는 것입니다.
빌림 체커를 인식된 문제를 해결하는 데 양질의 도움을 제공하게 하려 많은 노력이 투자되었습니다.
빌림 체커 문제를 만났을 때는 먼저 보고된 오류를 느리고 주의 깊게 읽고, 설명된 오류를 이해한 뒤에야 코드를 접근할 수 있습니다.

두번째 단계는 [`Cell`][Cell], [`RefCell`][RefCell], 그리고 [`Cow`][Cow] 같이 Rust 표준 라이브러리가 제공하는, 소유권 및 변경 가능성에 관련된 컨테이너 타입들에 친숙해지는 것입니다.
이들은 특정한 소유권 및 변경 가능성 상황을 표현하는 데 유용하고 필요한 도구로, 최소한의 성능 비용만 지불하도록 작성되었습니다.

빌림 체커를 이해하는 데 가장 중요한 부분은 연습입니다.
Rust의 강력한 정적 분석 보장은 많은 프로그래머가 이전에 겪어 본 것에 비해 엄격하고 꽤 다릅니다.
모든 것에 완전히 익숙해지려면 얼마간의 시간이 필요할 것입니다.

만약 빌림 체커와 다투고 있거나 인내가 바닥이 났다면, 언제든 [Rust 커뮤니티](community.html)에 도움을 청해 보세요.

<h3><a href="#when-is-rc-useful" name="when-is-rc-useful">
<code>Rc</code>가 유용한 때는 언제인가요?
</a></h3>

[`Rc`][Rc]는 Rust의 원자적이지 않은 참조 카운팅되는 포인터 타입으로, 이 질문은 공식 문서에서 커버하고 있습니다.
요약하자면 [`Rc`][Rc] 및 스레드 안전한 버전인 [`Arc`][Arc]는 공유된 소유권을 표현하는 데 유용하고, 아무도 접근하지 않을 때 연관된 메모리를 시스템이 자동으로 해제하도록 합니다.

<h3><a href="#how-do-i-return-a-closure-from-a-function" name="how-do-i-return-a-closure-from-a-function">
함수에서 어떻게 클로저를 반환하나요?
</a></h3>

함수에서 클로저를 반환하려면 그 클로저는 "이동 클로저"로, 클로저가 `move` 예약어로 선언되었어야만 합니다.
[《Rust 프로그래밍 언어》에서 설명](https://doc.rust-lang.org/book/closures.html#move-closures)하듯, 이 예약어는 클로저가 갈무리된 변수들을 부모 스택 프레임과 무관한 사본으로 가지게 합니다.
안 그랬다가는, 클로저를 반환하면 더 이상 올바르지 않은 변수에 접근할 수 있게 될테니 안전하지 않게 됩니다.
다르게 말하면 잠재적으로 잘못된 메모리를 읽을 수 있게 된단 얘기죠.
클로저는 또한 [`Box`][Box]로 감싸져 있어서 힙에 할당되어야만 합니다.
[《Rust 프로그래밍 언어》](https://doc.rust-lang.org/book/closures.html#returning-closures)에서 자세한 내용을 읽어 보세요.

<h3><a href="#what-are-deref-coercions" name="what-are-deref-coercions">
Deref 변환(coercion)이 무엇이고 어떻게 동작하나요?
</a></h3>

[Deref 변환](https://doc.rust-lang.org/book/deref-coercions.html)은 포인터에 대한 참조(예를 들어 `&Rc<T>`나 `&Box<T>`)를 자동으로 그 내용물의 참조(예를 들어 `&T`)로 변환하는 편리한 변환입니다.
Deref 변환은 Rust를 더 사용하기 편리하게 하려 존재하며, [`Deref`][Deref] 트레이트를 통해 구현됩니다.

Deref 구현은 구현하는 타입이 `deref` 메소드를 호출하여 대상 타입으로 변환될 수 있다는 걸 나타냅니다.
이 메소드는 호출되는 타입의 변경 불가능한 참조를 받아서 (같은 수명을 가지는) 대상 타입의 참조를 반환합니다.
`*` 전위 연산자는 `deref` 메소드의 축약입니다.

이들이 "변환"이라 불리는 건 [《Rust 프로그래밍 언어》](https://doc.rust-lang.org/stable/book/deref-coercions.html)에서 언급하듯 다음 규칙 때문입니다:

> 만약 타입 `U`가 `Deref<Target=T>`를 구현하면, `&U` 값은 자동으로 `&T`로 변환됩니다.

예를 들어 `&Rc<String>`이 있으면 이 규칙에 따라 `&String`으로 변환되며, 이는 다시 같은 방법으로 `&str`로 변환됩니다.
따라서 함수가 `&str` 인자를 받는다면 `&Rc<String>`을 그대로 넘겨 주고 모든 변환 과정이 `Deref` 트레이트로 자동으로 처리되도록 할 수 있습니다.

가장 흔한 deref 변환의 종류로는 이런 게 있습니다:

- `&Rc<T>`를 `&T`로
- `&Box<T>`를 `&T`로
- `&Arc<T>`를 `&T`로
- `&Vec<T>`를 `&[T]`로
- `&String`을 `&str`로

<h2 id="lifetimes">수명</h2>

<h3><a href="#why-lifetimes" name="why-lifetimes">
수명(lifetime)을 왜 쓰나요?
</a></h3>

수명은 메모리 안전성에 대한 Rust의 해답입니다.
Rust는 수명을 사용해서 쓰레기 수거(garbage collection)의 성능 비용 없이 메모리 안전성을 보장합니다.
수명 개념은 다양한 학술 연구에 기반해 있으며, 《[Rust 프로그래밍 언어](https://doc.rust-lang.org/stable/book/bibliography.html#type-system)》에서 참조를 확인할 수 있습니다.

<h3><a href="#why-is-the-lifetime-syntax-the-way-it-is" name="why-is-the-lifetime-syntax-the-way-it-is">
왜 수명 문법이 지금과 같은가요?
</a></h3>

`'a` 문법은 ML 계열의 프로그래밍 언어에서 따 왔는데, 여기서 `'a` 문법은 일반화된 타입 인자를 나타내는 데 사용됩니다.
Rust의 경우 수명 문법은 모호하지 않고, 눈에 띄어야 했으며 타입 선언에서 트레이트와 참조와 함께 쓰기 좋아야 했습니다.
다른 문법도 의논되었으나 이보다 확실히 더 좋은 문법이 제시되진 않았습니다.

<h3><a href="#how-do-i-return-a-borrow-to-something-i-created-from-a-function" name="how-do-i-return-a-borrow-to-something-i-created-from-a-function">
함수 안에서 만든 무언가를 빌려서 반환하려면 어떻게 하나요?
</a></h3>

빌린 물건이 함수보다 더 오래 살아 남는다는 걸 보장해야 합니다.
이는 다음과 같이 출력 수명을 입력 수명에 매어 놓아서 가능합니다:

```rust
type Pool = TypedArena<Thing>;

// (아래 수명 표시는 설명을 위해서 명시적으로 쓰인 것뿐이며,
// 뒤의 질문에서 설명하는 탈락(eilsion) 규칙에 따라
// 생략할 수 있습니다)
fn create_borrowed<'a>(pool: &'a Pool,
                       x: i32,
                       y: i32) -> &'a Thing {
    pool.alloc(Thing { x: x, y: y })
}
```

또는 [`String`][String] 같이 소유하는 타입들을 반환해서 참조를 아예 없애는 대안도 있습니다:

```rust
fn happy_birthday(name: &str, age: i64) -> String {
    format!("Hello {}! You're {} years old!", name, age)
}
```

이 접근은 더 간단하지만 종종 불필요한 할당이 일어납니다.

<h3><a href="#when-are-lifetimes-required-to-be-explicit" name="when-are-lifetimes-required-to-be-explicit">
왜 어떤 참조에는 <code>&amp;'a T</code>같이 수명이 있고 다른 참조에는 <code>&amp;T</code>같이 없는 건가요?
</a></h3>

사실 *모든* 참조 타입에는 수명이 있지만, 대부분의 경우 직접 쓸 필요가 없습니다.
규칙은 다음과 같습니다:

1. 함수 몸체에서는 수명을 명시적으로 쓸 필요가 전혀 없으며 항상 올바른 값이 추론될 것입니다.
2. 함수 *서명* (예를 들어 인자 타입이나 반환 타입) 안에서는 수명을 명시적으로 써야 할 수*도* 있습니다.
   여기에서는 ["수명 탈락(elision)"](https://doc.rust-lang.org/book/lifetimes.html#lifetime-elision)이라는 간단한 기본값이 적용되는데 이는 다음 세 규칙으로 구성되어 있습니다:
  - 인자에서 탈락된 각 수명은 서로 다른 인자가 됩니다.
  - 입력 수명이 하나 뿐이면, 그게 탈락되었든 아니든 그 함수의 반환 값에 있는 모든 탈락된 수명에 할당됩니다.
  - 입력 수명이 여럿 있지만 그 중 하나가 &self거나 &mut self라면, self의 수명이 모든 탈락된 출력 수명에 할당됩니다.
3. 마지막으로 `struct`와 `enum` 정의에서는 모든 수명이 명시적으로 선언되어야 합니다.

만약 이 규칙이 컴파일 오류를 일으킨다면, Rust 컴파일러는 일어난 오류를 가리키는 오류 메시지를 제공하면서 그 오류가 일어난 추론 단계에 따라 가능한 수정을 제시할 것입니다.

<h3><a href="#how-can-rust-guarantee-no-null-pointers" name="how-can-rust-guarantee-no-null-pointers">
Rust는 어떻게 "널 포인터가 없다"는 것과 "유령 포인터(dangling pointer)가 없다"는 것을 보장하나요?
</a></h3>

`&Foo`와 `&mut Foo` 타입의 값을 만드는 유일한 방법은 이미 존재하는 `Foo` 타입의 값을 참조가 가리키는 값으로 명시하는 것 뿐입니다.
참조는 주어진 코드 영역(즉, 참조의 수명) 안에서 원래 값을 "빌리며", 참조가 값을 빌린 동안에는 원래 값을 옮기거나 소멸시킬 수 없습니다.

<h3><a href="#how-do-i-express-the-absense-of-a-value-without-null" name="how-do-i-express-the-absense-of-a-value-without-null">
<code>null</code> 없이 값이 없다는 걸 어떻게 표현하나요?
</a></h3>

[`Option`][Option] 타입을 씁니다.
이 타입은 `Some(T)`이거나 `None`일 수 있는데, `Some(T)`은 `T` 타입의 값이 들어 있다는 걸 나타내고, `None`은 값이 없다는 걸 나타냅니다.

<h2 id="generics">일반화 (제너릭)</h2>

<h3><a href="#what-is-monomorphisation" name="what-is-monomorphisation">
"단형화"(monomorphisation)가 무엇인가요?
</a></h3>

단형화는 일반화된 함수(나 구조체)의 각 사용을, 함수를 호출하는 데 쓰인 인자 타입(이나 구조체의 용례)에 따라 각 인스턴스로 특수화합니다.

단형화 과정에서는 함수가 인스턴스화된 타입의 집합 하나 하나마다 일반화된 함수의 새 사본이 번역됩니다.
이는 C++가 사용하는 전략과 같습니다.
모든 함수 호출에 대해 특수화되고 정적으로 디스패치되는 빠른 코드가 만들어지지만, 서로 다른 타입에 대해 인스턴스된 함수는 "코드 팽창"을 일으켜 다른 번역 전략에 비해 더 큰 바이너리를 생성해낼 수 있다는 트레이드오프가 있습니다.

타입 인자 대신 [트레이트 객체](https://doc.rust-lang.org/book/trait-objects.html)를 받는 함수들은 단형화를 거치지 않습니다.
대신 트레이트 객체의 함수들은 실행 시간에 동적으로 디스패치됩니다.

<h3><a href="#whats-the-difference-between-a-function-and-a-closure-that-doesnt-capture" name="whats-the-difference-between-a-function-and-a-closure-that-doesnt-capture">
함수와 아무 변수도 갈무리하지 않는 클로저 사이에 어떤 차이가 있나요?
</a></h3>

함수와 클로저는 동작상으로는 동일하지만, 구현이 다르기 때문에 실행 시간에는 다른 표현을 가집니다.

함수는 언어에 내장된 원시 타입이며, 클로저는 근본적으로 [`Fn`][Fn], [`FnMut`][FnMut], 그리고 [`FnOnce`][FnOnce] 세 트레이트에 대한 문법 설탕입니다.
클로저를 만들 때 Rust 컴파일러는 자동으로 이들 세 트레이트 중 적절한 것들을 구현하고, 갈무리된 환경의 변수들을 멤버로 가지는 구조체를 만들어 함수로 불릴 수 있도록 합니다.
벌거벗은(bare) 함수는 환경을 갈무리할 수 없습니다.

이들 트레이트 사이의 큰 차이는 `self` 인자를 받는 방법에 있습니다.
[`Fn`][Fn]은 `&self`를, [`FnMut`][FnMut]은 `&mut self`를, 그리고 [`FnOnce`][FnOnce]는 `self`를 받습니다.

클로저가 환경에서 아무 변수도 갈무리하지 않는다 하여도, 다른 클로저와 마찬가지로 실행 시간에는 두 개의 포인터로 표현됩니다.

<h3><a href="#what-are-higher-kinded-types" name="what-are-higher-kinded-types">
상류(higher-kinded) 타입이 무엇인가요? 왜 제가 그걸 필요로 할 수 있나요? Rust에 상류 타입이 없는 이유는 무엇인가요?
<!-- 역주: higher-order 고차, higher-rank 상위, higher-kind 상류. -->
</a></h3>

상류 타입은 인자가 채워져 있지 않은 타입입니다.
[`Vec`][Vec], [`Result`][Result]나 [`HashMap`][HashMap] 같은 타입 생성자는 모두 상류 타입의 예로, 각각 특정한 타입을 가리키려면 `Vec<u32>`와 같이 추가로 타입 인자가 필요합니다.
상류 타입을 지원한다는 얘기는 "완전한" 타입들이 쓰일 수 있는 곳 어디에나, 이를테면 일반화된 함수 같은 곳에서 "불완전한" 타입도 쓸 수 있다는 뜻입니다.

[`i32`][i32], [`bool`][bool]이나 [`char`][char] 같은 완전한 타입은 종류(kind)가 `*`입니다(이 표기법은 타입 이론에서 유래합니다).
[`Vec<T>`][Vec] 같이 인자가 하나인 타입은 종류가 `* -> *`이며, 이는 이를테면 [`Vec<T>`][Vec]은 [`i32`][i32] 같은 완전한 타입을 받아서 `Vec<i32>` 같은 완전한 타입을 반환한다는 뜻입니다.
[`HashMap<K, V, S>`][HashMap] 같이 인자가 세 개인 타입은 종류가 `* -> * -> * -> *`이며, 세 개의 완전한 타입([`i32`][i32], [`String`][String], [`RandomState`][RandomState] 같이)을 받아서 새로운 완전한 타입 `HashMap<i32, String, RandomState>`를 만들어 냅니다.

위 예제에 덧붙여 타입 생성자는 *수명* 인자도 받을 수 있는데 여기서는 `Lt`라고 표기하겠습니다.
예를 들어 `slice::Iter`는 종류가 `Lt -> * -> *`인데, `Iter<'a, u32>` 처럼 인스턴스화되어야 하기 때문입니다.

상류 타입 지원이 없으면 특정한 종류의 일반화된 코드를 짜기 어렵습니다.
특히 흔히 수명 등으로 종종 매개변수화되는 반복자 같은 개념을 추상화하는 데 문제가 생깁니다.
이것 때문에 Rust의 컬렉션을 추상화하는 트레이트는 그간 만들 수 없었습니다.

또 다른 흔한 예제로는 함수자(functor)나 모나드(monad) 같은 개념으로, 둘 다 하나의 타입이 아닌 타입 생성자입니다.

Rust는 현재 상류 타입을 지원하지 않는데 저희가 하고자 하는 다른 개선점보다 우선순위가 낮았기 때문입니다.
이 설계는 대규모로 많은 곳들에 영향을 미치기 때문에 주의깊게 접근하고 싶은 것도 있습니다.
하지만 현재 상류 타입을 지원하지 않는 데는 다른 특별한 이유가 있는 건 아닙니다.

<h3><a href="#what-do-named-type-parameters-in-generic-types-mean" name="what-do-named-type-parameters-in-generic-types-mean">
일반화 타입에 <code>&lt;T=Foo&gt;</code> 같은 이름이 붙은 타입 인자는 무엇인가요?
</a></h3>

이들은 [연관 타입(associated type)](https://doc.rust-lang.org/stable/book/associated-types.html)이라 하며, `where` 절로 표현할 수 없는 트레이트 제약을 가능케 합니다.
예를 들어 일반화된 제약 `X: Bar<T=Foo>`는 "`X`는 `Bar` 트레이트를 구현해야 하고, 그 `Bar`의 구현에서 `X`는 `Bar`의 연관 타입 `T`로 `Foo`를 선택해야 한다"는 뜻입니다.
`where` 절만으로 표현할 수 없는 제약의 예제로는 `Box<Bar<T=Foo>>` 같은 트레이트 객체가 있습니다.

연관 타입은, 일반화 과정에서 타입의 무리가 존재해 한 타입 인자가 무리의 모든 타입을 결정하는 경우가 종종 있기 때문에 존재합니다.
예를 들어 그래프 트레이트는 그래프 자신을 가리키는 `Self` 타입을 가질 수 있고, 정점과 간선을 위한 연관 타입을 가질 수 있습니다.
각 그래프 타입은 연관 타입들을 유일하게 결정합니다.
연관 타입은 이러한 타입의 무리를 다루는 걸 훨씬 간명하게 만들고, 많은 경우 타입 추론에도 도움이 됩니다.

<h3><a href="#how-do-i-overload-operators" name="how-do-i-overload-operators">
연산자를 오버로드할 수 있나요? 어떤 게 가능하고 어떻게 하나요?
</a></h3>

대응되는 트레이트를 구현해서 여러 연산자들에 원하는 구현을 제공할 수 있습니다. `+`라면 [`Add`][Add], `*`라면 [`Mul`][Mul] 등등이 있습니다. 이런 식으로 씁니다:

```rust
use std::ops::Add;

struct Foo;

impl Add for Foo {
    type Output = Foo;
    fn add(self, rhs: Foo) -> Self::Output {
        println!("Adding!");
        self
    }
}
```

다음 연산자들을 오버로드할 수 있습니다:

| 연산자               | 트레이트                       |
|:---------------------|:-------------------------------|
| `+`                  | [`Add`][Add]                   |
| `+=`                 | [`AddAssign`][AddAssign]       |
| `이항 -`             | [`Sub`][Sub]                   |
| `-=`                 | [`SubAssign`][SubAssign]       |
| `*`                  | [`Mul`][Mul]                   |
| `*=`                 | [`MulAssign`][MulAssign]       |
| `/`                  | [`Div`][Div]                   |
| `/=`                 | [`DivAssign`][DivAssign]       |
| `단항 -`             | [`Neg`][Neg]                   |
| `%`                  | [`Rem`][Rem]                   |
| `%=`                 | [`RemAssign`][RemAssign]       |
| `&`                  | [`BitAnd`][BitAnd]             |
| <code>&#124;</code>  | [`BitOr`][BitOr]               |
| <code>&#124;</code>= | [`BitOrAssign`][BitOrAssign]   |
| `^`                  | [`BitXor`][BitXor]             |
| `^=`                 | [`BitXorAssign`][BitXorAssign] |
| `!`                  | [`Not`][Not]                   |
| `<<`                 | [`Shl`][Shl]                   |
| `<<=`                | [`ShlAssign`][ShlAssign]       |
| `>>`                 | [`Shr`][Shr]                   |
| `>>=`                | [`ShrAssign`][ShrAssign]       |
| `*`                  | [`Deref`][Deref]               |
| `mut *`              | [`DerefMut`][DerefMut]         |
| `[]`                 | [`Index`][Index]               |
| `mut []`             | [`IndexMut`][IndexMut]         |

<h3><a href="#why-the-split-between-eq-partialeq-and-ord-partialord" name="why-the-split-between-eq-partialeq-and-ord-partialord">
<code>Eq</code>/<code>PartialEq</code>와 <code>Ord</code>/<code>PartialOrd</code>가 나뉜 이유는 무엇인가요?
</a></h3>

Rust의 몇몇 타입들은 그 값들이 부분적으로만 순서가 있거나, 부분적으로만 등치 관계입니다.
부분 순서(partial ordering)란 주어진 타입의 어떤 값들이 서로 작지도 크지도 않을 수 있다는 뜻입니다.
부분 등치(partial equality)란 주어진 타입의 어떤 값들이 자기 자신과 같지 않을 수 있다는 뜻입니다.

부동 소숫점 타입들([`f32`][f32]와 [`f64`][f64])이 좋은 예제입니다.
모든 부동 소숫점 타입에는 `NaN`("not a number", 즉 "숫자가 아님"을 뜻함) 값이 있습니다.
`NaN`은 자기 자신과 같지도 않고(`NaN == NaN`은 거짓입니다), 다른 부동 소숫점 값보다 작지도 크지도 않습니다.
따라서 [`f32`][f32]와 [`f64`][f64]는 [`PartialOrd`][PartialOrd]와 [`PartialEq`][PartialEq]를 구현하지만 [`Ord`][Ord]와 [`Eq`][Eq]는 구현하지 않습니다.

[부동 소숫점에 대한 이전 질문](#why-cant-i-compare-floats)에서 설명되었듯, 이러한 구분은 몇몇 컬렉션이 올바른 결과를 내는데 전순서/전등치에 의존하기 때문에 중요합니다.

<h2 id="input-output">입출력</h2>

<h3><a href="#how-do-i-read-a-file-into-a-string" name="how-do-i-read-a-file-into-a-string">
파일을 <code>String</code>로 읽으려면 어떻게 하나요?
</a></h3>

[`std::io`][std-io]의 [`Read`][Read] 트레이트에 있는 [`read_to_string()`][read__read_to_string] 메소드를 씁니다.

```rust
use std::io::Read;
use std::fs::File;

fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut f = try!(File::open(path));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));  // `s`는 "foo.txt"의 내용을 담습니다
    Ok(s)
}

fn main() {
    match read_file("foo.txt") {
        Ok(_) => println!("Got file contents!"),
        Err(err) => println!("Getting file contents failed with error: {}", err)
    };
}
```

<h3><a href="#how-do-i-read-file-input-efficiently" name="how-do-i-read-file-input-efficiently">
파일을 효율적으로 읽으려면 어떻게 하나요?
</a></h3>

[`File`][File] 타입은 [`Read`][Read] 트레이트를 구현하며, 여기에는 데이터를 읽고 쓰는 다양한 함수들이 존재하는데 [`read()`][read__read], [`read_to_end()`][read__read_to_end], [`bytes()`][read__bytes], [`chars()`][read__chars], 그리고 [`take()`][read__take] 등이 포함됩니다.
각 함수들은 주어진 파일로부터 일정한 만큼의 입력을 읽어 들입니다.
[`read()`][read__read]는 기반 시스템이 한 번의 호출에서 제공하는 한 최대한 많은 입력을 읽어 들입니다.
[`read_to_end()`][read__read_to_end]는 전체 버퍼를 벡터로 읽어 들이고, 필요에 따라 공간을 할당합니다.
[`bytes()`][read__bytes]와 [`chars()`][read__chars]는 파일 안의 바이트와 문자에 대해 반복을 수행합니다.
마지막으로 [`take()`][read__take]로 파일로부터 최대 지정한 만큼의 바이트를 읽을 수 있습니다.
이들 함수들로 필요한 어떤 데이터라도 효율적으로 읽어 들일 수 있습니다.

버퍼를 통해서 읽고 싶다면 [`BufReader`][BufReader] 구조체를 사용하세요.
이 구조체는 읽는 과정에서 시스템 호출의 숫자를 줄이는 데 도움이 됩니다.

<h3><a href="#how-do-i-do-asynchronous-input-output-in-rust" name="how-do-i-do-asynchronous-input-output-in-rust">
Rust에서 어떻게 비동기 입출력을 하나요?
</a></h3>

Rust에서 비동기 입출력을 제공하는 라이브러리로는 [mioco](https://github.com/dpc/mioco), [coio-rs](https://github.com/zonyitoo/coio-rs), 그리고 [rotor](https://github.com/tailhook/rotor) 등이 있습니다.

<h3><a href="#how-do-i-get-command-line-arguments" name="how-do-i-get-command-line-arguments">
Rust에서 명령행 인자를 받는 방법은 무엇인가요?
</a></h3>

가장 쉬운 방법은 [`Args`][Args]를 써서 입력 인자들에 대한 반복자를 받는 것입니다.

더 강력한 무언가를 찾고 있다면 [crates.io에 여러 선택이 있습니다](https://crates.io/keywords/argument).

<h2 id="error-handling">오류 처리</h2>

<h3><a href="#why-doesnt-rust-have-exceptions" name="why-doesnt-rust-have-exceptions">
Rust에는 왜 예외(exception)가 없나요?
</a></h3>

예외는 제어 흐름을 이해하기 복잡하게 만들고, 타입 시스템을 넘어서는 유효성/무효성을 표현하며, (Rust의 주요 촛점인) 멀티스레딩된 코드와 잘 상호 작용하지 않습니다.

Rust는 오류 처리에 타입 기반의 접근을 선호하며, [《Rust 프로그래밍 언어》에서 길게 다루고 있습니다](https://doc.rust-lang.org/stable/book/error-handling.html).
이는 Rust의 제어 흐름, 동시성 및 여타 다른 것들에 더 잘 맞아 들어 갑니다.

<h3><a href="#whats-the-deal-with-unwrap" name="whats-the-deal-with-unwrap">
여기 저기 보이는 <code>unwrap()</code>를 어떻게 할 수 없나요?
</a></h3>

`unwrap()`은 [`Option`][Option]이나 [`Result`][Result] 안에 있는 값을 뽑아 내고 아무 값도 없으면 패닉을 일으키는 함수입니다.

`unwrap()`이 잘못된 사용자 입력 같이 예상할 수 있는 오류들을 기본으로 다루는 방법이 되어서는 안 됩니다.
현업 코드에서 이는 값이 비어 있지 않으며 만에 하나 비어 있다면 프로그램이 깨지는 단언(assertion)처럼 취급되어야 합니다.

또한 `unwrap()`은 아직 오류를 처리하고 싶지 않은 빠른 프로토타입이나, 오류 처리가 주요 논점을 흐릴 수 있는 블로그 글에서도 유용합니다.

<h3><a href="#why-do-i-get-errors-with-try" name="why-do-i-get-errors-with-try">
<code>try!</code> 매크로를 쓰는 예제 코드를 실행하려고 할 때 오류가 나는 이유는 뭔가요?
</a></h3>

아마도 함수의 반환 타입에 문제가 있을 겁니다.
[`try!`][TryMacro] 매크로는 [`Result`][Result]에서 값을 뽑아 내거나, [`Result`][Result]가 들고 있는 오류를 먼저 반환합니다.
즉 [`try`][TryMacro]는 [`Result`][Result]를 반환하는 함수에서만 동작하며, 이 때 `Err`로 만들어지는 타입은 `From::from(err)`을 구현해야 합니다.
특히 이는 `main` 함수에서는 [`try!`][TryMacro] 매크로를 쓸 수 없다는 뜻이기도 합니다.

<h3><a href="#error-handling-without-result" name="error-handling-without-result">
모든 곳에 <code>Result</code>를 쓰는 것 말고 더 쉽게 오류를 처리할 방법이 없나요?
</a></h3>

다른 사람의 코드에 있는 [`Result`][Result]를 처리하지 않는 방법을 원한다면 항상 [`unwrap()`][unwrap]를 쓸 수 있지만, 아마도 원하는 게 아닐 겁니다.
[`Result`][Result]는 어떤 계산이 성공적으로 끝나거나 끝나지 않을 수 있다는 표시입니다.
이러한 실패를 처리하도록 요구하는 건 Rust가 튼튼한 코드를 권장하는 방법 중 하나입니다.
Rust는 실패를 더 편리하게 처리할 수 있도록 [`try!` 매크로][TryMacro] 같은 도구를 제공합니다.

정말로 오류를 처리하고 싶지 않다면 [`unwrap()`][unwrap]를 쓰세요.
하지만 이렇게 하면 실패시 코드가 패닉을 일으키고, 보통 이는 프로세스를 종료시킨다는 점을 유의하시길 바랍니다.

<h2 id="concurrency">동시성</h2>

<h3><a href="#can-i-use-static-values-across-threads-without-an-unsafe-block" name="can-i-use-static-values-across-threads-without-an-unsafe-block">
<code>unsafe</code> 블록 없이 여러 스레드에서 정적인 값을 사용할 수는 없을까요?
</a></h3>

변경은 동기화가 된다면 안전합니다.
([lazy-static](https://crates.io/crates/lazy_static/) 크레이트로 지연되어 초기화된) 정적인 [`Mutex`][Mutex]를 변경하는 데는 `unsafe` 블록이 필요 없으며, 정적인 [`AtomicUsize`][AtomicUsize]를 변경하는 데도 필요 없습니다(이건 lazy_static 없이 초기화할 수 있습니다).

좀 더 일반적으로, 타입이 [`Sync`][Sync]를 구현하고 [`Drop`][Drop]을 구현하지 않는다면 [`static`에서 사용할 수 있습니다](https://doc.rust-lang.org/book/const-and-static.html#static).

<h2 id="macros">매크로</h2>

<h3><a href="#can-i-write-a-macro-to-generate-identifiers" name="can-i-write-a-macro-to-generate-identifiers">
식별자를 생성하는 매크로를 짤 수 있나요?
</a></h3>

현재는 안 됩니다.
Rust 매크로는 ["위생적인(hygienic) 매크로"](https://en.wikipedia.org/wiki/Hygienic_macro)로, 예상치 못 하게 다른 식별와 겹치는 식별자를 갈무리하거나 만드는 걸 피합니다.
이런 매크로의 능력은 C 전처리기에서 흔히 쓰이는 스타일의 매크로와는 상당히 다릅니다.
매크로 호출은 명시적으로 지원되는 곳, 즉 아이템, 메소드 선언, 문장, 수식 및 패턴에서만 나타날 수 있습니다.
여기서 "메소드 선언"이란 메소드를 집어 넣을 수 있는 빈 공간을 말합니다.
이들은 부분적인 메소드 선언을 채우는 데 쓰일 수는 없습니다.
같은 논리로, 이들은 부분적인 변수 선언을 채우는 데 쓰일 수 없습니다.

<h2 id="debugging">디버깅 및 도구</h2>

<h3><a href="#how-do-i-debug-rust-programs" name="how-do-i-debug-rust-programs">
Rust 프로그램은 어떻게 디버깅하나요?
</a></h3>

Rust 프로그램은 C나 C++와 같이 [gdb](https://sourceware.org/gdb/current/onlinedocs/gdb/)나 [lldb](http://lldb.llvm.org/tutorial.html)로 디버깅할 수 있습니다.
사실은 모든 Rust 설치에는 (플랫폼 지원에 따라) rust-gdb나 rust-lldb 둘 중 하나가 함께 들어 있습니다.
이들은 gdb와 lldb에 Rust 값을 보기 좋게 출력해 주도록 감싼 것입니다.

<h3><a href="#how-do-i-locate-a-panic" name="how-do-i-locate-a-panic">
<code>rustc</code>가 표준 라이브러리 코드에서 패닉(panic)이 일어났다고 하는데, 제 코드의 실수를 어떻게 찾을 수 있을까요?
</a></h3>

이 오류는 보통 사용자 코드에서 `None`이나 `Err`을 [`unwrap()`][unwrap]해서 일어납니다.
`RUST_BACKTRACE=1` 환경 변수를 설정해서 스택 추적(backtrace)을 켜는 게 더 많은 정보를 얻는데 도움이 됩니다.
디버그 모드로 컴파일하거나(`cargo build`의 기본값), 함께 들어 있는 `rust-gdb`나 `rust-lldb` 같은 디버거를 쓰는 것도 도움이 됩니다.

<h3><a href="#what-ide-should-i-use" name="what-ide-should-i-use">
무슨 IDE를 써야 하나요?
</a></h3>

Rsut 개발 환경에는 여러 선택이 있으며 자세한 사항은 공식 [IDE 지원 페이지](https://forge.rust-lang.org/ides.html)에 설명되어 있습니다.

<h3><a href="#wheres-rustfmt" name="wheres-rustfmt">
<code>gofmt</code>은 멋져요. <code>rustfmt</code> 같은 건 없나요?
</a></h3>

`rustfmt`은 [여기](https://github.com/rust-lang-nursery/rustfmt) 있고, Rust 코드를 가능한한 읽기 쉽고 예측 가능하게 만들도록 활발히 개발되고 있습니다.

<h2 id="low-level">저수준</h2>

<h3><a href="#how-do-i-memcpy-bytes" name="how-do-i-memcpy-bytes">
<code>memcpy</code>같이 바이트를 복사하려면 어떻게 하나요?
</a></h3>

이미 존재하는 슬라이스를 안전하게 복제하려면 [`clone_from_slice`][clone_from_slice]를 쓸 수 있습니다.

서로 겹칠 수 있는 바이트들을 복사하려면 [`copy`][copy]를 쓰세요.
서로 겹칠 수 없는 바이트들을 복사하려면 [`copy_nonoverlapping`][copy_nonoverlapping]을 쓰세요.
이들 함수들은 언어의 안전성 보장을 깨뜨리는 데 쓰일 있기 때문에 `unsafe`입니다.
사용에 주의를 기울이세요.

<h3><a href="#does-rust-work-without-the-standard-library" name="does-rust-work-without-the-standard-library">
표준 라이브러리 없이 Rust를 사용하는 건 할 만한가요?
</a></h3>

물론입니다.
Rust 프로그램은 `#![no_std]` 속성으로 표준 라이브러리를 불러 들이지 않도록 설정할 수 있습니다.
이 속성이 설정되어도 플랫폼 독립적인 원시 타입만 제공되는 Rust 코어 라이브러리는 여전히 사용할 수 있습니다.
따라서 여기에는 I/O, 동시성, 힙(heap) 할당 같은 건 포함되지 않습니다.

<h3><a href="#can-i-write-an-operating-system-in-rust" name="can-i-write-an-operating-system-in-rust">
Rust로 운영체제를 작성할 수 있나요?
</a></h3>

네! 사실 [정확히 그걸 하는 프로젝트가 여럿 있습니다](http://wiki.osdev.org/Rust).

<h3><a href="#how-can-i-write-endian-independent-values" name="how-can-i-write-endian-independent-values">
<code>i32</code>나 <code>f64</code> 같은 수치형을 빅 엔디안이나 리틀 엔디안 형식으로 파일 및 다른 바이트 스트림에 읽고 쓰려면 어떻게 하나요?
</a></h3>

[byteorder 크레이트](http://burntsushi.net/rustdoc/byteorder/)가 정확히 그걸 하는 유틸리티이니 살펴 보세요.

<h3><a href="#does-rust-guarantee-data-layout" name="does-rust-guarantee-data-layout">
Rust가 메모리 상에 값이 어떻게 배치될 지가 고정되어 있나요?
</a></h3>

기본적으로는 아닙니다.
일반적으로 `enum`과 `struct`의 배치는 정의되지 않습니다.
따라서 컴파일러가 패딩을 구분값(discriminant)을 넣는데 재사용하거나, 중첩된 `enum`들의 변종(variant)들을 압축하거나, 패딩을 없애기 위해 필드를 재배치하는 등의 잠재적인 최적화를 할 수 있게 됩니다.
데이터를 들고 있지 않은 ("C와 비슷한") `enum`은 정의된 표현을 가지도록 할 수 있습니다.
이러한 `enum`은 데이터를 들고 있지 않은 이름들만의 단순 목록이므로 쉽게 구분할 수 있습니다:

```rust
enum CLike {
    A,
    B = 32,
    C = 34,
    D
}
```

이러한 `enum`에 `#[repr(C)]` 속성을 적용하면 대응되는 C 코드가 가질 표현과 같은 표현이 되도록 할 수 있습니다.
따라서 FFI 코드에서 C `enum`이 쓰일 대부분의 상황에서 Rust `enum`을 쓸 수 있습니다.
마찬가지로 `struct`에도 이 속성을 적용하면 C `struct`가 가질 배치와 같은 배치가 되도록 할 수 있습니다.

<h2 id="cross-platform">다중 플랫폼</h2>

<!--
### How do I build a Windows binary that doesn't display the console window?

TODO: Write this answer.
-->

<!--
### How do I make the console-less binary not crash on panic!?

TODO: Write this answer.
-->

<h3><a href="#how-do-i-express-platform-specific-behavior" name="how-do-i-express-platform-specific-behavior">
Rust에서 플랫폼 의존적인 동작을 표현하는 일반적인 방법은 무엇인가요?
</a></h3>

플랫폼 의존적인 동작은 `target_os`나 `target_family`, `target_endian` 같은 [조건부 컴파일 속성](https://doc.rust-lang.org/reference.html#conditional-compilation)으로 표현할 수 있습니다.

<h3><a href="#can-rust-be-used-for-android-ios-programs" name="can-rust-be-used-for-android-ios-programs">
Rust를 안드로이드 및 iOS 프로그래밍에 쓸 수 있나요?
</a></h3>

네 할 수 있습니다!
이미 Rust를 [안드로이드](https://github.com/tomaka/android-rs-glue)와 [iOS](https://www.bignerdranch.com/blog/building-an-ios-app-in-rust-part-1/)에서 사용하는 예제가 있습니다.
설정에 조금 시간이 들긴 하지만 Rust는 두 플랫폼에서 모두 잘 동작합니다.

<h3><a href="#can-i-run-my-rust-program-in-a-web-browser" name="can-i-run-my-rust-program-in-a-web-browser">
제 Rust 프로그램을 웹 브라우저에서 실행할 수 있나요?
</a></h3>

아직은 아니지만, [Emscripten](https://kripken.github.io/emscripten-site/)을 써서 Rust를 웹으로 컴파일하려는 노력이 현재 진행 중입니다.

<h3><a href="#how-do-i-cross-compile-rust" name="how-do-i-cross-compile-rust">
Rust에서 크로스 컴파일은 어떻게 하나요?
</a></h3>

Rust에서는 크로스 컴파일을 할 수 있지만 설치 과정이 [좀 필요합니다](https://github.com/japaric/rust-cross/blob/master/README.md).
모든 Rust 컴파일러는 크로스 컴파일러지만 라이브러리는 해당 플랫폼 용으로 크로스 컴파일될 필요가 있습니다.

Rust는 지원되는 플랫폼에 대해서 [표준 라이브러리의 사본](https://static.rust-lang.org/dist/)을 배포하고 있으며, 배포판 페이지의 각 빌드 디렉토리에 있는 `rust-std-*` 파일들로 들어 있습니다만, 아직 이걸 자동으로 설치하는 방법은 없습니다.

<h2 id="modules-and-crates">모듈 및 크레이트</h2>

<h3><a href="#what-is-the-relationship-between-a-module-and-a-crate" name="what-is-the-relationship-between-a-module-and-a-crate">
모듈과 크레이트 사이에 어떤 관계가 있나요?
</a></h3>

- 크레이트는 컴파일 단위로, Rust 컴파일러가 다룰 수 있는 가장 작은 규모의 코드입니다.
- 모듈은 크레이트 안에 있는 코드 구조의 (중첩될 수도 있는) 단위입니다.
- 크레이트에는 암묵적이고 이름이 없는 최상위 모듈이 포함됩니다.
- 재귀 정의는 여러 모듈에 걸쳐 있을 수 있지만 여러 크레이트에는 걸칠 수 없습니다.

<h3><a href="#why-cant-the-rust-compiler-find-a-library-im-using" name="why-cant-the-rust-compiler-find-a-library-im-using">
Rust 컴파일러가 제가 <code>use</code>한 라이브러리를 찾지 못 하는 이유는 뭔가요?
</a></h3>

여러 가능한 답이 있습니다만, 흔한 실수로는 `use` 속성이 크레이트 최상단에 상대적이라는 걸 깨닫지 못 하는 게 있습니다.
선언을 재작성해서 경로가 프로젝트의 최상단 파일에 선언되었을 때랑 똑같은 경로가 되도록 한 뒤 문제가 해결되는지를 확인해 보세요.

또한 `self`와 `super`를 써서 `use` 경로를 각각 현재 모듈이나 상위 모듈에 상대적으로 만들 수 있습니다.

라이브러리를 `use`하는 데 대한 완전한 정보에 대해선 《Rust 프로그래밍 언어》의 ["크레이트와 모듈"](https://doc.rust-lang.org/stable/book/crates-and-modules.html) 장을 읽으세요.

<h3><a href="#why-do-i-have-to-declare-modules-with-mod" name="why-do-i-have-to-declare-modules-with-mod">
왜 모듈 파일을 정의하기 위해 크레이트 최상위에 <code>mod</code>를 넣어야 하나요? 그냥 <code>use</code>로 지정하면 안 되나요?
</a></h3>

There are two ways to declare modules in Rust, inline or in another file. Here is an example of each:

```rust
// In main.rs
mod hello {
    pub fn f() {
        println!("hello!");
    }
}

fn main() {
    hello::f();
}
```

```rust
// In main.rs
mod hello;

fn main() {
    hello::f();
}

// In hello.rs
pub fn f() {
    println!("hello!");
}
```

In the first example, the module is defined in the same file it's used. In the second example, the module declaration in the main file tells the compiler to look for either `hello.rs` or `hello/mod.rs`, and to load that file.

Note the difference between `mod` and `use`: `mod` declares that a module exists, whereas `use` references a module declared elsewhere, bringing its contents into scope within the current module.

<h3><a href="#how-do-i-configure-cargo-to-use-a-proxy" name="how-do-i-configure-cargo-to-use-a-proxy">
Cargo가 프록시를 사용하도록 설정하려면 어떻게 하나요?
</a></h3>

Cargo [환경설정 문서](http://doc.crates.io/config.html)에 설명되어 있듯, 환경설정 파일의 `[http]` 아래에 "proxy" 변수를 설정해서 Cargo가 프록시를 쓰도록 할 수 있습니다.

<h3><a href="#why-cant-the-compile-find-method-implementations" name="why-cant-the-compile-find-method-implementations">
이미 크레이트를 <code>use</code>했는데도 왜 컴파일러가 메소드 구현을 찾지 못 하는 걸까요?
</a></h3>

For methods defined on a trait, you have to explicitly import the trait declaration. This means it's not enough to import a module where a struct implements the trait, you must also import the trait itself.

<h3><a href="#why-cant-the-compiler-infer-use-statements" name="why-cant-the-compiler-infer-use-statements">
왜 컴파일러가 <code>use</code> 선언을 자동으로 추론하지 못 하나요?
</a></h3>

It probably could, but you also don't want it to. While in many cases it is likely that the compiler could determine the correct module to import by simply looking for where a given identifier is defined, this may not be the case in general. Any decision rule in `rustc` for choosing between competing options would likely cause surprise and confusion in some cases, and Rust prefers to be explicit about where names are coming from.

For example, the compiler could say that in the case of competing identifier definitions the definition from the earliest imported module is chosen. So if both module `foo` and module `bar` define the identifier `baz`, but `foo` is the first registered module, the compiler would insert `use foo::baz;`.

```rust
mod foo;
mod bar;

// use foo::baz  // to be inserted by the compiler.

fn main() {
  baz();
}
```

If you know this is going to happen, perhaps it saves a small number of keystrokes, but it also greatly increases the possibility for surprising error messages when you actually meant for `baz()` to be `bar::baz()`, and it decreases the readability of the code by making the meaning of a function call dependent on module declaration. These are not tradeoffs we are willing to make.

However, in the future, an IDE could help manage declarations, which gives you the best of both worlds: machine assistance for pulling in names, but explicit declarations about where those names are coming from.

<!--
### How do I package and archive crates from [https://crates.io](https://crates.io)?

TODO: Write this answer.
-->

<h3><a href="#how-do-i-do-dynamic-rust-library-loading" name="how-do-i-do-dynamic-rust-library-loading">
Rust 라이브러리를 어떻게 동적으로 읽어 들이나요?
</a></h3>

여러 플랫폼에서 동적 링크를 제공하는 [libloading](https://crates.io/crates/libloading)으로 동적 라이브러리를 불러 들이세요.

<h3><a href="#why-doesnt-crates-io-have-namespaces" name="why-doesnt-crates-io-have-namespaces">
왜 crates.io에는 이름 공간이 없나요?
</a></h3>

[https://crates.io](https://crates.io)의 설계에 대한 [공식 설명](https://internals.rust-lang.org/t/crates-io-package-policies/1041)을 전재하자면:

> crates.io의 첫 한 달동안 여러 분들께서 저희들에게 [패키지 이름 공간](https://github.com/rust-lang/crates.io/issues/58)을 도입할 가능성에 대해 질문을 해 왔습니다.<br><br>
>
> 패키지 이름 공간은 여러 저자들이 하나의 일반적인 이름을 쓸 수 있게 하지만, 패키지가 Rust 코드에서 참조되는 방법과 패키지에 대한 사람들 사이의 소통에 복잡도를 더합니다.
> 얼핏 보면 여러 저자들이 `http` 같은 이름을 쓸 수 있을 것 같지만, 실은 이는 사람들이 이 패키지들을 `wycats의 http`나 `reem의 http` 같은 식으로 가리켜야 한다는 뜻일 뿐입니다.
> `wycats-http`나 `reem-http` 같은 패키지 이름에 비해 별달리 장점이 없지요.<br><br>
>
> 이름 공간이 없는 패키지 생태계들을 살펴 본 결과 사람들이 ("tenderlove의 libxml2" 대신 `nokogiri` 같이) 더 창의적인 이름들을 쓰는 편이라는 걸 알게 되었습니다.
> 아무런 계층도 담고 있지 않는다는 것도 있고 해서, 이런 창의적인 이름들은 짧고 기억하기 쉬운 편입니다.
> 이들은 패키지에 대해 간결하고 모호함 없이 소통하기 쉽게 만들고, 신나는 브랜드를 만들지요.
> 또한 우리는 NPM이나 RubyGems 같이 수만개의 패키지가 있는 성공적인 생태계에서 커뮤니티가 하나의 이름 공간만으로 번창하는 것 또한 보아 왔습니다.<br><br>
>
> 요약하자면 우리는 Piston이 단순히 `piston` 대신 `bvssvni/game-engine` 같은 이름을 선택했다고 해서 (그래서 다른 사용자가 `wycats/game-engine`을 고를 수 있게 된다 해서) Cargo 생태계가 더 좋아질 거라고 생각하지 않습니다.<br><br>
>
> 이름 공간은 그 자체로 여러 방면에서 복잡하고, 나중에 필요해진다면 호환되는 방법으로 추가할 수 있다는 점에서, 우리는 하나의 공유된 이름 공간을 계속 쓰려고 합니다.

<h2 id="libraries">라이브러리</h2>

<h3><a href="#how-can-i-make-an-http-request" name="how-can-i-make-an-http-request">
HTTP 요청을 어떻게 보내나요?
</a></h3>

표준 라이브러리에는 HTTP 구현이 포함되어 있지 않으므로 외부 크레이트를 사용해야 합니다.
[Hyper](https://github.com/hyperium/hyper)가 가장 널리 쓰이지만 [다른 라이브러리들도 여럿 있습니다](https://crates.io/keywords/http).

<h3><a href="#how-can-i-write-a-gui-application" name="how-can-i-write-a-gui-application">
Rust로 GUI 애플리케이션을 작성하려면 어떻게 해야 하나요?
</a></h3>

Rust로 GUI 애플리케이션을 만드는 방법은 여럿 있습니다. [GUI 프레임워크들의 목록](https://github.com/kud1ing/awesome-rust#gui)을 참고하세요.

<h3><a href="#how-can-i-parse-json-xml" name="how-can-i-parse-json-xml">
JSON/XML을 파싱하는 방법은 무엇인가요?
</a></h3>

[Serde](https://github.com/serde-rs/serde)는 Rust 데이터를 다른 여러 포맷으로 직렬화(serialize)하고 역직렬화(deserialize)하는데 추천하는 라이브러리입니다.

<h3><a href="#is-there-a-standard-2d-vector-crate" name="is-there-a-standard-2d-vector-crate">
표준 2차원/3차원/... 벡터 및 도형 크레이트가 있나요?
</a></h3>

아직요! 만들어 보실래요?

<h3><a href="#how-do-i-write-an-opengl-app" name="how-do-i-write-an-opengl-app">
Rust로 OpenGL 앱을 작성하려면 어떻게 해야 하죠?
</a></h3>

[Glium](https://github.com/tomaka/glium)는 Rust에서 OpenGL 프로그래밍에 쓰이는 주요 라이브러리입니다.
[GLFW](https://github.com/bjz/glfw-rs) 또한 괜찮은 대안입니다.

<h3><a href="#can-i-write-a-video-game-in-rust" name="can-i-write-a-video-game-in-rust">
Rust로 비디오 게임을 만들 수 있나요?
</a></h3>

가능합니다!
Rust로 만들어진 주요 게임 프로그래밍 라이브러리는 [Piston](http://www.piston.rs/)이 있으며, [Rust 게임 프로그래밍을 위한 서브레딧](https://www.reddit.com/r/rust_gamedev/)과 IRC 채널([Mozilla IRC](https://wiki.mozilla.org/IRC)의 `#rust-gamedev` 채널)도 있습니다.

<h2 id="design-patterns">디자인 패턴</h2>

<h3><a href="#is-rust-object-oriented" name="is-rust-object-oriented">
Rust는 객체 지향적(object-oriented)인가요?
</a></h3>

Rust는 여러 패러다임을 지원합니다.
객체 지향 언어에서 할 수 있는 많은 것들은 Rust에서도 할 수 있지만, 전부 가능한 건 아니고, 여러분에게 익숙한 추상화를 사용하지 않을 수도 있습니다.

<h3><a href="#how-do-i-map-object-oriented-concepts-to-rust" name="how-do-i-map-object-oriented-concepts-to-rust">
객체 지향적인 개념을 Rust에 어떻게 대응시키죠?
</a></h3>

상황에 따라 다릅니다.
[다중 상속](https://www.reddit.com/r/rust/comments/2sryuw/ideaquestion_about_multiple_inheritence/)과 같은 객체 지향 개념을 Rust로 옮기는 방법은 _여럿_ 있습니다만, Rust는 객체 지향이 아니기에 객체 지향 언어들과는 상당히 다르게 보일 수 있습니다.

<h3><a href="#how-do-i-configure-a-struct-with-optional-parameters" name="how-do-i-configure-a-struct-with-optional-parameters">
선택 인자가 있는 구조체를 설정하는 인터페이스를 어떻게 만들어야 할까요?
</a></h3>

The easiest way is to use the [`Option`][Option] type in whatever function you're using to construct instances of the struct (usually `new()`). Another way is to use the [builder pattern](https://aturon.github.io/ownership/builders.html), where only certain functions instantiating member variables must be called before the construction of the built type.

<h3><a href="#how-do-i-do-global-variables" name="how-do-i-do-global-variables">
Rust에서 전역 변수를 쓰려면 어떻게 하죠?
</a></h3>

Globals in Rust can be done using `const` declarations for compile-time computed global constants, while `static` can be used for mutable globals. Note that modifying a `static mut` variable requires the use of `unsafe`, as it allows for data races, one of the things guaranteed not to happen in safe Rust. One important distinction between `const` and `static` values is that you can take references to `static` values, but not references to `const` values, which don't have a specified memory location. For more information on `const` vs. `static`, read [the Rust book](https://doc.rust-lang.org/book/const-and-static.html).

<h3><a href="#how-can-i-set-compile-time-constants-that-are-defined-procedurally" name="how-can-i-set-compile-time-constants-that-are-defined-procedurally">
절차적으로 정의되는 컴파일 시간 상수는 어떻게 설정하나요?
</a></h3>

Rust currently has limited support for compile time constants. You can define primitives using `const` declarations (similar to `static`, but immutable and without a specified location in memory) as well as define `const` functions and inherent methods.

To define procedural constants that can't be defined via these mechanisms, use the [`lazy-static`](https://github.com/rust-lang-nursery/lazy-static.rs) crate, which emulates compile-time evaluation by automatically evaluating the constant at first use.

<h3><a href="#can-i-run-code-before-main" name="can-i-run-code-before-main">
`main` 이전에 실행되는 초기화 코드를 만들 수 있나요?
</a></h3>

Rust has no concept of "life before `main`". The closest you'll see can be done through the [`lazy-static`](https://github.com/Kimundi/lazy-static.rs) crate, which simulates a "before main" by lazily initializing static variables at their first usage.

<!--

This answer needs significant work. Let's revise after the initial posting. --aturon

<h3><a href="#why-doesnt-rust-have-inheritance" name="why-doesnt-rust-have-inheritance">
Why doesn't Rust have inheritance?
</a></h3>

There are two meanings for the word "inheritance": _subtyping_, and _interface sharing_. Both purposes are already handled by traits.

For the first, subtyping exists for polymorphism, which traits already provide.

For the second, interface sharing is handled via trait methods, which define a collection of related functions that must be implemented for any implementation of the trait.

Rust has consistently worked to avoid having features with overlapping purposes, preferring to keep features orthogonal. For this reason, and given that the two major purposes are already handled by traits, Rust has opted not to include inheritance.

-->

<h3><a href="#does-rust-allow-non-constant-expression-values-for-globals" name="does-rust-allow-non-constant-expression-values-for-globals">
Rust에서 상수 수식이 아닌 값을 전역에 넣을 수 있나요?
</a></h3>

No. Globals cannot have a non-constant-expression constructor and cannot have a destructor at all. Static constructors are undesirable because portably ensuring a static initialization order is difficult. Life before main is often considered a misfeature, so Rust does not allow it.

See the [C++ FQA](http://yosefk.com/c++fqa/ctors.html#fqa-10.12) about the "static initialization order fiasco", and [Eric Lippert's blog](https://ericlippert.com/2013/02/06/static-constructors-part-one/) for the challenges in C#, which also has this feature.

You can approximate non-constant-expression globals with the [lazy-static](https://crates.io/crates/lazy_static/) crate.

<h2 id="other-languages">다른 언어들</h2>

<h3><a href="#how-can-i-use-static-fields" name="how-can-i-use-static-fields">
C의 <code>struct X { static int X; };</code> 같은 코드를 Rust에서는 어떻게 만드나요?
</a></h3>

Rust does not have `static` fields as shown in the code snippet above. Instead, you can declare a `static` variable in a given module, which is kept private to that module.

<h3><a href="#how-can-i-convert-a-c-style-enum-to-an-integer" name="how-can-i-convert-a-c-style-enum-to-an-integer">
C 스타일의 열거형을 정수로 바꾸거나 반대로 하려면 어떻게 하나요?
</a></h3>

Converting a C-style enum to an integer can be done with an `as` expression, like `e as i64` (where `e` is some enum).

Converting in the other direction can be done with a `match` statement, which maps different numeric values to different potential values for the enum.

<h3><a href="#why-do-rust-programs-use-more-memory-than-c" name="why-do-rust-programs-use-more-memory-than-c">
왜 Rust 프로그램이 C보다 많은 메모리를 사용하는 거죠?
</a></h3>

There are several factors that contribute to Rust programs having, by default, larger binary sizes than functionally-equivalent C programs. In general, Rust's preference is to optimize for the performance of real-world programs, not the size of small programs.

__Monomorphization__

Rust monomorphizes generics, meaning that a new version of a generic function or type is generated for each concrete type it's used with in the program. This is similar to how templates work in C++. For example, in the following program:

```rust
fn foo<T>(t: T) {
    // ... do something
}

fn main() {
    foo(10);       // i32
    foo("hello");  // &str
}
```

Two distinct versions of `foo` will be in the final binary, one specialized to an `i32` input, one specialized to a `&str` input. This enables efficient static dispatch of the generic function, but at the cost of a larger binary.

__Debug symbols__

Rust programs compile with some debug symbols retained, even when compiling in release mode. These are used for providing backtraces on panics, and can be removed with `strip`, or another debug symbol removal tool. It is also useful to note that compiling in release mode with Cargo is equivalent to setting optimization level 3 with rustc. An alternative optimization level (called `s` or `z`) [has recently landed](https://github.com/rust-lang/rust/pull/32386) and tells the compiler to optimize for size rather than performance.

__Jemalloc__

Rust uses jemalloc as the default allocator, which adds some size to compiled Rust binaries. Jemalloc is chosen because it is a consistent, quality allocator that has preferable performance characteristics compared to a number of common system-provided allocators. There is work being done to [make it easier to use custom allocators](https://github.com/rust-lang/rust/issues/32838), but that work is not yet finished.

__Link-time optimization__

Rust does not do link-time optimization by default, but can be instructed to do so. This increases the amount of optimization that the Rust compiler can potentially do, and can have a small effect on binary size. This effect is likely larger in combination with the previously mentioned size optimizing mode.

__Standard library__

The Rust standard library includes libbacktrace and libunwind, which may be undesirable in some programs. Using `#![no_std]` can thus result in smaller binaries, but will also usually result in substantial changes to the sort of Rust code you're writing. Note that using Rust without the standard library is often functionally closer to the equivalent C code.

As an example, the following C program reads in a name and says "hello" to the person with that name.

```c
#include <stdio.h>

int main(void) {
    printf("What's your name?\n");
    char input[100] = {0};
    scanf("%s", input);
    printf("Hello %s!\n", input);
    return 0;
}
```

Rewriting this in Rust, you may get something like the following:

```rust
use std::io;

fn main() {
    println!("What's your name?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("Hello {}!", input);
}
```

This program, when compiled and compared against the C program, will have a larger binary and use more memory. But this program is not exactly equivalent to the above C code. The equivalent Rust code would instead look something like this:

```rust
#![feature(lang_items)]
#![feature(libc)]
#![feature(no_std)]
#![feature(start)]
#![no_std]

extern crate libc;

extern "C" {
    fn printf(fmt: *const u8, ...) -> i32;
    fn scanf(fmt: *const u8, ...) -> i32;
}

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    unsafe {
        printf(b"What's your name?\n\0".as_ptr());
        let mut input = [0u8; 100];
        scanf(b"%s\0".as_ptr(), &mut input);
        printf(b"Hello %s!\n\0".as_ptr(), &input);
        0
    }
}

#[lang="eh_personality"] extern fn eh_personality() {}
#[lang="panic_fmt"] fn panic_fmt() -> ! { loop {} }
#[lang="stack_exhausted"] extern fn stack_exhausted() {}
```

Which should indeed roughly match C in memory usage, at the expense of more programmer complexity, and a lack of static guarantees usually provided by Rust (avoided here with the use of `unsafe`).

<h3><a href="#why-no-stable-abi" name="why-no-stable-abi">
왜 Rust는 C 같이 안정화된 ABI가 없는 건가요? 그리고 왜 `extern`을 달아야 하는 거죠?
</a></h3>

Committing to an ABI is a big decision that can limit potentially advantageous language changes in the future. Given that Rust only hit 1.0 in May of 2015, it is still too early to make a commitment as big as a stable ABI. This does not mean that one won't happen in the future, though. (Though C++ has managed to go for many years without specifying a stable ABI.)

The `extern` keyword allows Rust to use specific ABI's, such as the well-defined C ABI, for interop with other languages.

<h3><a href="#can-rust-code-call-c-code" name="can-rust-code-call-c-code">
Rust 코드가 C 코드를 호출할 수 있나요?
</a></h3>

네.
C 코드를 Rust에서 부르는 것은 C++에서 C 코드를 부르는 것만큼 효율적이도록 설계되었습니다.

<h3><a href="#can-c-code-call-rust-code" name="can-c-code-call-rust-code">
C 코드가 Rust 코드를 호출할 수 있나요?
</a></h3>

네.
Rust 코드가 `extern` 선언으로 노출되어 C의 ABI와 호환되도록 만들어야 합니다.
이러한 함수는 C 코드에 함수 포인터로 전달되거나, `#[no_mangle]` 속성으로 기호 꾸미기(symbol mangling)를 껐을 경우, C 코드에서 바로 호출될 수 있습니다.

<h3><a href="#why-rust-vs-cxx" name="why-rust-vs-cxx">
전 이미 C++를 완벽하게 짤 수 있는데, Rust가 어떤 잇점이 있나요?
</a></h3>

현대적인 C++는 안전하고 올바른 코드를 짜기 더 수월하도록 하는 많은 기능들을 가지고 있습니다만, 완벽한 건 아니고 여전히 위험을 불러오기가 쉽습니다.
C++ 코어 개발자들은 이 문제를 해결하려 노력하고 있지만, C++는 그들이 지금 구현하려 하는 많은 아이디어에 앞서는 오랜 역사로 제약을 받습니다.

Rust는 첫 날부터 안전한 시스템 프로그래밍 언어로 설계되었으며, 이는 C++를 제대로 안전하게 만들기를 매우 복잡하게 만드는 역사적인 설계 결정들에 제한을 받지 않는다는 뜻입니다.
C++에서 안전성은 주의깊은 개인적인 규율로 달성되고 틀리기 매우 쉽습니다.
Rust에서 안전성은 기본값입니다.
Rust는 여러분보다 덜 완전한 사람들을 포함하는 팀에서, 안전성 버그를 피하려 그들의 코드를 재확인하는 데 시간을 쓸 필요가 없이 함께 일할 수 있도록 합니다.

<h3><a href="#how-to-get-cxx-style-template-specialization" name="how-to-get-cxx-style-template-specialization">
C++의 템플릿 특수화 같은 걸 Rust에서는 어떻게 할 수 있을까요?
</a></h3>

Rust doesn't currently have an exact equivalent to template specialization, but it is [being worked on](https://github.com/rust-lang/rfcs/pull/1210) and will hopefully be added soon. However, similar effects can be achieved via [associated types](https://doc.rust-lang.org/stable/book/associated-types.html).

<h3><a href="#how-does-ownership-relate-to-cxx-move-semantics" name="how-does-ownership-relate-to-cxx-move-semantics">
Rust의 소유권 시스템이 C++의 "이동" 의미론과 어떻게 연관되나요?
</a></h3>

The underlying concepts are similar, but the two systems work very
differently in practice. In both systems, "moving" a value is a way to
transfer ownership of its underlying resources. For example, moving a
string would transfer the string's buffer rather than copying it.

In Rust, ownership transfer is the default behavior. For example, if I
write a function that takes a `String` as argument, this function will
take ownership of the `String` value supplied by its caller:

```rust
fn process(s: String) { }

fn caller() {
    let s = String::from("Hello, world!");
    process(s); // Transfers ownership of `s` to `process`
    process(s); // Error! ownership already transferred.
}
```

As you can see in the snippet above, in the function `caller`, the
first call to `process` transfers ownership of the variable `s`. The
compiler tracks ownership, so the second call to `process` results in
an error, because it is illegal to give away ownership of the same
value twice. Rust will also prevent you from moving a value if there
is an outstanding reference into that value.

C++ takes a different approach. In C++, the default is to copy a value
(to invoke the copy constructor, more specifically). However, callees
can declare their arguments using an "rvalue reference", like
`string&&`, to indicate that they will take ownership of some of the
resources owned by that argument (in this case, the string's internal
buffer). The caller then must either pass a temporary expression or
make an explicit move using `std::move`. The rough equivalent to the
function `process` above, then, would be:

```
void process(string&& s) { }

void caller() {
    string s("Hello, world!");
    process(std::move(s));
    process(std::move(s));
}
```

C++ compilers are not obligated to track moves. For example, the code
above compiles without a warning or error, at least using the default
settings on clang. Moreover, in C++ ownership of the string `s` itself
(if not its internal buffer) remains with `caller`, and so the
destructor for `s` will run when `caller` returns, even though it has
been moved (in Rust, in contrast, moved values are dropped only by
their new owners).

<h3><a href="#how-to-interoperate-with-cxx" name="how-to-interoperate-with-cxx">
C++에서 Rust와 상호작용하거나, Rust에서 C++와 상호작용하려면 어떻게 하나요?
</a></h3>

Rust and C++ can interoperate through C. Both Rust and C++ provide a [foreign function interface](https://doc.rust-lang.org/book/ffi.html) for C, and can use that to communicate between each other. If writing C bindings is too tedious, you can always use [rust-bindgen](https://github.com/crabtw/rust-bindgen) to help automatically generate workable C bindings.

<h3><a href="#does-rust-have-cxx-style-constructors" name="does-rust-have-cxx-style-constructors">
Rust에는 C++ 같은 생성자가 있나요?
</a></h3>

No. Functions serve the same purpose as constructors without adding language complexity. The usual name for the constructor-equivalent function in Rust is `new()`, although this is just a convention rather than a language rule. The `new()` function in fact is just like any other function. An example of it looks like so:

```rust
struct Foo {
    a: i32,
    b: f64,
    c: bool,
}

impl Foo {
    fn new() -> Foo {
        Foo {
            a: 0,
            b: 0.0,
            c: false,
        }
    }
}
```

<h3><a href="#does-rust-have-copy-constructors" name="does-rust-have-copy-constructors">
Rust에는 복사 생성자가 있나요?
</a></h3>

Not exactly. Types which implement `Copy` will do a standard C-like "shallow copy" with no extra work (similar to "plain old data" in C++). It is impossible to implement `Copy` types that require custom copy behavior. Instead, in Rust "copy constructors" are created by implementing the `Clone` trait, and explicitly calling the `clone` method. Making user-defined copy operators explicit surfaces the underlying complexity, making it easier for the developer to identify potentially expensive operations.

<h3><a href="#does-rust-have-move-constructors" name="does-rust-have-move-constructors">
Rust에는 이동 생성자가 있나요?
</a></h3>

아뇨.
모든 타입의 값들은 `memcpy`로 옮겨집니다.
덕분에 일반적인 안전하지 않은 코드를 짜기 훨씬 간단해지는데, 대입이나 인자를 넘기고 반환하는 과정에서 되감기(unwinding) 같은 부수 효과가 일어날 수 없다는 걸 보장할 수 없기 때문입니다.

<h3><a href="#compare-go-and-rust" name="compare-go-and-rust">
Go와 Rust가 비슷한 점은 무엇이고 다른 점은 무엇인가요?
</a></h3>

Rust and Go have substantially different design goals. The following differences are not the only ones (which are too numerous to list), but are a few of the more important ones:

- Rust is lower level than Go. For example, Rust does not require a garbage collector, whereas Go does. In general, Rust affords a level of control that is comparable to C or C++.
- Rust's focus is on ensuring safety and efficiency while also providing high-level affordances, while Go's is on being a small, simple language which compiles quickly and can work nicely with a variety of tools.
- Rust has strong support for generics, which Go does not.
- Rust has strong influences from the world of functional programming, including a type system which draws from Haskell's typeclasses. Go has a simpler type system, using interfaces for basic generic programming.

<h3><a href="#how-do-rust-traits-compare-to-haskell-typeclasses" name="how-do-rust-traits-compare-to-haskell-typeclasses">
Rust 트레이트를 하스켈 타입 클래스와 비교하면 어떤가요?
</a></h3>

Rust traits are similar to Haskell typeclasses, but are currently not as powerful, as Rust cannot express higher-kinded types. Rust's associated types are equivalent to Haskell type families.

Some specific difference between Haskell typeclasses and Rust traits include:

- Rust traits have an implicit first parameter called `Self`. `trait Bar` in Rust corresponds to `class Bar self` in Haskell, and `trait Bar<Foo>` in Rust corresponds to `class Bar foo self` in Haskell.
- "Supertraits" or "superclass constraints" in Rust are written `trait Sub: Super`, compared to `class Super self => Sub self` in Haskell.
- Rust forbids orphan instances, resulting in different coherence rules in Rust compared to Haskell.
- Rust's `impl` resolution considers the relevant `where` clauses and trait bounds when deciding whether two `impl`s overlap, or choosing between potential `impl`s. Haskell only considers the constraints in the `instance` declaration, disregarding any constraints provided elsewhere.
- A subset of Rust's traits (the ["object safe"](https://github.com/rust-lang/rfcs/blob/master/text/0255-object-safety.md) ones) can be used for dynamic dispatch via trait objects. The same feature is available in Haskell via GHC's `ExistentialQuantification`.

<h2 id="documentation">문서</h2>

<h3><a href="#why-are-so-many-rust-answers-on-stackoverflow-wrong" name="why-are-so-many-rust-answers-on-stackoverflow-wrong">
왜 Stack Overflow의 Rust 답변 중에는 틀린 게 많은가요?
</a></h3>

Rust 언어는 여러 해 동안 개발되어 왔으며, 2015년 5월에서야 1.0 버전에 도달했습니다.
그 이전에는 언어가 상당히 많이 바뀌었고 Stack Overflow의 답변 중 많은 것들은 언어가 옛날 버전일 때 작성된 것입니다.

시간이 지날수록 더 많은 답변이 현재 버전을 기준으로 작성되고, 따라서 오래된 답변의 비율이 줄어들 것이므로 이 문제는 자연히 개선될 것입니다.

<h3><a href="#where-do-i-report-issues-in-the-rust-documentation" name="where-do-i-report-issues-in-the-rust-documentation">
Rust 문서에 문제를 보고하려면 어디에 하나요?
</a></h3>

Rust 문서의 문제는 Rust 컴파일러의 [이슈 트래커](https://github.com/rust-lang/rust/issues)에 보고할 수 있습니다.
보고에 앞서 먼저 [기여 가이드라인](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#writing-documentation)을 읽어 주세요.

<h3><a href="#how-do-i-view-rustdoc-documentation-for-a-library-my-project-depends-on" name="how-do-i-view-rustdoc-documentation-for-a-library-my-project-depends-on">
제 프로젝트가 의존하는 라이브러리의 rustdoc 문서를 어떻게 볼 수 있나요?
</a></h3>

When you use `cargo doc` to generate documentation for your own project, it also generates docs for the active dependency versions. These are put into the `target/doc` directory of your project. Use `cargo doc --open` to open the docs after building them, or just open up `target/doc/index.html` yourself.

[Vec]: https://doc.rust-lang.org/stable/std/vec/struct.Vec.html
[HashMap]: https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html
[Into]: https://doc.rust-lang.org/stable/std/convert/trait.Into.html
[From]: https://doc.rust-lang.org/stable/std/convert/trait.From.html
[Eq]: https://doc.rust-lang.org/stable/std/cmp/trait.Eq.html
[PartialEq]: https://doc.rust-lang.org/stable/std/cmp/trait.PartialEq.html
[Ord]: https://doc.rust-lang.org/stable/std/cmp/trait.Ord.html
[PartialOrd]: https://doc.rust-lang.org/stable/std/cmp/trait.PartialOrd.html
[f32]: https://doc.rust-lang.org/stable/std/primitive.f32.html
[f64]: https://doc.rust-lang.org/stable/std/primitive.f64.html
[i32]: https://doc.rust-lang.org/stable/std/primitive.i32.html
[i64]: https://doc.rust-lang.org/stable/std/primitive.i64.html
[bool]: https://doc.rust-lang.org/stable/std/primitive.bool.html
[Hash]: https://doc.rust-lang.org/stable/std/hash/trait.Hash.html
[BTreeMap]: https://doc.rust-lang.org/stable/std/collections/struct.BTreeMap.html
[VecMacro]: https://doc.rust-lang.org/stable/std/macro.vec!.html
[String]: https://doc.rust-lang.org/stable/std/string/struct.String.html
[to_string]: https://doc.rust-lang.org/stable/std/string/trait.ToString.html#tymethod.to_string
[str]: https://doc.rust-lang.org/stable/std/primitive.str.html
[str__find]: https://doc.rust-lang.org/stable/std/primitive.str.html#method.find
[str__as_bytes]: https://doc.rust-lang.org/stable/std/primitive.str.html#method.as_bytes
[u8]: https://doc.rust-lang.org/stable/std/primitive.u8.html
[char]: https://doc.rust-lang.org/stable/std/primitive.char.html
[Weak]: https://doc.rust-lang.org/stable/std/rc/struct.Weak.html
[IntoIterator]: https://doc.rust-lang.org/stable/std/iter/trait.IntoIterator.html
[Rc]: https://doc.rust-lang.org/stable/std/rc/struct.Rc.html
[UnsafeCell]: https://doc.rust-lang.org/stable/std/cell/struct.UnsafeCell.html
[Copy]: https://doc.rust-lang.org/stable/std/marker/trait.Copy.html
[Clone]: https://doc.rust-lang.org/stable/std/clone/trait.Clone.html
[Cell]: https://doc.rust-lang.org/stable/std/cell/struct.Cell.html
[RefCell]: https://doc.rust-lang.org/stable/std/cell/struct.RefCell.html
[Cow]: https://doc.rust-lang.org/stable/std/borrow/enum.Cow.html
[Deref]: https://doc.rust-lang.org/stable/std/ops/trait.Deref.html
[Arc]: https://doc.rust-lang.org/stable/std/sync/struct.Arc.html
[Box]: https://doc.rust-lang.org/stable/std/boxed/struct.Box.html
[Option]: https://doc.rust-lang.org/stable/std/option/enum.Option.html
[Fn]: https://doc.rust-lang.org/stable/std/ops/trait.Fn.html
[FnMut]: https://doc.rust-lang.org/stable/std/ops/trait.FnMut.html
[FnOnce]: https://doc.rust-lang.org/stable/std/ops/trait.FnOnce.html
[Result]: https://doc.rust-lang.org/stable/std/result/enum.Result.html
[RandomState]: https://doc.rust-lang.org/stable/std/collections/hash_map/struct.RandomState.html
[Add]: https://doc.rust-lang.org/stable/std/ops/trait.Add.html
[AddAssign]: https://doc.rust-lang.org/stable/std/ops/trait.AddAssign.html
[Sub]: https://doc.rust-lang.org/stable/std/ops/trait.Sub.html
[SubAssign]: https://doc.rust-lang.org/stable/std/ops/trait.SubAssign.html
[Mul]: https://doc.rust-lang.org/stable/std/ops/trait.Mul.html
[MulAssign]: https://doc.rust-lang.org/stable/std/ops/trait.MulAssign.html
[Div]: https://doc.rust-lang.org/stable/std/ops/trait.Div.html
[DivAssign]: https://doc.rust-lang.org/stable/std/ops/trait.DivAssign.html
[Neg]: https://doc.rust-lang.org/stable/std/ops/trait.Neg.html
[Rem]: https://doc.rust-lang.org/stable/std/ops/trait.Rem.html
[RemAssign]: https://doc.rust-lang.org/stable/std/ops/trait.RemAssign.html
[BitAnd]: https://doc.rust-lang.org/stable/std/ops/trait.BitAnd.html
[BitAndAssign]: https://doc.rust-lang.org/stable/std/ops/trait.BitAndAssign.html
[BitOr]: https://doc.rust-lang.org/stable/std/ops/trait.BitOr.html
[BitOrAssign]: https://doc.rust-lang.org/stable/std/ops/trait.BitOrAssign.html
[BitXor]: https://doc.rust-lang.org/stable/std/ops/trait.BitXor.html
[BitXorAssign]: https://doc.rust-lang.org/stable/std/ops/trait.BitXorAssign.html
[Not]: https://doc.rust-lang.org/stable/std/ops/trait.Not.html
[Shl]: https://doc.rust-lang.org/stable/std/ops/trait.Shl.html
[ShlAssign]: https://doc.rust-lang.org/stable/std/ops/trait.ShlAssign.html
[Shr]: https://doc.rust-lang.org/stable/std/ops/trait.Shr.html
[ShrAssign]: https://doc.rust-lang.org/stable/std/ops/trait.ShrAssign.html
[Deref]: https://doc.rust-lang.org/stable/std/ops/trait.Deref.html
[DerefMut]: https://doc.rust-lang.org/stable/std/ops/trait.DerefMut.html
[Index]: https://doc.rust-lang.org/stable/std/ops/trait.Index.html
[IndexMut]: https://doc.rust-lang.org/stable/std/ops/trait.IndexMut.html
[read__read_to_string]: https://doc.rust-lang.org/stable/std/io/trait.Read.html#method.read_to_string
[Read]: https://doc.rust-lang.org/stable/std/io/trait.Read.html
[std-io]: https://doc.rust-lang.org/stable/std/io/index.html
[File]: https://doc.rust-lang.org/stable/std/fs/struct.File.html
[read__read]: https://doc.rust-lang.org/stable/std/io/trait.Read.html#tymethod.read
[read__read_to_end]: https://doc.rust-lang.org/stable/std/io/trait.Read.html#method.read_to_end
[read__bytes]: https://doc.rust-lang.org/stable/std/io/trait.Read.html#method.bytes
[read__chars]: https://doc.rust-lang.org/stable/std/io/trait.Read.html#method.chars
[read__take]: https://doc.rust-lang.org/stable/std/io/trait.Read.html#method.take
[BufReader]: https://doc.rust-lang.org/stable/std/io/struct.BufReader.html
[Args]: https://doc.rust-lang.org/stable/std/env/struct.Args.html
[TryMacro]: https://doc.rust-lang.org/stable/std/macro.try!.html
[unwrap]: https://doc.rust-lang.org/stable/core/option/enum.Option.html#method.unwrap
[Mutex]: https://doc.rust-lang.org/stable/std/sync/struct.Mutex.html
[AtomicUsize]: https://doc.rust-lang.org/stable/std/sync/atomic/struct.AtomicUsize.html
[Sync]: https://doc.rust-lang.org/stable/std/marker/trait.Sync.html
[Drop]: https://doc.rust-lang.org/stable/std/ops/trait.Drop.html
[clone_from_slice]: https://doc.rust-lang.org/stable/std/primitive.slice.html#method.clone_from_slice
[copy]: https://doc.rust-lang.org/stable/std/ptr/fn.copy.html
[copy_nonoverlapping]: https://doc.rust-lang.org/stable/std/ptr/fn.copy_nonoverlapping.html
[clone]: https://doc.rust-lang.org/stable/std/clone/trait.Clone.html#tymethod.clone
