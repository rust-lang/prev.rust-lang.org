---
layout: it-IT/default
title: Contribuire a Rust &mdash; trovare, analizzare e risolvere problemi &middot; Linguaggio di programmazione Rust
---

# Contribuire a Rust &mdash; trovare, analizzare e risolvere problemi

La manutenzione giorno per giorno del progetto ruota attorno al 
[controllo bug][issue tracker] e alle [richieste di unione][PR],
una mano in più è sempre richiesta.

Il modo più semplice per iniziare a contribuire a Rust è leggere le
descrizioni dei bug con le etichette [E-easy] o [E-mentor].
Questi bug sono stati raccolti con la finalità di essere accessibili
anche per i programmatori di Rust alla loro prima contribuzione.

Nelle problematiche identificate da `E-mentor` uno sviluppatore di Rust
esperto si offre volontario di aiutarti nella risoluzione del problema e
a [spedire le tue modifiche con una richiesta di pull su GitHub][pull].
Potrai contattarlo per discutere del problema direttamente dalla pagina
di controllo bug [@menzionando][@mentioning] dal nome nel commento,
su IRC o tramite email. Nota che ricevendo molte notifiche gli sviluppatori
Rust potrebbero non vedere il tuo messaggio; non esitare quindi a cercare
di contattarli a tutti i costi!

Molti altri progetti in Rust mantengono una lista simile di problematiche
facili, includendo il browser web [Servo], la libreria [hyper], il suggeritore
di sintassi [rustfmt], la libreria di supporto a Unix [nix] e la collezione
di aiuti automatici alla programmazione [clippy].

Anche se Rust possiede una [comprensiva suite di prova][test] c'è 
sempre di più da controllare.
I problemi classificati come [E-needstest] indicano le problematiche
che si pensano risolte ma non possiedono ancora delle verifiche automatiche.
Scrivere queste verifiche è un ottimo modo per comprendere un nuovo progetto
e per iniziare a contribuire a esso.

Rust è continuamente alla ricerca di persone che [analizzino][triage] problemi:
riprodurli, ridurre la suite di prove automatiche, applicare etichette, chiudere
i problemi risolti.
Nota che per applicare etichette è richiesta un'autorizzazione elevata su GitHub
facilmente ottenibile per coloro che hanno un po' di esperienza con il progetto.
Chiedi a un [membro del team][team].

Una volta che ti sarai orientato nel progetto, creerai una serie di pull request 
in una particolare area. Considera di iniziare a visionare le richieste degli altri:
la capacità di saper valutare il codice altrui è un'abilità rara e sempre
apprezzata. Non è richiesta alcuna autorizzazione &mdash; inizia semplicemente
a commentare gentilmente e costruttivamente le pull request che t'interessano.
Se vuoi più informazioni su come fare una buona valutazione del codice altrui
segui [questa guida][reviews].

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
