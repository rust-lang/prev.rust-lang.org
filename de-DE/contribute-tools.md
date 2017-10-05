---
layout: de-DE/default
title: Bei Rust mitwirken &mdash; Tooling, IDEs und Infrastruktur &middot; Die Programmiersprache Rust
---

# Bei Rust mitwirken &mdash; Tooling, IDEs und Infrastruktur

Codewerkzeuge spielen im Erfolg einer Sprache eine große Rolle,
und es gibt in diesem Bereich noch eine Menge Arbeit.
**Ein großer Schwerpunkt der aktuellen Entwicklung von Rust
ist es, [die IDE-Experience zu verbessern][ides]**.
Dies beinhaltet Arbeit am gesamten Rust-Stack, vom Compiler selbst
bis zu deiner Lieblings-IDE. Folge dem Link für mehr Information.

Sowohl Cargo, der Paketmanager, als auch der Dokumentationsgenerator rustdoc
sind zwar voll funktionsfähig, leiden aber an einem Entwicklermangel.
Rustdoc hat viele offene Probleme, die im Main-Repository unter dem Label
[A-rustdoc] gelistet sind. Es sind hauptsächlich Bugs, die lediglich einen
Bugfix und einen Pull Request erfordern.
Cargo hat [sein eigenes Repository mit Issue-Tracker][Cargo], und
wer beitragen möchte, kann sich gerne in [#cargo] melden.

Obwohl Rust-Programme sowohl mit den bekannten GDB- und LLDB-Debuggern eingeschränkt
untersucht werden können, gibt es noch immer viele Fälle, in denen Debugging
unerwartete Ergebnisse liefert oder unerwartetes Verhalten zeigt.
Diese Fälle werden im [A-debuginfo]-Label verwaltet.

Ideen für weitere Tooling-Projekte zum Mithelfen finden sich in
[awesome-rust].

Es gibt häufig andere interessante Tooling-Projekte, welche nur auf
die richtigen Leute warten, um implementiert zu werden.
Diese können mit anderen Tooling-Enthusiasten in
[#rust-tools] diskutiert werden.

[#cargo]: https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rustc
[#rust-tools]: https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-tools
[A-debuginfo]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-debuginfo
[A-rustdoc]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-rustdoc
[Cargo]: https://github.com/rust-lang/cargo/issues
[awesome-rust]: https://github.com/kud1ing/awesome-rust
[ides]: https://forge.rust-lang.org/ides.html
