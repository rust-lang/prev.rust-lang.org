---
layout: it-IT/basic
title: Domande Frequenti &middot; Linguaggio di programmazione Rust
---

# Domande Frequenti

<p class="faq-intro">
Questa pagina esiste per rispondere a domande comuni inerenti il linguaggio di programmazione Rust.
Non rappresenta una guida completa al linguaggio e nemmeno uno strumento per insegnarlo.
Costituisce invece un riferimento per rispondere alle domande piú frequenti concernenti le scelte di progettazione dietro a Rust.
</p>

<p class="faq-intro">
Se c'é una domanda comune o importante che pensi sia ingiustamente non inclusa qui, sentiti libero di <a href="https://github.com/rust-lang/rust-www/blob/master/CONTRIBUTING.md">aiutarci ad aggiungerla</a>.
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
Qual'é lo scopo del progetto?
</a></h3>

Progettare e implementare un linguaggio sicuro, concorrente e pratico per la programmazione di sistemi.

Rust nasce perché altri linguaggi a questo livello di astrazione e efficienza non sono soddisfacenti. In particolare:

1. Non viene posta la dovuta attenzione alla sicurezza.
2. Supportano malamente la concorrenza.
3. Vi é una carenza di ergonomia.
4. Offrono un controllo limitato delle risorse.

