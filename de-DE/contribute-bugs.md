---
layout: de-DE/default
title: Bei Rust mitwirken &mdash; Fehler finden, sichten und beheben &middot; Die Programmiersprache Rust
---

# Bei Rust mitwirken &mdash; Fehler finden, sichten und beheben

Die tägliche Pflege des Projekts dreht sich um den [Issue Tracker][issue tracker] und [Pull Requests][pull] - Mithilfe wird immer benötigt! Der einfachste Weg um einzusteigen sind Tickets mit den [E-easy] und [E-mentor]-Labels, die für neue Rust-Programmierer geeignet sind.

Bei Issues, die mit `E-mentor` gekennzeichnet sind, hat sich ein erfahrener Entwickler bereit erklärt, dich durch den Lösungsprozess und das [Einsenden eines Pull Request][pull] zu begleiten. Kontaktiere den Mentor auf dem Issue Tracker indem du ihn mit [@mentioning] in einem Kommentar erwähnst, oder ihn per Mail oder IRC direkt anschreibst. Denke daran, dass Rust-Entwickler täglich viele Benachrichtigungen erhalten und einzelne Nachrichten übersehen werden können, also scheue dich nicht noch einmal nachzuhaken, falls du einmal keine Antwort erhältst.

Andere Rust-Projekte bieten ähnliche Einsteigeraufgaben, so etwa der Webbrowser [Servo], die HTTP-Bibliothek [hyper], das Code-Formatierungswerkzeug [rustfmt], die Unix-Bindings [nix] oder der Linter [clippy].

Obwohl Rust bereits eine [ansehnliche Test-Suite][test] besitzt, gibt es immer Testfälle, die noch nicht abgedeckt sind. Das [E-needstest]-Label kennzeichnet Issues, die wahrscheinlich schon gelöst sind, zu deren Problemstellung aber noch ein Unit-Test fehlt. Testfälle zu schreiben ist eine gute Gelegenheit, um mit einer neuen Projektstruktur vertraut zu werden und zum erstem Mal mitzuwirken.

Rust braucht immer Freiwillige, die Issues [sichten][triage], also Bugs reproduzieren, Testfälle minimieren, Labels zuweisen und gelöste Tickets schließen. Um Labels zuzuweisen, brauchst du zusätzliche Berechtigungen auf GitHub, die aber leicht zu bekommen sind, sobald du etwas Erfahrung im Projekt gesammelt hast. Frag einfach ein [Teammitglied][team].

Sobald du dich im Projekt zurechtfindest und ein paar Pull Requests zu einem Teilgebiet eingereicht hast, kannst du versuchen Pull Requests anderer Benutzer durchzusehen. Gute Reviewer sind rar und immer gern gesehen. Dazu brauchst du keine besonderen Berechtigungen &mdash; kommentiere einfach Pull Requests die dich interessieren konstruktiv und höflich. Wenn du mehr zu guten Code Reviews lernen möchtest, wirf einen Blick in [diesen Leitfaden][reviews].

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
