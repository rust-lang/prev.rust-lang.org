---
layout: fr-FR/default
title: Contribuer à Rust &mdash; la documentation &middot; Rust, le langage de programmation
---

# Contribuer à Rust &mdash; la documentation

Une documentation n'est jamais parfaite et il en manque toujours. Il n'est pas nécessaire de connaître tous les détails de Rust pour contribuer à de nombreux aspects de la documentation. Améliorer, écrire, relire ou éditer la documentation relative à Rust sont autant de moyens d'apprendre Rust. De plus, les pistes d'amélioration sont faciles à identifier et nombreuses. La formulation n'est pas claire ? Quelque chose n'est pas encore documenté ? Votre *pull request* sera acceptée avec joie.

***La plus précieuse documentation que vous pouvez écrire concerne [les « crates » qui constituent l'écosystème Rust][crate_docs]***. Bien que la documentation sur l'arborescence du code liée à Rust soit relativement complète, ce n'est pas encore le cas pour [de nombreuses *crates* et outils][awesome-rust] que les programmeurs Rust utilisent quotidiennement. Contribuer à la documentation d'une API concernant un projet Rust populaire vous attirera vraisemblablement la plus vive sympathie de la personne maintenant ce projet.

[Le livre][The book] constitue la première source de documentation pour Rust. Cette documentation est maintenue dans le dépôt principal. Elle possède sa propre étiquette pour le traqueur : [A-book] et les pages de ce livre évoluent en permanence. Les autres sources de documentation du dépôt principal sont : [la référence Rust][The Rust Reference], [la documentation de l'API de la bibliothèque standard][std], [le Rustonomicon][The Rustonomicon] (un guide pour correctement utiliser `unsafe`). [Le guide stylistique][Rust Style Guidelines] est encore trop incomplet, un contributeur ambitieux pourrait sans aucun doute faire progresser ces documents. [L'index des erreurs][err] fournit de nombreuses explications sur les erreurs produites par le compilateur. De nouvelles erreurs sont ajoutées au fur et à mesure et cette documentation [doit être maintenue][err-issue]. Il y a toujours des erreurs qui doivent être ajoutées dans cet index. La plupart de la documentation réside dans le répertoire [src/doc] et les problèmes liés sont identifiés dans le traqueur par l'étiquette [A-docs]. Enfin ce document et les autres contenus du site web sont maintenus [sur le dépôt Git du site web pour Rust][Rust website Git repository]. Pour contribuer à ce site, il suffit d'éditer et de soumettre une *pull request*.

Une bonne partie de la documentation Rust n'est pas dans le dépôt principal et n'est pas maintenue au sein de ce projet, mais cette documentation est néanmoins une condition *sine qua non* du succès de Rust. Si vous cherchez d'excellents projet de documentation, actifs et qui ont besoin de contributeurs, en voici quelques-uns : [Rust By Example], [Rust Design Patterns], et [rust-rosetta].
Pour découvrir d'autres projets de documentation auxquels contribuer, vous pouvez consulter [rust-learning].

Vous pouvez discuter avec d'autres personnes contribuant à la documentation sur le canal [#rust-docs].

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
