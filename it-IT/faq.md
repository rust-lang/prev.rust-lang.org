---
layout: it-IT/basic
title: Domande Frequenti &middot; Linguaggio di programmazione Rust
---

# Domande Frequenti

<p class="faq-intro">
Questa pagina risponde a domande comuni inerenti il linguaggio di programmazione Rust.
Non rappresenta una guida completa al linguaggio e nemmeno uno strumento per insegnarlo.
Costituisce invece un riferimento per rispondere alle domande più frequenti concernenti le scelte di progettazione su cui Rust si basa.
</p>

<p class="faq-intro">
Se c'è una domanda comune o importante che pensi sia ingiustamente esclusa qui, sentiti libero di <a href="https://github.com/rust-lang/rust-www/blob/master/CONTRIBUTING.md">aiutarci ad aggiungerla</a>.
</p>

<div id="toc">
    <h2>Indice</h2><a href="#toggle-toc"></a>
    <div class="contents">
        <ol id="toc-contents">
            <li><a href="#project">Il progetto Rust</a></li>
            <li><a href="#performance">Prestazione</a></li>
            <li><a href="#syntax">Sintassi</a></li>
            <li><a href="#numerics">Numeri</a></li>
            <li><a href="#strings">Strings</a></li>
            <li><a href="#collections">Collezioni</a></li>
            <li><a href="#ownership">Possesso</a></li>
            <li><a href="#lifetimes">Campo di esistenza</a></li>
            <li><a href="#generics">Generici</a></li>
            <li><a href="#input-output">Input / Output</a></li>
            <li><a href="#error-handling">Gestione Errori</a></li>
            <li><a href="#concurrency">Concorrenza</a></li>
            <li><a href="#macros">Macro</a></li>
            <li><a href="#debugging">Debugging e Strumentazione</a></li>
            <li><a href="#low-level">Basso Livello</a></li>
            <li><a href="#cross-platform">Multipiattaforma</a></li>
            <li><a href="#modules-and-crates">Moduli e Pacchetti</a></li>
            <li><a href="#libraries">Librerie</a></li>
            <li><a href="#design-patterns">Paradigmi di progettazione</a></li>
            <li><a href="#other-languages">Altri Linguaggi</a></li>
            <li><a href="#documentation">Documentazione</a></li>
        </ol>
    </div>
</div>


<h2 id="project">Il progetto Rust</h2>

<h3><a href="#what-is-this-projects-goal" name="what-is-this-projects-goal">
Qual è lo scopo del progetto?
</a></h3>

Progettare e implementare un linguaggio sicuro, concorrente e pratico per la programmazione di sistemi.

Rust nasce perchè altri linguaggi a questo livello di astrazione e efficienza non sono soddisfacenti. In particolare:

1. Non viene posta la dovuta attenzione alla sicurezza.
2. Supportano malamente la concorrenza.
3. Vi è una carenza di ergonomia.
4. Offrono un controllo limitato delle risorse.

Rust esiste come un'alternativa che fornisce sia codice efficiente che un livello confortevole di astrazione, contemporaneamente migliorando tutti questi quattro punti.
<h3><a href="#is-this-project-controlled-by-mozilla" name="is-this-project-controlled-by-mozilla">
Questo progetto è controllato da Mozilla?
</a></h3>

