---
layout: de-DE/default
title: Bei Rust mitwirken &mdash; Sprache, Compiler und Standardbibliothek &middot; Die Programmiersprache Rust
---

# Bei Rust mitwirken &mdash; Sprache, Compiler und Standardbibliothek

Der Quelltext von Compiler und Standardbibliothek liegen im Hauptrepository, und nachdem seine Verwaltung die Hauptaufgabe dieses Repositories ist, beziehen sich auch viele Labels im Issue-Tracker darauf. Ein paar der ergiebigeren Labels sind [A-codegen] für die Übersetzung von Rust in die LLVM-IR; [A-debuginfo] für die Erzeugung von Metadaten für Debugger, [A-diagnostics] für Fehlermeldungen und anderes Feedback des Compilers; [A-libs] für Tickets zur Standardbibliothek, [A-macros] und [A-syntaxext] für Syntaxerweiterungen und [A-typesystem] für das Typsystem der Sprache.

Es existiert keine regelmäßig gepflegte Dokumentation zur Architektur des Compilers, aber [eine kurze Übersicht findet sich im Repository][rustc-guide]. Die [API-Dokumentation für die Crates des Compilers][internals-docs] kann ebenso wie der Codebrowser [Rust DXR] bei der Navigation im Quelltext helfen. Die [Richtlinien zur Rust-Testsuite][testsuite] und [`make tips`][tips] geben eine Anleitung zum Gebrauch des Buildsystems.

Eine der größten Entwicklungsziele in absehbarer Zukunft ist, die Interna des Compilers nicht mehr auf dem Syntaxbaum des Quelltexts, sondern auf der [Zwischenrepräsentation MIR][mir] arbeiten zu lassen. Es wird erwartet, dass diese Änderung durch Vereinfachung des Compilers viele neue Möglichkeiten mit sich bringt. Hilfe wird unter Anderem bei der Implementierung eines neuen MIR-basierten Übersetzungsschritts, neuer MIR-basierter Optimierungsstrategien und inkrementeller Kompilation benötigt. Es existiert noch keine zentrale Sammelstelle für Aufgaben zu diesem Thema, du kannst aber jederzeit auf [internals.rust-lang.org] oder [#rust-internals] nachfragen.

Es ist peinlich [wenn der Compiler einmal abstürzt][ice] &mdash; der gefürchtete Internal Compiler Error (ICE). Das [I-ICE]-Label markiert derartige Fehler auf dem Issue Tracker, und dort gibt es einige davon. Diese Bugs zu beheben sind eine gute Einsteigeraufgabe, da sie oft isoliert auftreten und es einfach ist festzustellen, wenn der Fehler behoben ist.

Die Performance von Rust-Code ist einer der größten Vorteile der Sprache; die des Compilers dagegen eine ihrer größten Schwächen. Jegliche Verbesserungen an Laufzeit- oder &mdash; noch besser &mdash; Compilezeitperformance werden gefeiert. Die [I-slow] und [A-optimization]-Labels beschäftigen sich mit Laufzeit- und [I-compiletime] mit Compilezeitperformance. Es gibt [eine Seite, die die Übersetzungsgeschwindigkeit][rustc-perf] für einige Szenarien verfolgt. Die Compiler-Performance kann mit Hilfe des `-Z time-passes`-Flags rückverfolgt werden, und Rust-Code kann mit gängigen Profilern wie `perf` unter Linux analysiert werden.

Große Neuerungen an Sprache und Standardbibliothek werden durch den [Request for Comments (RFC)][rfc]-Prozess verabschiedet, in dem das Design ausgearbeitet wird. Obwohl er öffentlich zugänglich ist handelt es sich um einen sozialen Prozess zwischen Entwicklern, die schon Erfahrung in der Zusammenarbeit haben. Wir empfehlen dir, dich dem Prozess langsam vertraut zu machen &mdash; ein vorschnell eingereichter RFC ohne ausreichendes Verständnis des historischen, technischen oder sozialen Kontextes hinter dem Problem führt leicht zu einem schlechten ersten Eindruck und Enttäuschung deinerseits. Lies am besten das oben erwähnte Readme, um mit dem Vorgehen vertraut zu werden. In der Geschichte von Rust sind schon viele Ideen diskutiert worden, viele verworfen, manche bis zu einem späteren Termin aufgeschoben, und der [RFC-Issue Tracker][rfc-issues] verwaltet einige Wunschideen die erst noch ihren Weg in die Sprache finden müssen. Kurz bevor ein RFC zur Implementierung freigegeben wird findet die _Final Comment Period_ statt, die durch das [final-comment-period][rfc-fcp]-Label im RFC-Repository gekennzeichnet wird. Ebenso findet nochmals eine [_Final Comment Period_][issue-fcp] statt, bevor ein Feature im stable-Branch des Compilers aktiviert wird (ein Vorgehen, dass als „Ungating“ bekannt ist). Beide FCPs sind kritische Zeitpunkte, um Meinungen einzubringen und die Entwicklungsrichtung der Sprache zu Beeinflussen. Sie werden im wöchentlichen Teambericht auf [internals.rust-lang.org] angekündigt.

Andere Rust-Compilerentwickler kannst du in [#rustc], Sprachdesigner in [#rust-lang] und Bibliotheksentwickler in [#rust-libs] kennenlernen.

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
