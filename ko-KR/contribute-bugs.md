---
layout: ko-KR/default
title: Rust에 기여하기 &mdash; 이슈를 찾고, 관리하고, 고치기 &middot; Rust 프로그래밍 언어
---

# Rust에 기여하기 &mdash; 이슈를 찾고, 관리하고, 고치기

The day-to-day maintenance of the project revolves around Rust's
[issue tracker] and [pull requests][PR], and more help is always
needed. The most basic way to get started contributing to Rust is to
look for the [E-easy] or [E-mentor] labels. These are meant to be
approachable for new Rust programmers.

On `E-mentor` issues an experienced Rust developer has volunteered in
the comments to mentor you through solving the issue and [submitting
the fix via GitHub pull request][pull]. Contact them about the issue,
on the issue tracker by [@mentioning] their name in a comment, on IRC,
or through email. Note that Rust developers get a lot of notifications
and it is easy to miss some; don't hesitate to hunt them down by
whatever means necessary!

Other projects in Rust maintain similar entry-level tasks, including
the web browser [Servo], the HTTP library [hyper], the
source-formatter [rustfmt], the Unix library bindings [nix],
and the lint collection [clippy].

While Rust has an [extensive test suite][test] there is always more to
test. The [E-needstest] label indicates issues that are thought to be
fixed but don't have tests. Writing test cases is a great way to
understand a new project and get started contributing.

Rust is always in need of people to [triage] issues: reproduce bugs,
minimize test cases, apply labels, close resolved issues. Note that
you'll need elevated GitHub permissions to apply labels, but this is
easy to obtain for somebody with a bit of experience in the
project. Ask a [team member][team].

Once you've found your way around the project and have created a few
pull requests in a particular area, consider reviewing others' pull
requests: good reviewership is a rare skill and always appreciated. No
prior permission is needed &mdash; just start constructively and politely
commenting on pull requests that interest you. If you want training
on conducting good code reviews [read this guide][reviews].

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
