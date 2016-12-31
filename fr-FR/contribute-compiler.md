---
layout: fr-FR/default
title: Contribuer à Rust &mdash; le langage, le compilateur et la bibliothèque standard &middot; Rust, le langage de programmation
---

# Contribuer à Rust &mdash; le langage, le compilateur et la bibliothèque standard

Le code source du compilateur et de la bibliothèque standard est présent dans le dépôt principal. C'est l'objectif principal de ce dépôt que de maintenir ce code source. De nombreuses étiquettes sont utilisées dans le traqueur pour ce code. On pourra ainsi trouver des étiquettes comme [A-codegen] pour la traduction de  Rust en LLVM IR ; [A-debuginfo] pour la génération des métadonnées par les débogueurs ; [A-diagnostics] pour les retours fournis par le compilateur lorsqu'il y a des erreurs ; [A-libs] pour les problèmes liés à la bibliothèque standard ; [A-macros] et [A-syntaxext] qui sont relatifs aux extensions de la syntaxe et [A-typesystem] à propos des types de données.

Il n'existe pas de guide maintenu à propos de l'architecture du compilateur, mais [il y a un rapide aperçu de l'arborescence][rustc-guide]. [La documentation d'API pour les *crates* qui composent le compilateur][internals-docs] peut aider à naviguer dans le code. L'explorateur de code source, [Rust DXR], est également un outil appréciable pour s'y retrouver. [Le guide à propos de la suite de tests pour Rust][testsuite] vous apprendra comment manipuler le système de compilation de Rust efficacement et comment lancer [`make tips`][tips] en ligne de commandes.

Dans un avenir proche, l'une des principales avancées pour le compilateur Rust consistera à convertir l'implémentation interne afin de travailler avec une [représentation intermédiaire intitulée MIR][mir] plutôt que de traiter directement le code AST. Ce projet permettra de simplifier le compilateur et d'ouvrir certaines possibilités. Nous avons besoin d'aide afin de construire une étape de traduction basée sur MIR, d'ajouter des optimisations basées sur MIR et d'implémenter une compilation incrémentale. À l'heure actuelle, il n'existe pas de source d'informations à ce sujet, mais [internals.rust-lang.org] ou [#rust-internals] sont de bons points d'entrée pour obtenir de l'aide.

[C'est plutôt embarrassant lorsque le compilateur plante][ice] &mdash; les fameuses '*internal compiler error*' (ICE, qui signifie « erreur interne du compilateur »). L'étiquette [I-ICE] permet de suivre ces problèmes, souvent nombreux. Ces bugs sont généralement un bon point de départ, car on sait facilement lorsque le bug est résolu. De plus, ces bugs sont souvent isolés.

La performance du code Rust est l'un des atouts du langage et les performances du compilateur Rust sont l'une de ses plus grandes faiblesses. Toute amélioration du temps d'exécution ou, surtout, des performances de compilation est appréciée au plus haut point. Les étiquettes [I-slow] et [A-optimization] permettent de suivre les points liés aux performances de l'exécution et l'étiquette [I-compiletime] est relative aux temps de compilation. Nous avons [un site qui suit les performances de compilation][rustc-perf] sur différentes charges utiles. L'option `-Z time-passes` du compilateur peut souvent aider à comprendre les performances. Le code Rust peut quant à lui être profilé avec les profileurs standards tels que `perf` sur Linux.

L'ajout de fonctionnalités principales s'effectue via un processus de [Request for Comments (RFC)][rfc] afin de s'accorder quant à la conception de la fonctionnalité. Bien que ce processus soit ouvert à tous, les échanges ont souvent lieu entre les développeurs qui ont déjà une certaine expérience de collaboration. Il est recommandé de s'impliquer au fur et à mesure. Soumettre une RFC sans tenir compte du contexte historique, technique ou social donnera une mauvaise impression et sera source de frustration pour la suite. Veuillez lire les fichiers readme liés ci-dessus afin de comprendre comment cet ensemble fonctionne. Dans l'histoire de Rust, de nombreuses idées ont été débattues, certaines ont été rejetées, certaines ont été remises à plus tard et le [traqueur relatif aux RFC][rfc-issues] liste certaines des idées qui n'ont pas encore vu le jour dans le langage. Peu après qu'une RFC a été acceptée pour être implémentée, elle entre dans une « phase de commentaires finaux » indiquée par l'étiquette [final-comment-period sur le dépôt rust-lang/rfcs][rfc-fcp]. De la même façon, avant qu'une fonctionnalité soit activée dans la version stable du compilateur (ce qui est appelé « *ungating* »), elle entre dans une phase de [final-comment-period sur le dépôt rust-lang/rust][issue-fcp]. Ces deux phases « FCP » sont des moments critiques pour s'impliquer et exprimer son opinion à propos de la direction du langage. Ces phases sont indiquées dans les rapports hebdomadaires des différentes équipes sur [internals.rust-lang.org].

Vous pouvez discuter avec d'autres ingénieurs travaillant sur le compilateur Rust sur le canal [#rustc], avec les concepteurs du langage sur [#rust-lang] et avec les concepteurs des bibliothèques sur [#rust-libs].

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