Rust esiste come un'alternativa che fornisce sia codice efficiente che un livello confortevole di astrazione, contemporaneamente migliorando tutti questi quattro punti.
<h3><a href="#is-this-project-controlled-by-mozilla" name="is-this-project-controlled-by-mozilla">
Questo progetto é controllato da Mozilla?
</a></h3>
No. Rust é iniziato come un progetto hobbystico da parte di Graydon Hoare nel 2006 ed é rimasto cosí per oltre 3 anni. La Mozilla é entrata nel 2009, dopo che il linguaggio si é dimostrato abbastanza maturo per eseguire una serie di basici test automatizzati e dimostrare la valenza dei suoi principi base. Anche se sponsorizzato da Mozilla, Rust é un progetto sviluppato da
una variegata comunitá di appassionati da molti paesi del mondo. Il [Team di Rust](team.html) é composto sia da membri Mozilla che da esterni e `rust` su GitHub ha avuto oltre [1,500 sviluppatori diversi](https://github.com/rust-lang/rust/) fino ad oggi.

Finché concesso [dalla politica di gestione del progetto](https://github.com/rust-lang/rfcs/blob/master/text/1068-rust-governance.md), Rust é amministrato da un team base che imposta
la visione e le prioritá del progetto, guidandolo globalmente.
Esistono anche dei sottogruppi per guidare e incoraggiare lo sviluppo in alcune aree di interesse, inclusi il linguaggio, il compilatore, le librerie, gli strumenti, la moderazione delle comunitá ufficiali.
La progettazione é guidata da un processo [RFC](https://github.com/rust-lang/rfcs). 
Per cambiamenti che non richiedono una RFC, le decisioni sono fatte attraverso richieste di unione sul [repository `rustc`](https://github.com/rust-lang/rust).

<h3><a href="#what-are-some-non-goals" name="what-are-some-non-goals">
WQuali sono i non-obiettivi di Rust?
</a></h3>

1. Non impieghiamo nessuna tecnologia particolarmente nuova. Le vecchie tecniche ben consolidate sono meglio.
2. Non ci preme incentivare l'espressivitá, il minimalismo o l'eleganza sopra ogni altra cosa. Sono obiettivi desiderati ma non fondamentali.
3. Non ci interessa coprire tutte le funzionalitá del C++ o di qualsiasi altro linguaggio. Rust dovrebbe fornire le funzionalitá piú richieste.
4. Non vogliamo essere immobili al 100%, sicuri al 100%, riflessivi al 100% o troppo dogmatici in qualche modo. Esistono dei compromessi.
5. Non domandiamo che Rust funzioni "su ogni piattaforma possibile". Dovrá eventualmente funzionare senza inutili compromessi sulle piattaforme hardware e software piú diffuse.

<h3><a href="#how-does-mozilla-use-rust" name="how-does-mozilla-use-rust">
In quali progetti Mozilla utilizza Rust?
</a></h3>

Principalmente in [Servo](https://github.com/servo/servo), un motore di navigazione sperimentale a cui Mozilla sta lavorando. Stanno anche lavorando per [integrare componenti Rust](https://bugzilla.mozilla.org/show_bug.cgi?id=1135640) in Firefox.

<h3><a href="#what-examples-are-there-of-large-rust-projects" name="what-examples-are-there-of-large-rust-projects">
Che esempi ci sono di grandi progetti in Rust?
</a></h3>

I due piú grandi progetti open source sono al momento [Servo](https://github.com/servo/servo) e il [compilatore Rust](https://github.com/rust-lang/rust) stesso.

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

Il modo piú facile per provare Rust é sulla [playpen](https://play.rust-lang.org/), un'applicazione online per scrivere e provare codice Rust. Se invece desideri provare Rust sul tuo computer, [installalo](https://www.rust-lang.org/install.html) e prova a seguire la guida al [gioco dell'indovino](https://doc.rust-lang.org/stable/book/guessing-game.html) dal libro.

<h3><a href="#how-do-i-get-help-with-rust-issues" name="how-do-i-get-help-with-rust-issues">
Come posso ricevere aiuto con Rust?
</a></h3>

Ci sono diversi modi. Puoi:

- Scrivere su [users.rust-lang.org](https://users.rust-lang.org/), il forum ufficiale di Rust
- Chiedere sul [canale IRC ufficiale di Rust](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust) (#rust on irc.mozilla.org)
- Chiedere su [Stack Overflow](https://stackoverflow.com/questions/tagged/rust) with the "rust" tag
- Scrivere su [/r/rust](https://www.reddit.com/r/rust), il subreddit non ufficiale di Rust

<h3><a href="#why-has-rust-changed-so-much" name="why-has-rust-changed-so-much">
Perché Rust é cambiato cosí tanto col tempo?
</a></h3>

Rust é iniziato con l'obiettivo di creare un linguaggio di programmazione per sistemi stabile e utilizzabile. Per perseguire questo scopo ha esplorato molte idee, alcune sono state preservate(campi di esistenza, tratti) mentre altre sono state scartate (il sistema di stati dei tipi, il green threading).
Inoltre durante la transizione verso la versione 1.0, buona parte della libreria standard é stata riscritta per consentire al codice passato di sfruttare al meglio le funzionalitá di Rust,
fornendo interfacce di programmazione di qualitá, stabili e multipiattaforma.
Ora che Rust é alla versione 1.0, il linguaggio é garantito come "stabile"; e mentre potrebbe continuare ad evolversi, il codice funzionante sulla versione attuale dovrebbe continuare a farlo
anche nelle versioni future.

<h3><a href="#how-does-rust-language-versioning-work" name="how-does-rust-language-versioning-work">
Come funziona il numero di versione di Rust?
</a></h3>

Rust segue lo standard [SemVer](http://semver.org/), dove cambiamenti non compatibili con le versioni passate sono ammesse nelle versioni minori se questi cambiamenti
risolvono errori del compilatore, risolvono problemi di sicurezza o cambiano le regole di dichiarazione o inferenza dei tipi in modo da richiedere ulteriore specifica.
Linee guida piú dettagliate per cambi di versione minori sono disponibili come RFC approvate sia per il [linguaggio](https://github.com/rust-lang/rfcs/blob/master/text/1122-language-semver.md) che per la [libreria standard](https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md).

Rust mantiene tre canali di rilascio: stabile, beta e "nightly".
I canali stabile e beta sono aggiornati ogni sei settimane, con la nightly che diviene la nuova beta e la beta che diviene la nuova stabile.
Le funzionalitá del linguaggio e della libreria standard indicate come non stabili o nascoste dietro a blocchi di implementazione possono essere utilizzati solo 
nel canale di rilascio "nightly". Le nuove funzionalitá arrivano come instabili ma sono "liberate" una volta approvate dal team di sviluppo e relativi sottogruppi.
Questo approccio consente di sperimentare e di contemporaneamente fornire una forte garanzia di retrompatibilitá del canale stabile.

Per dettagli ulteriori, leggi il post sul blog di Rust ["Stability as a Deliverable."](http://blog.rust-lang.org/2014/10/30/Stability.html)

<h3><a href="#can-i-use-unstable-features-on-the-beta-or-stable-channel" name="can-i-use-unstable-features-on-the-beta-or-stable-channel">
Posso utilizzare funzionalitá non stabili nei canali stabile e beta?
</a></h3>

No, non puoi. Rust cerca duramente di fornire garanzie sulla stabilitá delle funzioni incluse nei canali beta e stabile.
Quando qualcosa é non stabile significa che non é possibile ancora garantire il suo utilizzo e quindi non desideriamo che ci 
si basi su queste funzionalitá o che queste non vengano modificate.
Questo ci permette di provare i cambiamenti nel canale nightly, preservando le promesse di stabilitá.

Molte cose vengono incluse nella stabile e i canali beta e stabile vengono aggiornati ogni sei settimane, con occasionali modifiche dirette anche al canale beta a volte.
Se stai aspettando la disponbilitá di una funzionalitá e non vuoi usufruire del canale nighly, puoi seguirne lo stato controllando l'etichetta [`B-unstable`](https://github.com/rust-lang/rust/issues?q=is%3Aissue+is%3Aopen+tracking+label%3AB-unstable) sulla bacheca dei problemi.

<h3><a href="#what-are-feature-gates" name="what-are-feature-gates">
Cosa sono i "Feature Gates"?
</a></h3>

I "Feature gates" sono il modo con cui Rust stabilizza funzionalitá del compilatore, del linguaggio e della libreria standard. 
Una funzione protetta é accessibile esclusivamente nel canale di rilascio "nighly" e solo quando abilitata esplicitamente con la direttiva `#[feature]` o con l'argomento a linea di comando `-Z unstable-options`. 
Quando una funzione é stabilizzata diviene disponbile sul canale di rilascio stabile e non é necessario abilitarla esplicitamente. 
A quel punto la funzione é considerata "libera". 
I "Feature gates" consentono agli sviluppatori di provare funzionalitá sperimentali mentre sono in fase di implementazione, prima che giungano nel linguaggio stabile.

<h3><a href="#why-a-dual-mit-asl2-license" name="why-a-dual-mit-asl2-license">
Perché la doppia licenza MIT/ASL2?
</a></h3>

La licenza Apache include importanti protezioni contro le aggressioni legali ma non é compatibile con la GPL versione 2.
Per evitare problemi nell'utilizzo di Rust e GPL2 é aggiunta la licenza alternativa MIT.

<h3><a href="#why-a-permissive-license" name="why-a-permissive-license">
Perché una licenza simil-BSD aperta invece che la MPL o la tri-licenza?
</a></h3>

Questo é parzialmente dovuto alla preferenza dello sviluppatore originario (Graydon) e parzialmente al fatto che i linguaggi tendono ad avere un pubblico piú vasto e una serie piú variegata di implementazioni e utilizzi di altri prodotti come i browser web. Noi vorremmo appellarci al maggior numero possibile di sviluppatori.

<h2 id="performance">Prestazioni</h2>

<h3><a href="#how-fast-is-rust" name="how-fast-is-rust">
Quanto é performante Rust?
</a></h3>

Molto! Rust é giá competitivo con programmi C e C++ ben scritti in una serie di prove (come nel [Benchmarks Game](https://benchmarksgame.alioth.debian.org/u64q/compare.php?lang=rust&lang2=gpp) e [altri](https://github.com/kostya/benchmarks)).

Come il C++, il Rust possiede [astrazioni a costo zero](http://blog.rust-lang.org/2015/05/11/traits.html) come uno dei suoi principi chiave: nessuna delle astrazioni di Rust impone un rallentamento, in qualsiasi caso.

Dato che Rust utilizza LLVM e cerca di assomigliare a Clang dal punto di vista dell'interazione con LLVM, ogni miglioramento di LLVM é condiviso da Rust.
Nel lungo periodo, la quantitá elevata di informazioni presente nel sistema dei tipi di Rust dovrebbe permettere ottimizzazioni difficili o impossibili da implementare in C/C++.

<h3><a href="#is-rust-garbage-collected" name="is-rust-garbage-collected">
Rust é un linguaggio dotato di garbage collection?
</a></h3>

No. Una delle innovazioni fondamentali di Rust é il garantire la sicurezza della memoria (nessun errore di segmentazione) *senza* richiedere un garbage collector.

Evitando di utilizzare un GC, Rust offre numerosi vantaggi: liberazione prevedibile delle risorse, gestione della memoria meno onerosa e soprattutto nessun sistema aggiuntivo operante durante l'esecuzione.
Tutte queste caratteristiche rendono Rust leggero e facile da implementare in contesti arbitrari e rendono piú facile [integrare Rust con i linguaggi in possesso di un GC](http://calculist.org/blog/2015/12/23/neon-node-rust/).

Rust non necessita di un GC grazie al suo sistema di possesso e passaggio ma lo stesso sistema aiuta con una moltitudine di altri problemi, inclusi [la gestione delle risorse in generale](http://blog.skylight.io/rust-means-never-having-to-close-a-socket/)
e la [concorrenza](http://blog.rust-lang.org/2015/04/10/Fearless-Concurrency.html).

Per quando il possesso di un valore non fosse abbastanza, i programmi Rust fanno riferimento al tipo puntatore intelligente standard a conteggio dei riferimenti [`Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html) e alla sua versione sicura in contesti paralleli [`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html), invece di affidarsi a un GC.

Stiamo comunque investigando una garbage collection *opzionale* come estensione futura.
L'obiettivo é integrarsi fluidamente con ambienti garbage-collected, come quelli offerti dai
motori di Javascript [Spidermonkey](https://developer.mozilla.org/en-US/docs/Mozilla/Projects/SpiderMonkey) e [V8](https://developers.google.com/v8/?hl=en).
Inoltre, qualcuno sta investigando l'implementazione di un garbage collector
[interamente in Rust](https://manishearth.github.io/blog/2015/09/01/designing-a-gc-in-rust/)
senza supporto del compilatore.

<h3><a href="#why-is-my-program-slow" name="why-is-my-program-slow">
Perché il mio programma é lento?
</a></h3>

Il compilatore di Rust non compila con le ottimizzazioni se non esplicitamente istruito [considerato che le ottimizzazioni rallentano la compilazione e sono scosigliate durante lo sviluppo](https://users.rust-lang.org/t/why-does-cargo-build-not-optimise-by-default/4150/3).

Se compili con `cargo` usa il parametro `--release`.
Se compili direttamente con `rustc`, usa il parametro `-O`.
Ciascuno di questi abiliterá le ottimizzazioni. 

<h3><a href="#why-is-rustc-slow" name="why-is-rustc-slow">
Rust compila lentamente. Perché?
</a></h3>

Principalmente per la traduzione del codice e le ottimizzazioni.
Rust fornisce delle astrazioni ad alto livello che vengono compilate in un codice macchina efficiente e queste trasformazioni richiedono molto tempo per essere effettuate, specialmente se in combinazione con le ottimizzazioni.

Ma il tempo di compilazione di Rust non é male come sembra e c'é da essere fiduciosi in un suo miglioramento futuro.
Comparando progetti di dimensioni simili tra il C++ e Rust il tempo di compilazione é generalmente comparabile.
La percezione di lentezza é largamente dovuta alle differenze tra il *modello di compilazione* del C++ da quello di Rust, l'unitá base del C++ é il file mentre in Rust é l'intero pacchetto, composto da molti file.
Di conseguenza durante lo sviluppo la modifica di un file causa molta meno ricompilazione che in Rust.
In questo momento é in corso uno sforzo per implementare una ristrutturazione del compilatore che permetta di effettuare la
[compilazione incrementale](https://github.com/rust-lang/rfcs/blob/master/text/1298-incremental-compilation.md), che consentirá a Rust di implementare un modello piú rapido e simile al C++.

Oltre al modello di compilazione, ci sono molti altri aspetti dell'implementazione di Rust e il suo compilatore che ne impattano le prestazioni in fase di compilazione.

Inizialmente, Rust has un sistema di tipi moderatamente complesso e deve spendere un discreto quantitativo di tempo durante la compilazione a verificarne limitazioni e utilizzi, rendendo Rust sicuro durante la sua esecuzione.

Secondariamente, il compilatore di Rust soffre di un debito tecnico di lunga data e risaputamente genera una rappresentazione intermedia per LLVM di bassa qualitá a cui LLVM deve porre rimedio.
Si spera che i futuri passaggi di trasformazione e ottimizzazione [basati su MIR](https://github.com/rust-lang/rfcs/blob/master/text/1211-mir.md) riducano la quantitá di lavoro che Rust impone a LLVM.

Terziariamente, l'utilizzo da parte di Rust di LLVM per la generazione del codice macchina é una spada a doppio taglio: se da un lato questo permette a Rust di avere prestazioni degne di nota,
LLVM é un insieme di strumenti non focalizzato alla velocitá di compilazione, specialmente con input di bassa qualitá.

Inoltre, mentre la strategia preferita da Rust per monomorfizzare i generici (simil C++) produca codice performante, domanda di generare molto piú codice rispetto ad altre strategie di implementazione.
I programmatori Rust possono utilizzare i tratti per rimuovere questo codice extra e utilizzando un dispacciamento dinamico.

<h3><a href="#why-are-rusts-hashmaps-slow" name="why-are-rusts-hashmaps-slow">
Perché le <code>HashMap</code> di Rust sono cosílente?
</a></h3>

Di base, le [`HashMap`][HashMap] di Rust utilizzano l'algoritmo di hashing [SipHash](https://131002.net/siphash/), progettato per prevenire [attacchi basati sulla collisione di hash](http://programmingisterrible.com/post/40620375793/hash-table-denial-of-service-attacks-revisited) fornendo comunque [un livello di prestazione accettabile in una varietá di compiti](https://www.reddit.com/r/rust/comments/3hw9zf/rust_hasher_comparisons/cub4oh6).

Mentre SipHash [dimostra prestazioni competitive](http://cglab.ca/%7Eabeinges/blah/hash-rs/) in molti casi, un caso in cui é conclamatamente lento é in presenza di chiavi corte, come ad esempio i numeri interi.
Questo é perché i programmatori Rust spesso incontrano problemi prestazionali con [`HashMap`][HashMap]. 
[FNV hasher](https://crates.io/crates/fnv) é spesso consigliato per queste casistiche, tenendo comunque in considerazione che non possiede le stesse caratteristiche di protezione dagli attacchi a collisione di SipHash.

<h3><a href="#why-is-there-no-integrated-benchmarking" name="why-is-there-no-integrated-benchmarking">
Perché non esiste una piattaforma integrata di misurazione delle prestazioni?
</a></h3>

Esiste ma é riservata al canale di rilascio "nightly".
Progettiamo di costituire un sistema integrato e modulare di misurazione delle prestazioni ma nel frattempo il sistema attuale é [considerato instabile](https://github.com/rust-lang/rust/issues/29553).

<h3><a href="#does-rust-do-tail-call-optimization" name="does-rust-do-tail-call-optimization">
Rust ottimizza le tail-call?
</a></h3>

In generale, no.
Le ottimizzazioni delle tail-call sono riservate ad alcune [condizioni particolari](http://llvm.org/docs/CodeGenerator.html#sibling-call-optimization)
ma [non é assicurato](https://mail.mozilla.org/pipermail/rust-dev/2013-April/003557.html). 
Siccome é sempre stata una funzionalitá desiderata, Rust ha riservato l'identificatore (`become`), 
anche se non é ancora chiaro se sia possibile questa ottimizzazione o se verrá implementata.
Era stata [proposta un'estensione](https://github.com/rust-lang/rfcs/pull/81) che avrebbe permesso di eliminare le tail-call in alcuni casi ma al momento é stata rimandata.

<h3><a href="#does-rust-have-a-runtime" name="does-rust-have-a-runtime">
Rust ha un ambiente di esecuzione?
</a></h3>

Non nel senso tipico utilizzato da linguaggi come il Java ma componenti della libreria standard di Rust possono essere considerati un "ambiente di esecuzione", fornendo controlli per heap, backtrace, unwinding, e stack.
C'é un piccolo quantitativo [di codice di inizializzazione](https://github.com/rust-lang/rust/blob/33916307780495fe311fe9c080b330d266f35bfb/src/libstd/rt.rs#L43) che viene eseguito prima della funzione `main`. 
La libreria standard di Rust inoltre é collegata alla libreria standard del C, che effettua una simile [inizializzazione](http://www.embecosm.com/appnotes/ean9/html/ch05s02.html). 
Il codice di Rust puó essere compilato anche senza la libreria standard, in questo caso il suo ambiente é circa equivalente a quello del C.

<h2 id="syntax">Sintassi</h2>

<h3><a href="#why-curly-braces" name="why-curly-braces">
Perché le parentesi graffe? La sintassi di Rust non poteva ispirarsi a Haskell o a Python?
</a></h3>

L'utilizzo delle graffe per indicare blocchi di codice é una scelta comune in una moltitudine di linguaggi di programmazione e l'adesione di Rust allo standard é utile per le persone giá familiari con lo stile.

Le graffe consentono inoltre una sintassi piú flessibile per il programmatore e per un preprocessore piú semplice nel compilatore.

<h3><a href="#why-brackets-around-blocks" name="why-brackets-around-blocks">
Posso omettere le parentesi nelle condizioni degli <code>if</code>, 
perché allora devo mettere delle parentesi attorno a righe di codice singole?
Perché non é ammesso lo stile del C?
</a></h3>

Mentre il C richiede parentesi obbligatorie per le condizioni dell'istruzione `if` ma rende le parentesi del corpo opzionali,
Rust fa la scelta opposta per i suoi `if`.
Questo lascia la condizione separata chiaramente dal corpo e evita il pericolo delle parentesi opzionali,
che possono condurre in inganno durante la modifica del codice, come il famoso errore [goto fail](https://gotofail.com/) commesso da Apple.

<h3><a href="#why-no-literal-syntax-for-dictionaries" name="why-no-literal-syntax-for-dictionaries">
Perché non c'é una sintassi letterale per i dizionari?
</a></h3>

Le scelte stilistiche di Rust sono per limitare la dimensione del *linguaggio* contemporaneamente sfruttando potenti *librerie*.
Anche se Rust fornisce una sintassi per inizializzare array e costanti letterali stringa, questi sono gli unici tipi collezione inseriti nel linguaggio.
Tutti gli altri tipi definiti da librerie, incluso l'onnipresente [`Vec`][Vec] utilizzano macro per la loro inizializzazione, ad esempio la macro [`vec!`][VecMacro].

La scelta di utilizzare le macro di Rust per facilitare l'inizializzazione di collezioni verrá probabilmente estesa a altre collezioni in futuro, permettendo inizializzazioni semplici non solo di 
[`HashMap`][HashMap] e [`Vec`][Vec], ma anche altri tipi di collezioni come [`BTreeMap`][BTreeMap]. 
Nel frattempo, se vuoi una sintassi piú comoda per inizializzare le collezioni, puoi [creare la tua macro](https://stackoverflow.com/questions/27582739/how-do-i-create-a-hashmap-literal) per fornirla.

<h3><a href="#when-should-i-use-an-implicit-return" name="when-should-i-use-an-implicit-return">
Quando dovrei utilizzare un ritorno implicito?
</a></h3>

Rust é un linguaggio molto espression-centrico e i ritorni impliciti fanno parte del suo design.
Costrutti come gli `if`, i `match`e i normali blocchi sono tutte espressioni in Rust. 

Ad esempio, il codice seguente controlla se un [`i64`][i64] é dispari, ritornando il risultato semplicamente fornendone il risultato:

```rust
fn dispari(x: i64) -> bool {
    if x % 2 != 0 { true } else { false }
}
```

Anche se puó essere ulteriormente semplificato come qui:


```rust
fn dispari(x: i64) -> bool {
    x % 2 != 0
}
```

In ciascun esempio, l'ultima riga rappresenta il valore di ritorno della funzione.
Risulta importante specificare che se una funzione termina con un punto e virgola il suo tipo di ritorno sará `()`, indicando nessun valore di ritorno. 
I ritorni impliciti funzionano quindi esclusivamente in assenza del punto e virgola.

I ritorni espliciti sono utilizzati solo se un ritorno implicito risulta impossibile perché si desidera ritornare un valore prima della fine del corpo della funzione.
Mentre ciascuna delle funzioni sopra avrebbe potuto includere un `return` e un punto e virgola, questa aggiunta sarebbe inutilmente prolissa e inconsistente con le convenzioni del codice Rust.

<h3><a href="#why-arent-function-signatures-inferred" name="why-arent-function-signatures-inferred">
Perché i tipi delle funzioni non sono dedotti automaticamente?
</a></h3>

In Rust le dichiarazioni sono tendelzialmente accompagnate da un tipo esplicito, mentre nel codice i tipi vengono dedotti.
Ci sono multiple ragioni per questa scelta:

- Dichiarazioni obbligatorie dei tipi obbligano a mantenere un'interfaccia stabile a livello di modulo e di pacchetto.
- I tipi facilitano la comprensione del codice per il programmatore, eliminando la necessitá di un ambiente di sviluppo che possa desumere il tipo analizzando il codice dell'intero pacchetto:
il tipo é sempre disponibile nelle vicinanze e in modo esplicito.
- Semplifica gli algoritmi che gesticono la deduzione dei tipi, visto che puó essere analizzata una funzione alla volta.

<h3><a href="#why-does-match-have-to-be-exhaustive" name="why-does-match-have-to-be-exhaustive">
Perché <code>match</code> deve essere cosí completo?
</a></h3>

Per assistere nelle modifiche e in chiarezza.

Prima di tutto, se ogni possibilitá viene coperta da un `match`, l'aggiunta di varianti a un `enum` causerá un errore di compilazione, invece che un problema durante l'esecuzione.
Questo tipo di assistenza permette di modificare il codice Rust liberamente e senza paura.

Secondariamente, il controllo dettagliato rende esplicito il caso predefinito: in generale l'unico modo per creare un `match` non esaustivo sarebbe di far andare in errore il processo se non viene incontrato alcun valore previsto.
Le versioni iniziali di Rust non prevedevano la completezza di `match` ed é stato accertato che questa scelta ha causato una moltitudine di problematiche.


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

Se ció che conta é la precisione massima, [`f64`][f64] é da preferire. 
Se invece si vuole minimizzare l'impatto in memoria e l'efficienza, ignorando la precisione persa, [`f32`][f32] é piú appropriato. 
Svolgere operazioni sugli [`f32`][f32] é generalmente piú veloce, anche in piattaforme a 64-bit. 
Come esempio, la grafica sfrutta tipicamente gli [`f32`][f32] perché richiede alte prestazioni e i numeri in virgola mobile a 32-bit bastano per rappresentare i pixel a schermo.

Nel dubbio, scegli [`f64`][f64] per una maggiore precisione.

<h3><a href="#why-cant-i-compare-floats" name="why-cant-i-compare-floats">
Perché non posso comparare i numeri a virgola mobile o usarli come chiave per un <code>HashMap</code> o un <code>BTreeMap</code>?
</a></h3>

I numeri a virgola mobile si possono comparare con gli operatori `==`, `!=`, `<`, `<=`, `>`, e `>=` , e attraverso la funzione `partial_cmp()`. 
`==` e `!=` possiedono il tratto [`PartialEq`][PartialEq], mentre `<`, `<=`, `>`, `>=`, e `partial_cmp()` possiedono il tratto [`PartialOrd`][PartialOrd].

I numeri a virgola mobile non sono confrontabili con la funzione `cmp()`, possidente il tratto [`Ord`][Ord], dato che i numeri a virgola mobile non costituiscono insieme totalmente ordinato. 
Inoltre non esiste relazione di uguaglianza completa numeri a virgola mobile e quindi non implementano il tratto[`Eq`][Eq].

Non esiste l'ordinamento totale o l'uguaglianza tra numeri a virgola mobile a causa del valore [`NaN`](https://en.wikipedia.org/wiki/NaN) che non é minore, maggiore o uguale di alcun altro numero o sé stesso.

Visto che i numeri a virgola mobile non implementano i tratti [`Eq`][Eq] o [`Ord`][Ord], non sono utilizzabili nei tipi le cui limitazioni esigono l'implementazione di queste caratteristiche, come [`BTreeMap`][BTreeMap] o [`HashMap`][HashMap]. 
Questo é importante perché questi tipi *suppongono* che le chiavi forniscano relazioni di ordinamento totale o di uguaglianza, a pena di malfunzionamenti.

Esiste [un pacchetto](https://crates.io/crates/ordered-float) che racchiude [`f32`][f32] e [`f64`][f64] in modo da fornire i tratti [`Ord`][Ord] e [`Eq`][Eq] che potrebbe assistere in certi casi.

<h3><a href="#how-can-i-convert-between-numeric-types" name="how-can-i-convert-between-numeric-types">
Come posso convertire tra i tipi numerici?
</a></h3>

Ci sono due modi: la parola chiave `as`, che esegue una semplice conversione per tipi primitivi e i tratti [`Into`][Into] e [`From`][From], 
che sono implementati per una serie di conversioni tra tipi numerici (e che anche tu puoi implementare sui tuoi tipi).
I tratti [`Into`][Into] e [`From`][From] sono implementati esclusivamente per le conversioni prive di perdita, qundi ad esempio, `f64::from(0f32)` funziona mentre `f32::from(0f64)` no. 
D'altro canto, `as` convertirá tra qualsiasi coppia di tipi primitivi, effettuando i necessari troncamenti..


<h3><a href="#why-doesnt-rust-have-increment-and-decrement-operators" name="why-doesnt-rust-have-increment-and-decrement-operators">
Perché Rust non possiede operatori per il decremento e incremento?
</a></h3>

Gli operatori di preincremento e postincremento (e relativi opposti equivalenti), anche se convenienti presentano una discreta complessitá;
Richiedono una conoscenza dell'ordine di esecuzione e spesso conducono a piccoli problemi e comportamenti anormali in C e C++.
`x = x + 1` o `x += 1` é leggermente piú prolisso ma chiaro.

<h2 id="strings">Stringhe</h2>

<h3><a href="#how-to-convert-string-or-vec-to-slice" name="how-to-convert-string-or-vec-to-slice">
Come posso convertire una <code>String</code> o un <code>Vec&lt;T&gt;</code> a una partizione (<code>&amp;str</code> e <code>&amp;[T]</code>)?
</a></h3>

Solitamente, puoi passare un riferimento a una `String` o ad un `Vec<T>` ovunque ci si aspetti una partizione.
Utilizzando le [costrizione da de-riferimento](https://doc.rust-lang.org/stable/book/deref-coercions.html), [`String`][String] e [`Vec`][Vec] verrano automaticamente ridotti alle rispettive partizioni quando passate come riferimento tramite `&` o `& mut`.

I metodi implementati su `&str` e `&[T]` possono essere utilizzati direttamente su `String` e `Vec<T>`.
Ad esempio `una_stringa.char_at(0)` funzionerá anche se `char_at` é un metodo su `&str` e `una_stringa` é una `String`.

In alcuni casi, come nella programmazione generica é necessario convertire manualmente, questa operazione é effettuabile utilizzando l'operatore di partizione, in questo modo: `&mio_vettore[..]`.

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

[`String`][String] é un'area di memoria allocata nel heap di byte UTF-8. 
Le [`String`][String] mutabili possono essere modificate, espanendosi se necessario. 
[`&str`][str] é una "vista" di capacitá fissata in una [`String`][String] allocata altrove, generalmente nel heap, in caso di partizioni riferite a [`String`][String], o in memoria statica, per le costanti letterali.

[`&str`][str] é un tipo primitivo implementato nel linguaggio Rust, mentre [`String`][String] é implementato dalla libreria standard.

<h3><a href="#how-do-i-do-o1-character-access-in-a-string" name="how-do-i-do-o1-character-access-in-a-string">
Come accedo ai caratteri di una <code>String</code> con complessitá asintottica O(1)?
</a></h3>

Non é possibile. Senza una chiara comprensione di cosa intendi per "carattere" e una pre-elaborazione della stringa per ritrovare l'indice del carattere desiderato.

Le stringhe in Rust sono codificate in UTF-8.
Un singolo carattere UTF-8 non é obbligatoriamente grande un singolo byte come sarebbe una stringa codificata in ASCII.
Ogni byte é chiamato una "unitá di codice" (nello UTF-16, le unitá di codice sono di 2 byte, nello UTF-32 sono di 4 byte).
I "punti di codice" sono composti di una o piú unitá di codice, combinate in "gruppi di grafemi" che fedelmente ricalcano il concetto tradizionale di caratteri.

Anche con la possibilitá di indicizzare i byte in una stringa UTF-8, non puoi accedere all'`i`-esimo elemento del gruppo di grafemi in un tempo costante.
Ad ogni modo, se conosci a quale byte si trova il punto di codice o gruppo di grafemi desiderato, quindi _puoi_ accedervi in tempo costante.
Le funzioni, incluse [`str::find()`][str__find] e le espressioni regolari ritornano indici dei byte, facilitando questo tipo di accesso.

<h3><a href="#why-are-strings-utf-8" name="why-are-strings-utf-8">
Perché le stringhe sono UTF-8?
</a></h3>

Le [`str`][str] sono UTF-8 perché nel mondo questa codifica é frequente - specialmente in trasmissioni di rete, che ignorano l'ordine di bit della piattaforma - pensiamo quindi sia meglio che il trattamento standard dell'I/O non preveda la ricodifica in entrambe le direzioni.

Questo significa che individuare un particolare punto di codice Unicode dentro una stringa é un'operazione O(n), anche se, conoscendo l'indice del byte ci si puó accedere in un tempo O(1) come previsto.
Sotto un certo punto di vista questo é chiaramente sconveniente; ma d'altro canto questo problema é pieno di compromessi e vorremmo sottolineare alcune precisazioni importanti:

Scorrere una [`str`][str] per valori della codifica ASCII puó essere fatto in sicurezza byte per byte,
utilizzando [`.as_bytes()`][str__as_bytes] ed estraendo [`u8`][u8] con un costo `O(1)` e producendo valori che possono essere trasformati e comparati con la codifica ASCII con il tipo [`char`][char]. 
Quindi se per (ad esempio) vogliamo andare a capo ad ogni `'\n'`, una ricerca byte a byte continua a essere funzionante, grazie alla flessibilitá dello standard UTF-8.

La maggior parte delle operazioni orientate ai caratteri sul testo funzionano su presupposti molto ristretti come ad esempio l'esclusione dei caratteri non ASCII.
Anche in questo caso all'esterno della codifica ASCII si tende a utilizzare comunque un algoritmo complesso(con complessitá non costante) per determinare i bordi di caratteri, parole e paragrafi.
Noi consigliamo di utilizzare un algoritmo "onesto", approvato da Unicode e adattato al linguaggio da considerare.

Il tipo [`char`][char] é UTF-32. 
Se hai la certezza di dover richiedere l'analisi di un "punto di codice" alla volta é semplicissimo scrivere `type wstr = [char]` e caricarci dentro una [`str`][str] in un solo passaggio, per poi lavorare con il nuovo `wstr`. 
In altre parole: il fatto che il linguaggio non decodifichi a UTF32 come prima opzione non ti deve inibire da decodificare(o ri-codificare per il processo inverso) i caratteri se necessiti di lavorare in quella codifica.

Per una spiegazione piú dettagliata su perché UTF-8 é preferibile rispetto a UTF-16 o UTF-32, leggi il [manifesto di UTF-8 Everywhere](http://utf8everywhere.org/).

<h3><a href="#what-string-type-should-i-use" name="what-string-type-should-i-use">
Quale tipo stringa dovrei usare?
</a></h3>

Rust ha quattro paia di tipi stringa, [ciascuno assolve un ruolo distinto](http://www.suspectsemantics.com/blog/2016/03/27/string-types-in-rust/).
In ciascun paio, c'é un tipo stringa "posseduto" e un tipo stringa "partizione".
I tipi sono suddivisi cosí:

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

Ci sono diverse opzioni, a seconda delle necessitá della funzione:

- Se la funzione richiede una stringa posseduta ma vuole accettare una qualsiasi stringa utilizza la restrizione al tratto `Into<String>`.
- Se la funzione richiede una partizione di stringa ma vuole accettare una qualsiasi stringa utilizza la restrizione al tratto `AsRef<str>`.
- Se alla funzione non interessa del tipo di stringa e vuole gestire uniformemente le due possibilitá utilizza il tipo `Cow<str>.`

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

In questo esempio, la funzione accetta un `Cow<str>`, che non é un tipo generico ma un contenitore, contenente o una stringa posseduta o una partizione di stringa all'occorrenza.

```rust
fn accetta_cow(s: Cow<str>) {
    // ... il corpo della funzione
}
```


<h2 id="collections">Collezioni</h2>

<h3><a href="#can-i-implement-linked-lists-in-rust" name="can-i-implement-linked-lists-in-rust">
Posso implementare efficientemente strutture dati come vettori e liste concatenate in Rust?
</a></h3>

If vuoi implementare queste strutture dati per utilizzarle in altri programmi non é necessario, essendo implementazioni efficienti di queste strutture giá disponibili nella libreria standard.

Se invece, [vuoi solo imparare](http://cglab.ca/~abeinges/blah/too-many-lists/book/), probabilmente dovrai entrare nel codice insicuro.
Anche se é _possibile_ implementarle solo con codice sicuro, le prestazioni sarebbero probabilmente peggiori di come sarebbe stato lo stesso codice con l'utilizzo di codice insicuro.
La semplice ragione per ció é che le strutture dati come vettori e liste concatenate fanno affidamento a operazioni su puntatori e memoria che sono proibiti nel Rust sicuro.

Per esempio, una lista concatenata doppia richiede due riferimenti mutabili a ciascun nodo ma questo viola le regole di Rust sull'esclusivitá dei riferimenti mutabili.
Si puó risolvere il problema utilizzando [`Weak<T>`][Weak], ma le prestazioni sarebbero probabilmente peggiori di quanto desiderato.
Con il codice insicuro puoi ignorare le regole di esclusivitá ma devi verificare manualmente che il tuo codice non introduca violazioni nella sua gestione della memoria.

<h3><a href="#how-can-i-iterate-over-a-collection-without-consuming-it" name="how-can-i-iterate-over-a-collection-without-consuming-it">
Come posso iterare una collezione senza muoverla o consumarla?
</a></h3>

Il modo piú semplice é utilizzare l'implementazione del tratto [`IntoIterator`][IntoIterator]. 
Ecco qui un esempio con [`&Vec`][Vec]:

```rust
let v = vec![1,2,3,4,5];
for oggetto in &v {
    print!("{} ", oggetto);
}
println!("\nLunghezza: {}", v.len());
```

I cicli `for` in Rust chiamano la funzione `into_iter()` (definita dal tratto [`IntoIterator`][IntoIterator]) per qualsiasi cosa stiano analizzando. 
Tutto ció che implementa il tratto [`IntoIterator`][IntoIterator] puó essere traversato con un ciclo `for`. 
[`IntoIterator`][IntoIterator] é implementato per [`&Vec`][Vec] e [`&mut Vec`][Vec], obbligando l'iteratore generato da `into_iter()` a prendere in prestito i contenuti della collezione, al posto di consumarli o muoverli. 
Lo stesso é vero per le altre collezioni della libreria standard.

Se si desidera un iteratore che muova/consumi i valori, basta scrivere lo stesso ciclo `for` omettendo `&` o `&mut`.

Se si necessita accesso diretto all'iteratore, vi si puó usualmente accedere invocando il metodo `iter()`.

<h3><a href="#why-do-i-need-to-type-the-array-size-in-the-array-declaration" name="why-do-i-need-to-type-the-array-size-in-the-array-declaration">
Perché devo inserire la dimensione del vettore alla sua dichiarazione?
</a></h3>

Non é necessario. Se dichiari direttamente un vettore, la dimensione é dedotta dal numero di elementi. Se invece dichiari una funzione che accetta un vettore di dimensione prefissata il compilatore deve avere modo di sapere quale sará la dimensione di quel vettore.

Una cosa da notare é che attualmente Rust non offere generici su array di dimensioni diverse.
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

Si puó fare ma é inutile.
La struttura diventa permanentemente prestata a sé stessa e quindi non puó essere copiata.
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
Che differenza sussiste tra passare per valore, consumare, spostare e trasferire la proprietá?
</a></h3>

Sono parole diverse per dire la stessa cosa.
In tutti i casi significa che il valore é stato trasferito a un nuovo proprietario, o che la proprietá é stata trasferita dal possessore originario, che quindi non puó piú accedervi.
Se un tipo implementa il tratto `Copy`, il valore del proprietario originale non viene invalidato e puó essere utilizzato nuovamente.

<h3><a href="#why-can-values-of-some-types-by-reused-while-others-are-consumed" name="why-can-values-of-some-types-by-reused-while-others-are-consumed">
Perché alcuni valori di alcuni tipi possono essere utilizzati dopo il passaggio a una funzione mentre il riutilizzo di valori di altri tipi genera errori?
</a></h3>

Se un tipo implementa il tratto [`Copy`][Copy], esso verrá copiato durante il suo passaggio a una funzione. 
Tutti i tipi numerici in Rust implementano [`Copy`][Copy] ma le strutture in maniera predefinita non implementano [`Copy`][Copy], quindi invece di essere copiate sono mosse. 
Ció implica che la struttura non possa piú essere utilizzata altrove, se non viene ritornata dalla funzione tramite un `return`.

<h3><a href="#how-do-you-deal-with-a-use-of-moved-value-error" name="how-do-you-deal-with-a-use-of-moved-value-error">
Come si gestice l'errore "utilizzo di un valore spostato"?
</a></h3>

Questo errore significa che il valore che stai cercando di utilizzare é stato trasferito a un nuovo proprietario.
La prima cosa da controllare in questo caso é se il trasferimento era davvero necessario: se il valore era stato mosso a una funzione, potrebbe essere possibile riscriverla per utilizzare un riferimento invece che il trasferimento, ad esempio.
Altrimenti se il tipo mosso implementa il tratto [`Clone`][Clone], chiamare `clone()` su di esso prima di muoverlo trasferirá una sua copia, mantenendo l'originale disponibile per utilizzi futuri. 
Nota che la clonazione di un valore dovrebbe essere l'ultima spiaggia, essendo la procedura di clonazione costosa e causa di allocazioni di memoria.

Se il valore mosso é uno dei tuoi tipi personalizzati, considera implementare il tratto [`Copy`][Copy] (per la copia implicita al posto del trasferimento) o [`Clone`][Clone] (copia esplicita). 
[`Copy`][Copy] é generalmente implementato con la direttiva `#[derive(Copy, Clone)]` ([`Copy`][Copy] richiede [`Clone`][Clone]), e [`Clone`][Clone] con la direttiva `#[derive(Clone)]`.

Se nulla di tutto questo é possibile, probabilmente devi modificare la funzione che ha acquisito il possesso per fare in modo che restituisca la proprietá alla sua uscita.

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
> - uno o piú riferimenti (&T) a una risorsa.
> - esattamente un riferimento mutabile (&mut T)

Nonostante le regole siano molto semplici, seguirle correttamente puó essere molto complicato, specialmente per coloro che non sono abitutati a ragionare in termini di campi di esistenza e possesso.

La prima regola é comprendere che il gestore dei prestiti identifica veramente gli errori che produce: molto lavoro é stato profuso per renderlo un assistente di qualitá nella risoluzione delle problematiche che individua.
Quando incontri un problema segnalato dal gestore dei prestiti, il primo passo é di leggere lentamente e con attenzione l'errore e poi approcciarsi al codice una volta compreso davvero l'errore descritto.

Il secondo passo é cercare di familiarizzare con i tipi contenitore associati con i concetti di possesso e mutabilitá forniti dalla libreria standard di Rust, includendo [`Cell`][Cell], [`RefCell`][RefCell] e [`Cow`][Cow]. 
Questi utili e necessari strumenti possono aiutare ad esprimere efficientemente alcune situazioni complesse di possesso e mutabilitá.

La singola cosa piú importante nella comprensione del gestore dei prestiti é la pratica.
La potente analisi statica fatta da Rust é particolare e molto differente da molte esperienze di programmazione precedente.
Ci vorrá un po' di tempo per acquisire la giusta tranquillitá.

Se ti ritrovi in difficoltá con il gestore dei prestiti, oppure hai finito la pazienza, sentiti libero di chiedere un aiuto alla [comunitá di Rust](community.html).

<h3><a href="#when-is-rc-useful" name="when-is-rc-useful">
Quando puó venire utile utilizzare una <code>Rc</code>?
</a></h3>

Questo é coperto dalla documentazione ufficiale per [`Rc`][Rc] il tipo di puntatore non-atomico utilizzante il reference counting di Rust. 
In breve, [`Rc`][Rc] e il suo cugino, amico del multithreading [`Arc`][Arc] sono utili per indicare possesso condiviso e vengono deallocati automaticamente dal sistema quando nessuno vi accede.

<h3><a href="#how-do-i-return-a-closure-from-a-function" name="how-do-i-return-a-closure-from-a-function">
Come posso ritornare una chiusura da una funzione?
</a></h3>

Per ritornare una chiusura da una funzione, essa deve essere una "chiusura da movimento", ovvero che essa deve essere dichiarata dalla parola `move`.
Come [illustrato nel libro di Rust](https://doc.rust-lang.org/book/closures.html#move-closures), questo fornisce alla chiusura la sua copia delle variabili catturate, indipendentemente dal quadro di allocazione del chiamante. 
Altrimenti, ritornare da una chiusura sarebbe insicuro, visto che permetterebbe di accedere a variabili non piú disponibili; detto in un altro modo: permetterebbe di leggere dati da locazioni di memoria errate.
La chiusura deve anche essere racchiusa in un [`Box`][Box], in modo da essere allocata nella heap. 
Puoi saperne di piú [nel libro](https://doc.rust-lang.org/book/closures.html#returning-closures).

<h3><a href="#what-are-deref-coercions" name="what-are-deref-coercions">
Cos'é un deriferimento forzato e come funziona?
</a></h3>

Un [deriferimento forzato](https://doc.rust-lang.org/book/deref-coercions.html) é una pratica conversione automatica di delle referenze 
a puntatori (es., `&Rc<T>` or `&Box<T>`) a referenze ai loro contenuti
(es., `&T`).
Il deriferimento forzato esiste per rendere l'utilizzo di Rust piú comodo e sono implementati dal tratto [`Deref`][Deref].

Una implementazione di `Deref` indica che il tipo implementante potrebbe essere convertito in un valore con una chiamata al metodo `deref`, che prende un riferimento immutabile al tipo chiamante e ritorna un riferimento(senza violare peró il campo di esistenza) del tipo obiettivo. 
L'operatore `*`, se utilizzato come prefisso é un metodo piú veloce per accedere a `deref`.

Sono chiamate "forzature" a causa delle regole spiegate qui [nel libro di Rust](https://doc.rust-lang.org/stable/book/deref-coercions.html):

> Se hai un tipo `U` ed esso implementa `Deref<Target=T>`, i valori `&U` verranno automaticamente convertiti in `&T`.

Ad esempio, se hai un `&Rc<String>`, esso verrá forzato per questa regola a `&String`, che puó essere forzato anche a `&str` nello stesso modo. 
Quindi se una funzione accettasse un parametro `&str`, puoi passare direttamente un `&Rc<String>` e tutte le forzature e verranno gestite automaticamente dal tratto `Deref`.

I tipici deriferimenti forzati sono:

- `&Rc<T>` a `&T`
- `&Box<T>` a `&T`
- `&Arc<T>` a `&T`
- `&Vec<T>` a `&[T]`
- `&String` a `&str`

<h2 id="lifetimes">Campi di esistenza</h2>

<h3><a href="#why-lifetimes" name="why-lifetimes">
Perché i campi di esistenza?
</a></h3>

I campi di esistenza sono la soluzione di Rust al problema della sicurezza della memoria.
Questi consentono a Rust di assicurare la sicurezza della memoria senza i costi prestazionali della garbage collection.
Sono basati su una serie di articoli accademici che possono essere trovati nel [libro di Rust](https://doc.rust-lang.org/stable/book/bibliography.html#type-system).

<h3><a href="#why-is-the-lifetime-syntax-the-way-it-is" name="why-is-the-lifetime-syntax-the-way-it-is">
Perché la sintassi dei campi di esistenza é fatta cosí?
</a></h3>

La sintassi `'a` proviene dalla famiglia ML di linguaggi di programmazione, dove `'a` viene utilizzato per indicare un parametro di tipo generico.
In Rust, la sintassi doveva rappresentare qualcosa di univoco, chiaramente visibile e integrato con le altre dichiarazioni dei tipi insieme ai vari tratti e riferimenti.
Sono state prese in considerazione anche altre scelte ma nessuna sintassi alternativa si é dimostrata chiaramente migliore.

<h3><a href="#how-do-i-return-a-borrow-to-something-i-created-from-a-function" name="how-do-i-return-a-borrow-to-something-i-created-from-a-function">
Come posso ritornare un prestito a qualcosa che ho creato in una funzione?
</a></h3>

Devi assicurarti che il campo di esistenza del valore prestato sia piú lungo di quello della funzione.
Puoi ottenere questo effetto puoi assegnare il campo di esistenza dell'uscita a quello di un parametro di ingresso come qui:

```rust
type Gruppo = TypedArena<Cosa>;

// (Il campo di esistenza sotto é esplicitato esclusivamente
/// per facilitarne la comprensione; esso puó essere omesso
// tramite le regole di elisione consultabili in un'altra
// risposta presente in questa pagina)

fn crea_prestato<'a>(gruppo: &'a Gruppo,
                       x: i32,
                       y: i32) -> &'a Cosa {
    gruppo.alloc(Cosa { x: x, y: y })
}
```

Un'alternativa é eliminare interamente il riferimento ritornando un valore posseduto come [`String`][String]:

```rust
fn buon_compleanno(nome: &str, eta: i64) -> String {
    format!("Ciao {}! Hai {} anni!", nome, eta)
}
```

Questo approccio é piú semplice ma spesso genera allocazioni in memoria non necessarie.

<h3><a href="#when-are-lifetimes-required-to-be-explicit" name="when-are-lifetimes-required-to-be-explicit">
Perché alcuni riferimenti hanno un campo di esistenza, come <code>&amp;'a T</code> e altri no, tipo <code>&amp;T</code>?
</a></h3>

In realtá, *tutti* i riferimenti hanno un campo di esistenza ma nella maggior parte dei casi non 
ti devi preoccupare di gestirlo esplicitamente. Le regole sono le seguenti:

1. Nel corpo di una funzione non devi mai specificare un campo di esistenza esplicitamente;
   il valore corretto dovrebbe essere sempre dedotto correttamente.
2. Nella *dichiarazione* di una funzione (ad esempio, nei tipi dei suoi parametri
   o il suo tipo di ritorno), *potresti* dover specificare un campo di esistenza manualmente.
   I campi di esistenza in questo contesto utilizzano un semplice metodo chiamato
   ["elisione del campo di esistenza"](https://doc.rust-lang.org/book/lifetimes.html#lifetime-elision),
   che a sua volta consiste in queste tre regole:
  - Ciascun campo di esistenza eliso nei parametri di una funzione diviene un campo di esistenza distinto.
  - Se vi é esattamente un singolo campo di esistenza in ingresso, eliso o no, quel campo di esistenza
    viene assegnato a tutti i campi di esistenza elisi utilizzati per i valori di ritorno di quella funzione.
  - Se ci sono piú campi di esistenza in ingresso ma uno di quelli é un `&self` o un `&mut self`, il campo di esistenza
    di `self` viene assegnato a tutti i campi di esistenza di uscita.
3. Se si sta definendo una `struct` o `enum` i campi di esistenza sono da dichiarare espressamente.

Se queste regole danno origine a un errore di compilazione, il compilatore di Rust fornirá un messaggio di errore indicante l'errore e anche una potenziale soluzione basata sugli algoritmi di deduzione.

<h3><a href="#how-can-rust-guarantee-no-null-pointers" name="how-can-rust-guarantee-no-null-pointers">
Come puó Rust garantire l'assenza di "puntatori nulli" e "puntatori sospesi"?
</a></h3>

L'unico modo di creare un valore `&Cosa` or `&mut Cosa` é di specificare un valore preesistente di tipo `Coso` a cui la referenza deve fare riferimento.
Il riferimento in questo modo ottiene in prestito il valore originale per un determinato blocco di codice(il campo di esistenza del riferimento) e il valore prestato non puó essere spostato o distrutto per tutta la durata del prestito.

<h3><a href="#how-do-i-express-the-absense-of-a-value-without-null" name="how-do-i-express-the-absense-of-a-value-without-null">
Come faccio a indicare l'assenza di un valore senza utilizzare <code>null</code>?
</a></h3>

Puoi fare ció con il tipo [`Option`][Option] che puó alternativamente essere `Some(T)` o `None`. 
`Some(T)` indica che un valore di tipo `T` é contenuto all'interno, mentre `None` ne indica l'assenza.

<h2 id="generics">Generici</h2>

<h3><a href="#what-is-monomorphisation" name="what-is-monomorphisation">
Cos'é la "monomorfizzazione"?
</a></h3>

La monomorfizzazione specializza ciascun utilizzo di una funzione(o struttura) generica in base ai tipi di parametri di ciascuna chiamata(o agli utilizzi della struttura).

Durante la monomorfizzazione viene generata una nuova versione specializzata della funzione per ciascun set univoco di tipi.
Questa strategia, giá utilizzata nel C++, genera del codice macchina efficiente, specializzato per ciascuna chiamata e invocato staticamente, con lo svantaggio che una funzione istanziata con tanti tipi diversi 
puó dare luogo a molta duplicazione nel codice generato, generando quindi eseguibili piú grandi rispetto ad altre strategie di traduzione.

Le funzioni che accettano [oggetti caratterizzati solo da tratti](https://doc.rust-lang.org/book/trait-objects.html) invece dei tipi non sono soggette alla monomorfizzazione. 
Invece, i metodi invocati su oggetti tratto sono gestiti dinamicamente durante l'esecuzione.

<h3><a href="#whats-the-difference-between-a-function-and-a-closure-that-doesnt-capture" name="whats-the-difference-between-a-function-and-a-closure-that-doesnt-capture">
Qual'é la differenza tra una funzione e una chiusura che non cattura nessuna variabile?
</a></h3>

Le funzioni e le chiusure si utilizzano allo stesso modo ma hanno una gestione differente in fase di esecuzione a causa di una implementazione diversa.

Le funzioni sono un costrutto fondamentale del linguaggio, mentre le chiusure sono essenzialemente un modo piú semplice per indicare uno di questi tre tratti: [`Fn`][Fn], [`FnMut`][FnMut] e [`FnOnce`][FnOnce]. 
Quando scrivi una chiusura, il compilatore di Rust automaticamente provvede a generare una struttura implementante il tratto piú idoneo tra quei tre e a catturare le variabili corrette come membri,
generando anche la possibilitá di utilizzare la struttura come una funzione. Le strutture, al contrario, non catturano alcuna variabile.

La fondamentale differenza tra questi tratti é come acquisiscono il parametro `self`.
[`Fn`][Fn] prende `&self`, [`FnMut`][FnMut] prende `&mut self` mentre[`FnOnce`][FnOnce] prende `self`.

Anche se una cattura non cattura alcuna variabile di ambiente, viene rappresentata in fase di esecuzione tramite due puntatori, come qualsiasi altra chiusura.

<h3><a href="#what-are-higher-kinded-types" name="what-are-higher-kinded-types">
Cosa sono i tipi di piú alto livello, perché sono richiesti e perché Rust non li implementa?
</a></h3>

I tipi di piú alto livello sono tipi con parametri non specificati. I costruttori di tipi come [`Vec`][Vec], [`Result`][Result] e [`HashMap`][HashMap] sono tutti esempi di tipi di piú alto livello: 
ciascuno richiede alcuni tipi aggiuntivi per poter denotare effettivamente il suo tipo, come nel caso di `Vec<u32>`. 
Il supporto per i tipi di alto livello significa che questi tipi "incompleti" possono essere utilizzati ovunque possano essere utilizzati anche i tipi "completi", non escludendo le funzioni generiche.

Ogni tipo completo, come [`i32`][i32], [`bool`][bool], o [`char`][char] é di tipo `*` (questa notazione deriva dalla teoria correlata con il sistema dei tipi). 
Un tipo a parametro singolo, come [`Vec<T>`][Vec] é invece `* -> *`, ovvero che vec [`Vec<T>`][Vec] accetta un tipo completo come [`i32`][i32] e ritorna il tipo completo `Vec<i32>`. 
Un tipo con tre parametri, come [`HashMap<K, V, S>`][HashMap] é di tipo `* -> * -> * -> *` perché accetta tre tipi completi (come [`i32`][i32], [`String`][String] e [`RandomState`][RandomState]) per generare un nuovo tipo completo `HashMap<i32, String, RandomState>`.

In aggiunta a questi esempi, i costruttori di tipo possono accettare dei parametri sul *campo di esistenza*, che denoteremo con `Lt`.
Ad esempio `slice::Iter` ha il tipo `Lt -> * -> *`, perché va istanziato ad esempio come `Iter<'a, u32>`.

La mancanza di supporto per i tipi di piú alto livello rende difficile scrivere alcuni tipi di codice generico.
Risulta particolarmente problematico astrarre su concetti come gli iteratori, dato che essi sono spesso parametrizzati nei confronti di uno specifico campo di esistenza.
Queste premesse hanno impedito la creazione di tratti che astraggano ulteriormente le collezioni presenti in Rust.

Un altro esempio frequente é da ricercare nei concetti di functor e monad, entrambi dei quali sono costruttori di tipi, invece che tipi individuali.

Rust al momento non possiede supporto per i tipi di piú alto livello perché non é stato prioritizzato lo sviluppo di questa funzione rispetto ad altre funzionalitá che rispecchiano meglio gli obiettivi del progetto.
Essendo la progettazione di funzionalitá importanti come queste un campo minato, vorremmo procedere con cautela, non c'é un'altra ragione particolare sul perché Rust non possiede questa funzionalitá.

<h3><a href="#what-do-named-type-parameters-in-generic-types-mean" name="what-do-named-type-parameters-in-generic-types-mean">
Cosa significano i parametri fatti tipo <code>&lt;T=Foo&gt;</code> nel codice generico?
</a></h3>

Questi sono chiamati [tipi associati](https://doc.rust-lang.org/stable/book/associated-types.html), permettono di indicare limitazioni di tratto non esprimibili con un costrutto `where`. 
Ad esempio, una limitazione generica `X: Bar<T=Foo>` significa che `X` deve implementare il tratto `Bar` e in tale implementazione di bar `Bar`, `X` deve scegliere `Foo` come il tipo associato di `Bar`, `T`. 
Gli esempi in cui una limitazione di tal genere non é esprimibile con un costrutto `where` includono i tipi tratto come `Box<Bar<T=Foo>>`.

I tipi associati esistono perché spesso i generici riguardano famiglie di tipi, dove un tipo determina tutti gli altri membri. Per esempio, un tratto per grafi potrebbe avere per `Self` il grafo stesso e avere dei tipi correlati per i suoi nodi e vertici. Ciascun tipo grafo identifica univocamente i tipi associato, rendendo molto piú conciso lavorare con questi tipi di strutture e fornendo anche una migliore gestione sulla deduzione dei tipi in molti casi.

<h3><a href="#how-do-i-overload-operators" name="how-do-i-overload-operators">
Posso sovrascrivere gli operatore? Se sí, quali? Come faccio?
</a></h3>

Puoi personalizzare l'implementazione di una varietá di operatori utilizzando i loro tratti associati: [`Add`][Add] per il  `+`, [`Mul`][Mul] per il `*` e via dicendo. Si puó fare cosí:

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

Ci sono alcuni tipi in Rust i cui valori sono solo parzialmente ordinati oppure hanno relazioni di equivalenza parziali. Ordinamento parziale significa che potrebbero esserci valori di quel tipo che non sono né piú piccoli né piú grandi di un altro. Uguaglianza parziale significa che ci potrebbero essere dei valori di un certo tipo che non sono uguali a loro stessi.

I tipi a virgola mobile ([`f32`][f32] e [`f64`][f64]) sono un buon esempio di questo. Ogni tipo in virgola mobile potrebbe avere il valore `NaN` (ovvero "non un numero"). `NaN` non é uguale a sé stesso (`NaN == NaN` é falso) e nemmeno piú grande o piú piccolo di un qualsiasi valore. 
Di conseguenza sia [`f32`][f32] che [`f64`][f64] implementano [`PartialOrd`][PartialOrd] e [`PartialEq`][PartialEq] ma non [`Ord`][Ord] e nemmeno [`Eq`][Eq].

Come spiegato nella [precedente domanda sui numeri in virgola mobile](#why-cant-i-compare-floats), queste distinzioni sono importanti perché alcune collezioni fanno affidamento sul totale ordinamento/uguaglianza per funzionare.

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

Il modo piú semplice é utilizzare [`Args`][Args], che fornisce un iteratore sui parametri da riga di comando.

Se stai cercando qualcosa di piú potente, ci sono [una serie di librerie disponbili su crates.io](https://crates.io/keywords/argument).

<h2 id="error-handling">Gestione degli errori</h2>

<h3><a href="#why-doesnt-rust-have-exceptions" name="why-doesnt-rust-have-exceptions">
Perché Rust non ha le eccezioni?
</a></h3>

Le eccezioni complicano la comprensione del flusso del programma, esprimono validitá o invaliditá all'infuori del sistema dei tipi e funzionano male in ambienti multicore(un obiettivo primario per Rust).

Rust preferisce un approccio alla gestione degli errori basato sui tipi come [spiegato nel dettaglio nel libro](https://doc.rust-lang.org/stable/book/error-handling.html). 
Questo é piú compatibile con il flusso di controllo tipico di Rust, la concorrenza e tutto il resto.

<h3><a href="#whats-the-deal-with-unwrap" name="whats-the-deal-with-unwrap">
Perché c'é <code>unwrap()</code> ovunque?
</a></h3>

`unwrap()` é una funzione che estrae il valore da una [`Option`][Option] o un [`Result`][Result] e va in errore se il valore non é presente.

`unwrap()` non dovrebbe essere il tuo modo principale per gestire gli errori prevedibili, tipo un errore nell'input dell'utente.
Nel tuo codice, dovrebbe essere trattato come test per la non nullitá di un valore, pena il mandare in errore il programma.

Viene utilizzato anche per provare velocemente quando non si vuole ancora gestire tutti i casi o negli articoli, quando la gestione degli errori potrebbe distrarre dal resto.

<h3><a href="#why-do-i-get-errors-with-try" name="why-do-i-get-errors-with-try">
Perché ottengo un errore quando provo a eseguire codice di esempio che utilizza la macro <code>try!</code>?
</a></h3>

Quasi sicuramente é un problema con il tipo ritornato dalla funzione. La macro [`try!`][TryMacro] estrae un valore da [`Result`][Result] o ritorna con l'errore portato in [`Result`][Result]. 
Ció significa che [`try`][TryMacro] vale solo per le funzioni che ritornano un [`Result`][Result], dove il tipo costruito `Err` implementa `From::from(err)`. 
In particolare ció significa che la macro [`try!`][TryMacro] non é utlizzabile nella funzione `main`.

<h3><a href="#error-handling-without-result" name="error-handling-without-result">
Is there an easier way to do error handling than having <code>Result</code>s everywhere?
</a></h3>

If you're looking for a way to avoid handling [`Result`s][Result] in other people's code, there's always [`unwrap()`][unwrap], but it's probably not what you want. [`Result`][Result] is an indicator that some computation may or may not complete successfully. Requiring you to handle these failures explicitly is one of the ways that Rust encourages robustness. Rust provides tools like the [`try!` macro][TryMacro] to make handling failures ergonomic.

If you really don't want to handle an error, use [`unwrap()`][unwrap], but know that doing so means that the code panics on failure, which usually results in a shutting down the process.

<h2 id="concurrency">Concurrency</h2>

<h3><a href="#can-i-use-static-values-across-threads-without-an-unsafe-block" name="can-i-use-static-values-across-threads-without-an-unsafe-block">
Can I use static values across threads without an <code>unsafe</code> block?
</a></h3>

Mutation is safe if it's synchronized. Mutating a static [`Mutex`][Mutex] (lazily initialized via the [lazy-static](https://crates.io/crates/lazy_static/) crate) does not require an `unsafe` block, nor does mutating a static [`AtomicUsize`][AtomicUsize] (which can be initialized without lazy_static).

More generally, if a type implements [`Sync`][Sync] and does not implement [`Drop`][Drop], it [can be used in a `static`](https://doc.rust-lang.org/book/const-and-static.html#static).

<h2 id="macros">Macros</h2>

<h3><a href="#can-i-write-a-macro-to-generate-identifiers" name="can-i-write-a-macro-to-generate-identifiers">
Can I write a macro to generate identifiers?
</a></h3>

Not currently. Rust macros are ["hygienic macros"](https://en.wikipedia.org/wiki/Hygienic_macro), which intentionally avoid capturing or creating identifiers that may cause unexpected collisions with other identifiers. Their capabilities are significantly different than the style of macros commonly associated with the C preprocessor. Macro invocations can only appear in places where they are explicitly supported: items, method declarations, statements, expressions, and patterns. Here, "method declarations" means a blank space where a method can be put. They can't be used to complete a partial method declaration. By the same logic, they can't be used to complete a partial variable declaration.

<h2 id="debugging">Debugging and Tooling</h2>

<h3><a href="#how-do-i-debug-rust-programs" name="how-do-i-debug-rust-programs">
How do I debug Rust programs?
</a></h3>

Rust programs can be debugged using [gdb](https://sourceware.org/gdb/current/onlinedocs/gdb/) or [lldb](http://lldb.llvm.org/tutorial.html), the same as C and C++. In fact, every Rust installation comes with one or both of rust-gdb and rust-lldb (depending on platform support). These are wrappers over gdb and lldb with Rust pretty-printing enabled.

<h3><a href="#how-do-i-locate-a-panic" name="how-do-i-locate-a-panic">
<code>rustc</code> said a panic occurred in standard library code. How do I locate the mistake in my code?
</a></h3>

This error is usually caused by [`unwrap()`ing][unwrap] a `None` or `Err` in client code. Enabling backtraces by setting the environment variable `RUST_BACKTRACE=1` helps with getting more information. Compiling in debug mode (the default for `cargo build`) is also helpful. Using a debugger like the provided `rust-gdb` or `rust-lldb` is also helpful.

<h3><a href="#what-ide-should-i-use" name="what-ide-should-i-use">
What IDE should I use?
</a></h3>

There are a number of options for development environment with Rust, all of which are detailed on the official [IDE support page](https://forge.rust-lang.org/ides.html).

<h3><a href="#wheres-rustfmt" name="wheres-rustfmt">
<code>gofmt</code> is great. Where's <code>rustfmt</code>?
</a></h3>

`rustfmt` is [right here](https://github.com/rust-lang-nursery/rustfmt), and is being actively developed to make reading Rust code as easy and predictable as possible.

<h2 id="low-level">Low-Level</h2>

<h3><a href="#how-do-i-memcpy-bytes" name="how-do-i-memcpy-bytes">
How do I <code>memcpy</code> bytes?
</a></h3>

If you want to clone an existing slice safely, you can use [`clone_from_slice`][clone_from_slice].

To copy potentially overlapping bytes, use [`copy`][copy]. To copy nonoverlapping bytes, use [`copy_nonoverlapping`][copy_nonoverlapping]. Both of these functions are `unsafe`, as both can be used to subvert the language's safety guarantees. Take care when using them.

<h3><a href="#does-rust-work-without-the-standard-library" name="does-rust-work-without-the-standard-library">
Can Rust function reasonably without the standard library?
</a></h3>

Absolutely. Rust programs can be set to not load the standard library using the `#![no_std]` attribute. With this attribute set, you can continue to use the Rust core library, which is nothing but the platform-agnostic primitives. As such, it doesn't include IO, concurrency, heap allocation, etc.

<h3><a href="#can-i-write-an-operating-system-in-rust" name="can-i-write-an-operating-system-in-rust">
Can I write an operating system in Rust?
</a></h3>

Yes! In fact there are [several projects underway doing just that](http://wiki.osdev.org/Rust).

<h3><a href="#how-can-i-write-endian-independent-values" name="how-can-i-write-endian-independent-values">
How can I read or write numeric types like <code>i32</code> or <code>f64</code> in big-endian or little-endian format in a file or other byte stream?
</a></h3>

You should check out the [byteorder crate](http://burntsushi.net/rustdoc/byteorder/), which provides utilities for exactly that.

<h3><a href="#does-rust-guarantee-data-layout" name="does-rust-guarantee-data-layout">
Does Rust guarantee a specific data layout?
</a></h3>

Not by default. In the general case, `enum` and `struct` layouts are undefined. This allows the compiler to potentially do optimizations like re-using padding for the discriminant, compacting variants of nested `enum`s, reordering fields to remove padding, etc. `enums` which carry no data ("C-like") are eligible to have a defined representation. Such `enums` are easily distinguished in that they are simply a list of names that carry no data:

```rust
enum CLike {
    A,
    B = 32,
    C = 34,
    D
}
```

The `#[repr(C)]` attribute can be applied to such `enums` to give them the same representation they would have in equivalent C code. This allows using Rust `enum`s in FFI code where C `enum`s are also used, for most use cases. The attribute can also be applied to `struct`s to get the same layout as a C `struct` would.

<h2 id="cross-platform">Cross-Platform</h2>

<!--
### How do I build a Windows binary that doesn't display the console window?

TODO: Write this answer.
-->

<!--
### How do I make the console-less binary not crash on panic!?

TODO: Write this answer.
-->

<h3><a href="#how-do-i-express-platform-specific-behavior" name="how-do-i-express-platform-specific-behavior">
What's the idiomatic way to express platform-specific behavior in Rust?
</a></h3>

Platform-specific behavior can be expressed using [conditional compilation attributes](https://doc.rust-lang.org/reference.html#conditional-compilation) such as `target_os`, `target_family`, `target_endian`, etc.

<h3><a href="#can-rust-be-used-for-android-ios-programs" name="can-rust-be-used-for-android-ios-programs">
Can Rust be used for Android/iOS programming?
</a></h3>

Yes it can! There are already examples of using Rust for both [Android](https://github.com/tomaka/android-rs-glue) and [iOS](https://www.bignerdranch.com/blog/building-an-ios-app-in-rust-part-1/). It does require a bit of work to set up, but Rust functions fine on both platforms.

<h3><a href="#can-i-run-my-rust-program-in-a-web-browser" name="can-i-run-my-rust-program-in-a-web-browser">
Can I run my Rust program in a web browser?
</a></h3>

Not yet, but there are efforts underway to make Rust compile to the web with [Emscripten](https://kripken.github.io/emscripten-site/).

<h3><a href="#how-do-i-cross-compile-rust" name="how-do-i-cross-compile-rust">
How do I cross-compile in Rust?
</a></h3>

Cross compilation is possible in Rust, but it requires [a bit of work](https://github.com/japaric/rust-cross/blob/master/README.md) to set up. Every Rust compiler is a cross-compiler, but libraries need to be cross-compiled for the target platform.

Rust does distribute [copies of the standard library](https://static.rust-lang.org/dist/) for each of the supported platforms, which are contained in the `rust-std-*` files for each of the build directories found on the distribution page, but there are not yet automated ways to install them.

<h2 id="modules-and-crates">Modules and Crates</h2>

<h3><a href="#what-is-the-relationship-between-a-module-and-a-crate" name="what-is-the-relationship-between-a-module-and-a-crate">
What is the relationship between a module and a crate?
</a></h3>

- A crate is a compilation unit, which is the smallest amount of code that the Rust compiler can operate on.
- A module is a (possibly nested) unit of code organization inside a crate.
- A crate contains an implicit, un-named top-level module.
- Recursive definitions can span modules, but not crates.

<h3><a href="#why-cant-the-rust-compiler-find-a-library-im-using" name="why-cant-the-rust-compiler-find-a-library-im-using">
Why can't the Rust compiler find this library I'm <code>use</code>ing?
</a></h3>

There are a number of possible answers, but a common mistake is not realizing that `use` declarations are relative to the crate root. Try rewriting your declarations to use the paths they would use if defined in the root file of your project and see if that fixes the problem.

There are also `self` and `super`, which disambiguate `use` paths as being relative to the current module or parent module, respectively.

For complete information on `use`ing libraries, read the Rust book's chapter ["Crates and Modules"](https://doc.rust-lang.org/stable/book/crates-and-modules.html).

<h3><a href="#why-do-i-have-to-declare-modules-with-mod" name="why-do-i-have-to-declare-modules-with-mod">
Why do I have to declare module files with <code>mod</code> at the top level of the crate, instead of just <code>use</code>ing them?
</a></h3>

There are two ways to declare modules in Rust, inline or in another file. Here is an example of each:

```rust
// In main.rs
mod hello {
    pub fn f() {
        println!("hello!");
    }
}

fn main() {
    hello::f();
}
```

```rust
// In main.rs
mod hello;

fn main() {
    hello::f();
}

// In hello.rs
pub fn f() {
    println!("hello!");
}
```

In the first example, the module is defined in the same file it's used. In the second example, the module declaration in the main file tells the compiler to look for either `hello.rs` or `hello/mod.rs`, and to load that file.

Note the difference between `mod` and `use`: `mod` declares that a module exists, whereas `use` references a module declared elsewhere, bringing its contents into scope within the current module.

<h3><a href="#how-do-i-configure-cargo-to-use-a-proxy" name="how-do-i-configure-cargo-to-use-a-proxy">
How do I configure Cargo to use a proxy?
</a></h3>

As explained on the Cargo [configuration documentation](http://doc.crates.io/config.html), you can set Cargo to use a proxy by setting the "proxy" variable under `[http]` in the configuration file.

<h3><a href="#why-cant-the-compile-find-method-implementations" name="why-cant-the-compile-find-method-implementations">
Why can't the compiler find the method implementation even though I'm already <code>use</code>ing the crate?
</a></h3>

For methods defined on a trait, you have to explicitly import the trait declaration. This means it's not enough to import a module where a struct implements the trait, you must also import the trait itself.

<h3><a href="#why-cant-the-compiler-infer-use-statements" name="why-cant-the-compiler-infer-use-statements">
Why can't the compiler infer <code>use</code> declarations for me?
</a></h3>

It probably could, but you also don't want it to. While in many cases it is likely that the compiler could determine the correct module to import by simply looking for where a given identifier is defined, this may not be the case in general. Any decision rule in `rustc` for choosing between competing options would likely cause surprise and confusion in some cases, and Rust prefers to be explicit about where names are coming from.

For example, the compiler could say that in the case of competing identifier definitions the definition from the earliest imported module is chosen. So if both module `foo` and module `bar` define the identifier `baz`, but `foo` is the first registered module, the compiler would insert `use foo::baz;`.

```rust
mod foo;
mod bar;

// use foo::baz  // to be inserted by the compiler.

fn main() {
  baz();
}
```

If you know this is going to happen, perhaps it saves a small number of keystrokes, but it also greatly increases the possibility for surprising error messages when you actually meant for `baz()` to be `bar::baz()`, and it decreases the readability of the code by making the meaning of a function call dependent on module declaration. These are not tradeoffs we are willing to make.

However, in the future, an IDE could help manage declarations, which gives you the best of both worlds: machine assistance for pulling in names, but explicit declarations about where those names are coming from.

<!--
### How do I package and archive crates from [https://crates.io](https://crates.io)?

TODO: Write this answer.
-->

<h3><a href="#how-do-i-do-dynamic-rust-library-loading" name="how-do-i-do-dynamic-rust-library-loading">
How do I do dynamic Rust library loading?
</a></h3>

Import dynamic libraries in Rust with [libloading](https://crates.io/crates/libloading), which provides a cross-platform system for dynamic linking.

<h3><a href="#why-doesnt-crates-io-have-namespaces" name="why-doesnt-crates-io-have-namespaces">
Why doesn't crates.io have namespaces?
</a></h3>

Quoting the [official explanation](https://internals.rust-lang.org/t/crates-io-package-policies/1041) of [https://crates.io](https://crates.io)'s design:

> In the first month with crates.io, a number of people have asked us about the possibility of introducing [namespaced packages](https://github.com/rust-lang/crates.io/issues/58).<br><br>
>
> While namespaced packages allow multiple authors to use a single, generic name, they add complexity to how packages are referenced in Rust code and in human communication about packages. At first glance, they allow multiple authors to claim names like `http`, but that simply means that people will need to refer to those packages as `wycats' http` or `reem's http`, offering little benefit over package names like `wycats-http` or `reem-http`.<br><br>
>
> When we looked at package ecosystems without namespacing, we found that people tended to go with more creative names (like `nokogiri` instead of "tenderlove's libxml2"). These creative names tend to be short and memorable, in part because of the lack of any hierarchy. They make it easier to communicate concisely and unambiguously about packages. They create exciting brands. And we've seen the success of several 10,000+ package ecosystems like NPM and RubyGems whose communities are prospering within a single namespace.<br><br>
>
> In short, we don't think the Cargo ecosystem would be better off if Piston chose a name like `bvssvni/game-engine` (allowing other users to choose `wycats/game-engine`) instead of simply `piston`.<br><br>
>
> Because namespaces are strictly more complicated in a number of ways, and because they can be added compatibly in the future should they become necessary, we're going to stick with a single shared namespace.

<h2 id="libraries">Libraries</h2>

<h3><a href="#how-can-i-make-an-http-request" name="how-can-i-make-an-http-request">
Come faccio a fare una richiesta HTTP?
</a></h3>

La libreria standard non contiene un'implementazione di HTTP quindi dovrai utilizzare un pacchetto esterno.
[Hyper](https://github.com/hyperium/hyper) é la piú popolare ma ce ne sono [tante altre](https://crates.io/keywords/http).

<h3><a href="#how-can-i-write-a-gui-application" name="how-can-i-write-a-gui-application">
Come faccio a scrivere un applicativo con interfaccia grafica in Rust?
</a></h3>

Ci sono molti modi per fare applicazioni con interfaccia grafica in Rust. 
Guarda questa lista di [librerie per realizzare interfacce grafiche](https://github.com/kud1ing/awesome-rust#gui).

<h3><a href="#how-can-i-parse-json-xml" name="how-can-i-parse-json-xml">
Come faccio a deserializzare JSON/XML?
</a></h3>

[Serde](https://github.com/serde-rs/serde) é la libreria consigliata per serializzare e deserializzare di dati in Rust da e verso una moltitudine di formati.

<h3><a href="#is-there-a-standard-2d-vector-crate" name="is-there-a-standard-2d-vector-crate">
Esiste una libreria standard per la geometria e vettoriali 2D+?
</a></h3>

Non ancora! Puoi farne una tu?

<h3><a href="#how-do-i-write-an-opengl-app" name="how-do-i-write-an-opengl-app">
Come faccio a creare un applicativo OpenGL in Rust?
</a></h3>

[Glium](https://github.com/tomaka/glium) é la principale libreria per utilizzare OpenGL in Rust. [GLFW](https://github.com/bjz/glfw-rs) é un'altra opzione valida.

<h3><a href="#can-i-write-a-video-game-in-rust" name="can-i-write-a-video-game-in-rust">
Posso fare un videogioco in Rust?
</a></h3>

Certo! La principale libreria per programmare giochi in Rust é [Piston](http://www.piston.rs/) ci  sono sia un [subreddit per la creazione di videogiochi in Rust](https://www.reddit.com/r/rust_gamedev/) e un canale IRC (`#rust-gamedev` su [Mozilla IRC](https://wiki.mozilla.org/IRC)).

<h2 id="design-patterns">Paradigmi di programmazione</h2>

<h3><a href="#is-rust-object-oriented" name="is-rust-object-oriented">
Rust é orientato agli oggetti?
</a></h3>

Rust é multi paradigma. Molte cose possibili in linguaggi orientati agli oggetti sono possibili in Rust ma non proprio tutto e non sempre utilizzando un livello di astrazione uguale a quello a cui si é abituati.

<h3><a href="#how-do-i-map-object-oriented-concepts-to-rust" name="how-do-i-map-object-oriented-concepts-to-rust">
How do I map object-oriented concepts to Rust?
</a></h3>

That depends. There _are_ ways of translating object-oriented concepts like [multiple inheritance](https://www.reddit.com/r/rust/comments/2sryuw/ideaquestion_about_multiple_inheritence/) to Rust, but as Rust is not object-oriented the result of the translation may look substantially different from its appearance in an OO language.

<h3><a href="#how-do-i-configure-a-struct-with-optional-parameters" name="how-do-i-configure-a-struct-with-optional-parameters">
How do I handle configuration of a struct with optional parameters?
</a></h3>

The easiest way is to use the [`Option`][Option] type in whatever function you're using to construct instances of the struct (usually `new()`). Another way is to use the [builder pattern](https://aturon.github.io/ownership/builders.html), where only certain functions instantiating member variables must be called before the construction of the built type.

<h3><a href="#how-do-i-do-global-variables" name="how-do-i-do-global-variables">
How do I do global variables in Rust?
</a></h3>

Globals in Rust can be done using `const` declarations for compile-time computed global constants, while `static` can be used for mutable globals. Note that modifying a `static mut` variable requires the use of `unsafe`, as it allows for data races, one of the things guaranteed not to happen in safe Rust. One important distinction between `const` and `static` values is that you can take references to `static` values, but not references to `const` values, which don't have a specified memory location. For more information on `const` vs. `static`, read [the Rust book](https://doc.rust-lang.org/book/const-and-static.html).

<h3><a href="#how-can-i-set-compile-time-constants-that-are-defined-procedurally" name="how-can-i-set-compile-time-constants-that-are-defined-procedurally">
How can I set compile-time constants that are defined procedurally?
</a></h3>

Rust currently has limited support for compile time constants. You can define primitives using `const` declarations (similar to `static`, but immutable and without a specified location in memory) as well as define `const` functions and inherent methods.

To define procedural constants that can't be defined via these mechanisms, use the [`lazy-static`](https://github.com/rust-lang-nursery/lazy-static.rs) crate, which emulates compile-time evaluation by automatically evaluating the constant at first use.

<h3><a href="#can-i-run-code-before-main" name="can-i-run-code-before-main">
Can I run initialization code that happens before main?
</a></h3>

Rust has no concept of "life before `main`". The closest you'll see can be done through the [`lazy-static`](https://github.com/Kimundi/lazy-static.rs) crate, which simulates a "before main" by lazily initializing static variables at their first usage.

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
Does Rust allow non-constant-expression values for globals?
</a></h3>

No. Globals cannot have a non-constant-expression constructor and cannot have a destructor at all. Static constructors are undesirable because portably ensuring a static initialization order is difficult. Life before main is often considered a misfeature, so Rust does not allow it.

See the [C++ FQA](http://yosefk.com/c++fqa/ctors.html#fqa-10.12) about the "static initialization order fiasco", and [Eric Lippert's blog](https://ericlippert.com/2013/02/06/static-constructors-part-one/) for the challenges in C#, which also has this feature.

You can approximate non-constant-expression globals with the [lazy-static](https://crates.io/crates/lazy_static/) crate.

<h2 id="other-languages">Other Languages</h2>

<h3><a href="#how-can-i-use-static-fields" name="how-can-i-use-static-fields">
How can I implement something like C's <code>struct X { static int X; };</code> in Rust?
</a></h3>

Rust does not have `static` fields as shown in the code snippet above. Instead, you can declare a `static` variable in a given module, which is kept private to that module.

<h3><a href="#how-can-i-convert-a-c-style-enum-to-an-integer" name="how-can-i-convert-a-c-style-enum-to-an-integer">
How can I convert a C-style enum to an integer, or vice-versa?
</a></h3>

Converting a C-style enum to an integer can be done with an `as` expression, like `e as i64` (where `e` is some enum).

Converting in the other direction can be done with a `match` statement, which maps different numeric values to different potential values for the enum.

<h3><a href="#why-do-rust-programs-use-more-memory-than-c" name="why-do-rust-programs-use-more-memory-than-c">
Why do Rust programs use more memory than C?
</a></h3>

There are several factors that contribute to Rust programs having, by default, larger binary sizes than functionally-equivalent C programs. In general, Rust's preference is to optimize for the performance of real-world programs, not the size of small programs.

__Monomorphization__

Rust monomorphizes generics, meaning that a new version of a generic function or type is generated for each concrete type it's used with in the program. This is similar to how templates work in C++. For example, in the following program:

```rust
fn foo<T>(t: T) {
    // ... do something
}

fn main() {
    foo(10);       // i32
    foo("hello");  // &str
}
```

Two distinct versions of `foo` will be in the final binary, one specialized to an `i32` input, one specialized to a `&str` input. This enables efficient static dispatch of the generic function, but at the cost of a larger binary.

__Debug symbols__

Rust programs compile with some debug symbols retained, even when compiling in release mode. These are used for providing backtraces on panics, and can be removed with `strip`, or another debug symbol removal tool. It is also useful to note that compiling in release mode with Cargo is equivalent to setting optimization level 3 with rustc. An alternative optimization level (called `s` or `z`) [has recently landed](https://github.com/rust-lang/rust/pull/32386) and tells the compiler to optimize for size rather than performance.

__Jemalloc__

Rust uses jemalloc as the default allocator, which adds some size to compiled Rust binaries. Jemalloc is chosen because it is a consistent, quality allocator that has preferable performance characteristics compared to a number of common system-provided allocators. There is work being done to [make it easier to use custom allocators](https://github.com/rust-lang/rust/issues/32838), but that work is not yet finished.

__Link-time optimization__

Rust does not do link-time optimization by default, but can be instructed to do so. This increases the amount of optimization that the Rust compiler can potentially do, and can have a small effect on binary size. This effect is likely larger in combination with the previously mentioned size optimizing mode.

__Standard library__

The Rust standard library includes libbacktrace and libunwind, which may be undesirable in some programs. Using `#![no_std]` can thus result in smaller binaries, but will also usually result in substantial changes to the sort of Rust code you're writing. Note that using Rust without the standard library is often functionally closer to the equivalent C code.

As an example, the following C program reads in a name and says "hello" to the person with that name.

```c
#include <stdio.h>

int main(void) {
    printf("What's your name?\n");
    char input[100] = {0};
    scanf("%s", input);
    printf("Hello %s!\n", input);
    return 0;
}
```

Rewriting this in Rust, you may get something like the following:

```rust
use std::io;

fn main() {
    println!("What's your name?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("Hello {}!", input);
}
```

This program, when compiled and compared against the C program, will have a larger binary and use more memory. But this program is not exactly equivalent to the above C code. The equivalent Rust code would instead look something like this:

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
        printf(b"What's your name?\n\0".as_ptr());
        let mut input = [0u8; 100];
        scanf(b"%s\0".as_ptr(), &mut input);
        printf(b"Hello %s!\n\0".as_ptr(), &input);
        0
    }
}

#[lang="eh_personality"] extern fn eh_personality() {}
#[lang="panic_fmt"] fn panic_fmt() -> ! { loop {} }
#[lang="stack_exhausted"] extern fn stack_exhausted() {}
```

Which should indeed roughly match C in memory usage, at the expense of more programmer complexity, and a lack of static guarantees usually provided by Rust (avoided here with the use of `unsafe`).

<h3><a href="#why-no-stable-abi" name="why-no-stable-abi">
Why does Rust not have a stable ABI like C does, and why do I have to annotate things with extern?
</a></h3>

Committing to an ABI is a big decision that can limit potentially advantageous language changes in the future. Given that Rust only hit 1.0 in May of 2015, it is still too early to make a commitment as big as a stable ABI. This does not mean that one won't happen in the future, though. (Though C++ has managed to go for many years without specifying a stable ABI.)

The `extern` keyword allows Rust to use specific ABI's, such as the well-defined C ABI, for interop with other languages.

<h3><a href="#can-rust-code-call-c-code" name="can-rust-code-call-c-code">
Can Rust code call C code?
</a></h3>

Yes. Calling C code from Rust is designed to be as efficient as calling C code from C++.

<h3><a href="#can-c-code-call-rust-code" name="can-c-code-call-rust-code">
Can C code call Rust code?
</a></h3>

Yes. The Rust code has to be exposed via an `extern` declaration, which makes it C-ABI compatible. Such a function can be passed to C code as a function pointer or, if given the `#[no_mangle]` attribute to disable symbol mangling, can be called directly from C code.

<h3><a href="#why-rust-vs-cxx" name="why-rust-vs-cxx">
I already write perfect C++. What does Rust give me?
</a></h3>

Modern C++ includes many features that make writing safe and correct code less error-prone, but it's not perfect, and it's still easy to introduce unsafety. This is something the C++ core developers are working to overcome, but C++ is limited by a long history that predates a lot of the ideas they are now trying to implement.

Rust was designed from day one to be a safe systems programming language, which means it's not limited by historic design decisions that make getting safety right in C++ so complicated. In C++, safety is achieved by careful personal discipline, and is very easy to get wrong. In Rust, safety is the default. It gives you the ability to work in a team that includes people less perfect than you are, without having to spend your time double-checking their code for safety bugs.

<h3><a href="#how-to-get-cxx-style-template-specialization" name="how-to-get-cxx-style-template-specialization">
How do I do the equivalent of C++ template specialization in Rust?
</a></h3>

Rust doesn't currently have an exact equivalent to template specialization, but it is [being worked on](https://github.com/rust-lang/rfcs/pull/1210) and will hopefully be added soon. However, similar effects can be achieved via [associated types](https://doc.rust-lang.org/stable/book/associated-types.html).

<h3><a href="#how-does-ownership-relate-to-cxx-move-semantics" name="how-does-ownership-relate-to-cxx-move-semantics">
How does Rust's ownership system relate to move semantics in C++?
</a></h3>

The underlying concepts are similar, but the two systems work very
differently in practice. In both systems, "moving" a value is a way to
transfer ownership of its underlying resources. For example, moving a
string would transfer the string's buffer rather than copying it.

In Rust, ownership transfer is the default behavior. For example, if I
write a function that takes a `String` as argument, this function will
take ownership of the `String` value supplied by its caller:

```rust
fn process(s: String) { }

fn caller() {
    let s = String::from("Hello, world!");
    process(s); // Transfers ownership of `s` to `process`
    process(s); // Error! ownership already transferred.
}
```

As you can see in the snippet above, in the function `caller`, the
first call to `process` transfers ownership of the variable `s`. The
compiler tracks ownership, so the second call to `process` results in
an error, because it is illegal to give away ownership of the same
value twice. Rust will also prevent you from moving a value if there
is an outstanding reference into that value.

C++ takes a different approach. In C++, the default is to copy a value
(to invoke the copy constructor, more specifically). However, callees
can declare their arguments using an "rvalue reference", like
`string&&`, to indicate that they will take ownership of some of the
resources owned by that argument (in this case, the string's internal
buffer). The caller then must either pass a temporary expression or
make an explicit move using `std::move`. The rough equivalent to the
function `process` above, then, would be:

```
void process(string&& s) { }

void caller() {
    string s("Hello, world!");
    process(std::move(s));
    process(std::move(s));
}
```

C++ compilers are not obligated to track moves. For example, the code
above compiles without a warning or error, at least using the default
settings on clang. Moreover, in C++ ownership of the string `s` itself
(if not its internal buffer) remains with `caller`, and so the
destructor for `s` will run when `caller` returns, even though it has
been moved (in Rust, in contrast, moved values are dropped only by
their new owners).

<h3><a href="#how-to-interoperate-with-cxx" name="how-to-interoperate-with-cxx">
How can I interoperate with C++ from Rust, or with Rust from C++?
</a></h3>

Rust and C++ can interoperate through C. Both Rust and C++ provide a [foreign function interface](https://doc.rust-lang.org/book/ffi.html) for C, and can use that to communicate between each other. If writing C bindings is too tedious, you can always use [rust-bindgen](https://github.com/crabtw/rust-bindgen) to help automatically generate workable C bindings.

<h3><a href="#does-rust-have-cxx-style-constructors" name="does-rust-have-cxx-style-constructors">
Does Rust have C++-style constructors?
</a></h3>

No. Functions serve the same purpose as constructors without adding language complexity. The usual name for the constructor-equivalent function in Rust is `new()`, although this is just a convention rather than a language rule. The `new()` function in fact is just like any other function. An example of it looks like so:

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
Does Rust have copy constructors?
</a></h3>

Not exactly. Types which implement `Copy` will do a standard C-like "shallow copy" with no extra work (similar to "plain old data" in C++). It is impossible to implement `Copy` types that require custom copy behavior. Instead, in Rust "copy constructors" are created by implementing the `Clone` trait, and explicitly calling the `clone` method. Making user-defined copy operators explicit surfaces the underlying complexity, making it easier for the developer to identify potentially expensive operations.

<h3><a href="#does-rust-have-move-constructors" name="does-rust-have-move-constructors">
Does Rust have move constructors?
</a></h3>

No. Values of all types are moved via `memcpy`. This makes writing generic unsafe code much simpler since assignment, passing and returning are known to never have a side effect like unwinding.

<h3><a href="#compare-go-and-rust" name="compare-go-and-rust">
How are Go and Rust similar, and how are they different?
</a></h3>

Rust and Go have substantially different design goals. The following differences are not the only ones (which are too numerous to list), but are a few of the more important ones:

- Rust is lower level than Go. For example, Rust does not require a garbage collector, whereas Go does. In general, Rust affords a level of control that is comparable to C or C++.
- Rust's focus is on ensuring safety and efficiency while also providing high-level affordances, while Go's is on being a small, simple language which compiles quickly and can work nicely with a variety of tools.
- Rust has strong support for generics, which Go does not.
- Rust has strong influences from the world of functional programming, including a type system which draws from Haskell's typeclasses. Go has a simpler type system, using interfaces for basic generic programming.

<h3><a href="#how-do-rust-traits-compare-to-haskell-typeclasses" name="how-do-rust-traits-compare-to-haskell-typeclasses">
How do Rust traits compare to Haskell typeclasses?
</a></h3>

Rust traits are similar to Haskell typeclasses, but are currently not as powerful, as Rust cannot express higher-kinded types. Rust's associated types are equivalent to Haskell type families.

Some specific difference between Haskell typeclasses and Rust traits include:

- Rust traits have an implicit first parameter called `Self`. `trait Bar` in Rust corresponds to `class Bar self` in Haskell, and `trait Bar<Foo>` in Rust corresponds to `class Bar foo self` in Haskell.
- "Supertraits" or "superclass constraints" in Rust are written `trait Sub: Super`, compared to `class Super self => Sub self` in Haskell.
- Rust forbids orphan instances, resulting in different coherence rules in Rust compared to Haskell.
- Rust's `impl` resolution considers the relevant `where` clauses and trait bounds when deciding whether two `impl`s overlap, or choosing between potential `impl`s. Haskell only considers the constraints in the `instance` declaration, disregarding any constraints provided elsewhere.
- A subset of Rust's traits (the ["object safe"](https://github.com/rust-lang/rfcs/blob/master/text/0255-object-safety.md) ones) can be used for dynamic dispatch via trait objects. The same feature is available in Haskell via GHC's `ExistentialQuantification`.

<h2 id="documentation">Documentazione</h2>

<h3><a href="#why-are-so-many-rust-answers-on-stackoverflow-wrong" name="why-are-so-many-rust-answers-on-stackoverflow-wrong">
Perché su Stack Overflow molte delle risposte su Rust sono sbagliate?
</a></h3>

Il linguaggio Rust é pubblico da diversi anni e ha raggiunto la versione 1.0 a Maggio del 2015.
Nei periodi precedenti il linguaggio ha ricevuto delle modifiche sostanziali e molte risposte fanno riferimento a versioni vecchie del linguaggio.

Nel tempo sempre piú risposte saranno disponibili per la versione corrente, migliorando la problematica alterando il rapporto tra le risposte corrette e quelle sbagliate. 

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
