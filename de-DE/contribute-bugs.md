---
layout: de-DE/default
title: Zu Rust beitragen &mdash; Fehler finden, sichten und beheben &middot; Die Programmiersprache Rust
---

# Zu Rust beitragen &mdash; Fehler finden, sichten und beheben

Knotenpunkt der tagtäglichen Maintenance des Projekts ist der [Issue-Tracker]
sowie die Durchsicht von ausstehenden [Pull Requests][PR] und mehr Hände sind
immer hilfreich. Der einfachste Weg, mit Fehlern anzufangen, ist es, nach einem
Fehler mit dem [E-easy] oder [E-mentor] Label zu schauen. Diese sind dafür
gedacht, zugänglich für neue Rust-Entwickler zu sein.

Für `E-mentor`-Issues gibt es einen überwachenden Rust-Entwickler, der als
erfahrener Leiter freiwillig andere Leute duch das Lösen von Fehlern führen kann,
sowie [den Einreichung des Bugfixes via GitHub pull durchführen darf][pull].
Kontaktiere den Leiter über das Problem, oder auf dem Issue-Tracker via
[@mentioning], via IRC oder durch E-Mail. Bitte sei dir bewusst, dass Rust-Entwickler
möglicherweise viele Benachrichtigungen kriegen - es ist leicht, ein paar zu
übersehen. Also zögere nicht, die Entwickler auf mehreren Plattformen zu kontaktieren.

Andere Rust-Projekte haben in einer ähnlichen Weise Einsteiger-Aufgaben. Dazu
gehören: der Web-Browser [Servo], die HTTP-Bibliothek [hyper], der automatische
Quellcode-Formatierer [rustfmt], die Unix-Binding Bibliothek [nix] sowie die
Lint-Kollection [clippy].

Obwohl Rust eine [große Test-Suite][test] hat, gibt es immer mehr zu testen. Das
[E-needstest]-Label signalisiert dass Fehler möglicherweise bereits behoben wurden,
aber noch kein Test besteht. Test-cases zu schreiben ist ein großartiger Weg,
ein neues Projekt kennenzulernen und mit dem Beitragen zu Rust anzufangen.

Rust braucht immer mehr Hände um Fehler zu [sichten]: Fehler reproduzieren,
Test-cases verbessern, Fehler kategoriesieren, gelöste Fehler zu schließen.
Bitte beachte, dass du auf GitHub erhöhte Privilegien brauchst, um Labels zu
erstellen und Issues zu schließen. Allerdings ist es rech einfach, jemanden
um Erlaubnis dafür zu fragen - frage einfach einen [Team-Mitarbeiter][team].

Wenn du mit einem Projekt vertraut geworden bist und ein paar Pull Requests in
einer bestimmten Gebiet gemacht hast, denk darüber nach ob du nicht vielleicht
die Pull Requests von anderen Leuten prüfen könntest: gute Code-Review-Kenntnisse
ist eine außergewöhnliche Eigenschaft und stets gut angesehen. Keine vorherigen
Berechtigungen sind nötig &mdash; leg einfach mit konstruktiver Kritik los und
kommentiere höflich deine Ideen zu Pull Requests, die dich interessieren. Wenn du
Training in Sachen Code Review haben möchtest, [lies diese Anleitungen][reviews].

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
