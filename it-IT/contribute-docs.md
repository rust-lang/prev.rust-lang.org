---
layout: it-IT/default
title: Contribuire a Rust &mdash; documentazione &middot; Linguaggio di programmazione Rust
---

# Contribuire a Rust &mdash; documentazione

La documentazione non è mai abbastanza buona e non ce ne mai troppa.
Molti aspetti della documentazione di Rust non richiedono una profonda conoscenza
per essere migliorati, scritti, corretti o modificati, inoltre modificare la documentazione
è un ottimo modo per imparare Rust.
Ulteriormente i miglioramenti sono semplici da identificare e potenzialmente infiniti:
non ti piace come suona una frase? Hai scoperto qualcosa che nella documentazione non è presente?
La tua richiesta di modifica verrà calorosamente accolta.

***La documentazione più importante che tu possa scrivere è per [i pacchetti
che fanno parte dell'ecosistema di Rust][crate_docs]***.

Mentre la documentazione principale è relativamente esaustiva, ciò non è vero
per [molti pacchetti e strumenti popolari][awesome-rust] con cui gli sviluppatori
Rust interagiscono ogni giorno.
Contribuire alla documentazione delle API per un progetto popolare ti garantirà
l'affetto da parte dello sviluppatore del progetto.

[Il libro][The Book] è la documentazione base per Rust, 
scritto nel repository principale.
Possiede anche la sua etichetta per le problematiche, [A-book] ed
è continuamente in miglioramento.
Ulteriore documentazione nel repository principale include [il riferimento di Rust][The Rust Reference], 
la [documentazione della libreria standard][std], [Il Rustonomico][The Rustonomicon] (un guida su come usare `unsafe`
correttamente). Le [guide di stile di Rust][Rust Style Guidelines] sono così incomplete che non vengono elencate spesso;
uno sviluppatore ambizioso può sicuramente fare molta strada contribuendo lì.
L'[indice degli errori][err] fornisce spiegazioni dettagliate agli errori del compilatore. 
Quando vengono aggiunti nuovi errori, essi devono essere [inclusi nella documentazione][err-issue], quindi ci sono
sempre errori non presenti ancora ma con la necessità di essere aggiunti.
La maggior parte della documentazione nel repository principale risiede nella cartella [src/doc].
Tutti i problemi di documentazione sono sotto all'etichetta [A-docs] sul portale delle problematiche.
Finalmente questo documento e altri correlati con il sito web sono mantenuti nel [repository Git del sito][Rust website Git repository].
Per contribuire modificalo e manda una pull request.

Una buona quantità di documentazione importante per Rust non si trova
nel repository principale o non è mantenuta dal progetto Rust ma è 
comunque importantissima per il successo di Rust.
Esempi di documentazione eccellente sotto attivo sviluppo con domanda
per altri sviluppatori includono [Rust By Example], [Rust Design Patterns] e [rust-rosetta].
Per altri progetti di documentazione esistenti a cui contribuire vedi [rust-learning].

Incontra altri documentatori su [#rust-docs].

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
