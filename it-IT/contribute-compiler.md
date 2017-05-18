---
layout: it-IT/default
title: Contribuire a Rust &mdash; linguaggio, compilatore e la libreria standard &middot; Linguaggio di programmazione Rust
---

# Contribuire a Rust &mdash; linguaggio, compilatore e la libreria standard

Il sorgente della libreria standard e del compilatore sono nel repository principale
ed essendo la loro manutenzione uno degli obiettivi primari di quel repository
la maggior parte dei problemi riportati nel tracciatore delle problematiche li riguardano.
Alcune tra le etichette interessanti includono [A-codegen] per la traduzione di Rust in 
rappresentazione intermedia LLVM; [A-debuginfo], la generazione di metadati utilizzata
per la risoluzione dei problemi; [A-diagnostics] per gli errori mostrati dal compilatore;
[A-libs] per i problemi con la libreria standard; [A-macros] e [A-syntaxext], entrambi
dedicati all'espansione della sintassi e [A-typesystem] per parlare dei tipi.

Non esiste alcuna guida ben mantenuta all'architettura del compilatore 
ma ne [esiste una piccola carrellata nel repository principale][rustc-guide].
La [documentazione delle API che compongono il compilatore][internals-docs]
possono aiutarti a navigare il codice, come può farlo  anche il navigatore di codice
[Rust DXR]. La [guida per Rust e la sua piattaforma di controllo][testsuite]
ti insegnerà come utilizzare al meglio il sistema di compilazione di Rust,
come farebbe anche il comando [`make tips`][tips] da riga di comando.

Per il futuro prossimo, una delle maggiori spinte per il compilatore Rust 
è convertire le sue componenti interne per lavorare direttamente dall'Albero
di Sintassi Astratto, per elaborare su una [rappresentazione intermedia chiamata MIR][mir].
Questo lavoro si preannuncia come l'inizio di molte nuove possibilità
per semplificare il compilatore ed è richiesto dell'aiuto per esempio
creare una fase di traduzione basata su MIR, ottimizzare la MIR e implementare
la compilazione incrementale.
Non esiste una fonte unica per informazioni sul lavoro richiesto ma chiedi
su [internals.rust-lang.org] o su
[#rust-internals] per ulteriori informazioni.

[È imbarazzante se il nostro compilatore si blocca][ice] &mdash; il 
nefasto 'errore interno di compilazione' ('internal compiler error' ICE). 
L'etichetta [I-ICE] tiene traccia di questi problemi e spesso sono molti.
Questi sono usualmente degli ottimi problemi per iniziare a contribuire
perché è facile comprendere quando vengono riparati e sono relativamente
isolati dal resto del codice.

Le prestazioni di Rust è uno dei suoi più grossi vantaggi; e quella
del suo compilatore è uno dei suoi più grossi problemi.
Ogni miglioramento della prestazione degli eseguibili or &mdash; specialmente &mdash;
delle prestazioni del compilatore sono largamente celebrati.
Le etichette [I-slow] e [A-optimization] si occupano delle prestazioni
degli eseguibili e [I-compiletime] di quella del compilatore. Abbiamo un
[sito che tiene traccia delle prestazioni del compilatore][rustc-perf] 
durante alcuni carichi di lavoro.
L'opzione da riga di comando `-Z time-passes` può aiutare a ispezionare
le prestazioni del compilatore e il codice di Rust può essere profilato
da tutti i profilatori standard come `perf` su Linux.

Funzionalità importanti passano attraverso il processo di [Request for Comments (RFC)][rfc]
dal quale ci si accorda sul design. Anche se aperto a tutti è un'interazione tra
sviluppatori che già hanno un discreto quantitivo di esperienza in comune ed è 
consigliato prendere parte al processo lentamente &mdash;
inviare una RFC ostica senza comprendere il contesto storico, tecnico o sociale
è un modo facile per dare una cattiva impressione e andarsene delusi.
Leggi il collegamento sopra per comprendere al meglio come tutto il sistema funziona.
Molte idee sono già state discusse durante il passato di Rust, alcune sono state
rifiutate, altre rimandate al futuro e la [portale delle RFC][rfc-issues]
elenca alcune idee desiderate che non sono ancora riuscite a entrare
nel linguaggio.
Poco prima che una RFC venga accettata per l'implementazione, entra in 
una 'fase finale di commento' indicata dall'etichetta
[final-comment-period sul repository rust-lang/rfcs][rfc-fcp].
Similarmente, prima che una funzionalità venga abilitata nel compilatore
stabile (procedura definita 'ungating') essa entra nella [final-comment-period sul repository rust-lang/rust][issue-fcp]. 
Entrambe le fasi di commento finale sono momenti critici per essere coinvolti
ed esprimere opinioni sulla direzione del linguaggio e sono indicate
nei report settimanali dei vari gruppi di lavoro su [internals.rust-lang.org].

Incontra altri progettisti di compilatori su [#rustc], progettisti del linguaggio
su [#rust-lang] e progettisti di librerie su [#rust-libs].

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