No. Rust ebbe inizio nel 2006 come un progetto hobbystico di Graydon Hoare ed è rimasto così per oltre 3 anni. Mozilla è entrata nel 2009, dopo che il linguaggio si è dimostrato abbastanza maturo per eseguire una serie di test di base automatizzati e dimostrare la valenza dei suoi principi base. Anche se sponsorizzato da Mozilla, Rust è un progetto sviluppato da
una variegata comunità di appassionati da molti paesi del mondo. Il [Team di Rust](team.html) è composto sia da membri Mozilla che da esterni e `rust` su GitHub ha avuto oltre [1900 sviluppatori diversi](https://github.com/rust-lang/rust/) fino a oggi.

Finché concesso [dalla politica di gestione del progetto](https://github.com/rust-lang/rfcs/blob/master/text/1068-rust-governance.md), Rust è amministrato da un team base che imposta
la visione e le priorità del progetto, guidandolo globalmente.
Esistono anche dei sottogruppi per guidare e incoraggiare lo sviluppo in alcune aree di interesse, inclusi il linguaggio, il compilatore, le librerie, gli strumenti, la moderazione delle comunità ufficiali.
La progettazione è guidata da un processo [RFC](https://github.com/rust-lang/rfcs).

Per cambiamenti che non richiedono una RFC, le decisioni sono fatte attraverso richieste di unione sul [repository `rustc`](https://github.com/rust-lang/rust).

<h3><a href="#what-are-some-non-goals" name="what-are-some-non-goals">
WQuali sono i non-obiettivi di Rust?
</a></h3>

1. Non impieghiamo nessuna tecnologia particolarmente nuova. Le vecchie tecniche ben consolidate sono ancora le migliori.
2. Non ci preme incentivare l'espressività, il minimalismo o l'eleganza sopra ogni altra cosa. Sono obiettivi desiderati ma non fondamentali.
3. Non ci interessa coprire tutte le funzionalità del C++ o di qualsiasi altro linguaggio. Rust dovrebbe fornire le funzionalità più richieste.
4. Non vogliamo essere immobili al 100%, sicuri al 100%, riflessivi al 100% o troppo dogmatici in qualche modo. Esistono dei compromessi.
5. Non chiediamo che Rust funzioni "su ogni piattaforma possibile". Dovrà eventualmente funzionare senza inutili compromessi sulle piattaforme hardware e software più diffuse.

<h3><a href="#how-does-mozilla-use-rust" name="how-does-mozilla-use-rust">
In quali progetti Mozilla utilizza Rust?
</a></h3>

Principalmente in [Servo](https://github.com/servo/servo), un motore di navigazione sperimentale a cui Mozilla sta lavorando. Stanno anche lavorando per [integrare componenti Rust](https://bugzilla.mozilla.org/show_bug.cgi?id=1135640) in Firefox.

<h3><a href="#what-examples-are-there-of-large-rust-projects" name="what-examples-are-there-of-large-rust-projects">
Che esempi ci sono di grandi progetti in Rust?
</a></h3>

I due più grandi progetti open source sono al momento [Servo](https://github.com/servo/servo) e il [compilatore Rust](https://github.com/rust-lang/rust) stesso.

<h3><a href="#who-else-is-using-rust" name="who-else-is-using-rust">
Chi altro utilizza Rust?
</a></h3>

[Un crescente numero di organizzazioni!](friends.html)

<!--
### What projects are good examples of idiomatic Rust code?

TODO: Write this answer.
-->

<h3><a href="#how-can-i-try-rust-easily" name="how-can-i-try-rust-easily">
Come posso provare Rust facilmente?
</a></h3>

Il modo più semplice per provare Rust è con la [playpen](https://play.rust-lang.org/), un'applicazione online per scrivere e provare codice Rust. Se invece desideri provare Rust sul tuo computer, [installalo](https://www.rust-lang.org/install.html) e prova a seguire la guida al [gioco dell'indovino](https://doc.rust-lang.org/stable/book/guessing-game.html) dal libro.

<h3><a href="#how-do-i-get-help-with-rust-issues" name="how-do-i-get-help-with-rust-issues">
Come posso ricevere aiuto con Rust?
</a></h3>

Ci sono diversi modi. Puoi:

- Scrivere su [users.rust-lang.org](https://users.rust-lang.org/), il forum ufficiale di Rust
- Chiedere sul [canale IRC ufficiale di Rust](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust) (#rust on irc.mozilla.org)
- Chiedere su [Stack Overflow](https://stackoverflow.com/questions/tagged/rust) with the "rust" tag
- Scrivere su [/r/rust](https://www.reddit.com/r/rust), il subreddit non ufficiale di Rust

<h3><a href="#why-has-rust-changed-so-much" name="why-has-rust-changed-so-much">
Perché Rust è cambiato così tanto col tempo?
</a></h3>

L'obiettivo iniziale di Rust fu quello di creare un linguaggio di programmazione per sistemi stabile e usabile. Per perseguire questo scopo molte idee sono state esplorate, alcune sono state preservate (campi di esistenza, tratti) mentre altre sono state scartate (il sistema di stati dei tipi, il green threading).
Inoltre durante la transizione verso la versione 1.0, buona parte della libreria standard è stata riscritta per consentire al codice passato di sfruttare al meglio le funzionalità di Rust,
fornendo interfacce di programmazione di qualità, stabili e multipiattaforma.
Ora che Rust è alla versione 1.0, il linguaggio è garantito come "stabile"; e mentre potrebbe continuare a evolversi, il codice funzionante sulla versione attuale dovrebbe continuare a farlo
anche nelle versioni future.

<h3><a href="#how-does-rust-language-versioning-work" name="how-does-rust-language-versioning-work">
Come funziona il numero di versione di Rust?
</a></h3>

Rust segue lo standard [SemVer](http://semver.org/), dove cambiamenti non compatibili con le versioni passate sono ammesse nelle versioni minori se questi cambiamenti
risolvono errori del compilatore, risolvono problemi di sicurezza o cambiano le regole di dichiarazione o inferenza dei tipi in modo da richiedere ulteriore specifica.
Linee guida più dettagliate per cambi di versione minori sono disponibili come RFC approvate sia per il [linguaggio](https://github.com/rust-lang/rfcs/blob/master/text/1122-language-semver.md) che per la [libreria standard](https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md).

Rust mantiene tre canali di rilascio: stabile, beta e "nightly".
I canali stabile e beta sono aggiornati ogni sei settimane, con la nightly che diviene la nuova beta e la beta che diviene la nuova stabile.
Le funzionalità del linguaggio e della libreria standard indicate come non stabili o nascoste dietro a blocchi di implementazione possono essere utilizzati solo
nel canale di rilascio "nightly". Le nuove funzionalità arrivano come instabili ma sono "liberate" una volta approvate dal team di sviluppo e relativi sottogruppi.
Questo approccio consente di sperimentare e di contemporaneamente fornire una forte garanzia di retrompatibilità del canale stabile.

Per dettagli ulteriori, leggi il post sul blog di Rust ["Stability as a Deliverable."](http://blog.rust-lang.org/2014/10/30/Stability.html)

<h3><a href="#can-i-use-unstable-features-on-the-beta-or-stable-channel" name="can-i-use-unstable-features-on-the-beta-or-stable-channel">
Posso utilizzare funzionalità non stabili nei canali stabile e beta?
</a></h3>

No, non puoi. Rust cerca duramente di fornire garanzie sulla stabilità delle funzioni incluse nei canali beta e stabile.
Quando qualcosa è non stabile significa che non è possibile ancora garantire il suo utilizzo e quindi non desideriamo che ci
si basi su queste funzionalità o che queste non vengano modificate.
Questo ci permette di provare i cambiamenti nel canale nightly, preservando le promesse di stabilità.

Molte cose vengono incluse nella stabile e i canali beta e stabile vengono aggiornati ogni sei settimane, con occasionali modifiche dirette anche al canale beta a volte.
Se stai aspettando la disponbilità di una funzionalità e non vuoi usufruire del canale nighly, puoi seguirne lo stato controllando l'etichetta [`B-unstable`](https://github.com/rust-lang/rust/issues?q=is%3Aissue+is%3Aopen+tracking+label%3AB-unstable) sulla bacheca dei problemi.

<h3><a href="#what-are-feature-gates" name="what-are-feature-gates">
Cosa sono i "Feature Gates"?
</a></h3>

I "Feature gates" sono il modo con cui Rust stabilizza funzionalità del compilatore, del linguaggio e della libreria standard.
Una funzione protetta è accessibile esclusivamente nel canale di rilascio "nighly" e solo quando abilitata esplicitamente con la direttiva `#[feature]` o con l'argomento a linea di comando `-Z unstable-options`.
Quando una funzione è stabilizzata diviene disponbile sul canale di rilascio stabile e non è necessario abilitarla esplicitamente.
A quel punto la funzione è considerata "libera".
I "Feature gates" consentono agli sviluppatori di provare funzionalità sperimentali mentre sono in fase di implementazione, prima che giungano nel linguaggio stabile.

<h3><a href="#why-a-dual-mit-asl2-license" name="why-a-dual-mit-asl2-license">
Perchè la doppia licenza MIT/ASL2?
</a></h3>

La licenza Apache include importanti protezioni contro le aggressioni legali ma non è compatibile con la GPL versione 2.
Per evitare problemi nell'utilizzo di Rust e GPL2 è stata aggiunta la licenza alternativa MIT.

<h3><a href="#why-a-permissive-license" name="why-a-permissive-license">
Perchè una licenza simil-BSD aperta invece che la MPL o la tri-licenza?
</a></h3>

Questo è parzialmente dovuto alla preferenza dello sviluppatore originario (Graydon) e parzialmente al fatto che i linguaggi tendono ad avere un pubblico più vasto e una serie più variegata di implementazioni e utilizzi di altri prodotti come i browser web. Noi vorremmo appellarci al maggior numero possibile di sviluppatori.

<h2 id="performance">Prestazioni</h2>

<h3><a href="#how-fast-is-rust" name="how-fast-is-rust">
Quanto è performante Rust?
</a></h3>

Molto! Rust è già competitivo con programmi C e C++ ben scritti in una serie di prove (come nel [Benchmarks Game](https://benchmarksgame.alioth.debian.org/u64q/compare.php?lang=rust&lang2=gpp) e [altri](https://github.com/kostya/benchmarks)).

Come il C++, il Rust possiede [astrazioni a costo zero](http://blog.rust-lang.org/2015/05/11/traits.html) come uno dei suoi principi chiave: nessuna delle astrazioni di Rust impone un rallentamento, in qualsiasi caso.

Dato che Rust utilizza LLVM e cerca di assomigliare a Clang dal punto di vista dell'interazione con LLVM, ogni miglioramento di LLVM è condiviso da Rust.
Nel lungo periodo, la quantità elevata di informazioni presente nel sistema dei tipi di Rust dovrebbe permettere ottimizzazioni difficili o impossibili da implementare in C/C++.

<h3><a href="#is-rust-garbage-collected" name="is-rust-garbage-collected">
Rust è un linguaggio dotato di garbage collection?
</a></h3>

No. Una delle innovazioni fondamentali di Rust è il garantire la sicurezza della memoria (nessun errore di segmentazione) *senza* richiedere un garbage collector.

Evitando di utilizzare un GC, Rust offre numerosi vantaggi: liberazione prevedibile delle risorse, gestione della memoria meno onerosa e soprattutto nessun sistema aggiuntivo operante durante l'esecuzione.
Tutte queste caratteristiche rendono Rust leggero e facile da implementare in contesti arbitrari e rendono più facile [integrare Rust con i linguaggi in possesso di un GC](http://calculist.org/blog/2015/12/23/neon-node-rust/).

Rust non necessita di un GC grazie al suo sistema di possesso e passaggio ma lo stesso sistema aiuta con una moltitudine di altri problemi, inclusi [la gestione delle risorse in generale](http://blog.skylight.io/rust-means-never-having-to-close-a-socket/)
e la [concorrenza](http://blog.rust-lang.org/2015/04/10/Fearless-Concurrency.html).

Per quando il possesso di un valore non fosse abbastanza, i programmi Rust fanno riferimento al tipo puntatore intelligente standard a conteggio dei riferimenti [`Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html) e alla sua versione sicura in contesti paralleli [`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html), invece di affidarsi a un GC.

Stiamo comunque investigando una garbage collection *opzionale* come estensione futura.
L'obiettivo è integrarsi fluidamente con ambienti garbage-collected, come quelli offerti dai
motori di Javascript [Spidermonkey](https://developer.mozilla.org/en-US/docs/Mozilla/Projects/SpiderMonkey) e [V8](https://developers.google.com/v8/?hl=en).
Inoltre, qualcuno sta investigando l'implementazione di un garbage collector
[interamente in Rust](https://manishearth.github.io/blog/2015/09/01/designing-a-gc-in-rust/)
senza supporto del compilatore.

<h3><a href="#why-is-my-program-slow" name="why-is-my-program-slow">
Perché il mio programma è lento?
</a></h3>

Il compilatore di Rust non compila con le ottimizzazioni se non esplicitamente istruito [considerato che le ottimizzazioni rallentano la compilazione e sono scosigliate durante lo sviluppo](https://users.rust-lang.org/t/why-does-cargo-build-not-optimise-by-default/4150/3).

Se compili con `cargo` usa il parametro `--release`.
Se compili direttamente con `rustc`, usa il parametro `-O`.
Ciascuno di questi abiliterà le ottimizzazioni.

<h3><a href="#why-is-rustc-slow" name="why-is-rustc-slow">
Rust compila lentamente. Perchè?
</a></h3>

Principalmente per la traduzione del codice e le ottimizzazioni.
Rust fornisce delle astrazioni ad alto livello che vengono compilate in un codice macchina efficiente e queste trasformazioni richiedono molto tempo per essere effettuate, specialmente se in combinazione con le ottimizzazioni.

Ma il tempo di compilazione di Rust non è male come sembra e c'è da essere fiduciosi in un suo miglioramento futuro.
Comparando progetti di dimensioni simili tra il C++ e Rust il tempo di compilazione è generalmente comparabile.
La percezione di lentezza è largamente dovuta alle differenze tra il *modello di compilazione* del C++ da quello di Rust, l'unità base del C++ è il file mentre in Rust è l'intero pacchetto, composto da molti file.
Di conseguenza durante lo sviluppo la modifica di un file causa molta meno ricompilazione che in Rust.
In questo momento è in corso uno sforzo per implementare una ristrutturazione del compilatore che permetta di effettuare la
[compilazione incrementale](https://github.com/rust-lang/rfcs/blob/master/text/1298-incremental-compilation.md), che consentirà a Rust di implementare un modello più rapido e simile al C++.

Oltre al modello di compilazione, ci sono molti altri aspetti dell'implementazione di Rust e il suo compilatore che ne impattano le prestazioni in fase di compilazione.

Inizialmente, Rust has un sistema di tipi moderatamente complesso e deve spendere un discreto quantitativo di tempo durante la compilazione a verificarne limitazioni e utilizzi, rendendo Rust sicuro durante la sua esecuzione.

Secondariamente, il compilatore di Rust soffre di un debito tecnico di lunga data e risaputamente genera una rappresentazione intermedia per LLVM di bassa qualità a cui LLVM deve porre rimedio.
Si spera che i futuri passaggi di trasformazione e ottimizzazione [basati su MIR](https://github.com/rust-lang/rfcs/blob/master/text/1211-mir.md) riducano la quantità di lavoro che Rust impone a LLVM.

Terziariamente, l'utilizzo da parte di Rust di LLVM per la generazione del codice macchina è una spada a doppio taglio: se da un lato questo permette a Rust di avere prestazioni degne di nota,
LLVM è un insieme di strumenti non focalizzato alla velocità di compilazione, specialmente con input di bassa qualità.

Inoltre, mentre la strategia preferita da Rust per monomorfizzare i generici (simil C++) produca codice performante, domanda di generare molto più codice rispetto ad altre strategie di implementazione.
I programmatori Rust possono utilizzare i tratti per rimuovere questo codice extra e utilizzando un dispacciamento dinamico.

<h3><a href="#why-are-rusts-hashmaps-slow" name="why-are-rusts-hashmaps-slow">
Perché le <code>HashMap</code> di Rust sono così lente?
</a></h3>

Di base, le [`HashMap`][HashMap] di Rust utilizzano l'algoritmo di hashing [SipHash](https://131002.net/siphash/), progettato per prevenire [attacchi basati sulla collisione di hash](http://programmingisterrible.com/post/40620375793/hash-table-denial-of-service-attacks-revisited) fornendo comunque [un livello di prestazione accettabile in una varietà di compiti](https://www.reddit.com/r/rust/comments/3hw9zf/rust_hasher_comparisons/cub4oh6).

Mentre SipHash [dimostra prestazioni competitive](http://cglab.ca/%7Eabeinges/blah/hash-rs/) in molti casi, un caso in cui è conclamatamente lento è in presenza di chiavi corte, come ad esempio i numeri interi.
Questo perché i programmatori Rust spesso incontrano problemi prestazionali con [`HashMap`][HashMap].
[FNV hasher](https://crates.io/crates/fnv) è spesso consigliato per queste casistiche, tenendo comunque in considerazione che non possiede le stesse caratteristiche di protezione dagli attacchi a collisione di SipHash.

<h3><a href="#why-is-there-no-integrated-benchmarking" name="why-is-there-no-integrated-benchmarking">
Perchè non esiste una piattaforma integrata di misurazione delle prestazioni?
</a></h3>

Esiste ma è riservata al canale di rilascio "nightly".
Progettiamo di costituire un sistema integrato e modulare di misurazione delle prestazioni ma nel frattempo il sistema attuale è [considerato instabile](https://github.com/rust-lang/rust/issues/29553).

<h3><a href="#does-rust-do-tail-call-optimization" name="does-rust-do-tail-call-optimization">
Rust ottimizza le tail-call?
</a></h3>

In generale, no.
Le ottimizzazioni delle tail-call sono riservate ad alcune [condizioni particolari](http://llvm.org/docs/CodeGenerator.html#sibling-call-optimization)
ma [non è assicurato](https://mail.mozilla.org/pipermail/rust-dev/2013-April/003557.html).
Siccome è sempre stata una funzionalità desiderata, Rust ha riservato l'identificatore (`become`),
anche se non è ancora chiaro se sia possibile questa ottimizzazione o se verrà implementata.
Era stata [proposta un'estensione](https://github.com/rust-lang/rfcs/pull/81) che avrebbe permesso di eliminare le tail-call in alcuni casi ma al momento è stata rimandata.

<h3><a href="#does-rust-have-a-runtime" name="does-rust-have-a-runtime">
Rust ha un ambiente di esecuzione?
</a></h3>

Non nel senso tipico utilizzato da linguaggi come il Java ma componenti della libreria standard di Rust possono essere considerati un "ambiente di esecuzione", fornendo controlli per heap, backtrace, unwinding, e stack.
C'è un piccolo quantitativo [di codice di inizializzazione](https://github.com/rust-lang/rust/blob/33916307780495fe311fe9c080b330d266f35bfb/src/libstd/rt.rs#L43) che viene eseguito prima della funzione `main`.
La libreria standard di Rust inoltre è collegata alla libreria standard del C, che effettua una simile [inizializzazione](http://www.embecosm.com/appnotes/ean9/html/ch05s02.html).
Il codice di Rust può essere compilato anche senza la libreria standard, in questo caso il suo ambiente è circa equivalente a quello del C.

<h2 id="syntax">Sintassi</h2>

<h3><a href="#why-curly-braces" name="why-curly-braces">
Perchè le parentesi graffe? La sintassi di Rust non poteva ispirarsi a Haskell o a Python?
</a></h3>

L'utilizzo delle graffe per indicare blocchi di codice è una scelta comune in una moltitudine di linguaggi di programmazione e l'adesione di Rust allo standard è utile per le persone già familiari con lo stile.

Le graffe consentono inoltre una sintassi più flessibile per il programmatore e per un preprocessore più semplice nel compilatore.

<h3><a href="#why-brackets-around-blocks" name="why-brackets-around-blocks">
Posso omettere le parentesi nelle condizioni degli <code>if</code>,
perché allora devo mettere delle parentesi attorno a righe di codice singole?
Perché non è ammesso lo stile del C?
</a></h3>

Mentre il C richiede parentesi obbligatorie per le condizioni dell'istruzione `if` ma rende le parentesi del corpo opzionali,
Rust fa la scelta opposta per i suoi `if`.
Questo lascia la condizione separata chiaramente dal corpo e evita il pericolo delle parentesi opzionali,
che possono condurre in inganno durante la modifica del codice, come il famoso errore [goto fail](https://gotofail.com/) commesso da Apple.

<h3><a href="#why-no-literal-syntax-for-dictionaries" name="why-no-literal-syntax-for-dictionaries">
Perché non c'è una sintassi letterale per i dizionari?
</a></h3>

Le scelte stilistiche di Rust sono per limitare la dimensione del *linguaggio* contemporaneamente sfruttando potenti *librerie*.
Anche se Rust fornisce una sintassi per inizializzare array e costanti letterali stringa, questi sono gli unici tipi collezione inseriti nel linguaggio.
Tutti gli altri tipi definiti da librerie, incluso l'onnipresente [`Vec`][Vec] utilizzano macro per la loro inizializzazione, ad esempio la macro [`vec!`][VecMacro].

La scelta di utilizzare le macro di Rust per facilitare l'inizializzazione di collezioni verrà probabilmente estesa a altre collezioni in futuro, permettendo inizializzazioni semplici non solo di
[`HashMap`][HashMap] e [`Vec`][Vec], ma anche altri tipi di collezioni come [`BTreeMap`][BTreeMap].
Nel frattempo, se vuoi una sintassi più comoda per inizializzare le collezioni, puoi [creare la tua macro](https://stackoverflow.com/questions/27582739/how-do-i-create-a-hashmap-literal) per fornirla.

<h3><a href="#when-should-i-use-an-implicit-return" name="when-should-i-use-an-implicit-return">
Quando dovrei utilizzare un ritorno implicito?
</a></h3>

Rust è un linguaggio molto espression-centrico e i ritorni impliciti fanno parte del suo design.
Costrutti come gli `if`, i `match`e i normali blocchi sono tutte espressioni in Rust.

Ad esempio, il codice seguente controlla se un [`i64`][i64] è dispari, ritornando il risultato semplicamente fornendone il risultato:

```rust
fn dispari(x: i64) -> bool {
    if x % 2 != 0 { true } else { false }
}
```

Anche se può essere ulteriormente semplificato come qui:


```rust
fn dispari(x: i64) -> bool {
    x % 2 != 0
}
```

In ciascun esempio, l'ultima riga rappresenta il valore di ritorno della funzione.
Risulta importante specificare che se una funzione termina con un punto e virgola il suo tipo di ritorno sarà `()`, indicando nessun valore di ritorno.
I ritorni impliciti funzionano quindi esclusivamente in assenza del punto e virgola.

I ritorni espliciti sono utilizzati solo se un ritorno implicito risulta impossibile perchè si desidera ritornare un valore prima della fine del corpo della funzione.
Mentre ciascuna delle funzioni sopra avrebbe potuto includere un `return` e un punto e virgola, questa aggiunta sarebbe inutilmente prolissa e inconsistente con le convenzioni del codice Rust.

<h3><a href="#why-arent-function-signatures-inferred" name="why-arent-function-signatures-inferred">
Perchè i tipi delle funzioni non sono dedotti automaticamente?
</a></h3>

In Rust le dichiarazioni sono tendelzialmente accompagnate da un tipo esplicito, mentre nel codice i tipi vengono dedotti.
Ci sono multiple ragioni per questa scelta:

- Dichiarazioni obbligatorie dei tipi obbligano a mantenere un'interfaccia stabile a livello di modulo e di pacchetto.
- I tipi facilitano la comprensione del codice per il programmatore, eliminando la necessità di un ambiente di sviluppo che possa desumere il tipo analizzando il codice dell'intero pacchetto:
il tipo è sempre disponibile nelle vicinanze e in modo esplicito.
- Semplifica gli algoritmi che gesticono la deduzione dei tipi, visto che può essere analizzata una funzione alla volta.

<h3><a href="#why-does-match-have-to-be-exhaustive" name="why-does-match-have-to-be-exhaustive">
Perché <code>match</code> deve essere così completo?
</a></h3>

Per assistere nelle modifiche e in chiarezza.

Prima di tutto, se ogni possibilità viene coperta da un `match`, l'aggiunta di varianti a un `enum` causerà un errore di compilazione, invece che un problema durante l'esecuzione.
Questo tipo di assistenza permette di modificare il codice Rust liberamente e senza paura.

Secondariamente, il controllo dettagliato rende esplicito il caso predefinito: in generale l'unico modo per creare un `match` non esaustivo sarebbe di far andare in errore il processo se non viene incontrato alcun valore previsto.
Le versioni iniziali di Rust non prevedevano la completezza di `match` ed è stato accertato che questa scelta ha causato una moltitudine di problematiche.


Risulta comunque semplice ignorare tutti i casi non specificati usando il carattere speciale `_`:

```rust
match val.fai_qualcosa() {
    Gatto(a) => { /* ... */ }
    _        => { /* ... */ }
}
```

<h2 id="numerics">Numerici</h2>

<h3><a href="#which-type-of-float-should-i-use" name="which-type-of-float-should-i-use">
Quale dovrei scegliere tra <code>f32</code> e <code>f64</code> per i calcoli in virgola mobile?
</a></h3>

La scelta dipende dallo scopo del programma.

Se ciò che conta è la precisione massima, [`f64`][f64] è da preferire.
Se invece si vuole minimizzare l'impatto in memoria e l'efficienza, ignorando la precisione persa, [`f32`][f32] è più appropriato.
Svolgere operazioni sugli [`f32`][f32] è generalmente più veloce, anche in piattaforme a 64-bit.
Come esempio, la grafica sfrutta tipicamente gli [`f32`][f32] perché richiede alte prestazioni e i numeri in virgola mobile a 32-bit bastano per rappresentare i pixel a schermo.

Nel dubbio, scegli [`f64`][f64] per una maggiore precisione.

<h3><a href="#why-cant-i-compare-floats" name="why-cant-i-compare-floats">
Perchè non posso comparare i numeri a virgola mobile o usarli come chiave per un <code>HashMap</code> o un <code>BTreeMap</code>?
</a></h3>

I numeri a virgola mobile si possono comparare con gli operatori `==`, `!=`, `<`, `<=`, `>`, e `>=` , e attraverso la funzione `partial_cmp()`.
`==` e `!=` possiedono il tratto [`PartialEq`][PartialEq], mentre `<`, `<=`, `>`, `>=`, e `partial_cmp()` possiedono il tratto [`PartialOrd`][PartialOrd].

I numeri a virgola mobile non sono confrontabili con la funzione `cmp()`, possidente il tratto [`Ord`][Ord], dato che i numeri a virgola mobile non costituiscono insieme totalmente ordinato.
Inoltre non esiste relazione di uguaglianza completa numeri a virgola mobile e quindi non implementano il tratto[`Eq`][Eq].

Non esiste l'ordinamento totale o l'uguaglianza tra numeri a virgola mobile a causa del valore [`NaN`](https://en.wikipedia.org/wiki/NaN) che non è minore, maggiore o uguale di alcun altro numero o se stesso.

Visto che i numeri a virgola mobile non implementano i tratti [`Eq`][Eq] o [`Ord`][Ord], non sono utilizzabili nei tipi le cui limitazioni esigono l'implementazione di queste caratteristiche, come [`BTreeMap`][BTreeMap] o [`HashMap`][HashMap].
Questo è importante perché questi tipi *suppongono* che le chiavi forniscano relazioni di ordinamento totale o di uguaglianza, a pena di malfunzionamenti.

Esiste [un pacchetto](https://crates.io/crates/ordered-float) che racchiude [`f32`][f32] e [`f64`][f64] in modo da fornire i tratti [`Ord`][Ord] e [`Eq`][Eq] che potrebbe assistere in certi casi.

<h3><a href="#how-can-i-convert-between-numeric-types" name="how-can-i-convert-between-numeric-types">
Come posso convertire tra i tipi numerici?
</a></h3>

Ci sono due modi: la parola chiave `as`, che esegue una semplice conversione per tipi primitivi e i tratti [`Into`][Into] e [`From`][From],
che sono implementati per una serie di conversioni tra tipi numerici (e che anche tu puoi implementare sui tuoi tipi).
I tratti [`Into`][Into] e [`From`][From] sono implementati esclusivamente per le conversioni prive di perdita, qundi ad esempio, `f64::from(0f32)` funziona mentre `f32::from(0f64)` no.
D'altro canto, `as` convertirà tra qualsiasi coppia di tipi primitivi, effettuando i necessari troncamenti..

<h3><a href="#why-doesnt-rust-have-increment-and-decrement-operators" name="why-doesnt-rust-have-increment-and-decrement-operators">
Perchè Rust non possiede operatori per il decremento e incremento?
</a></h3>

Gli operatori di preincremento e postincremento (e relativi opposti equivalenti), anche se convenienti presentano una discreta complessità;
Richiedono una conoscenza dell'ordine di esecuzione e spesso conducono a piccoli problemi e comportamenti anormali in C e C++.
`x = x + 1` o `x += 1` è leggermente più prolisso ma chiaro.

<h2 id="strings">Stringhe</h2>

<h3><a href="#how-to-convert-string-or-vec-to-slice" name="how-to-convert-string-or-vec-to-slice">
Come posso convertire una <code>String</code> o un <code>Vec&lt;T&gt;</code> a una partizione (<code>&amp;str</code> e <code>&amp;[T]</code>)?
</a></h3>

Solitamente, puoi passare un riferimento a una `String` o ad un `Vec<T>` ovunque ci si aspetti una partizione.
Utilizzando le [costrizione da de-riferimento](https://doc.rust-lang.org/stable/book/deref-coercions.html), [`String`][String] e [`Vec`][Vec] verrano automaticamente ridotti alle rispettive partizioni quando passate come riferimento tramite `&` o `& mut`.

I metodi implementati su `&str` e `&[T]` possono essere utilizzati direttamente su `String` e `Vec<T>`.
Ad esempio `una_stringa.trim()` funzionerà anche se `trim` è un metodo su `&str` e `una_stringa` è una `String`.

In alcuni casi, come nella programmazione generica è necessario convertire manualmente, questa operazione è effettuabile utilizzando l'operatore di partizione, in questo modo: `&mio_vettore[..]`.

<h3><a href="#how-to-convert-between-str-and-string" name="how-to-convert-between-str-and-string">
Come posso convertire una <code>&amp;str</code> in <code>String</code> e viceversa?
</a></h3>

Il metodo [`to_string()`][to_string] converte una [`&str`][str] in una [`String`][String] e le [`String`][String] sono automaticamente convertite in [`&str`][str] quando ne acquisisci un riferimento.
Entrambe sono visibili nell'esempio seguente:

```rust
fn main() {
    let s = "Maria Rossi".to_string();
    saluta(&s);
}

fn saluta(nome: &str) {
    println!("Ciao {}!", nome);
}
```

<h3><a href="#what-are-the-differences-between-str-and-string" name="what-are-the-differences-between-str-and-string">
In cosa differiscono i due tipi stringa?
</a></h3>

[`String`][String] è un'area di memoria allocata nell'heap, di byte UTF-8.
Le [`String`][String] mutabili possono essere modificate, espanendosi se necessario.
[`&str`][str] è una "vista" di capacità fissata in una [`String`][String] allocata altrove, generalmente nel heap, in caso di partizioni riferite a [`String`][String], o in memoria statica, per le costanti letterali.

[`&str`][str] è un tipo primitivo implementato nel linguaggio Rust, mentre [`String`][String] è implementato dalla libreria standard.

<h3><a href="#how-do-i-do-o1-character-access-in-a-string" name="how-do-i-do-o1-character-access-in-a-string">
Come accedo ai caratteri di una <code>String</code> con complessità asintottica O(1)?
</a></h3>

Non è possibile. Senza una chiara comprensione di cosa intendi per "carattere" e una pre-elaborazione della stringa per ritrovare l'indice del carattere desiderato.

Le stringhe in Rust sono codificate in UTF-8.
Un singolo carattere UTF-8 non è obbligatoriamente grande un singolo byte come sarebbe una stringa codificata in ASCII.
Ogni byte è chiamato una "unità di codice" (nello UTF-16, le unità di codice sono di 2 byte, nello UTF-32 sono di 4 byte).
I "punti di codice" sono composti di una o più unità di codice, combinate in "gruppi di grafemi" che fedelmente ricalcano il concetto tradizionale di caratteri.

Anche con la possibilità di indicizzare i byte in una stringa UTF-8, non puoi accedere all'`i`-esimo elemento del gruppo di grafemi in un tempo costante.
Ad ogni modo, se conosci a quale byte si trova il punto di codice o gruppo di grafemi desiderato, quindi _puoi_ accedervi in tempo costante.
Le funzioni, incluse [`str::find()`][str__find] e le espressioni regolari ritornano indici dei byte, facilitando questo tipo di accesso.

<h3><a href="#why-are-strings-utf-8" name="why-are-strings-utf-8">
Perchè le stringhe sono UTF-8?
</a></h3>

Le [`str`][str] sono UTF-8 perché nel mondo questa codifica è frequente - specialmente in trasmissioni di rete, che ignorano l'ordine di bit della piattaforma - pensiamo quindi sia meglio che il trattamento standard dell'I/O non preveda la ricodifica in entrambe le direzioni.

Questo significa che individuare un particolare punto di codice Unicode dentro una stringa è un'operazione O(n), anche se, conoscendo l'indice del byte ci si può accedere in un tempo O(1) come previsto.
Sotto un certo punto di vista questo è chiaramente sconveniente; ma d'altro canto questo problema è pieno di compromessi e vorremmo sottolineare alcune precisazioni importanti:

Scorrere una [`str`][str] per valori della codifica ASCII può essere fatto in sicurezza byte per byte,
utilizzando [`.as_bytes()`][str__as_bytes] ed estraendo [`u8`][u8] con un costo `O(1)` e producendo valori che possono essere trasformati e comparati con la codifica ASCII con il tipo [`char`][char].
Quindi se per (ad esempio) vogliamo andare a capo ad ogni `'\n'`, una ricerca byte a byte continua a essere funzionante, grazie alla flessibilità dello standard UTF-8.

La maggior parte delle operazioni orientate ai caratteri sul testo funzionano su presupposti molto ristretti come ad esempio l'esclusione dei caratteri non ASCII.
Anche in questo caso all'esterno della codifica ASCII si tende a utilizzare comunque un algoritmo complesso(con complessità non costante) per determinare i bordi di caratteri, parole e paragrafi.
Noi consigliamo di utilizzare un algoritmo "onesto", approvato da Unicode e adattato al linguaggio da considerare.

Il tipo [`char`][char] è UTF-32.
Se hai la certezza di dover richiedere l'analisi di un "punto di codice" alla volta è semplicissimo scrivere `type wstr = [char]` e caricarci dentro una [`str`][str] in un solo passaggio, per poi lavorare con il nuovo `wstr`.
In altre parole: il fatto che il linguaggio non decodifichi a UTF-32 come prima opzione non ti deve inibire da decodificare(o ri-codificare per il processo inverso) i caratteri se necessiti di lavorare in quella codifica.

Per una spiegazione più dettagliata su perché UTF-8 è preferibile rispetto a UTF-16 o UTF-32, leggi il [manifesto di UTF-8 Everywhere](http://utf8everywhere.org/).

<h3><a href="#what-string-type-should-i-use" name="what-string-type-should-i-use">
Quale tipo stringa dovrei usare?
</a></h3>

Rust ha quattro paia di tipi stringa, [ciascuno assolve un ruolo distinto](http://www.suspectsemantics.com/blog/2016/03/27/string-types-in-rust/).
In ciascun paio, c'è un tipo stringa "posseduto" e un tipo stringa "partizione".
I tipi sono suddivisi così:

|                       | tipo "Partizione" | tipo "Posseduto" |
|:----------------------|:------------------|:-----------------|
| UTF-8                 | `str`             | `String`         |
| Compatibile con il SO | `OsStr`           | `OsString`       |
| Compatibile con il C  | `CStr`            | `CString`        |
| Percorso di sistema   | `Path`            | `PathBuf`        |

I diversi tipi di stringa di Rust assolvono ruoli diversi.
`String` e `str` sono stringhe ad uso generico codificate in in UTF-8.
`OsString` e `OsStr` sono codificati a seconda della piattaforma corrente e sono utilizzati per interagire con il sistema operativo.
`CString` e `CStr` sono l'equivalente in Rust delle stringhe del C, si utilizzano nelle chiamate FFI, mentre `PathBuf` e `Path` sono un'aggiunta di `OsString` e `OsStr` ed implementano metodi specifici alla manipolazione di percorsi.

<h3><a href="#why-are-there-multiple-types-of-strings" name="why-are-there-multiple-types-of-strings">
Come faccio a scrivere una funzione che accetti sia <code>&str</code> che <code>String</code>?
</a></h3>

Ci sono diverse opzioni, a seconda delle necessità della funzione:

- Se la funzione richiede una stringa posseduta ma vuole accettare una qualsiasi stringa utilizza la restrizione al tratto `Into<String>`.
- Se la funzione richiede una partizione di stringa ma vuole accettare una qualsiasi stringa utilizza la restrizione al tratto `AsRef<str>`.
- Se alla funzione non interessa del tipo di stringa e vuole gestire uniformemente le due possibilità utilizza il tipo `Cow<str>.`

__Usare `Into<String>`__

In questo esempio, la funzione accetta sia stringhe possedute che partizioni di stringa o facendo nulla o convertendo l'ingresso in stringa posseduta.
Nota che la conversione deve essere fatta esplicitamente e non succede altrimenti.

```rust
fn accetta_entrambe<S: Into<String>>(s: S) {
    let s = s.into();   // Questo converte s in una `String`.
    // ... il resto della funzione
}
```

__Usare `AsRef<str>`__

In questo esempio, la funzione accetta sia stringhe possedute che partizioni di stringa o facendo nulla o convertendo l'ingresso in una partizione di stringa.
Questo viene fatto automaticamente prendendo l'ingresso per riferimento, come qui:

```rust
fn accetta_entrambe<S: AsRef<str>>(s: &S) {
    // ... il corpo della funzione
}
```

__Usare `Cow<str>`__

In questo esempio, la funzione accetta un `Cow<str>`, che non è un tipo generico ma un contenitore, contenente o una stringa posseduta o una partizione di stringa all'occorrenza.

```rust
fn accetta_cow(s: Cow<str>) {
    // ... il corpo della funzione
}
```


<h2 id="collections">Collezioni</h2>

<h3><a href="#can-i-implement-linked-lists-in-rust" name="can-i-implement-linked-lists-in-rust">
Posso implementare efficientemente strutture dati come vettori e liste concatenate in Rust?
</a></h3>

If vuoi implementare queste strutture dati per utilizzarle in altri programmi non è necessario, essendo implementazioni efficienti di queste strutture già disponibili nella libreria standard.

Se invece, [vuoi solo imparare](http://cglab.ca/~abeinges/blah/too-many-lists/book/), probabilmente dovrai entrare nel codice insicuro.
Anche se è _possibile_ implementarle solo con codice sicuro, le prestazioni sarebbero probabilmente peggiori di come sarebbe stato lo stesso codice con l'utilizzo di codice insicuro.
La semplice ragione per ciò è che le strutture dati come vettori e liste concatenate fanno affidamento a operazioni su puntatori e memoria che sono proibiti nel Rust sicuro.

Per esempio, una lista concatenata doppia richiede due riferimenti mutabili a ciascun nodo ma questo viola le regole di Rust sull'esclusività dei riferimenti mutabili.
Si può risolvere il problema utilizzando [`Weak<T>`][Weak], ma le prestazioni sarebbero probabilmente peggiori di quanto desiderato.
Con il codice insicuro puoi ignorare le regole di esclusività ma devi verificare manualmente che il tuo codice non introduca violazioni nella sua gestione della memoria.

<h3><a href="#how-can-i-iterate-over-a-collection-without-consuming-it" name="how-can-i-iterate-over-a-collection-without-consuming-it">
Come posso iterare una collezione senza muoverla o consumarla?
</a></h3>

Il modo più semplice è utilizzare l'implementazione del tratto [`IntoIterator`][IntoIterator].
Ecco qui un esempio con [`&Vec`][Vec]:

```rust
let v = vec![1,2,3,4,5];
for oggetto in &v {
    print!("{} ", oggetto);
}
println!("\nLunghezza: {}", v.len());
```

I cicli `for` in Rust chiamano la funzione `into_iter()` (definita dal tratto [`IntoIterator`][IntoIterator]) per qualsiasi cosa stiano analizzando.
Tutto ciò che implementa il tratto [`IntoIterator`][IntoIterator] può essere traversato con un ciclo `for`.
[`IntoIterator`][IntoIterator] è implementato per [`&Vec`][Vec] e [`&mut Vec`][Vec], obbligando l'iteratore generato da `into_iter()` a prendere in prestito i contenuti della collezione, al posto di consumarli o muoverli.
Lo stesso è vero per le altre collezioni della libreria standard.

Se si desidera un iteratore che muova/consumi i valori, basta scrivere lo stesso ciclo `for` omettendo `&` o `&mut`.

Se si necessita accesso diretto all'iteratore, vi si può usualmente accedere invocando il metodo `iter()`.

<h3><a href="#why-do-i-need-to-type-the-array-size-in-the-array-declaration" name="why-do-i-need-to-type-the-array-size-in-the-array-declaration">
Perchè devo inserire la dimensione del vettore alla sua dichiarazione?
</a></h3>

Non è necessario. Se dichiari direttamente un vettore, la dimensione è dedotta dal numero di elementi. Se invece dichiari una funzione che accetta un vettore di dimensione prefissata il compilatore deve avere modo di sapere quale sarà la dimensione di quel vettore.

Una cosa da notare è che attualmente Rust non offere generici su array di dimensioni diverse.
Se desideri quindi accettare un contenitore continuo di un numero variabile di valori, utilizza [`Vec`][Vec] o una partizione (a seconda che tu richieda il possesso o no).

<h2 id="ownership">Possesso</h2>

<h3><a href="#how-can-i-implement-a-data-structure-that-contains-cycles" name="how-can-i-implement-a-data-structure-that-contains-cycles">
Come posso implementare un grafo o altra struttura contenente cicli?
</a></h3>

Esistono almeno quattro diverse opzioni (largamente discusse in [Too Many Linked Lists](http://cglab.ca/~abeinges/blah/too-many-lists/book/)):

- Puoi implementarlo usando [`Rc`][Rc] e [`Weak`][Weak] per permettere il possesso condiviso dei nodi,
anche se questo approccio richiede la gestione della memoria.
- Puoi implementarlo usando codice `unsafe` e i puntatori.
Anche se efficiente questo metodo ignora i paradigmi di sicurezza di Rust.
- Usando vettori indici al loro interno. Esistono [diversi](http://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/) modi [disponibili](https://featherweightmusings.blogspot.com/2015/04/graphs-in-rust.html), esempi e spiegazioni di questo metodo.
- Usando [`UnsafeCell`][UnsafeCell]. Esistono [spiegazioni e codice](https://github.com/nrc/r4cppp/blob/master/graphs/README.md#node-and-unsafecell) per questo metodo.

<h3><a href="#how-can-i-define-a-struct-that-contains-a-reference-to-one-of-its-own-fields" name="how-can-i-define-a-struct-that-contains-a-reference-to-one-of-its-own-fields">
Come posso definire una struttura che contiene un riferimento a un suo campo?
</a></h3>

Si può fare ma è inutile.
La struttura diventa permanentemente prestata a se stessa e quindi non può essere copiata.
Ecco del codice per capire meglio:

```rust
use std::cell::Cell;

#[derive(Debug)]
struct Immobile<'a> {
    x: u32,
    y: Cell<Option<&'a u32>>,
}


fn main() {
    let test = Immobile { x: 42, y: Cell::new(None) };
    test.y.set(Some(&test.x));

    println!("{:?}", test);
}
```

<h3><a href="#what-is-the-difference-between-consuming-and-moving" name="what-is-the-difference-between-consuming-and-moving">
Che differenza sussiste tra passare per valore, consumare, spostare e trasferire la proprietà?
</a></h3>

Sono parole diverse per dire la stessa cosa.
In tutti i casi significa che il valore è stato trasferito a un nuovo proprietario, o che la proprietà è stata trasferita dal possessore originario, che quindi non può più accedervi.
Se un tipo implementa il tratto `Copy`, il valore del proprietario originale non viene invalidato e può essere utilizzato nuovamente.

<h3><a href="#why-can-values-of-some-types-by-reused-while-others-are-consumed" name="why-can-values-of-some-types-by-reused-while-others-are-consumed">
Perchè alcuni valori di alcuni tipi possono essere utilizzati dopo il passaggio a una funzione mentre il riutilizzo di valori di altri tipi genera errori?
</a></h3>

Se un tipo implementa il tratto [`Copy`][Copy], esso verrà copiato durante il suo passaggio a una funzione.
Tutti i tipi numerici in Rust implementano [`Copy`][Copy] ma le strutture in maniera predefinita non implementano [`Copy`][Copy], quindi invece di essere copiate sono mosse.
Ciò implica che la struttura non possa più essere utilizzata altrove, se non viene ritornata dalla funzione tramite un `return`.

<h3><a href="#how-do-you-deal-with-a-use-of-moved-value-error" name="how-do-you-deal-with-a-use-of-moved-value-error">
Come si gestice l'errore "utilizzo di un valore spostato"?
</a></h3>

Questo errore significa che il valore che stai cercando di utilizzare è stato trasferito a un nuovo proprietario.
La prima cosa da controllare in questo caso è se il trasferimento era davvero necessario: se il valore era stato mosso a una funzione, potrebbe essere possibile riscriverla per utilizzare un riferimento invece che il trasferimento, ad esempio.
Altrimenti se il tipo mosso implementa il tratto [`Clone`][Clone], chiamare `clone()` su di esso prima di muoverlo trasferirà una sua copia, mantenendo l'originale disponibile per utilizzi futuri.
Nota che la clonazione di un valore dovrebbe essere l'ultima spiaggia, essendo la procedura di clonazione costosa e causa di allocazioni di memoria.

Se il valore mosso è uno dei tuoi tipi personalizzati, considera implementare il tratto [`Copy`][Copy] (per la copia implicita al posto del trasferimento) o [`Clone`][Clone] (copia esplicita).
[`Copy`][Copy] è generalmente implementato con la direttiva `#[derive(Copy, Clone)]` ([`Copy`][Copy] richiede [`Clone`][Clone]), e [`Clone`][Clone] con la direttiva `#[derive(Clone)]`.

Se nulla di tutto questo è possibile, probabilmente devi modificare la funzione che ha acquisito il possesso per fare in modo che restituisca la proprietà alla sua uscita.

<h3><a href="#what-are-the-rules-for-different-self-types-in-methods" name="what-are-the-rules-for-different-self-types-in-methods">
Quali regole disciplinano l'utilizzo di <code>self</code>, <code>&amp;self</code> o <code>&amp;mut self</code> nella dichiarazione di un metodo?
</a></h3>

- Usa `self` quando una funzione deve consumare il valore
- Usa `&self` quando una funzione necessita solo di una copia di sola lettura del valore
- Usa `&mut self` quando una funzione necessita di modificare un valore ma senza consumarlo

<h3><a href="#how-can-i-understand-the-borrow-checker" name="how-can-i-understand-the-borrow-checker">
Come faccio a comprendere il gestore dei prestiti?
</a></h3>

Il gestore dei prestiti, mentre analizza il codice, applica poche semplici regole, che sono illustrate nella sezione del libro di Rust [dedicata ai prestiti](https://doc.rust-lang.org/stable/book/references-and-borrowing.html#the-rules).
Queste regole sono:

> Prima cosa, ogni prestito deve durare per un periodo di tempo non superiore alla vita del possessore originale.
Seconda cosa, puoi avere accesso a uno o l'altro di questi due tipi di prestiti, ma non a entrambi contemporaneamente:
>
> - uno o più riferimenti (&T) a una risorsa.
> - esattamente un riferimento mutabile (&mut T)

Nonostante le regole siano molto semplici, seguirle correttamente può essere molto complicato, specialmente per coloro che non sono abitutati a ragionare in termini di campi di esistenza e possesso.

La prima regola è comprendere che il gestore dei prestiti identifica veramente gli errori che produce: molto lavoro è stato profuso per renderlo un assistente di qualità nella risoluzione delle problematiche che individua.
Quando incontri un problema segnalato dal gestore dei prestiti, il primo passo è di leggere lentamente e con attenzione l'errore e poi approcciarsi al codice una volta compreso davvero l'errore descritto.

Il secondo passo è cercare di familiarizzare con i tipi contenitore associati con i concetti di possesso e mutabilità forniti dalla libreria standard di Rust, includendo [`Cell`][Cell], [`RefCell`][RefCell] e [`Cow`][Cow].
Questi utili e necessari strumenti possono aiutare ad esprimere efficientemente alcune situazioni complesse di possesso e mutabilità.

La singola cosa più importante nella comprensione del gestore dei prestiti è la pratica.
La potente analisi statica fatta da Rust è particolare e molto differente da molte esperienze di programmazione precedente.
Ci vorrà un po' di tempo per acquisire la giusta tranquillità.

Se ti ritrovi in difficoltà con il gestore dei prestiti, oppure hai finito la pazienza, sentiti libero di chiedere un aiuto alla [comunità di Rust](community.html).

<h3><a href="#when-is-rc-useful" name="when-is-rc-useful">
Quando può venire utile utilizzare una <code>Rc</code>?
</a></h3>

Questo è coperto dalla documentazione ufficiale per [`Rc`][Rc] il tipo di puntatore non-atomico utilizzante il reference counting di Rust.
In breve, [`Rc`][Rc] e il suo cugino, amico del multithreading [`Arc`][Arc] sono utili per indicare possesso condiviso e vengono deallocati automaticamente dal sistema quando nessuno vi accede.

<h3><a href="#how-do-i-return-a-closure-from-a-function" name="how-do-i-return-a-closure-from-a-function">
Come posso ritornare una chiusura da una funzione?
</a></h3>

Per ritornare una chiusura da una funzione, essa deve essere una "chiusura da movimento", ovvero che essa deve essere dichiarata dalla parola `move`.
Come [illustrato nel libro di Rust](https://doc.rust-lang.org/book/closures.html#move-closures), questo fornisce alla chiusura la sua copia delle variabili catturate, indipendentemente dal quadro di allocazione del chiamante.
Altrimenti, ritornare da una chiusura sarebbe insicuro, visto che permetterebbe di accedere a variabili non più disponibili; detto in un altro modo: permetterebbe di leggere dati da locazioni di memoria errate.
La chiusura deve anche essere racchiusa in un [`Box`][Box], in modo da essere allocata nella heap.
Puoi saperne di più [nel libro](https://doc.rust-lang.org/book/closures.html#returning-closures).

<h3><a href="#what-are-deref-coercions" name="what-are-deref-coercions">
Cos'è un deriferimento forzato e come funziona?
</a></h3>

Un [deriferimento forzato](https://doc.rust-lang.org/book/deref-coercions.html) è una pratica conversione automatica di delle referenze
a puntatori (es., `&Rc<T>` or `&Box<T>`) a referenze ai loro contenuti
(es., `&T`).
Il deriferimento forzato esiste per rendere l'utilizzo di Rust più comodo e sono implementati dal tratto [`Deref`][Deref].

Una implementazione di `Deref` indica che il tipo implementante potrebbe essere convertito in un valore con una chiamata al metodo `deref`, che prende un riferimento immutabile al tipo chiamante e ritorna un riferimento(senza violare peró il campo di esistenza) del tipo obiettivo.
L'operatore `*`, se utilizzato come prefisso è un metodo più veloce per accedere a `deref`.

Sono chiamate "forzature" a causa delle regole spiegate qui [nel libro di Rust](https://doc.rust-lang.org/stable/book/deref-coercions.html):

> Se hai un tipo `U` ed esso implementa `Deref<Target=T>`, i valori `&U` verranno automaticamente convertiti in `&T`.

Ad esempio, se hai un `&Rc<String>`, esso verrà forzato per questa regola a `&String`, che può essere forzato anche a `&str` nello stesso modo.
Quindi se una funzione accettasse un parametro `&str`, puoi passare direttamente un `&Rc<String>` e tutte le forzature e verranno gestite automaticamente dal tratto `Deref`.

I tipici deriferimenti forzati sono:

- `&Rc<T>` a `&T`
- `&Box<T>` a `&T`
- `&Arc<T>` a `&T`
- `&Vec<T>` a `&[T]`
- `&String` a `&str`

<h2 id="lifetimes">Campi di esistenza</h2>

<h3><a href="#why-lifetimes" name="why-lifetimes">
Perchè i campi di esistenza?
</a></h3>

I campi di esistenza sono la soluzione di Rust al problema della sicurezza della memoria.
Questi consentono a Rust di assicurare la sicurezza della memoria senza i costi prestazionali della garbage collection.
Sono basati su una serie di articoli accademici che possono essere trovati nel [libro di Rust](https://doc.rust-lang.org/stable/book/bibliography.html#type-system).

<h3><a href="#why-is-the-lifetime-syntax-the-way-it-is" name="why-is-the-lifetime-syntax-the-way-it-is">
Perché la sintassi dei campi di esistenza è fatta così?
</a></h3>

La sintassi `'a` proviene dalla famiglia ML di linguaggi di programmazione, dove `'a` viene utilizzato per indicare un parametro di tipo generico.
In Rust, la sintassi doveva rappresentare qualcosa di univoco, chiaramente visibile e integrato con le altre dichiarazioni dei tipi insieme ai vari tratti e riferimenti.
Sono state prese in considerazione anche altre scelte ma nessuna sintassi alternativa si è dimostrata chiaramente migliore.

<h3><a href="#how-do-i-return-a-borrow-to-something-i-created-from-a-function" name="how-do-i-return-a-borrow-to-something-i-created-from-a-function">
Come posso ritornare un prestito a qualcosa che ho creato in una funzione?
</a></h3>

Devi assicurarti che il campo di esistenza del valore prestato sia più lungo di quello della funzione.
Puoi ottenere questo effetto puoi assegnare il campo di esistenza dell'uscita a quello di un parametro di ingresso come qui:

```rust
type Gruppo = TypedArena<Cosa>;

// (Il campo di esistenza sotto è esplicitato esclusivamente
/// per facilitarne la comprensione; esso può essere omesso
// tramite le regole di elisione consultabili in un'altra
// risposta presente in questa pagina)

fn crea_prestato<'a>(gruppo: &'a Gruppo,
                       x: i32,
                       y: i32) -> &'a Cosa {
    gruppo.alloc(Cosa { x: x, y: y })
}
```

Un'alternativa è eliminare interamente il riferimento ritornando un valore posseduto come [`String`][String]:

```rust
fn buon_compleanno(nome: &str, eta: i64) -> String {
    format!("Ciao {}! Hai {} anni!", nome, eta)
}
```

Questo approccio è più semplice ma spesso genera allocazioni in memoria non necessarie.

<h3><a href="#when-are-lifetimes-required-to-be-explicit" name="when-are-lifetimes-required-to-be-explicit">
Perchè alcuni riferimenti hanno un campo di esistenza, come <code>&amp;'a T</code> e altri no, tipo <code>&amp;T</code>?
</a></h3>

In realtà, *tutti* i riferimenti hanno un campo di esistenza ma nella maggior parte dei casi non
ti devi preoccupare di gestirlo esplicitamente. Le regole sono le seguenti:

1. Nel corpo di una funzione non devi mai specificare un campo di esistenza esplicitamente;
   il valore corretto dovrebbe essere sempre dedotto correttamente.
2. Nella *dichiarazione* di una funzione (ad esempio, nei tipi dei suoi parametri
   o il suo tipo di ritorno), *potresti* dover specificare un campo di esistenza manualmente.
   I campi di esistenza in questo contesto utilizzano un semplice metodo chiamato
   ["elisione del campo di esistenza"](https://doc.rust-lang.org/book/lifetimes.html#lifetime-elision),
   che a sua volta consiste in queste tre regole:
  - Ciascun campo di esistenza eliso nei parametri di una funzione diviene un campo di esistenza distinto.
  - Se vi è esattamente un singolo campo di esistenza in ingresso, eliso o no, quel campo di esistenza
    viene assegnato a tutti i campi di esistenza elisi utilizzati per i valori di ritorno di quella funzione.
  - Se ci sono più campi di esistenza in ingresso ma uno di quelli è un `&self` o un `&mut self`, il campo di esistenza
    di `self` viene assegnato a tutti i campi di esistenza di uscita.
3. Se si sta definendo una `struct` o `enum` i campi di esistenza sono da dichiarare espressamente.

Se queste regole danno origine a un errore di compilazione, il compilatore di Rust fornirà un messaggio di errore indicante l'errore e anche una potenziale soluzione basata sugli algoritmi di deduzione.

<h3><a href="#how-can-rust-guarantee-no-null-pointers" name="how-can-rust-guarantee-no-null-pointers">
Come può Rust garantire l'assenza di "puntatori nulli" e "puntatori sospesi"?
</a></h3>

L'unico modo di creare un valore `&Cosa` or `&mut Cosa` è di specificare un valore preesistente di tipo `Coso` a cui la referenza deve fare riferimento.
Il riferimento in questo modo ottiene in prestito il valore originale per un determinato blocco di codice(il campo di esistenza del riferimento) e il valore prestato non può essere spostato o distrutto per tutta la durata del prestito.

<h3><a href="#how-do-i-express-the-absense-of-a-value-without-null" name="how-do-i-express-the-absense-of-a-value-without-null">
Come faccio a indicare l'assenza di un valore senza utilizzare <code>null</code>?
</a></h3>

Puoi fare ciò con il tipo [`Option`][Option] che può alternativamente essere `Some(T)` o `None`.
`Some(T)` indica che un valore di tipo `T` è contenuto all'interno, mentre `None` ne indica l'assenza.

<h2 id="generics">Generici</h2>

<h3><a href="#what-is-monomorphisation" name="what-is-monomorphisation">
Cos'è la "monomorfizzazione"?
</a></h3>

La monomorfizzazione specializza ciascun utilizzo di una funzione(o struttura) generica in base ai tipi di parametri di ciascuna chiamata(o agli utilizzi della struttura).

Durante la monomorfizzazione viene generata una nuova versione specializzata della funzione per ciascun set univoco di tipi.
Questa strategia, già utilizzata nel C++, genera del codice macchina efficiente, specializzato per ciascuna chiamata e invocato staticamente, con lo svantaggio che una funzione istanziata con tanti tipi diversi
può dare luogo a molta duplicazione nel codice generato, generando quindi eseguibili più grandi rispetto ad altre strategie di traduzione.

Le funzioni che accettano [oggetti caratterizzati solo da tratti](https://doc.rust-lang.org/book/trait-objects.html) invece dei tipi non sono soggette alla monomorfizzazione.
Invece, i metodi invocati su oggetti tratto sono gestiti dinamicamente durante l'esecuzione.

<h3><a href="#whats-the-difference-between-a-function-and-a-closure-that-doesnt-capture" name="whats-the-difference-between-a-function-and-a-closure-that-doesnt-capture">
Qual è la differenza tra una funzione e una chiusura che non cattura nessuna variabile?
</a></h3>

Le funzioni e le chiusure si utilizzano allo stesso modo ma hanno una gestione differente in fase di esecuzione a causa di una implementazione diversa.

Le funzioni sono un costrutto fondamentale del linguaggio, mentre le chiusure sono essenzialemente un modo più semplice per indicare uno di questi tre tratti: [`Fn`][Fn], [`FnMut`][FnMut] e [`FnOnce`][FnOnce].
Quando scrivi una chiusura, il compilatore di Rust automaticamente provvede a generare una struttura implementante il tratto più idoneo tra quei tre e a catturare le variabili corrette come membri,
generando anche la possibilità di utilizzare la struttura come una funzione. Le strutture, al contrario, non catturano alcuna variabile.

La fondamentale differenza tra questi tratti è come acquisiscono il parametro `self`.
[`Fn`][Fn] prende `&self`, [`FnMut`][FnMut] prende `&mut self` mentre[`FnOnce`][FnOnce] prende `self`.

Anche se una cattura non cattura alcuna variabile di ambiente, viene rappresentata in fase di esecuzione tramite due puntatori, come qualsiasi altra chiusura.

<h3><a href="#what-are-higher-kinded-types" name="what-are-higher-kinded-types">
Cosa sono i tipi di più alto livello, perché sono richiesti e perché Rust non li implementa?
</a></h3>

I tipi di più alto livello sono tipi con parametri non specificati. I costruttori di tipi come [`Vec`][Vec], [`Result`][Result] e [`HashMap`][HashMap] sono tutti esempi di tipi di più alto livello:
ciascuno richiede alcuni tipi aggiuntivi per poter denotare effettivamente il suo tipo, come nel caso di `Vec<u32>`.
Il supporto per i tipi di alto livello significa che questi tipi "incompleti" possono essere utilizzati ovunque possano essere utilizzati anche i tipi "completi", non escludendo le funzioni generiche.

Ogni tipo completo, come [`i32`][i32], [`bool`][bool], o [`char`][char] è di tipo `*` (questa notazione deriva dalla teoria correlata con il sistema dei tipi).
Un tipo a parametro singolo, come [`Vec<T>`][Vec] è invece `* -> *`, ovvero che vec [`Vec<T>`][Vec] accetta un tipo completo come [`i32`][i32] e ritorna il tipo completo `Vec<i32>`.
Un tipo con tre parametri, come [`HashMap<K, V, S>`][HashMap] è di tipo `* -> * -> * -> *` perché accetta tre tipi completi (come [`i32`][i32], [`String`][String] e [`RandomState`][RandomState]) per generare un nuovo tipo completo `HashMap<i32, String, RandomState>`.

In aggiunta a questi esempi, i costruttori di tipo possono accettare dei parametri sul *campo di esistenza*, che denoteremo con `Lt`.
Ad esempio `slice::Iter` ha il tipo `Lt -> * -> *`, perchè va istanziato ad esempio come `Iter<'a, u32>`.

La mancanza di supporto per i tipi di più alto livello rende difficile scrivere alcuni tipi di codice generico.
Risulta particolarmente problematico astrarre su concetti come gli iteratori, dato che essi sono spesso parametrizzati nei confronti di uno specifico campo di esistenza.
Queste premesse hanno impedito la creazione di tratti che astraggano ulteriormente le collezioni presenti in Rust.

Un altro esempio frequente è da ricercare nei concetti di functor e monad, entrambi dei quali sono costruttori di tipi, invece che tipi individuali.

Rust al momento non possiede supporto per i tipi di più alto livello perché non è stato prioritizzato lo sviluppo di questa funzione rispetto ad altre funzionalità che rispecchiano meglio gli obiettivi del progetto.
Essendo la progettazione di funzionalità importanti come queste un campo minato, vorremmo procedere con cautela, non c'è un'altra ragione particolare sul perché Rust non possiede questa funzionalità.

<h3><a href="#what-do-named-type-parameters-in-generic-types-mean" name="what-do-named-type-parameters-in-generic-types-mean">
Cosa significano i parametri fatti tipo <code>&lt;T=Foo&gt;</code> nel codice generico?
</a></h3>

Questi sono chiamati [tipi associati](https://doc.rust-lang.org/stable/book/associated-types.html), permettono di indicare limitazioni di tratto non esprimibili con un costrutto `where`.
Ad esempio, una limitazione generica `X: Bar<T=Foo>` significa che `X` deve implementare il tratto `Bar` e in tale implementazione di bar `Bar`, `X` deve scegliere `Foo` come il tipo associato di `Bar`, `T`.
Gli esempi in cui una limitazione di tal genere non è esprimibile con un costrutto `where` includono i tipi tratto come `Box<Bar<T=Foo>>`.

I tipi associati esistono perché spesso i generici riguardano famiglie di tipi, dove un tipo determina tutti gli altri membri. Per esempio, un tratto per grafi potrebbe avere per `Self` il grafo stesso e avere dei tipi correlati per i suoi nodi e vertici. Ciascun tipo grafo identifica univocamente i tipi associato, rendendo molto più conciso lavorare con questi tipi di strutture e fornendo anche una migliore gestione sulla deduzione dei tipi in molti casi.

<h3><a href="#how-do-i-overload-operators" name="how-do-i-overload-operators">
Posso sovrascrivere gli operatore? Se sì, quali? Come faccio?
</a></h3>

Puoi personalizzare l'implementazione di una varietà di operatori utilizzando i loro tratti associati: [`Add`][Add] per il  `+`, [`Mul`][Mul] per il `*` e via dicendo. Si può fare così:

```rust
use std::ops::Add;

struct Foo;

impl Add for Foo {
    type Uscita = Foo;
    fn add(self, rhs: Foo) -> Self::Uscita {
        println!("Sommando!");
        self
    }
}
```

I seguenti operatori possono essere sovrascritti:

| Operation            | Trait                          |
|:---------------------|:-------------------------------|
| `+`                  | [`Add`][Add]                   |
| `+=`                 | [`AddAssign`][AddAssign]       |
| `- binario`          | [`Sub`][Sub]                   |
| `-=`                 | [`SubAssign`][SubAssign]       |
| `*`                  | [`Mul`][Mul]                   |
| `*=`                 | [`MulAssign`][MulAssign]       |
| `/`                  | [`Div`][Div]                   |
| `/=`                 | [`DivAssign`][DivAssign]       |
| `unary -`            | [`Neg`][Neg]                   |
| `%`                  | [`Rem`][Rem]                   |
| `%=`                 | [`RemAssign`][RemAssign]       |
| `&`                  | [`BitAnd`][BitAnd]             |
| <code>&#124;</code>  | [`BitOr`][BitOr]               |
| <code>&#124;</code>= | [`BitOrAssign`][BitOrAssign]   |
| `^`                  | [`BitXor`][BitXor]             |
| `^=`                 | [`BitXorAssign`][BitXorAssign] |
| `!`                  | [`Not`][Not]                   |
| `<<`                 | [`Shl`][Shl]                   |
| `<<=`                | [`ShlAssign`][ShlAssign]       |
| `>>`                 | [`Shr`][Shr]                   |
| `>>=`                | [`ShrAssign`][ShrAssign]       |
| `*`                  | [`Deref`][Deref]               |
| `mut *`              | [`DerefMut`][DerefMut]         |
| `[]`                 | [`Index`][Index]               |
| `mut []`             | [`IndexMut`][IndexMut]         |

<h3><a href="#why-the-split-between-eq-partialeq-and-ord-partialord" name="why-the-split-between-eq-partialeq-and-ord-partialord">
Cosa distingue <code>Eq</code>/<code>PartialEq</code> e <code>Ord</code>/<code>PartialOrd</code>?
</a></h3>

Ci sono alcuni tipi in Rust i cui valori sono solo parzialmente ordinati oppure hanno relazioni di equivalenza parziali. Ordinamento parziale significa che potrebbero esserci valori di quel tipo che non sono né più piccoli né più grandi di un altro. Uguaglianza parziale significa che ci potrebbero essere dei valori di un certo tipo che non sono uguali a loro stessi.

I tipi a virgola mobile ([`f32`][f32] e [`f64`][f64]) sono un buon esempio di questo. Ogni tipo in virgola mobile potrebbe avere il valore `NaN` (ovvero "non un numero"). `NaN` non è uguale a se stesso (`NaN == NaN` è falso) e nemmeno più grande o più piccolo di un qualsiasi valore.
Di conseguenza sia [`f32`][f32] che [`f64`][f64] implementano [`PartialOrd`][PartialOrd] e [`PartialEq`][PartialEq] ma non [`Ord`][Ord] e nemmeno [`Eq`][Eq].

Come spiegato nella [precedente domanda sui numeri in virgola mobile](#why-cant-i-compare-floats), queste distinzioni sono importanti perchè alcune collezioni fanno affidamento sul totale ordinamento/uguaglianza per funzionare.

<h2 id="input-output">Input / Output</h2>

<h3><a href="#how-do-i-read-a-file-into-a-string" name="how-do-i-read-a-file-into-a-string">
Come legge un file in una <code>String</code>?
</a></h3>

Usando la funzione [`read_to_string()`][read__read_to_string], definita nel tratto [`Read`][Read] di [`std::io`][std-io].

```rust
use std::io::Read;
use std::fs::File;

fn leggi_file(path: &str) -> Result<String, std::io::Error> {
    let mut f = try!(File::open(path));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));  // `s` contiene il contenuto di "foo.txt"
    Ok(s)
}

fn main() {
    match leggi_file("foo.txt") {
        Ok(_) => println!("Letti i contenuti del file!"),
        Err(err) => println!("Non sono riuscito a leggere i contenuti del file, errore: {}", err)
    };
}
```

<h3><a href="#how-do-i-read-file-input-efficiently" name="how-do-i-read-file-input-efficiently">
Come leggo un file efficientemente?
</a></h3>

Il tipo [`File`][File] implementa il tratto [`Read`][Read] che include una moltitudine di funzioni per leggere e scrivere, includendo [`read()`][read__read], [`read_to_end()`][read__read_to_end], [`bytes()`][read__bytes], [`chars()`][read__chars] e [`take()`][read__take].
Ciascuna di queste funzioni legge un pochino dal file.
[`read()`][read__read] legge quanto il sottostante sistema di input fornisce.
[`read_to_end()`][read__read_to_end] legge l'intero buffer in un vettore, allocando lo spazio necessario. [`bytes()`][read__bytes] e [`chars()`][read__chars] permettono rispettivamente di iterare sui byte e caratteri del file, respectively.
Inoltre, [`take()`][read__take] permette di leggere un numero arbitrario di byte dal file. Insieme, questi metodi permettono di leggere efficientemente ogni tipo di file.

Per le letture con buffer utilizza la struttura [`BufReader`][BufReader] che aiuta a ridurre il carico di lavoro al sistema durante la lettura.

<h3><a href="#how-do-i-do-asynchronous-input-output-in-rust" name="how-do-i-do-asynchronous-input-output-in-rust">
Come faccio a gestire input e output asincroni in Rust?
</a></h3>

Ci sono molte librerie che forniscono input / output asincroni in Rust, come [mioco](https://github.com/dpc/mioco), [coio-rs](https://github.com/zonyitoo/coio-rs) e [rotor](https://github.com/tailhook/rotor).

<h3><a href="#how-do-i-get-command-line-arguments" name="how-do-i-get-command-line-arguments">
Come faccio a prendere parametri da riga di comando in Rust?
</a></h3>

Il modo più semplice è utilizzare [`Args`][Args], che fornisce un iteratore sui parametri da riga di comando.

Se stai cercando qualcosa di più potente, ci sono [una serie di librerie disponbili su crates.io](https://crates.io/keywords/argument).

<h2 id="error-handling">Gestione degli errori</h2>

<h3><a href="#why-doesnt-rust-have-exceptions" name="why-doesnt-rust-have-exceptions">
Perchè Rust non ha le eccezioni?
</a></h3>

Le eccezioni complicano la comprensione del flusso del programma, esprimono validità o invalidità all'infuori del sistema dei tipi e funzionano male in ambienti multicore(un obiettivo primario per Rust).

Rust preferisce un approccio alla gestione degli errori basato sui tipi come [spiegato nel dettaglio nel libro](https://doc.rust-lang.org/stable/book/error-handling.html).
Questo è più compatibile con il flusso di controllo tipico di Rust, la concorrenza e tutto il resto.

<h3><a href="#whats-the-deal-with-unwrap" name="whats-the-deal-with-unwrap">
Perché c'è <code>unwrap()</code> ovunque?
</a></h3>

`unwrap()` è una funzione che estrae il valore da una [`Option`][Option] o un [`Result`][Result] e va in errore se il valore non è presente.

`unwrap()` non dovrebbe essere il tuo modo principale per gestire gli errori prevedibili, tipo un errore nell'input dell'utente.
Nel tuo codice, dovrebbe essere trattato come test per la non nullità di un valore, pena il mandare in errore il programma.

Viene utilizzato anche per provare velocemente quando non si vuole ancora gestire tutti i casi o negli articoli, quando la gestione degli errori potrebbe distrarre dal resto.

<h3><a href="#why-do-i-get-errors-with-try" name="why-do-i-get-errors-with-try">
Perchè ottengo un errore quando provo a eseguire codice di esempio che utilizza la macro <code>try!</code>?
</a></h3>

Quasi sicuramente è un problema con il tipo ritornato dalla funzione. La macro [`try!`][TryMacro] estrae un valore da [`Result`][Result] o ritorna con l'errore portato in [`Result`][Result].
Ció significa che [`try`][TryMacro] vale solo per le funzioni che ritornano un [`Result`][Result], dove il tipo costruito `Err` implementa `From::from(err)`.
In particolare ciò significa che la macro [`try!`][TryMacro] non è utlizzabile nella funzione `main`.

<h3><a href="#error-handling-without-result" name="error-handling-without-result">
Esiste un modo più semplice per gestire gli errori rispetto a inserire <code>Result</code> ovunque?
</a></h3>

Se stai cercando un modo per evitare di gestire i [`Result`][Result] nel codice di altre persone, puoi sempre utilizzare [`unwrap()`][unwrap] ma probabilmente non è ciò che desideri.
[`Result`][Result] indica che una qualche operazione potrebbe fallire. Richiedere di gestire esplicitamente questi problemi è uno dei tanti modi in cui Rust incoraggia la scrittura di programmi affidabili.
Rust fornisce degli strumenti come la [macro `try!`][TryMacro] per gestire in ergonomia queste situazioni.

Se davvero desideri non gestire un errore, utilizza [`unwrap()`][unwrap] ma sappi che fare ciò implica che il codice arresterà la sua esecuzione in caso di fallimento, usualmente terminando il processo.

<h2 id="concurrency">Concorrenza</h2>

<h3><a href="#can-i-use-static-values-across-threads-without-an-unsafe-block" name="can-i-use-static-values-across-threads-without-an-unsafe-block">
Posso utilizzare valori statici attraverso i thread senza utilizzare <code>unsafe</code>?
</a></h3>

La mutabilità è sicura solo se sincronizzata. Mutare un [`Mutex`][Mutex] statico (inizializzato tramite il pacchetto [lazy-static](https://crates.io/crates/lazy_static/)) non richiede un blocco di codice `unsafe` come non lo richiede la modifica di un [`AtomicUsize`][AtomicUsize]
(inizializzabile anche senza lazy_static).

Piú in generale, se un tipo implementa il tratto [`Sync`][Sync] e non implementa [`Drop`][Drop], esso [è utilizzabile in una `static`](https://doc.rust-lang.org/book/const-and-static.html#static).

<h2 id="macros">Macro</h2>

<h3><a href="#can-i-write-a-macro-to-generate-identifiers" name="can-i-write-a-macro-to-generate-identifiers">
Posso scrivere una macro per generare identificatori?
</a></h3>

Al momento no.
Le macro di Rust sono ["hygienic macros"](https://en.wikipedia.org/wiki/Hygienic_macro) che intenzionalmente evitano la cattura o la creazione di identificatori che potrebbero collidere con altri.
Le loro capacità sono significativamente differenti dagli stili delle macro normalmente associate ai preprocessori C.
Le invocazioni delle macro possono comparire sono il luoghi dove sono esplicitamente supportate: oggetti, dichiarazioni, espressioni e motivi.
Dove per "dichiarazioni" si intende uno spazio dove è possibile inserire un metodo.
Non si possono utilizzare le macro per completare una dichiarazione avventura parzialmente e seguendo la stessa logica, nemmeno per completare dichiarazioni parziali di variabili.

<h2 id="debugging">Debugging e strumentazione</h2>

<h3><a href="#how-do-i-debug-rust-programs" name="how-do-i-debug-rust-programs">
Come si debuggano i programmi Rust?
</a></h3>

Si possono debuggare con [gdb](https://sourceware.org/gdb/current/onlinedocs/gdb/) o [lldb](http://lldb.llvm.org/tutorial.html), allo stesso modo di C e C++.
In realtà, ogni installazione di Rust viene fornita con uno o entrambi tra rust-gdb e rust-lldb(a seconda della piattaforma). Questi due componenti estendono gdb e lldb con funzioni per permettere una migliore esperienza.

<h3><a href="#how-do-i-locate-a-panic" name="how-do-i-locate-a-panic">
<code>rustc</code> ha detto che del codice della libreria standard è andato in crash, come faccio a trovare il problema?
</a></h3>

Questo errore è spesso causato dall'utilizzo di [`unwrap()`][unwrap] su un `None` o `Err`.
Abilitando la variabile di ambiente `RUST_BACKTRACE=1` potresti ottenere ulteriori informazioni.
Potrebbe essere di aiuto anche la compilazione in modalità debug (predefinita per il comando `cargo build`).
Si possono anche utilizzare i sopracitati `rust-gdb` o `rust-lldb`.

<h3><a href="#what-ide-should-i-use" name="what-ide-should-i-use">
Quale ambiente di sviluppo integrato dovrei utilizzare?
</a></h3>

Ci sono molte opzioni per sviluppare in Rust, tutte illustrate sulla pagina ufficiale [sul supporto agli ambienti di sviluppo](https://forge.rust-lang.org/ides.html).

<h3><a href="#wheres-rustfmt" name="wheres-rustfmt">
<code>gofmt</code> è fantastico. Dov'è <code>rustfmt</code>?
</a></h3>

`rustfmt` è [proprio qui](https://github.com/rust-lang-nursery/rustfmt), sta venendo sviluppato proprio per permettere di rendere il codice Rust il più semplice e prevedibile possibile.

<h2 id="low-level">Low-Level</h2>

<h3><a href="#how-do-i-memcpy-bytes" name="how-do-i-memcpy-bytes">
Come posso usare <code>memcpy</code>?
</a></h3>

Se vuoi clonare una partizione esistente in sicurezza, puoi usare [`clone_from_slice`][clone_from_slice].

Per copiare byte potenzialmente in conflitto usa [`copy`][copy].
Per copiare byte non in conflitto usa [`copy_nonoverlapping`][copy_nonoverlapping].
Entrambe le funzioni elencate sono `unsafe`, visto che possono eludere le garanzie di sicurezza. Sono quindi da utilizzare con attenzione.

<h3><a href="#does-rust-work-without-the-standard-library" name="does-rust-work-without-the-standard-library">
Può Rust operare senza la sua libreria standard?
</a></h3>

Si. I programmi Rust possono scegliere di non caricare la libreria standard utilizzando l'attributo `#![no_std]`.
Una volta impostato, è ancora possibile utilizzare la libreria chiave di Rust, composta esclusivamente da primitivi indipendenti dalla piattaforma di esecuzione. Essa non include IO, concorrenza, allocazioni nella heap, ecc.

<h3><a href="#can-i-write-an-operating-system-in-rust" name="can-i-write-an-operating-system-in-rust">
Posso scrivere un sistema operativo in Rust?
</a></h3>

Si! In realtà al momento [ci sono molti progetti che stanno facendo proprio questo](http://wiki.osdev.org/Rust).

<h3><a href="#how-can-i-write-endian-independent-values" name="how-can-i-write-endian-independent-values">
Come faccio a scrivere o leggere tipi numerici come <code>i32</code> o <code>f64</code> in formato big-endian o little-endian in un file o un flusso di bit?
</a></h3>

Dovresti provare il [pacchetto byteorder](http://burntsushi.net/rustdoc/byteorder/), che fornisce strumenti proprio per quello.

<h3><a href="#does-rust-guarantee-data-layout" name="does-rust-guarantee-data-layout">
Rust garantisce una specifica organizzazione dei dati?
</a></h3>

Non in maniera predefinita. In generale, `enum` e `struct` non sono definiti.
Questo per permettere al compilatore di effettuare delle ottimizzazioni tipo riutilizzare la distanziatura per il discriminante, compattare le varianti di `enum` annidate, riordinare campi per eliminare spaziature, ecc.
Le `enum` prive di dati ("simil-C") possono avere rappresentazione definita. Tali `enum` sono facilmente distinte dal fatto che sono semplicemente una lista di nomi senza dati:

```rust
enum SimilC {
    A,
    B = 32,
    C = 34,
    D
}
```

L'attributo `#[repr(C)]` se applicato a tali `enum` gli fornisce la stessa rappresentazione che avrebbero avuto nel C.
Questo permette nella maggior parte dei casi di utilizzare le `enum` di Rust nella FFI insieme alle `enum` fornite dal C.
Tale attributo è applicabile alle `struct` per ottenere la stessa rappresentazione delle `struct` del C.

<h2 id="cross-platform">Multipiattaforma</h2>

<!--
### How do I build a Windows binary that doesn't display the console window?

TODO: Write this answer.
-->

<!--
### How do I make the console-less binary not crash on panic!?

TODO: Write this answer.
-->

<h3><a href="#how-do-i-express-platform-specific-behavior" name="how-do-i-express-platform-specific-behavior">
Qual'è il modo consigliato per indicare comportamenti specifici a una piattaforma in Rust?
</a></h3>

I comportamenti specifici alla piattaforma sono esprimibili utilizzando [attributi di compilazione condizionale](https://doc.rust-lang.org/reference/attributes.html#conditional-compilation) come ad esempio `target_os`, `target_family`, `target_endian`, ecc.

<h3><a href="#can-rust-be-used-for-android-ios-programs" name="can-rust-be-used-for-android-ios-programs">
Posso programmare per Android/iOS in Rust?
</a></h3>

Si! Ci sono già alcuni esempi utilizzanti Rust sia per [Android](https://github.com/tomaka/android-rs-glue) che per [iOS](https://www.bignerdranch.com/blog/building-an-ios-app-in-rust-part-1/).
Richiede un pochino di lavoro di preparazione ma Rust funziona correttamente su entrambe le piattaforme.

<h3><a href="#can-i-run-my-rust-program-in-a-web-browser" name="can-i-run-my-rust-program-in-a-web-browser">
Posso eseguire il mio programma Rust in un browser web?
</a></h3>

Non ancora ma sono in corso degli sforzi per permettere di compilare Rust per il web con [Emscripten](https://kripken.github.io/emscripten-site/).

<h3><a href="#how-do-i-cross-compile-rust" name="how-do-i-cross-compile-rust">
Come faccio a usare la compilazione incrociata in Rust?
</a></h3>

La compilazione incrociata è possibile in Rust ma richiede [alcune accortezze](https://github.com/japaric/rust-cross/blob/master/README.md) per essere impostata.
Ogni compilatore Rust permette anche la compilazione incrociata ma le librerie necessitano di essere ricompilate per ogni piattaforma obiettivo.

Rust distribuisce [copie della libreria standard](https://static.rust-lang.org/dist/index.html) per ciascuna delle piattaforme supportate, ritrovabili nei file `rust-std-*` presenti nella pagina citata ma ad oggi non esistono metodi automatizzati per installarle.

<h2 id="modules-and-crates">Moduli e pacchetti</h2>

<h3><a href="#what-is-the-relationship-between-a-module-and-a-crate" name="what-is-the-relationship-between-a-module-and-a-crate">
Come si correlano moduli e pacchetti?
</a></h3>

- Un pacchetto è un'unità compilabile, ovvero la minima quantità di codice su cui il compilatore Rust può ancora operare.
- Un modulo è una organizzazione di unità compilabili(anche annidate) all'interno di un pacchetto.
- Un pacchetto contiene un modulo implicito e senza nome nel suo livello più alto.
- Le definizioni ricorsive sono propagabili ai moduli ma non ai pacchetti.

<h3><a href="#why-cant-the-rust-compiler-find-a-library-im-using" name="why-cant-the-rust-compiler-find-a-library-im-using">
Perchè il compilatore Rust non riesce a trovare questa libreria che sto importando con <code>use</code>?
</a></h3>

Ci sono diverse possibilità ma un errore comune è non comprendere che le dichiarazioni `use` sono relative al livello base del pacchetto.
Prova a riscrivere le tue dichiarazioni in modo che utilizzino i percorsi relativi alla cartella base del pacchett per provare a risolvere il problema.

Ci sono anche `self` e `super`, che rendono i percorsi di `use` riferiti rispettivamente al modulo corrente o al modulo padre.

Per ulteriori informazioni su come utilizzare  `use`, leggi il capitolo del libro di Rust ["Crates and Modules"](https://doc.rust-lang.org/stable/book/crates-and-modules.html).

<h3><a href="#why-do-i-have-to-declare-modules-with-mod" name="why-do-i-have-to-declare-modules-with-mod">
Perchè devo dichiarare i file dei moduli con <code>mod</code> al posto di poterli invocare con <code>use</code> direttamente?
</a></h3>

Ci sono due modi per dichiarare i moduli in Rust, in linea o in un altro file. Ecco un esempio:

```rust
// Dentro a main.rs
mod ciao {
    pub fn f() {
        println!("ciao!");
    }
}

fn main() {
    ciao::f();
}
```

```rust
// Dentro a main.rs
mod ciao;

fn main() {
    ciao::f();
}

// Dentro a ciao.rs
pub fn f() {
    println!("ciao!");
}
```

Nel primo esempio, il modulo è definito nello stesso file in cui è utilizzato, nel secondo, la dichiarazione del modulo dice al compilatore di cercare o il file `ciao.rs` o `ciao/mod.rs` e di caricarlo.

Notare la differenza tra `mod` e `use`: `mod` dichiara l'esistenza di un modulo, mentre `use` fa riferimento a un modulo dichiarato altrove, rendendone accessibili i suoi contenuti all'interno del modulo corrente.

<h3><a href="#how-do-i-configure-cargo-to-use-a-proxy" name="how-do-i-configure-cargo-to-use-a-proxy">
Come configuro Cargo all'utilizzo di un proxy?
</a></h3>

Come spiegato nella [guida alla configurazione di Cargo](http://doc.crates.io/config.html), può essere impostato un proxy impostando la variabile "proxy" sotto `[http]` nel file di configurazione.

<h3><a href="#why-cant-the-compile-find-method-implementations" name="why-cant-the-compile-find-method-implementations">
Perché il compilatore non riesce a trovare l'implementazione del metodo anche se ho già specificato la direttiva <code>use</code> sul pacchetto che la contiene?
</a></h3>

Per i metodi definiti su un tratto, devi esplicitamente importare la dichiarazione del tratto. Questo significa che non è sufficiente importare un modulo dove una `struct` implementa un tratto, bisogna importare anche il tratto stesso.

<h3><a href="#why-cant-the-compiler-infer-use-statements" name="why-cant-the-compiler-infer-use-statements">
Perché il compilatore non può capire gli <code>use</code> da solo?
</a></h3>

Probabilmente potrebbe ma non lo vorresti. Mentre in molti casi è probabile che il compilatore possa determinare correttamente il modulo da importare guardando le definizioni questo potrebbe non applicarsi a tutte le casistiche.
Ogni decisione fatta da `rustc` genererebbe sopresa e confusione in alcuni casi e Rust preferisce essere esplicito riguardo all'origine dei nomi.

Ad esempio, il compilatore potrebbe decidere che nel casi due identificatori fossero in conflitto sia da preferire l'identificatore la cui dichiarazione è meno recente.
In questo caso sia il modulo `foo` che il modulo `bar` definiscono l'identificatore `baz` ma `foo` viene registrato per primo e quindi il compilatore inserirebbe `use foo::baz;`.

```rust
mod foo;
mod bar;

// use foo::baz  // ciò che sarebbe inserito.

fn main() {
  baz();
}
```

Sapendo questa dinamica, probabilmente risparmieresti qualche carattere ma aumenteresti anche la possibilità di generare degli errori imprevisti quando in realtà al posto di `baz()` intendevi `bar::baz()`, diminuendo anche la leggibilità del codice,
avendo reso la chiamata alla funzione dipendente dall'ordine di dichiarazione. Questi sono compromessi che Rust non ha intenzione di prendere.

Ad ogni modo, in futuro, un ambiente di sviluppo integrato potrebbe assistere nella gestione delle dichiarazioni, fornendo il massimo di entrambi i mondi: assistenza automatica nelle dichiarazioni ma chiarezza sulle origini dei nomi importati.

<!--
### How do I package and archive crates from [https://crates.io](https://crates.io)?

TODO: Write this answer.
-->

<h3><a href="#how-do-i-do-dynamic-rust-library-loading" name="how-do-i-do-dynamic-rust-library-loading">
Come carico dinamicamente librerie in Rust?
</a></h3>

Puoi importare librerie dinamiche in Rust con [libloading](https://crates.io/crates/libloading), che fornisce un sistema multipiattaforma per il link dinamico.

<h3><a href="#why-doesnt-crates-io-have-namespaces" name="why-doesnt-crates-io-have-namespaces">
Perchè crates.io non ha uno spazio dei nomi?
</a></h3>

Citando la [spiegazione ufficiale](https://internals.rust-lang.org/t/crates-io-package-policies/1041) sul design di [https://crates.io](https://crates.io):

> Nel primo mese di crates.io, un buon numero di persone hanno richiesto la possibilità di introdurre [pacchetti con spazi dei nomi](https://github.com/rust-lang/crates.io/issues/58).<br><br>
>
> Mentre questi permettono a autori multipli di utilizzare un singolo nome generico, aggiungono complessità su come i pacchetti vengono indicati nel codice Rust e nella comunicazione su di essi. A una prima occhiata questo permetterebbe a più persone di associarsi al nome `http`, ma questo implicherebbe che per riferirsi a due pacchetti di autori diversi si debbe parlare ad esempio di `http di wycats` o di `http di reem`, offrendo pochi vantaggi rispetto a nomi come `wycats-http` o `reem-http`.<br><br>
>
> Inoltre, osservando questa scelta abbiamo scoperto che le persone tendono a utilizzare nomi più creativi (come `nokogiri` invece che "libxml2 di tendelove"). Questi nomi creativi tendono a essere brevi e memorabili, in parte anche grazie della mancanza di dipendenze da altri. Rendono anche più semplice parlare in modo conciso e non ambiguo di pacchetti, creando nuovi nomi altisonanti. Esistono diversi ecosistemi da oltre 10,000 pacchetti come NPM e RubyGems le cui comunità prosperano anche sotto un singolo spazio dei nomi.<br><br>
>
> In breve, non pensiamo che l'ecosistema Cargo avrebbe giovamento se Piston scegliesse un nome come `bvssvni/game-engine` (permettendo ad altri di scegliere `wycats/game-engine`) invece che semplicemente `piston`.<br><br>
>
> Proprio perché gli spazi dei nomi sono più complessi in diversi ambiti ed essendo la loro aggiunta possibile se necessario, per ora abbiamo intenzione di preservare un singolo spazio dei nomi condiviso.

<h2 id="libraries">Librerie</h2>

<h3><a href="#how-can-i-make-an-http-request" name="how-can-i-make-an-http-request">
Come faccio a fare una richiesta HTTP?
</a></h3>

La libreria standard non contiene un'implementazione di HTTP quindi dovrai utilizzare un pacchetto esterno.
[Hyper](https://github.com/hyperium/hyper) è la più popolare ma ce ne sono [tante altre](https://crates.io/keywords/http).

<h3><a href="#how-can-i-write-a-gui-application" name="how-can-i-write-a-gui-application">
Come faccio a scrivere un applicativo con interfaccia grafica in Rust?
</a></h3>

Ci sono molti modi per fare applicazioni con interfaccia grafica in Rust.
Guarda questa lista di [librerie per realizzare interfacce grafiche](https://github.com/kud1ing/awesome-rust#gui).

<h3><a href="#how-can-i-parse-json-xml" name="how-can-i-parse-json-xml">
Come faccio a deserializzare JSON/XML?
</a></h3>

[Serde](https://github.com/serde-rs/serde) è la libreria consigliata per serializzare e deserializzare di dati in Rust da e verso una moltitudine di formati.

<h3><a href="#is-there-a-standard-2d-vector-crate" name="is-there-a-standard-2d-vector-crate">
Esiste una libreria standard per la geometria e vettoriali 2D+?
</a></h3>

Non ancora! Puoi farne una tu?

<h3><a href="#how-do-i-write-an-opengl-app" name="how-do-i-write-an-opengl-app">
Come faccio a creare un applicativo OpenGL in Rust?
</a></h3>

[Glium](https://github.com/tomaka/glium) è la principale libreria per utilizzare OpenGL in Rust. [GLFW](https://github.com/bjz/glfw-rs) è un'altra opzione valida.

<h3><a href="#can-i-write-a-video-game-in-rust" name="can-i-write-a-video-game-in-rust">
Posso fare un videogioco in Rust?
</a></h3>

Certo! La principale libreria per programmare giochi in Rust è [Piston](http://www.piston.rs/) ci  sono sia un [subreddit per la creazione di videogiochi in Rust](https://www.reddit.com/r/rust_gamedev/) e un canale IRC (`#rust-gamedev` su [Mozilla IRC](https://wiki.mozilla.org/IRC)).

<h2 id="design-patterns">Paradigmi di programmazione</h2>

<h3><a href="#is-rust-object-oriented" name="is-rust-object-oriented">
Rust è orientato agli oggetti?
</a></h3>

Rust è multi paradigma. Molte cose possibili in linguaggi orientati agli oggetti sono possibili in Rust ma non proprio tutto e non sempre utilizzando un livello di astrazione uguale a quello a cui si è abituati.

<h3><a href="#how-do-i-map-object-oriented-concepts-to-rust" name="how-do-i-map-object-oriented-concepts-to-rust">
Come converto concetti della programmazione orientata agli oggetti in Rust?
</a></h3>

Dipende. _Esistono_ modi per convertire concetti orientati agli oggetti come [ereditarietà multiple](https://www.reddit.com/r/rust/comments/2sryuw/ideaquestion_about_multiple_inheritence/) a Rust ma non essendo Rust orientato agli oggetti il risultato della conversione potrenne apparire sostanzialmente diverso dalla sua rappresentazione in un linguaggio orientato agli oggetti.

<h3><a href="#how-do-i-configure-a-struct-with-optional-parameters" name="how-do-i-configure-a-struct-with-optional-parameters">
Come gestisco la configurazione di una `struct` con parametri opzionali?
</a></h3>

Il modo più semplice è utilizzare il tipo [`Option`][Option] in qualsiasi funzione venga utilizzata per costruire istanze della struttura (generalmente `new()`).
Un altro modo è utilizzare il [metodo del costruttore](https://aturon.github.io/ownership/builders.html), dove alcune funzioni devono essere chiamate dopo la costruzione del tipo.

<h3><a href="#how-do-i-do-global-variables" name="how-do-i-do-global-variables">
Come faccio le variabili globali in Rust?
</a></h3>

Le globali in Rust possono essere fatte utilizzando la dichiarazione `const` per le globali computate al momento della compilazione, mentre `static` è utilizzabile per globali mutabili.
Nota che la modifica di una variabile `static mut` richiede `unsafe`, visto che permette problemi di concorrenza, una cosa impossibile nel Rust sicuro.
Una differenza importante tra `const` e `static` è che si possono prendere riferimenti a valori `static` ma non a valori `const` se privi di posizione in memoria specificata.
Per ulteriori informazioni su `const` e `static`, leggi [il libro di Rust](https://doc.rust-lang.org/book/const-and-static.html).

<h3><a href="#how-can-i-set-compile-time-constants-that-are-defined-procedurally" name="how-can-i-set-compile-time-constants-that-are-defined-procedurally">
Come faccio a impostare delle costanti al momento della compilazione proceduralmente?
</a></h3>

Rust attualmente possiede un supporto limitato per le costanti al momento della compilazione.
Puoi definire dei primitivi con le dichiarazioni `const` (simili a `static` ma immutabili e senza una specifica locazione in memoria) funzioni `const` e metodi correlati.

Per definire costanti procedurali che non possono essere definite tramite questi meccanismi usa il pacchetto [`lazy-static`](https://github.com/rust-lang-nursery/lazy-static.rs), che emula l'assegnazione al momento della compilazione assegnando il valore al primo utilizzo.

<h3><a href="#can-i-run-code-before-main" name="can-i-run-code-before-main">
Posso eseguire del codice di inizializzazione prima di main?
</a></h3>

Rust non consente l'esistenza di qualcosa prima di `main`.
La cosa più vicina può essere fatta tramite il pacchetto [`lazy-static`](https://github.com/Kimundi/lazy-static.rs), simulante una situazione "pre-main" in cui le variabili statiche vengono inizializzate al loro primo utilizzo.

<!--

This answer needs significant work. Let's revise after the initial posting. --aturon

<h3><a href="#why-doesnt-rust-have-inheritance" name="why-doesnt-rust-have-inheritance">
Why doesn't Rust have inheritance?
</a></h3>

There are two meanings for the word "inheritance": _subtyping_, and _interface sharing_. Both purposes are already handled by traits.

For the first, subtyping exists for polymorphism, which traits already provide.

For the second, interface sharing is handled via trait methods, which define a collection of related functions that must be implemented for any implementation of the trait.

Rust has consistently worked to avoid having features with overlapping purposes, preferring to keep features orthogonal. For this reason, and given that the two major purposes are already handled by traits, Rust has opted not to include inheritance.

-->

<h3><a href="#does-rust-allow-non-constant-expression-values-for-globals" name="does-rust-allow-non-constant-expression-values-for-globals">
Rust permette di assegnare alle globali espressioni non costanti?
</a></h3>

No. Le globali non possono non avere un costruttore costante e non possiedono un destrutture. I costruttori statici sono sconvenienti perché assicurare un ordine di inizializzazione statico è complesso. Le situazioni "pre-main" sono spesso considerate sconvenienti e Rust non le consente.

Leggi anche il [domande frequenti del C++](http://yosefk.com/c++fqa/ctors.html#fqa-10.12) che fa menzione del "problema dell'ordine di inizializzazione per le static" e il [blog di Eric Lippert](https://ericlippert.com/2013/02/06/static-constructors-part-one/) per le problematiche in C#, che anche esso possiede queste funzioni.

Puoi emulare globali il cui valore non è costante con il pacchetto [lazy-static](https://crates.io/crates/lazy_static/).

<h2 id="other-languages">Altri linguaggi</h2>

<h3><a href="#how-can-i-use-static-fields" name="how-can-i-use-static-fields">
Come posso implementare in Rust quello che in C si può ottenere con <code>struct X { static int X; };</code> ?
</a></h3>

Rust non possiede campi `static` come nel codice sopra. Al loro posto puoi dichiarare una varibile `static` a un determinato modulo, che viene preservata privata all'interno dello stesso.

<h3><a href="#how-can-i-convert-a-c-style-enum-to-an-integer" name="how-can-i-convert-a-c-style-enum-to-an-integer">
Come converto una enum simil-C a un intero e vice versa?
</a></h3>

Convertire una enum simil-C a un intero è possibile con l'espressione `as` come in `e as i64` (dove `e` è una enum).

La conversione in altre direzioni può essere svolta tramite `match`, che associa differenti valori numerici a differenti potenziali valori per la enum.

<h3><a href="#why-do-rust-programs-use-more-memory-than-c" name="why-do-rust-programs-use-more-memory-than-c">
Perché i programmi in Rust sono più grandi su disco di programmi C?
</a></h3>

Ci sono diversi fattori che contribuiscono alla tendenza di Rust di avere in generale file binari più grandi di programmi funzionalmente equivalenti in C.
In generale Rust si focalizza su ottimizzare le prestazioni di programmi reali, non le dimensioni di piccoli programmi.

__Monomorfizzazione__

Rust monomorfizza i generici, ovvero che viene generata una nuova versione di una funzione generica o tipo per ciascuna dichiarazione effettuata con tipi distinti. Questo assomiglia ai template in C++. Ad esempio, nel programma seguente:

```rust
fn foo<T>(t: T) {
    // ... qualcosa
}

fn main() {
    foo(10);       // i32
    foo("ciao");  // &str
}
```

Nel file eseguibile finale vi saranno due versioni diverse di `foo`, una specifica al tipo in ingresso `i32` e l'altra specifica al tipo in ingresso `&str`.
Questo permette un efficiente dispacciamento statico della funzione generica ma aumentando le dimensioni dell'eseguibile finale.

__Simboli di debug__

I programmi Rust sono compilati con i simboli di debug inclusi, anche se in modalità rilascio. Questi sono utilizzabili per fornire informazioni in caso di crash e possono essere rimossi con `strip`, o un qualsiasi altro strumento per la rimozione di simboli di debug.
Risulta utile sapere anche che compilare in modalità rilascio con Cargo equivale a impostare il livello di ottimizzazione 3 con rustc.
Un livello alternativo di ottimizzazione (chiamato `s` o `z`) [aggiunto recentemente](https://github.com/rust-lang/rust/pull/32386) indica al compilatore di focalizzarsi invece che sulle prestazioni, sulle dimensioni dell'eseguibile finale.

__Jemalloc__

Rust utilizza jemalloc come il suo allocatore predefinito, questo aumenta le dimensioni dei binari compilati.
Jemalloc è stato scelto perché è un allocatore consistente e di qualità con caratteristiche prestazionali preferibili rispetto agli allocatori forniti da molti sistemi.
Al momento si stanno facendo esperimenti su come [rendere più facile l'utilizzo di allocatori personalizzati](https://github.com/rust-lang/rust/issues/32838) ma la funzionalità non è stata ancora ultimata.

__Ottimizzazione del linker__

Rust di base non effettua ottimizzazioni al momento del linking ma gli più essere detto di farlo.
Questo incrementa la quantità di ottimizzazioni effettuabili e può avere un effetto sulle dimensioni dei binari generati, questo effetto può essere amplificato se in combinazione con l'opzione di ottimizzazione per dimensioni sopracitata.

__Libreria standard__

La libreria standard di Rust include libbacktrace e libunwind, che potrebbero essere non volute in alcuni programmi.
Utilizzare `#![no_std]` può quindi fornire dei binari più piccoli ma cambia in modo sostanziale il modo in cui il codice deve essere scritto.
Nota che utilizzare Rust senza la libreria standard è spesso funzionalmente vicino al codice C equivalente.

Per esempio, il programma seguente in C legge un nome e dice "ciao" alla persona con quel nome:

```c
#include <stdio.h>

int main(void) {
    printf("Come ti chiami?\n");
    char input[100] = {0};
    scanf("%s", input);
    printf("Ciao %s!\n", input);
    return 0;
}
```

Per riscrivere questo programma in Rust scriveresti una cosa del genere:

```rust
use std::io;

fn main() {
    println!("Come ti chiami?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("Ciao {}!", input);
}
```

Questo programma, quando compilato e confrontato con il programma C avrà una dimensione maggiore e utilizzerà più memoria ma non è esattamente equivalente al codice C che lo precede.
Il reale equivalente in realtà assomiglia di più a questo:

```rust
#![feature(lang_items)]
#![feature(libc)]
#![feature(no_std)]
#![feature(start)]
#![no_std]

extern crate libc;

extern "C" {
    fn printf(fmt: *const u8, ...) -> i32;
    fn scanf(fmt: *const u8, ...) -> i32;
}

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    unsafe {
        printf(b"Come ti chiami?\n\0".as_ptr());
        let mut input = [0u8; 100];
        scanf(b"%s\0".as_ptr(), &mut input);
        printf(b"Ciao %s!\n\0".as_ptr(), &input);
        0
    }
}

#[lang="eh_personality"] extern fn eh_personality() {}
#[lang="panic_fmt"] fn panic_fmt() -> ! { loop {} }
#[lang="stack_exhausted"] extern fn stack_exhausted() {}
```

Che dovrebbe certamente eguagliare il C nell'utilizzo della memoria, incrementando peró la complessità e rimuovendo le garanzie fornite dal codice Rust (evitate utilizzando `unsafe`).

<h3><a href="#why-no-stable-abi" name="why-no-stable-abi">
Perchè Rust non ha una ABI stabile come il C e perché devo annotare le cose con extern?
</a></h3>

Dedicarsi a una ABI è una decisione importante che può limitare i cambiamenti vantaggiosi futuri. Dato che Rust ha raggiunto la versione 1.0 a Maggio 2015 è troppo presto per impegnarsi a costruire una ABI stabile. Ció peró non implica che non possa succedere nel futuro.
(Nonostante questo il C++ è riuscito a vivere per molti anni senza specificare una ABI stabile.)

Il lemma `extern` permette di usare con Rust specifiche ABI, come quella ben definita del C, per interoperare con altri linguaggi.

<h3><a href="#can-rust-code-call-c-code" name="can-rust-code-call-c-code">
Può il codice Rust chiamare il codice C?
</a></h3>

Si. Chiamare il C da Rust è progettato per avere la stessa efficienza delle chiamate di codice C dal C++.

<h3><a href="#can-c-code-call-rust-code" name="can-c-code-call-rust-code">
Può il codice C chiamare il codice Rust?
</a></h3>

Si. Il codice Rust deve essere sposto mediante una dichiarazione `extern`, che lo rende compatibile con la ABI del C.
Tale funzione può essere passata al codice C come puntatore a una funzione o, se con l'attributo `#[no_mangle]` chiamata direttamente dal C.

<h3><a href="#why-rust-vs-cxx" name="why-rust-vs-cxx">
Scrivo già del C++ perfetto. Cosa mi fornisce di più Rust?
</a></h3>

Il C++ moderno include molte funzioni che rendono la scrittura di codice sicuro e corretto meno prono ad errori ma non è perfetto e rimane comunque facile introdurre vulnerabilità.
Gli sviluppatori del C++ stanno cercando di porre rimedio a queste problematiche ma il C++ è limitato da una lunga storia che impedisce di attuare molte idee che si vorrebbero sperimentare.

Rust è stato disegnato sin dal primo giorno per essere un linguaggio di programmazione per sistemi sicuro, questo significa che non è limitato da scelte pregresse che potrebbero impedire di raggiungere il corretto livello di sicurezza come il C++.
In C++, la sicurezza si ottiene mediante una rigorosa disciplina personale ed è semplice commettere errori mentre in Rust, la sicurezza è predefinita.
Rust permette quindi di lavorare in un gruppo di persone meno perfette di te, senza dover spendere il tuo tempo per controllare il codice altrui per controllare potenziali falle di sicurezza nel codice altrui.

<h3><a href="#how-to-get-cxx-style-template-specialization" name="how-to-get-cxx-style-template-specialization">
Come creo l'equivalente della specializzazione dei template del C++ in Rust?
</a></h3>

Rust attualmente non ha un equivalente esatto alla specializzazione dei template ma [ci si sta lavorando su](https://github.com/rust-lang/rfcs/pull/1210) e verrà probabilmente aggiunta a presto.
Ad ogni modo, effetti simili si possono ottenere con i [tipi associati](https://doc.rust-lang.org/stable/book/associated-types.html).

<h3><a href="#how-does-ownership-relate-to-cxx-move-semantics" name="how-does-ownership-relate-to-cxx-move-semantics">
Come si compara il sistema dei possessi in Rust con le semantiche del movimento in C++?
</a></h3>

I concetti base sono simili ma i due sistemi differiscono nella pratica.
In entrambi i sistemi "muovere" un valore è un modo per trasferire il possesso
delle risorse sottostanti. Ad esempio, muovere una stringa trasferisce il suo buffer
al posto di copiarla.

In Rust il trasferimento di possesso è il comportamento standard.
Ad esempio, se scrivo una funzione che accetta una `String` come parametro,
questa funzione prenderà possesso del valore della `String` fornita dal chiamante:

```rust
fn elabora(s: String) { }

fn chiamante() {
    let s = String::from("Ciao mondo!");
    elabora(s); // Trasferisce la proprietà di `s` a `elabora`
    elabora(s); // Errore! il possesso è già stato trasferito.
}
```

Come puoi vedere nel frammento di codice sopra, nella funzione `chiamante`,
la prima chiamata a `elabora` trasferisce il possesso della variabile `s`.
Il compilatore tiene traccia del possesso, quindi una seconda chiamata a
`elabora` genere un errore perché non è consentito trasferire il possesso
dello stesso valore due volte.
Rust previene anche il movimento di un valore se vi è ancora un
riferimente ad esso.

Il C++ ha un approccio distinto, il comportamento predefinito è infatti
di copiare un valore (nello specifico invocandone il costruttore della copia).
Ad ogni modo i chiamati possono dichiarare i loro parametri utilizzando
un "riferimento rvalore" come `string&&`, per indicare che prenderanno
possesso di parte delle risorse possedute dal paramentro(in questo caso
il buffer interno della stringa).
Il chiamante deve quindi passare un'espressione temporanea o effettuare
un movimento esplicito utilizzando `std::move`.
Un abbozzo della funzione `elabora` sopra sarebbe quindi:

```
void elabora(string&& s) { }

void chiamante() {
    string s("Ciao mondo!");
    elabora(std::move(s));
    elabora(std::move(s));
}
```

I compilatori C++ non sono tenuti a tenere traccia dei movimenti.
Ad esempio, il codice sopra viene compilato senza alcun avviso o errore
utilizzando l'ultima versione di Clang.
Inoltre in C++ il possesso della stringa `s` stessa
(se non del suo buffer interno) rimane in `chiamante`, e quindi il
destruttore di `s` verrà eseguito quando `chiamante` ritorna, anche se
è stato spostato (in Rust, al contrario, i valori spostati sono rimossi
dai nuovi proprietari).

<h3><a href="#how-to-interoperate-with-cxx" name="how-to-interoperate-with-cxx">
Come posso interoperare il C++ da Rust, o il Rust da C++?
</a></h3>

Il Rust è il C++ possono interoperare tramite il C. Sia il Rust che il C++ forniscono una [foreign function interface](https://doc.rust-lang.org/book/ffi.html) per il C, che può essere utilizzata per comunicare tra di loro.
Se scrivere dei collegamenti in C è troppo complicato, puoi sempre utilizzare [rust-bindgen](https://github.com/servo/rust-bindgen) per generare automaticamente dei collegamenti C++ funzionanti.

<h3><a href="#does-rust-have-cxx-style-constructors" name="does-rust-have-cxx-style-constructors">
Rust possiede dei costruttori in stile C++?
</a></h3>

No. Al loro posto si utilizzano delle funzioni, il cui nome usuale è `new()`, ad ogni modo questa è semplicemente una convenzione e non una regola del linguaggio.
La funzione `new()` è semplicemente un'altra funzione. Un esempio di ciò è questo:

```rust
struct Foo {
    a: i32,
    b: f64,
    c: bool,
}

impl Foo {
    fn new() -> Foo {
        Foo {
            a: 0,
            b: 0.0,
            c: false,
        }
    }
}
```

<h3><a href="#does-rust-have-copy-constructors" name="does-rust-have-copy-constructors">
Rust possiede dei costruttori copia?
</a></h3>

Non esattamente. I tipi che implementano `Copy` faranno una copia simil-C senza alcun lavoro aggiuntivo.
Non è possibile peró implementare tipi `Copy` che richiedono un comportamento personalizzato alla copia.
Al loro posto in Rust i costruttori copia sono creati implementando il tratto e successivamente chiamando il metodo `clone`.
Permettere di definire manualmente l'operatore copia permette di ridurre la complessità, facilitando per lo sviluppatore l'identificazione di operazioni potenzialmente costose.

<h3><a href="#does-rust-have-move-constructors" name="does-rust-have-move-constructors">
Rust possiede dei costruttori di movimento?
</a></h3>

No. I valori di tutti i tipi sono mossi tramite `memcpy`.
Questo permette di scrivere del codice `unsafe` generico molto più semplice, dato che l'assegnazione, il passaggio e il ritorno di valori sono privi di effetti collaterali.

<h3><a href="#compare-go-and-rust" name="compare-go-and-rust">
In cosa si assomigliano Go e Rust, in cosa sono invece diversi?
</a></h3>

Rust e Go hanno degli obiettivi molto differenti. Le differenze seguenti non sono le uniche (sarebbero troppe per elencarle) ma eccone alcune tra le più importanti:

- Rust è di più basso livello di Go. Ad esempio, Rust non richiede un garbage collector, mentre Go sí. In generale Rust permette un livello di controllo comparabile con il C o il C++.
- Rust si focalizza sul garantire sicurezza ed efficienza mantenendo astrazioni di alto livello mentre Go vuole essere un linguaggio compatto e semplice che compila velocemente e può funzionare con molti strumenti.
- Rust supporta la programmazione generica, Go no.
- Rust ha forti influenze dal mondo della programmazione funzionale, includendo il sistema dei tipi derivato dalle typeclasses di Haskell. Go ha un sistema dei tipi più semplici, utilizzanti interfacce compatibili con la programmazione generica.

<h3><a href="#how-do-rust-traits-compare-to-haskell-typeclasses" name="how-do-rust-traits-compare-to-haskell-typeclasses">
Come si comparano i tratti di Rust con le typeclasses di Haskell?
</a></h3>

I tratti in Rust somigliano alle typeclasses di Haskell ma attualmente non sono così potenti, dato che Rust non può esprimere i tipi di più altro livello. I tipi associati di Rust sono gli equivalenti delle famiglie di tipi di Haskell.

Alcune differenze specifiche tra le typeclasses di Haskell e i tratti di Rust includono:

- I tratti in Rusta hanno un primo parametro implicito chiamato `Self`. `trait Bar` in Rust corrisponde a `class Bar self` in Haskell e `trait Bar<Foo>` in Rust corrisponde a `class Bar foo self` in Haskell.
- I "Supertratti" o "limitatori di superclass" in Rust sono scritti `trait Sub: Super`, mentre in Haskell `class Super self => Sub self`.
- Rust vieta istanze orfanes, indicando regole di coerenza differenti tra Rust e Haskell.
- La risoluzione dell `impl` di Rust considera le clausole `where` e i relativi tratti per decidere se due `impl` si sovrappongono, o per scegliere tra diverse `impl` possibili. Haskell considera ciò solo nelle dichiarazioni `instance`, ignorando ogni limitazione posta altrove.
- Un sottoinsieme dei tratti di Rust (Quelli ["object safe"](https://github.com/rust-lang/rfcs/blob/master/text/0255-object-safety.md)) può essere usato per il dispacciamento dinamico mediante tratti. La stessa funzionalità è disponibile in Haskell attraverso il metodo di GHC `ExistentialQuantification`.

<h2 id="documentation">Documentazione</h2>

<h3><a href="#why-are-so-many-rust-answers-on-stackoverflow-wrong" name="why-are-so-many-rust-answers-on-stackoverflow-wrong">
Perchè su Stack Overflow molte delle risposte su Rust sono sbagliate?
</a></h3>

Il linguaggio Rust è pubblico da diversi anni e ha raggiunto la versione 1.0 a Maggio del 2015.
Nei periodi precedenti il linguaggio ha ricevuto delle modifiche sostanziali e molte risposte fanno riferimento a versioni vecchie del linguaggio.

Nel tempo sempre più risposte saranno disponibili per la versione corrente, migliorando la problematica alterando il rapporto tra le risposte corrette e quelle sbagliate.

<h3><a href="#where-do-i-report-issues-in-the-rust-documentation" name="where-do-i-report-issues-in-the-rust-documentation">
Dove segnalo problemi alla documentazione di Rust?
</a></h3>

Puoi segnalare problemi con la documentazione sul [pannello delle problematiche](https://github.com/rust-lang/rust/issues) di Rust.
Assicurati peró di leggere [linee guida alla contribuzione](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#writing-documentation) prima.

<h3><a href="#how-do-i-view-rustdoc-documentation-for-a-library-my-project-depends-on" name="how-do-i-view-rustdoc-documentation-for-a-library-my-project-depends-on">
Come posso vedere la documentazione di rustdoc di una dipendenza del mio progetto?
</a></h3>

Quando utilizzi `cargo doc` per generare la documentazione per il tuo progetto, il comando genera anche la documentazione per le dipendenze attive.
Le puoi trovare nella cartella `target/doc` del progetto.
Usa `cargo doc --open` per aprire i documenti dopo averli generati o semplicemente apri da solo `target/doc/index.html`.

[Vec]: https://doc.rust-lang.org/stable/std/vec/struct.Vec.html
[HashMap]: https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html
[Into]: https://doc.rust-lang.org/stable/std/convert/trait.Into.html
[From]: https://doc.rust-lang.org/stable/std/convert/trait.From.html
[Eq]: https://doc.rust-lang.org/stable/std/cmp/trait.Eq.html
[PartialEq]: https://doc.rust-lang.org/stable/std/cmp/trait.PartialEq.html
[Ord]: https://doc.rust-lang.org/stable/std/cmp/trait.Ord.html
[PartialOrd]: https://doc.rust-lang.org/stable/std/cmp/trait.PartialOrd.html
[f32]: https://doc.rust-lang.org/stable/std/primitive.f32.html
[f64]: https://doc.rust-lang.org/stable/std/primitive.f64.html
[i32]: https://doc.rust-lang.org/stable/std/primitive.i32.html
[i64]: https://doc.rust-lang.org/stable/std/primitive.i64.html
[bool]: https://doc.rust-lang.org/stable/std/primitive.bool.html
[Hash]: https://doc.rust-lang.org/stable/std/hash/trait.Hash.html
[BTreeMap]: https://doc.rust-lang.org/stable/std/collections/struct.BTreeMap.html
[VecMacro]: https://doc.rust-lang.org/stable/std/macro.vec!.html
[String]: https://doc.rust-lang.org/stable/std/string/struct.String.html
[to_string]: https://doc.rust-lang.org/stable/std/string/trait.ToString.html#tymethod.to_string
[str]: https://doc.rust-lang.org/stable/std/primitive.str.html
[str__find]: https://doc.rust-lang.org/stable/std/primitive.str.html#method.find
[str__as_bytes]: https://doc.rust-lang.org/stable/std/primitive.str.html#method.as_bytes
[u8]: https://doc.rust-lang.org/stable/std/primitive.u8.html
[char]: https://doc.rust-lang.org/stable/std/primitive.char.html
[Weak]: https://doc.rust-lang.org/stable/std/rc/struct.Weak.html
[IntoIterator]: https://doc.rust-lang.org/stable/std/iter/trait.IntoIterator.html
[Rc]: https://doc.rust-lang.org/stable/std/rc/struct.Rc.html
[UnsafeCell]: https://doc.rust-lang.org/stable/std/cell/struct.UnsafeCell.html
[Copy]: https://doc.rust-lang.org/stable/std/marker/trait.Copy.html
[Clone]: https://doc.rust-lang.org/stable/std/clone/trait.Clone.html
[Cell]: https://doc.rust-lang.org/stable/std/cell/struct.Cell.html
[RefCell]: https://doc.rust-lang.org/stable/std/cell/struct.RefCell.html
[Cow]: https://doc.rust-lang.org/stable/std/borrow/enum.Cow.html
[Deref]: https://doc.rust-lang.org/stable/std/ops/trait.Deref.html
[Arc]: https://doc.rust-lang.org/stable/std/sync/struct.Arc.html
[Box]: https://doc.rust-lang.org/stable/std/boxed/struct.Box.html
[Option]: https://doc.rust-lang.org/stable/std/option/enum.Option.html
[Fn]: https://doc.rust-lang.org/stable/std/ops/trait.Fn.html
[FnMut]: https://doc.rust-lang.org/stable/std/ops/trait.FnMut.html
[FnOnce]: https://doc.rust-lang.org/stable/std/ops/trait.FnOnce.html
[Result]: https://doc.rust-lang.org/stable/std/result/enum.Result.html
[RandomState]: https://doc.rust-lang.org/stable/std/collections/hash_map/struct.RandomState.html
[Add]: https://doc.rust-lang.org/stable/std/ops/trait.Add.html
[AddAssign]: https://doc.rust-lang.org/stable/std/ops/trait.AddAssign.html
[Sub]: https://doc.rust-lang.org/stable/std/ops/trait.Sub.html
[SubAssign]: https://doc.rust-lang.org/stable/std/ops/trait.SubAssign.html
[Mul]: https://doc.rust-lang.org/stable/std/ops/trait.Mul.html
[MulAssign]: https://doc.rust-lang.org/stable/std/ops/trait.MulAssign.html
[Div]: https://doc.rust-lang.org/stable/std/ops/trait.Div.html
[DivAssign]: https://doc.rust-lang.org/stable/std/ops/trait.DivAssign.html
[Neg]: https://doc.rust-lang.org/stable/std/ops/trait.Neg.html
[Rem]: https://doc.rust-lang.org/stable/std/ops/trait.Rem.html
[RemAssign]: https://doc.rust-lang.org/stable/std/ops/trait.RemAssign.html
[BitAnd]: https://doc.rust-lang.org/stable/std/ops/trait.BitAnd.html
[BitAndAssign]: https://doc.rust-lang.org/stable/std/ops/trait.BitAndAssign.html
[BitOr]: https://doc.rust-lang.org/stable/std/ops/trait.BitOr.html
[BitOrAssign]: https://doc.rust-lang.org/stable/std/ops/trait.BitOrAssign.html
[BitXor]: https://doc.rust-lang.org/stable/std/ops/trait.BitXor.html
[BitXorAssign]: https://doc.rust-lang.org/stable/std/ops/trait.BitXorAssign.html
[Not]: https://doc.rust-lang.org/stable/std/ops/trait.Not.html
[Shl]: https://doc.rust-lang.org/stable/std/ops/trait.Shl.html
[ShlAssign]: https://doc.rust-lang.org/stable/std/ops/trait.ShlAssign.html
[Shr]: https://doc.rust-lang.org/stable/std/ops/trait.Shr.html
[ShrAssign]: https://doc.rust-lang.org/stable/std/ops/trait.ShrAssign.html
[Deref]: https://doc.rust-lang.org/stable/std/ops/trait.Deref.html
[DerefMut]: https://doc.rust-lang.org/stable/std/ops/trait.DerefMut.html
[Index]: https://doc.rust-lang.org/stable/std/ops/trait.Index.html
[IndexMut]: https://doc.rust-lang.org/stable/std/ops/trait.IndexMut.html
[read__read_to_string]: https://doc.rust-lang.org/stable/std/io/trait.Read.html#method.read_to_string
[Read]: https://doc.rust-lang.org/stable/std/io/trait.Read.html
[std-io]: https://doc.rust-lang.org/stable/std/io/index.html
[File]: https://doc.rust-lang.org/stable/std/fs/struct.File.html
[read__read]: https://doc.rust-lang.org/stable/std/io/trait.Read.html#tymethod.read
[read__read_to_end]: https://doc.rust-lang.org/stable/std/io/trait.Read.html#method.read_to_end
[read__bytes]: https://doc.rust-lang.org/stable/std/io/trait.Read.html#method.bytes
[read__chars]: https://doc.rust-lang.org/stable/std/io/trait.Read.html#method.chars
[read__take]: https://doc.rust-lang.org/stable/std/io/trait.Read.html#method.take
[BufReader]: https://doc.rust-lang.org/stable/std/io/struct.BufReader.html
[Args]: https://doc.rust-lang.org/stable/std/env/struct.Args.html
[TryMacro]: https://doc.rust-lang.org/stable/std/macro.try!.html
[unwrap]: https://doc.rust-lang.org/stable/core/option/enum.Option.html#method.unwrap
[Mutex]: https://doc.rust-lang.org/stable/std/sync/struct.Mutex.html
[AtomicUsize]: https://doc.rust-lang.org/stable/std/sync/atomic/struct.AtomicUsize.html
[Sync]: https://doc.rust-lang.org/stable/std/marker/trait.Sync.html
[Drop]: https://doc.rust-lang.org/stable/std/ops/trait.Drop.html
[clone_from_slice]: https://doc.rust-lang.org/stable/std/primitive.slice.html#method.clone_from_slice
[copy]: https://doc.rust-lang.org/stable/std/ptr/fn.copy.html
[copy_nonoverlapping]: https://doc.rust-lang.org/stable/std/ptr/fn.copy_nonoverlapping.html
[clone]: https://doc.rust-lang.org/stable/std/clone/trait.Clone.html#tymethod.clone
