---
layout: de-DE/default
title: Bei Rust mitwirken &mdash; Dokumentation &middot; Die Programmiersprache Rust
---

# Bei Rust mitwirken &mdash; Dokumentation

Dokumentation ist nie perfekt, und es gibt nie genug davon. Viele Aspekte der Sprache erfordern kein tiefes Verständnis um ihre Dokumentation zu verfassen, zu verbessern oder gegenzulesen. Im Gegenteil bietet das Dokumentieren eine großartige Gelegenheit, Rust zu lernen, und es ist leicht, Verbesserungsmöglichkeiten an den Dokumenten zu finden. Eine Passage ist zu umständlich formuliert? Wichtige Informationen werden nicht erwähnt? Dein Pull Request wird gerne angenommen!

**Die wirkungsvollste Dokumentation, die du schreiben kannst, ist die [für Crates des Rust-Ökosystems][crate_docs]**. Während das Kernprojekt relativ gut dokumentiert ist, gilt das selbe nicht für [viele der gängigen Crates und Werkzeuge][awesome-rust], mit denen wir täglich zu tun haben. Das Verfassen von API-Dokumentation wird dir viel Dankbarkeit entgegenbringen.

Das Buch [The Rust Programming Language][The book] ist das wichtigste Dokument zu Rust und wird im Hauptrepository verwaltet. Es hat sein eigenes Issue-Label, [A-book], und wird kontinuierlich verbessert. Andere wichtige Dokumentation findet sich in der [Rust-Referenz][The Rust Reference], der [Dokumentation zur Standardbibliothek][std], dem [Rustonomicon][The Rustonomicon] (einem Leitfaden zu korrektem `unsafe`-Code). Die [Rust-Stilrichtlinien][Rust Style Guidelines] sind noch so unvollständig, dass sie bisher nicht prominent angekündigt werden; hier kann ein engagierter Helfer viel erreichen. Der [Fehlerindex][err] gibt ausführlichere Erklärungen zu Compilerfehlern. Nachdem immer neue Fehlercodes hinzukommen, muss dieser Index [auf dem neuesten Stand gehalten werden][err-issue]. Die meiste Dokumentation im Rust-Repository liegt im Verzeichnis [src/doc] und wird mit dem Issue-Label [A-docs] gekennzeichnet. Zu guter Letzt wird dieses Dokument zusammen mit anderen Bestandteilen der Website im [Rust-Website-Repository][Rust website Git repository] verwaltet. Um mitzuwirken bearbeite einfach den Inhalt und stelle ein Pull Request!

Ein großer Teil der Rust-Dokumentation liegt nicht im Haupt-Repository oder wird nicht vom Rust-Projekt selbst verwaltet, ist aber trotzdem unersetzlich für den Erfolg von Rust. Beispiele für solch exzellente Werke sind [Rust By Example], [Rust Design Patterns] und [rust-rosetta]. Andere derartige Projekte, bei denen du mitwirken kannst, findest du bei [rust-learning].

Andere Dokumentationsbegeisterte findest du immer in [#rust-docs].

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
[The Rust Reference]: https://doc.rust-lang.org/reference
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
