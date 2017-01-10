---
layout: ro-RO/default
title: Contribuind la Rust &mdash; găsitul, triatul și rezolvatul problemelor &middot; Limbajul de programare Rust
---

# Contribuind la Rust &mdash; găsitul, triatul și rezolvatul problemelor

Menținerea de zi cu zi a proiectului se învârte în jurul
[issue tracker-ului] și a [pull request-urilor][PR], iar
întotdeauna este nevoie de și mai mult ajutor. Cel mai
ușor mod de a începe să contribuiți este să căutați probleme
clasate ca [E-easy] sau [E-mentor]. Acestea sunt menite să fie
ușor de abordat pentru programatori Rust începători.

În cazul problemelor [E-mentor], un dezvoltator Rust cu experiență
s-a oferit să vă mentoreze în a rezolva problema și a
[trimite un pull request pe GitHub][pull]. Vă încurajăm să îi
contactați în privința problemei pe *issue tracker* prin
[@menționare] lor într-un comentariu, pe IRC sau prin email. Luați
în considerare ca mulți dezvoltatori Rust primesc multe notificări
și că e ușor ca unele sa fie pierdute; nu ezitați să le atrageți
atenția prin orice mijloace!

Alte proiect în Rust mențin sarcini ușoare asemănătoare, inclusiv
web browser-ul [Servo], biblioteca HTTP [hyper], fortmatorul de cod
[rustfmt], legăturile bibliotecilor Unix [nix] și colecția de
*lint*-uri [clippy].

Deși Rust are [o suită de teste extensivă][test], întotdeauna mai
rămân lucruri de testat. Clasa de probleme [E-needstest] indică faptul
că aceste probleme au implementări care au nevoie de teste. Scrisul de
teste e o modalitate foarte bună de a înțelege un nou proiect și de a
începe contribuția.

Rust are întotdeauna nevoie de oameni care să trieze probleme:
reprodusul *bug*-urilor, minimizarea cazurilor de testare,
clasificarea și închiderea problemelor care sunt deja rezolvate.
Clasificarea se poate face doar în cazul în care aveți permisiuni
elevate pe GitHub, dar asta e ușor de obținut pentru cineva care are
puțină experiență cu proiectul. Întrebați un [membru de echipă][team].

Odată ce v-ați obișnuit cu proiectul și ați creat câteva
*pull request*-uri într-o zonă anume, încercați să criticați
*pull request*-urile celorlalți: abilitatea de a oferi critică de
calitate e una rară și mereu apreciată. Pentru asta nu e nevoie de
nicio permisiune &mdash; încercați doar să începeți constructiv și
politicos prin a comenta în *pull request*-urile care vă interesează.
Dacă doriți să vă atrenați simțul critic, [citiți acest ghid][reviews].

<!--
TODO: weekly triage email?
TODO: @nrc says suggesting everybody review w/o training is bad
-->

[@menționare]: https://github.com/blog/821
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
