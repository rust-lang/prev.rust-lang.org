---
layout: it-IT/default
title: Altri metodi di installazione &middot; Linguaggio di programmazione Rust
---

# Altri metodi di installazione di Rust

- [Quale installatore dovrei usare?](#which)
- [Altri modi per installare `rustup`](#more-rustup)
- [Installatori indipendenti](#standalone)
- [Codice sorgente](#source)

## Quale installatore dovrei usare?
<span id="which"></span>

Rust funziona su molte piattaforme e ci sono molti modi per installare Rust.
Se vuoi installare Rust nel modo più semplice e consigliato, segui le istruzioni
della [pagina di installazione principale][installation page].

Quella pagina descrive l'installazione tramite [`rustup`], uno strumento che
gestisce compilatori di rust multipli in modo stabile per tutte le piattaforme
supportate da Rust.
Perchè quindi uno dovrebbe desiderare di _non_ utilizzarlo?

- Installazione offline. `rustup` scarica componenti dalla rete su richiesta.
  se hai bisogno di installare Rust senza una connessione a internet, `rustup`
  non è adatto.
- Preferenza per il gestore di pacchetti di sistema. Su Linux in particolare may
  anche su macOS con [Homebrew] e in Windows con [Chocolatey], gli sviluppatori
  a volte preferiscono installare Rust con il gestore di pacchetti del loro
  sistema operativo.
- Astio contro `curl | sh`. Sugli Unix, installiamo `rustup` eseguendo
  uno script scaricato da `curl`. Alcuni non si fidano di questa operazione
  e preferiscono scaricare e installare Rust in autonomia.
- Verifica firme. Anche se `rustup` effettua download tramite HTTPS,
  l'unico modo ad oggi possibile per verificare le firme digitali è 
  l'installazione manuale.
- Installazione da interfaccia grafica e integrazione con "Aggiungi/Rimuovi Programmi"
  su Windows. `rustup` funziona da riga di comando e non registra le sue installazioni
  nel menu di Windows come gli altri programmi. Se preferisci un'installazione più
  tipica su Windows sono disponibili gli installatori indipendenti `.msi`.
  In futuro `rustup` includerà un'interfaccia grafica su Windows.

Il supporto di Rust alle piattaforme è definito in [tre fasce][three tiers], che
corrispondono strettamente con il metodo di installazione disponibile: in generale,
Rust fornisce eseguibili per tutte le piattaforme di prima e seconda fascia,
rendendoli disponibili anche allo strumento `rustup`.
Alcune piattaforme di seconda fascia ad ogni modo hanno solo la libreria standard,
non il compilatore, possono quindi essere usate solo per la compilazione incrociata
da un'altra piattaforma. Queste piattaforme sono installabili con il comando
`rustup target add`.

## Altri modi per installare `rustup`
<span id="rustup"></span>

Il modo per installare `rustup` differisce tra le piattaforme:

* Sugli Unix, esegui `curl https://sh.rustup.rs -sSf | sh` nella tua riga di comando.
  Questo scarica e esegue [`rustup-init.sh`], che a sua volta
  scarica e esegue la versione di `rustup-init` più 
  adatta alla piattaforma corrente.
* Su Windows, scarica e installa [`rustup-init.exe`].

`rustup-init` può essere configurato interattivamente, tutte le operazioni
possono inoltre essere comandate da degli argomenti da riga di comando da
fornire allo script di installazione. Passando `--help` a `rustup-init`
farà mostrare gli argomenti disponibili:

```
curl https://sh.rustup.rs -sSf | sh -s -- --help
```

Se preferisci non usare lo script, puoi scaricare direttamente `rustup-init`
per una piattaforma a tua scelta:

<div class="rustup-init-table">
  {% for column in site.data.platforms.rustup %}
  <div>
    {% for target in column %}
    {% if target contains 'windows' %}
    <a href="https://static.rust-lang.org/rustup/dist/{{ target }}/rustup-init.exe">
      {{ target }}
    </a>
    {% else %}
    <a href="https://static.rust-lang.org/rustup/dist/{{ target }}/rustup-init">
      {{ target }}
    </a>
    {% endif %}
    {% endfor %}
  </div>
  {% endfor %}
</div>

## Installatori indipendenti
<span id="standalone"></span>

L'installatore indipendente di Rust contiene una versione singola di rust e
permettono l'installazione in assenza di connessione a internet.
Sono disponibili in tre forme: pacchetti compressi tar(estensione `.tar.gz`),
funzionanti in ambienti Unix, installatori Windows(`.msi`) e pacchetti applicativi
Mac (`.pkg`), questi installatori contengono `rustc`, `cargo`, `rustdoc`, la libreria
standard e la documentazione standard ma non forniscono le possibilità di compilazione
incrociata offerte da `rustup`.

Le ragioni più comuni per utilizzarli sono:

- Installazione in assenza di rete
- Preferenza per un installatore grafico, più integrato con Windows

Ciascuno di questi eseguibili sono firmati digitalmente con la [firma digitale di Rust][Rust signing key]
disponibile su [keybase.io], rilasciata dall'infrastruttura di rilascio di Rust,
tramite [GPG].
Nelle tabelle sottostanti, i file `.asc` sono i file di firma digitale.


Le versioni passate si possono trovare negli [archivi][the archives].

{% for channel in site.channels %}

### {{ channel.name | capitalize }} ({{ channel.vers }})
<span id="{{ channel.name }}"></span>

<div class="installer-table {{ channel.name }}">
  {% for column in site.data.platforms[channel.name] %}
  <div>
    {% for target in column %}
    <div>
      <span>{{ target }}</span>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-{{ target }}.tar.gz">.tar.gz</a>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-{{ target }}.tar.gz.asc">.asc</a>
    </div>
    {% if target contains 'windows' %}
    <div>
      <span>{{ target }}</span>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-{{ target }}.msi">.msi</a>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-{{ target }}.msi.asc">.asc</a>
    </div>
    {% elsif target contains 'darwin' %}
    <div>
      <span>{{ target }}</span>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-{{ target }}.pkg">.pkg</a>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-{{ target }}.pkg.asc">.asc</a>
    </div>
    {% endif %}
    {% endfor %}
  </div>
  {% endfor %}
</div>

{% endfor %}

## Codice sorgente
<span id="source"></span>

<div class="installer-table">
  <div>
    <div>
      <span>Stabile</span>
      <a href="https://static.rust-lang.org/dist/rustc-{{ site.stable }}-src.tar.gz">.tar.gz</a>
      <a href="https://static.rust-lang.org/dist/rustc-{{ site.stable }}-src.tar.gz.asc">.asc</a>
    </div>
  </div>
  <div>
    <div>
      <span>Beta</span>
      <a href="https://static.rust-lang.org/dist/rustc-beta-src.tar.gz">.tar.gz</a>
      <a href="https://static.rust-lang.org/dist/rustc-beta-src.gz.asc">.asc</a>
    </div>
  </div>
  <div>
    <div>
      <span>Nightly</span>
      <a href="https://static.rust-lang.org/dist/rustc-nightly-src.tar.gz">.tar.gz</a>
      <a href="https://static.rust-lang.org/dist/rustc-nightly-src.tar.gz.asc">.asc</a>
    </div>
  </div>
</div>

[installation page]: install.html
[`rustup`]: https://github.com/rust-lang-nursery/rustup.rs
[other-rustup]: https://github.com/rust-lang-nursery/rustup.rs#other-installation-methods
[`rustup-init.exe`]: https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe
[`rustup-init.sh`]: https://static.rust-lang.org/rustup/rustup-init.sh
[Homebrew]: http://brew.sh/
[Chocolatey]: http://chocolatey.org/
[three tiers]: https://forge.rust-lang.org/platform-support.html
[Rust signing key]: https://static.rust-lang.org/rust-key.gpg.ascii
[GPG]: https://gnupg.org/
[available on keybase.io]: https://keybase.io/rust
[the archives]: https://static.rust-lang.org/dist/index.html
