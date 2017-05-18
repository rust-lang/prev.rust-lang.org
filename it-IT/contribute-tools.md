---
layout: it-IT/default
title: Contribuire a Rust &mdash; strumenti, ambienti di sviluppo e infrastrutture &middot; Linguaggio di programmazione Rust
---

# Contribuire a Rust &mdash; strumenti, ambienti di sviluppo e infrastrutture

Gli strumenti giocano una parte fondamentale nel successo di un linguaggio
e rimangono ancora molte cose da implementare.
***Al momento buona parte dello sviluppo in questa area di Rust si concentra
sul [migliorare l'esperienza con gli ambienti di sviluppo][ides]***.
Questo richiede impegno da parte di tutto l'ecosistema Rust, dal compilatore
stesso fino al tuo ambiente di sviluppo preferito.
Segui il link per saperne di più.

Sia Cargo, il gestore di pacchetti di Rust e rustdoc
il generatore di documentazione per Rust, sebbene
siano in possesso di tutte le funzioni richieste e 
validamente funzionanti, soffrono di una carenza di sviluppatori.
Rustdoc ha molte problematiche aperte sotto l'etichetta
[A-rustdoc] del repository principale.
Queste problematiche sono principalmente composte da
errori e contribuire è semplicmente una questione di 
risolverli e inviare una richiesta di unione.
Cargo, possiede [il suo personale repository con relativi problemi][Cargo],
coloro fossero interessati a contribuire possono anche presentarsi
nel canale IRC [#cargo].

Anche se Rust funziona sia con il debugger gdb che in lldb con discreti
risultati, ci sono ancora molti casi in cui la risoluzione di problemi
non funziona come previsto. L'etichetta [A-debuginfo] segue proprio
questi problemi.

Per idee su altri strumenti a cui contribuire vedi 
[awesome-rust].

Ci sono spesso altri progetti interessanti di strumenti in attesa
giusto di persone che possano venire ad implementarli.
Vieni a discuterne con gli altri appassionati di questo aspetto di
Rust su [#rust-tools].

[#cargo]: https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rustc
[#rust-tools]: https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-tools
[A-debuginfo]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-debuginfo
[A-rustdoc]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-rustdoc
[Cargo]: https://github.com/rust-lang/cargo/issues
[awesome-rust]: https://github.com/kud1ing/awesome-rust
[ides]: https://forge.rust-lang.org/ides.html
