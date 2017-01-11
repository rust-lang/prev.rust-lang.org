---
layout: fr-FR/default
title: Contribuer à Rust &mdash; les outils, les EDI et l'infrastructure &middot; Rust, le langage de programmation
---

# Contribuer à Rust &mdash; les outils, les EDI et l'infrastructure

Les outils sont un acteur majeur du succès d'un langage et il y a encore une bonne partie qui reste à implémenter. ***Un des axes principaux de développement de Rust est actuellement [l'amélioration de l'expérience avec les EDI][ides]***. Ce type de contribution implique de manipuler l'ensemble de la « pile » Rust : depuis le compilateur jusqu'à votre EDI préféré. Le lien ci-dessus vous fournira plus d'informations à ce sujet.

Cargo (le gestionnaire de paquets Rust) ou rustdoc (le générateur de documentation Rust) sont fonctionnels et assez complets mais ils souffrent d'un manque de développeurs. De nombreux problèmes associés à Rustdoc sont étiquetés avec [A-rustdoc] dans le traqueur du dépôt principal. Pour la plupart, ces problèmes sont des bugs et il s'agira de corriger le(s) bug(s) et d'envoyer une *pull request* pour contribuer. Cargo possède [son propre dépôt et traqueur][Cargo] et les personnes intéressées par ce projet pourront se présenter sur le canal [#cargo] afin de contribuer.

Bien que Rust puisse être manipulé avec les débogueurs gdb et lldb, cela reste plutôt limité et de nombreux cas de débogage ne fonctionnent pas encore correctement. L'étiquette [A-debuginfo] permet de repérer ces problèmes dans le traqueur.

Pour découvrir d'autres projets liés aux outils et auxquels contribuer, voir [awesome-rust].

De nombreux projets autour d'outils attendent simplement les bonnes personnes pour les implémenter. Vous pouvez discuter avec les autres personnes motivées par l'environnement Rust sur le canal [#rust-tools].

[#cargo]: https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rustc
[#rust-tools]: https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-tools
[A-debuginfo]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-debuginfo
[A-rustdoc]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-rustdoc
[Cargo]: https://github.com/rust-lang/cargo/issues
[awesome-rust]: https://github.com/kud1ing/awesome-rust
[ides]: https://forge.rust-lang.org/ides.html
