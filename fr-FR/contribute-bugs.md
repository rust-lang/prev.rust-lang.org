---
layout: fr-FR/default
title: Contribuer à Rust &mdash; trouver, trier et corriger les problèmes &middot; Rust, le langage de programmation
---

# Contribuer à Rust &mdash; trouver, trier et corriger les problèmes

Au quotidien, les outils utilisés pour la maintenance du projet Rust sont
[le traqueur][issue tracker] et [les *pull requests*][PR], il y a toujours besoin d'un coup de main. La façon la plus simple de commencer à contribuer à Rust est de regarder les étiquettes [E-easy][E-easy] ou [E-mentor][E-mentor]. Ces *issues* sont accessibles aux personnes qui débutent en Rust.

Les problèmes étiquetés `E-mentor` indiquent qu'un développeur Rust expérimenté s'est porté volontaire dans les commentaires afin de vous guider lors de la résolution et de [l'envoi du correctif via une *pull request* GitHub][pull]. Vous pouvez les contacter pour le problème en question via le traqueur en les [@mentionnant][@mentioning] avec leur nom dans un commentaire, sur IRC ou via email. Note : les développeurs Rust reçoivent de nombreuses notifications et peuvent en louper certaines, n'hésitez pas à les relancer si nécessaire !

D'autres projets gravitant autour de Rust maintiennent des listes de tâches analogues destinées aux débutants dont : le navigateur web [Servo][Servo], la bibliothèque HTTP [hyper][hyper], le formateur de code source [rustfmt][rustfmt], la bibliothèque de liaisons Unix [nix][nix] et la suite de *linting* [clippy][clippy].

Bien que Rust dispose [d'une suite de tests étendue][test], il y a toujours des endroits à tester. L'étiquette [E-needstest][E-needstest] indique que les problèmes semblent résolus, mais qu'aucun test n'est associé. Écrire des cas de tests est un bon exercice pour comprendre un nouveau projet et commencer à contribuer.

Rust a toujours besoin de personnes pour [trier][triage] les problèmes : reproduire les bogues, réduire les cas de test, ajouter des étiquettes, fermer les problèmes résolus. Note : vous aurez besoin de certaines permissions élevées sur le projet GitHub mais cela s'obtient sans problème en demandant à quelqu'un d'expérimenté sur le projet. Demandez à [un membre de l'équipe][team].

Une fois que vous avez trouvé comment contribuer au projet et que vous avez créé quelques *pull requests* dans un domaine donné, pensez à revoir les *pull requests* d'autres contributeurs. Avoir une bonne relecture est quelque chose de rare qui est toujours apprécié ! Il n'est pas nécessaire d'avoir une quelconque autorisation préalable &mdash; il suffit de commenter de façon constructive et polie sur les *pull requests* qui vous intéressent. Si vous souhaitez en savoir plus sur les bonnes façons de relire du code, vous pouvez [lire ce guide][reviews].

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
