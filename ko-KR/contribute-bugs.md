---
layout: ko-KR/default
title: Rust에 기여하기 &mdash; 이슈를 찾고, 관리하고, 고치기 &middot; Rust 프로그래밍 언어
---

# Rust에 기여하기 &mdash; 이슈를 찾고, 관리하고, 고치기

프로젝트의 매일 매일의 관리는 Rust의 [이슈 트래커][issue tracker]와
[풀 요청][PR](pull request)을 중심으로 일어나며, 언제나 더 많은 도움이 필요합니다.
Rust에 기여를 시작하는 가장 기본적인 방법은 [E-easy]나 [E-mentor] 라벨을 찾아 보는 겁니다.
이들은 새 Rust 프로그래머들이 접근할만한 이슈라는 걸 뜻합니다.

`E-mentor` 이슈에서는
이슈를 해결하고 [수정을 GitHub 풀 요청으로 보내는][pull] 과정에서
경험 있는 Rust 개발자들이 자원하여 댓글을 통해 멘토링을 합니다.
이슈에 대해 연락하려면 이슈 트래커의 댓글에서
아이디 앞에 [@를 붙여 언급][@mentioning]하거나, IRC 또는 이메일을 통하세요.
Rust 개발자들은 알림을 많이 받으며 일부를 놓치기 쉽다는 점을 알려 드립니다.
주저 말고 어떤 방법으로든 연락을 시도해 보세요!

Rust로 된 다른 프로젝트들, 이를테면 [Servo] 웹 브라우저나,
[hyper] HTTP 라이브러리, [rustfmt] 소스 포매터, [nix] 유닉스 라이브러리 바인딩,
그리고 [clippy] 컴파일러 오류 및 정보(lint) 모음 같은 곳에서도
비슷한 입문 단계 작업들을 관리합니다.

Rust는 [광범위한 테스트 수트][test]를 가지고 있지만 언제나 더 테스트할 건 많습니다.
[E-needstest] 라벨은 이미 고쳐졌다고 생각하지만 테스트가 없는 이슈를 가리킵니다.
테스트 케이스를 작성하는 건 새 프로제트를 이해하고 기여를 시작하는 좋은 방법입니다.

Rust는 언제나 이슈들을 [저울질하며][triage] 버그를 재현하고,
테스트 케이스를 최소화하고, 라벨을 붙이며, 해결된 이슈를 닫는 사람들이 필요합니다.
참고로 라벨을 붙이려면 더 높은 GitHub 권한이 필요한데,
프로젝트에 약간의 경험만 있다면 쉽게 얻을 수 있습니다. [팀원][team]한테 물어 보세요.

프로젝트에서 어떻게 할지 감이 잡혀서 특정 분야에서 여러 풀 요청을 만들어 냈다면,
다른 사람의 풀 요청을 리뷰하는 걸 생각해 보세요.
좋은 리뷰 작업은 드문 수완이며 언제나 감사를 드립니다.
미리 권한을 받을 필요는 없습니다.
그냥 관심이 가는 풀 요청에서 생산적이고 정중하게 댓글을 시작하세요.
코드 리뷰를 잘 하는 걸 연습하고 싶다면 [이 안내를 읽어 보세요][reviews].

<!--
TODO: weekly triage email?
TODO: @nrc says suggesting everybody review w/o training is bad
-->

[@mentioning]: https://github.com/blog/821
[E-easy]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AE-easy
[E-mentor]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AE-easy+label%3AE-mentor
[E-needstest]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AE-needstest
[PR]: https://github.com/rust-lang/rust/pulls
[Servo]: https://github.com/servo/servo
[clippy]: https://github.com/Manishearth/rust-clippy
[hyper]: https://github.com/hyperium/hyper
[issue tracker]: https://github.com/rust-lang/rust/issues
[nix]: https://github.com/nix-rust/nix/
[pull]: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#pull-requests
[reviews]: http://blog.originate.com/blog/2014/09/29/effective-code-reviews/
[rustfmt]: https://github.com/rust-lang-nursery/rustfmt
[team]: team.html
[test]: https://github.com/rust-lang/rust-wiki-backup/blob/master/Note-testsuite.md
[triage]: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#issue-triage
