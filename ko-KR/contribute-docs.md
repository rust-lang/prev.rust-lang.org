---
layout: ko-KR/default
title: Rust에 기여하기 &mdash; 문서 &middot; Rust 프로그래밍 언어
---

# Rust에 기여하기 &mdash; 문서

문서는 언제나 충분히 좋을 수도 충분히 많을 수도 없습니다.
Rust 문서의 많은 면에서는 깊은 지식이 필요하지 않으며,
문서를 쓰고, 리뷰하고 편집하는 것은 Rust를 배우는 좋은 방법입니다.
게다가 문서를 개선하는 건 알아 보기 쉽고 제한이 없습니다.
뭔가 잘 읽히지 않는다고요? 문서화가 안 된 정보를 발견했다고요?
여러분의 풀 요청(pull request)은 기꺼이 받아들여질 것입니다.

***여러분이 쓸 수 있는 가장 영향력 있는 문서는
Rust 생태계를 이루는 크레이트의 문서입니다***.
저장소 내의 문서는 비교적 완전하지만, Rust 프로그래머들이 매일같이 부대끼는
[많은 유명 크레이트와 도구][awesome-rust]에 대해서는 아직 그렇지 않습니다.
유명한 Rust 프로젝트에 API 문서를 기여하면
프로젝트 관리자의 끊임 없는 사랑을 얻게 될 겁니다.

《[Rust 프로그래밍 언어][The Book]》는 주 저장소에서 관리되는 Rust의 주 문서입니다.
이 책에는 별도의 이슈 라벨 [A-book]이 존재하며 지속적으로 개정되고 있습니다.
주 저장소에 있는 다른 문서로는 [Rust 참조 문서][The Rust Reference],
[표준 라이브러리 API 문서][std], 그리고 (`unsafe`를 올바르게 쓰는 안내서인)
《<i lang="en">[Rustonomicon][The Rustonomicon]</i>》이 있습니다.
[Rust 스타일 가이드라인][Rust Style Guidelines]은 많이 불완전하기 때문에
눈에 띄게 링크되어 있진 않으나, 의욕적인 기여자라면 많은 개선을 이루어낼 수 있을 겁니다.
[오류 색인][err]은 컴파일러가 만들어 낸 오류들에 대해 자세한 설명을 제공합니다.
오류가 새로 추가되면 이 문서도 [함께 관리되어야 하며][err-issue],
따라서 색인에 아직 반영되지 않아 추가되어야 하는 오류들이 항상 있습니다.
저장소 내 대부분의 문서는 [src/doc] 디렉토리에 있으며
이슈 트래커에서는 모두 [A-docs] 라벨로 커버됩니다.
마지막으로 이 문서와 다른 웹사이트 내용은
[Rust 웹사이트 Git 저장소][Rust website Git repository]에서 관리됩니다.
기여하려면 간단히 편집한 뒤에 풀 요청을 보내 주세요.

중요한 Rust 문서들 상당수는 주 저장소에도 없고, 프로젝트에서 관리하지도 않지만,
여전히 Rust의 성공에 결정적으로 중요합니다.
현재 개발 중이며 기여자가 필요한 탁월한 Rust 문서의 예로는
《[예제로 보는 Rust][Rust By Example]》,
《[Rust 디자인 패턴][Rust Design Patterns]》, 그리고 [rust-rosetta]가 있습니다.
기존의 다른 문서화 프로젝트 중 기여할 걸 찾고 있다면 [rust-learning]을 보세요.

[#rust-docs]에서 다른 Rust 문서 저자들을 만나세요.

<!--
TODO: blogging, translation
-->

[#rust-docs]: https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs
[A-book]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-book
[A-docs]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-docs
[Rust By Example]: https://github.com/rust-lang/rust-by-example
[Rust Design Patterns]: https://github.com/nrc/patterns
[Rust Style Guidelines]: https://doc.rust-lang.org/style/index.html
[The Book]: https://doc.rust-lang.org/book/index.html
[The Rust Reference]: https://doc.rust-lang.org/reference.html
[The Rustonomicon]: https://doc.rust-lang.org/nomicon/index.html
[awesome-rust]: https://github.com/kud1ing/awesome-rust
[crate_docs]: https://users.rust-lang.org/t/lets-talk-about-ecosystem-documentation/2791
[err-issue]: https://github.com/rust-lang/rust/issues/24407
[err]: https://doc.rust-lang.org/error-index.html
[rust-learning]: https://github.com/ctjhoa/rust-learning
[rust-rosetta]: https://github.com/Hoverbear/rust-rosetta
[src/doc]: https://github.com/rust-lang/rust/tree/master/src/doc
[std]: https://doc.rust-lang.org/std/index.html
[Rust website Git repository]: https://github.com/rust-lang/rust-www
