---
layout: de-DE/faq
title: Häufig gestellte Fragen &middot; Die Programmiersprache Rust
---

# Häufig gestellte Fragen

<p class="faq-intro">
Auf dieser Seite werden häufig gestellte Fragen über die Programmiersprache Rust beantwortet. Die Seite ist keine vollständige Anleitung und zum Lernen der Sprache ungeeignet. Sie versucht, Antworten auf Fragen zu geben, welche in der Rust-Community immer wieder aufgetreten sind, und verdeutlicht einige Überlegungen hinter den Design-Entscheidungen zu Rust.
</p>

<p class="faq-intro">
Wenn dir auffällt, dass hier eine wichtige oder verbreitete Frage fehlt, dann <a href="https://github.com/rust-lang/rust-www/blob/master/CONTRIBUTING.md">hilf uns, das zu ändern</a>.
</p>

<div id="toc">
    <h2>Inhaltsverzeichnis</h2><a href="#toggle-toc"></a>
    <div class="contents">
        <ol id="toc-contents">
            <li><a href="#project">Das Rust-Projekt</a></li>
            <li><a href="#performance">Performance</a></li>
            <li><a href="#syntax">Syntax</a></li>
            <li><a href="#numerics">Arithmetik</a></li>
            <li><a href="#strings">Strings</a></li>
            <li><a href="#collections">Collections</a></li>
            <li><a href="#ownership">Ownership</a></li>
            <li><a href="#lifetimes">Lifetimes</a></li>
            <li><a href="#generics">Generics</a></li>
            <li><a href="#input-output">Eingabe / Ausgabe</a></li>
            <li><a href="#error-handling">Fehlerbehandlung</a></li>
            <li><a href="#concurrency">Nebenläufigkeit</a></li>
            <li><a href="#macros">Makros</a></li>
            <li><a href="#debugging">Debugging und Tooling</a></li>
            <li><a href="#low-level">Low-Level</a></li>
            <li><a href="#cross-platform">Cross-Platform</a></li>
            <li><a href="#modules-and-crates">Module und Crates</a></li>
            <li><a href="#libraries">Bibliotheken</a></li>
            <li><a href="#design-patterns">Entwurfsmuster</a></li>
            <li><a href="#other-languages">Andere Sprachen</a></li>
            <li><a href="#documentation">Dokumentation</a></li>
        </ol>
    </div>
</div>


<h2 id="project">Das Rust-Projekt</h2>

<h3><a href="#what-is-this-projects-goal" name="what-is-this-projects-goal">
Was ist das Ziel des Rust-Projekts?
</a></h3>

Das Ziel des Rust-Projekts ist es, eine sichere, nebenläufige und praktikable Systemsprache zu implementieren.

Rust existiert, weil andere, ähnlich abstrakte und effiziente Sprachen folgende Erwartungen nicht erfüllen:

1. Es wird zu wenig auf Sicherheit geachtet.
2. Die Unterstützung für Nebenläufigkeit ist mangelhaft.
3. Für manche Sprachmerkmale gibt es keine praktische, einleuchtende Benutzung.
4. Es gibt nur eingeschränkte Kontrolle über Ressourcen.

Rust existiert als eine Alternative, welche sowohl effizienten Code als auch ein komfortables Abstraktionsniveau bietet, und dabei für jeden dieser vier Punkte Verbesserungen vorschlägt.

<h3><a href="#is-this-project-controlled-by-mozilla" name="is-this-project-controlled-by-mozilla">
Wird dieses Projekt von Mozilla kontrolliert?
</a></h3>

Nein. Im Jahr 2006 startete Graydon Hoare Rust als Teilzeitprojekt, und so blieb es für 3 Jahre. Erst als Rust 2009 die nötige Reife erreichte, um Kernkonzepte anhand einfacher Tests zu demonstrieren, beteiligte sich Mozilla. Rust wird vorrangig von Mozilla unterstützt, aber eine vielfältige, enthusiastische Community in der ganzen Welt arbeitet daran. Das [Rust-Team](https://www.rust-lang.org/team.html) besteht sowohl aus Mitarbeitern von Mozilla als auch aus unabhängigen Mitgliedern, und `rust` auf GitHub hatte bisher schon über [1,500 verschiedene Mitwirkende](https://github.com/rust-lang/rust/).

Die [Führung des Projektes](https://github.com/rust-lang/rfcs/blob/master/text/1068-rust-governance.md) besteht aus einem Kernteam, welches die Vision und die Prioritäten festlegt und das Projekt von einer globalen Perspektive aus leitet. Es gibt auch kleinere Teams, welche sich um die Entwicklung bestimmter Interessenbereiche kümmern. Das können zum Beispiel der Kern der Sprache, der Compiler, Rust-Bibliotheken, Tooling oder die Moderation der offiziellen Rust-Communities sein. Fortschritt in jedem dieser Bereiche wird über einen [RFC-Prozess](https://github.com/rust-lang/rfcs) erreicht. Änderungen, für welche kein RFC notwendig ist, werden normalerweise in einer Pull Request im [`rustc` Repository](https://github.com/rust-lang/rust) diskutiert.

<h3><a href="#what-are-some-non-goals" name="what-are-some-non-goals">
Welche Ziele werden nicht angestrebt?
</a></h3>

1. Wir verwenden keine besonders modernen Technologien. Alte, etablierte Technologien sind besser.
2. Ausdrucksstärke, Minimalismus oder Eleganz sind wünschenswerte, aber untergeordnete Ziele.
3. Wir beabsichtigen nicht, alle Eigenschaften und Möglichkeiten von C++ oder anderen Sprachen abzudecken. Rust sollte die häufigsten Fälle abdecken.
4. Wir beabsichtigen nicht, 100% statisch, 100% sicher, 100% reflektiv oder zu dogmatisch in irgendeiner anderen Hinsicht zu sein. Es gibt Kompromisse.
5. Wir zielen nicht darauf ab, dass Rust auf jeder möglichen Plattform läuft. Rust soll ohne unnötige Kompromisse auf üblichen, verbreiteten Hardware- und Softwareplattformen laufen.

<h3><a href="#how-does-mozilla-use-rust" name="how-does-mozilla-use-rust">
In welchen Projekten nutzt Mozilla Rust?
</a></h3>

Hauptsächlich wird Rust in [Servo](https://github.com/servo/servo), einer experimentellen Browser Engine, an welcher Mozilla arbeitet, verwendet. Mozilla arbeitet auch daran, weitere Rust-Komponenten [in Firefox zu integrieren](https://bugzilla.mozilla.org/show_bug.cgi?id=1135640).

<h3><a href="#what-examples-are-there-of-large-rust-projects" name="what-examples-are-there-of-large-rust-projects">
Welche großen Rust-Projekte gibt es?
</a></h3>

Die zwei im Moment größten quelloffenen Rust-Projekte sind [Servo](https://github.com/servo/servo) und der [Rust-Compiler](https://github.com/rust-lang/rust) selbst.

<h3><a href="#who-else-is-using-rust" name="who-else-is-using-rust">
Wer benutzt sonst noch Rust?
</a></h3>

[Eine wachsende Zahl von Organisationen!](friends.html)

<!--
### What projects are good examples of idiomatic Rust code?

TODO: Write this answer.
-->

<h3><a href="#how-can-i-try-rust-easily" name="how-can-i-try-rust-easily">
Wie kann ich Rust einfach ausprobieren?
</a></h3>

Der einfachste Weg, Rust auszuprobieren, ist der [Playpen](https://play.rust-lang.org/) - eine Online-Applikation, in welcher man einfach Rust-Code schreiben und Ausführen kann. Wenn du Rust auf deinem eigenen System ausprobieren willst, [installiere es](https://www.rust-lang.org/install.html) und gehe das [Guessing Game](https://doc.rust-lang.org/stable/book/guessing-game.html)-Tutorial im Buch durch.

<h3><a href="#how-do-i-get-help-with-rust-issues" name="how-do-i-get-help-with-rust-issues">
Wie bekomme ich bei Problemen mit Rust Hilfe?
</a></h3>

Es gibt viele Wege. Du kannst:

- Einen Forenpost im offiziellen Rust User Forum [users.rust-lang.org](https://nsers.rust-lang.org/) absetzen.
- Im offiziellen [Rust-IRC-Kanal](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust) (#rust on irc.mozilla.org) eine Frage stellen.
- Auf [Stack Overflow](https://stackoverflow.com/questions/tagged/rust) eine Frage stellen (denke daran, sie mit dem „rust“-Tag zu markieren!).
- Im inoffiziellen Rust-Subreddit [/r/rust](https://www.reddit.com/r/rust) posten.

<h3><a href="#why-has-rust-changed-so-much" name="why-has-rust-changed-so-much">
Warum hat sich Rust über die Zeit so stark verändert?
</a></h3>

Das ursprüngliche Ziel von Rust war es, eine sichere, aber einfach zu benutzende Systemprogrammiersprache zu erstellen.
Um dieses Ziel zu erreichen, verfolgte Rust eine Vielzahl von Ideen, von denen es manche behielt (Lifetimes, Traits) und andere wieder verwarf (das Typestate-System oder Green Threading). Außerdem wurde ein großer Teil der Rust-Standardbibliothek vor der Veröffentlichung der Version 1.0 neu geschrieben, um mithilfe der Features von Rust eine qualitativ hochwertige, konsistente, plattformübergreifende API anzubieten. Jetzt, wo Rust die Version 1.0 erreicht hat, wird die Stabilität der Sprache garantiert. Obwohl sich die Sprache weiter entwickelt wird Code, der auf aktuellen Versionen von Rust funktioniert, auch in zukünftigen Versionen des Compilers gültig sein.

<h3><a href="#how-does-rust-language-versioning-work" name="how-does-rust-language-versioning-work">
Wie funktioniert Versionierung in Rust?
</a></h3>

Die Sprachversionierung von Rust folgt [SemVer](http://semver.org/). Änderungen an stabilen APIs, welche die Abwärtskompatibilität nicht gewährleisten, sind in _minor_-Versionen nur erlaubt, wenn sie Fehler oder Sicherheitslücken im Compiler beheben, oder wenn die Änderungen weitere Annotationen für Dispatch oder Typinferenz erforderlich machen. Weitere, detailreichere Richtlinien für _minor_-Versionsänderungen sind als genehmigte RFCs sowohl für die [Sprache](https://github.com/rust-lang/rfcs/blob/master/text/1122-language-semver.md) als auch die [Standardbibliotheken](https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md) zu finden.

Es gibt drei Veröffentlichungskanäle für Rust: Stable, Beta, und Nightly. Die Kanäle Stable und Beta werden alle sechs Wochen aktualisiert, wobei der aktuelle Nightly zur neuen Beta und die aktuelle Beta das neue Stable wird. Teile der Standardbibliotheken sind als _unstable_ markiert oder durch _Feature Gates_ abgeschirmt. Diese können ausschließlich im Nightly-Kanal verwendet werden. Neue Features sind solange als _unstable_ markiert, bis das Kernteam und zuständige Unterteams ihre Zustimmung zur Freigabe gegeben haben. Diese Herangehensweise erlaubt es Entwicklern, zu experimentieren, ohne die Garantie auf Abwärtskompatibilität zu gefährden.

Mehr Details finden sich im Blogpost ["Stability as a Deliverable"](http://blog.rust-lang.org/2014/10/30/Stability.html).

<h3><a href="#can-i-use-unstable-features-on-the-beta-or-stable-channel" name="can-i-use-unstable-features-on-the-beta-or-stable-channel">
Kann ich auf dem Beta- oder Stable Channel Features aus dem Unstable Channel verwenden?
</a></h3>

Nein, das ist unmöglich. An Rust wird hart gearbeitet, um die Stabilität der Beta- und Stable Kanäle zu gewährleisten. Wir wollen nicht, dass sich jemand auf unstabile Features verlässt, für welche wir keine Stabilität gewährleisten und welche sich jederzeit ändern könnten. Das gibt uns auch die Möglichkeit, Änderungen im Nightly-Kanal realistisch auszutesten, während der Beta- und Stable Kanal stabil und unverändert bleiben.

Alle sechs Wochen werden die Beta- und Stable Kanäle mit den stabilisierten Features aktualisiert. Im Nightly-Kanal gibt es häufig stabilisierende Updates, während die anderen Kanäle seltener Fixes akzeptieren. Wenn du darauf wartest, dass ein Feature im Beta- oder Stable Kanal bereitgestellt wird, dann kannst du die zugehörige Issue mit dem Tag [`B-unstable`](https://github.com/rust-lang/rust/issues?q=is%3Aissue+is%3Aopen+tracking+label%3AB-unstable) auf dem Issue Tracker finden.

<h3><a href="#what-are-feature-gates" name="what-are-feature-gates">
Was sind Feature Gates?
</a></h3>

_Feature Gates_ sind der Sprachmechanismus, den Rust verwendet, um Features des Compilers, der Sprache und der Standardbibliotheken zu stabilisieren. Ein Feature hinter einem _Gate_ ist nur im Nightly-Kanal verfügbar, und auch nur dann, wenn es explizit durch ein `#[feature]`-Attribut oder das Kommandozeilenargument `-Z unstable-options` angefordert wurde. Wenn ein Feature stabilisiert und in den Stable-Kanal übernommen wird, muss es nicht mehr explizit angefordert werden. Dann wird dieses Feature als _ungated_ bezeichnet. Feature Gates erlauben es den Entwicklern, zu experimentieren, während sie in der Entwicklung sind. Erst wenn die Entwickler sich auf eine Implementierung festlegen, halten die Features in der stabilen Sprache Einzug.

<h3><a href="#why-a-dual-mit-asl2-license" name="why-a-dual-mit-asl2-license">
Warum eine MIT-ASL2 Doppellizenz?
</a></h3>

Die Apache-Lizenz enthält wichtigen Schutz gegen Patentaggressoren, aber ist mit der GPLv2 inkompatibel. Um Probleme bei der Verwendung von Rust mit der GPLv2-Lizenz zu vermeiden, ist es alternativ MIT-lizenziert.

<h3><a href="#why-a-permissive-license" name="why-a-permissive-license">
Warum eine BSD-artige Freizügige Lizenz anstelle von MPL oder einer dreifachen Lizenz?
</a></h3>

Das liegt zu einem Teil an einer persönlichen Vorliebe des originalen Entwicklers Graydon Hoare, zum anderen daran, dass Programmiersprachen im Gegensatz zu Produkten wie Webbrowsern normalerweise einen weiter gefächerten Einflussbereich und ein vielseitigeres Einsatzgebiet haben. Wir würden gerne möglichst viele dieser potenziellen Mitwirkenden anziehen.

<h2 id="performance">Performance</h2>

<h3><a href="#how-fast-is-rust" name="how-fast-is-rust">
Wie schnell ist Rust?
</a></h3>

Sehr schnell! Rust kann sich bereits in einigen Benchmarks (zu Beispiel dem [Benchmarks Game](https://benchmarksgame.alioth.debian.org/u64q/compare.php?lang=rust&lang2=gpp) und [anderen](https://github.com/kostya/benchmarks)) mit idiomatischem C und C++ Code messen.

Eins der wichtigsten Prinzipien in Rust (wie auch in C++) sind [Zero-Cost Abstractions](http://blog.rust-lang.org/2015/05/11/traits.html): Keine der Abstraktionen in Rust verursachen programmweite Verlangsamungen oder einen Mehraufwand zur Laufzeit.

Da Rust auf LLVM aufbaut und deshalb auch versucht, Clang-kompatiblen Code zu generieren, sind Leistungsverbesserungen in LLVM auch für Rust vorteilhaft. Langfristig sollten die detaillierten Informationen des Typsystems Optimierungen ermöglichen, welche in C/C++ unmöglich wären.

<h3><a href="#is-rust-garbage-collected" name="is-rust-garbage-collected">
Gibt es in Rust einen Garbage Collector?
</a></h3>

Nein. Eine der Schlüsselinnovationen von Rust ist es, *ohne* Garbage Collection Speichersicherheit (keine Segfaults) zu garantieren.

Rust erlangt dadurch einige Vorteile: Vorhersagbare Bereinigung von Ressourcen, niedrige Mehrkosten für Speichermanagement und ein minimales Laufzeitsystem. Diese Eigenschaften ermöglichen es, Rust in beliebige Umgebungen einzubinden, und vereinfachen die [Integration von Rust in Sprachen mit GC](http://calculist.org/blog/2015/12/23/neon-node-rust/).

Das Ownership und Borrowing System ermöglicht nicht nur Speichersicherheit ohne einen GC, sondern ist auch in anderen Zusammenhängen nützlich, zum Beispiel für [allgemeines Ressourcenmanagement](http://blog.skylight.io/rust-means-never-having-to-close-a-socket/) und [Nebenläufigkeit](http://blog.rust-lang.org/2015/04/10/Fearless-Concurrency.html).

In Fällen, in denen einfache Ownership-Semantik nicht genügt, nutzen Rust-Programme den üblichen Referenzzähler/_Smart Pointer_-Typ [`Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html) und sein Thread-sicheres Gegenstück [`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html).

Es wird jedoch daran gearbeitet, in Zukunft einen *optionalen* Garbage Collector als Erweiterung anzubieten, um eine gute Integration mit Laufzeitumgebungen wie [Spidermonkey](https://developer.mozilla.org/en-US/docs/Mozilla/Projects/SpiderMonkey)
und [V8](https://developers.google.com/v8/?hl=en) zu ermöglichen, welche Garbage Collection verwenden.

Es gibt auch experimentelle, in [purem Rust implementierte Kollektoren](https://manishearth.github.io/blog/2015/09/01/designing-a-gc-in-rust/), welche ohne Unterstützung des Compilers funktionieren.

<h3><a href="#why-is-my-program-slow" name="why-is-my-program-slow">
Warum ist mein Programm so langsam?
</a></h3>

Der Rust-Compiler optimiert Programme nur dann, wenn man das explizit anfordert, da [Optimierungen die Kompiliergeschwindigkeit verringern und allgemein während der Entwicklung unerwünscht sind](https://users.rust-lang.org/t/why-does-cargo-build-not-optimise-by-default/4150/3).

Wenn du mit `cargo` kompilierst, nutze die `--release` option. Wenn du dein Program direkt mit `rustc` erstellst, nutze die Option `-O`. Beide Optionen schalten Optimierungen ein.

<h3><a href="#why-is-rustc-slow" name="why-is-rustc-slow">
Warum ist die Erstellung meines Programmes so zeitaufwändig?
</a></h3>

Rustc übersetzt und optimiert Code. Die High-Level-Abstraktionen in Rust kompilieren zu effizientem Maschinencode, und um diese Übersetzungen durchzuführen, wird Zeit benötigt - insbesondere beim Optimieren.

Die Übersetzungszeit ist jedoch besser als sie scheint und es gibt Anlass zur Hoffnung, dass sie sich noch verbessern wird. Kompilierzeiten von ähnlich umfangreichen Rust- und C++ Projekten sind im Allgemeinen miteinander vergleichbar.

Die häufige Auffassung, dass Rust langsam kompiliert, kommt zum Großteil aus dem Unterschied zwischen dem *Kompiliermodell* von C++ und Rust: Eine Kompiliereinheit in C++ ist eine Datei, während in Rust immer ein gesamter Crate kompiliert wird, welche aus vielen Dateien bestehen kann. Eine einzelne Datei während der Entwicklung zu verändern führt in C++ normalerweise zu weniger Rekompilation als in Rust. Es wird derzeit viel Arbeit in [inkrementelle Programmerstellung](https://github.com/rust-lang/rfcs/blob/master/text/1298-incremental-compilation.md) investiert, welche die Vorteile des Modells von C++ in Rust übernimmt.

Neben dem Kompilationsmodell gibt es andere Aspekte des Sprachdesigns und der Compilerimplementation, welche die Übersetzungsleistung beeinflussen.

Rust hat zunächst ein relativ komplexes Typsystem, und der Compiler muss einige Zeit darauf verwenden, die Typbedingungen zu überprüfen, welche Rust zur Laufzeit absichern.

Außerdem sind einige Teile des Rust-Compilers ziemlich veraltet. Diese generieren insbesondere LLVM-IR niedriger Qualität, welche LLVM erst „reparieren“ muss. Es gibt Hoffnung, dass zukünftige, [MIR-basierte](https://github.com/rust-lang/rfcs/blob/master/text/1211-mir.md) Übersetzungs- und Optimierungsdurchläufe die Arbeit für LLVM leichter machen.

Drittens hat die Nutzung von LLVM auch ihre Kosten: Rust hat dadurch hohe Leistung zur Laufzeit, aber LLVM ist ein großes Framework das nicht auf hohe Leistung zur Übersetzungszeit fokussiert ist, insbesondere bei Eingaben mit mangelhafter Qualität.

Letztlich führt auch die übliche Strategie der Monomorphisierung von Generics (wie in C++) zwar zu schnellem Code zur Laufzeit, aber sie erfordert die Erzeugung von Signifikant mehr Code als andere Strategien. Durch die Verwendung von Trait-Objekten können Rust-Programmierer diese Aufblähung vermeiden, müssen dann aber auf späte Bindung mit ihren bekannten Nachteilen zurückgreifen.

<h3><a href="#why-are-rusts-hashmaps-slow" name="why-are-rusts-hashmaps-slow">
Warum ist die <code>HashMap</code> in Rust so langsam?
</a></h3>

Standardmäßig nutzt [`HashMap`][HashMap] den [SipHash](https://131002.net/siphash/)-Algorithmus, der entworfen wurde, um [Kollisionsattacken auf Hashtabellen](http://programmingisterrible.com/post/40620375793/hash-table-denial-of-service-attacks-revisited) zu verhindern und dabei trotzdem [für die häufigsten Anwendungsfälle gute Leistung](https://www.reddit.com/r/rust/comments/3hw9zf/rust_hasher_comparisons/cub4oh6) zu erzielen.

Obwohl SipHash [in vielen Fällen hohe Leistung vorweisen kann](http://cglab.ca/%7Eabeinges/blah/hash-rs/), ist der Algorithmus für Anwendungsfälle mit kurzen Schlüsseln wie Ganzzahlen merklich langsamer. Deshalb wird von Rust-Programmierern häufig niedrige Leistung bei der Verwendung einer [`HashMap`][HashMap] beobachtet. In solchen Fällen wird häufig der [FNV-Hasher](https://crates.io/crates/fnv) empfohlen, der aber nicht die Kollisionsresistenz von SipHash vorweisen kann.

<h3><a href="#why-is-there-no-integrated-benchmarking" name="why-is-there-no-integrated-benchmarking">
Warum gibt es keine integrierte Benchmarking-Infrastruktur?
</a></h3>

Es gibt eine, welche aber nur im Nightly-Kanal verfügbar ist. Wir planen ein modulares System, welches integrierte Leistungstests ermöglicht. In der Zwischenzeit wird das System als [instabil](https://github.com/rust-lang/rust/issues/29553) eingeschätzt.

<h3><a href="#does-rust-do-tail-call-optimization" name="does-rust-do-tail-call-optimization">
Unterstützt Rust Tail Call Elimination?
</a></h3>

Im Allgemeinen nicht. Optimierung von Endrekursion kann unter [bestimmten Vorraussetzungen](http://llvm.org/docs/CodeGenerator.html#sibling-call-optimization) erfolgen, das ist aber [nicht gewährleistet](https://mail.mozilla.org/pipermail/rust-dev/2013-April/003557.html). Da die Optimierung ein vielfach erwünschtes Sprachmerkmal ist, wurde das Schlüsselwort `become` dafür reserviert, wobei allerdings die technische Umsetzbarkeit noch nicht geklärt ist. Eine [vorgeschlagene Erweiterung](https://github.com/rust-lang/rfcs/pull/81), welche Tail Call Elimination ermöglichen würde, wurde vorgeschlagen, zunächst aber verschoben.

<h3><a href="#does-rust-have-a-runtime" name="does-rust-have-a-runtime">
Hat Rust ein Laufzeitsystem?
</a></h3>

Nicht im Sinne von gängigen Sprachen wie Java. Teile der Standardbibliotheken könnte man als "Laufzeitsystem" bezeichnen, die einen Heap, Backtraces, Unwinding, und Stack Guards anbietet. Eine relativ [kleine Menge Initialisierungscode](https://github.com/rust-lang/rust/blob/33916307780495fe311fe9c080b330d266f35bfb/src/libstd/rt.rs#L43) läuft vor der `main`-Funktion des Benutzers. Die Standardbibliotheken linken außerdem gegen die C-Standardbibliotheken, welche ebenfalls eine ähnliche [Laufzeitinitialisierung](http://www.embecosm.com/appnotes/ean9/html/ch05s02.html) vornehmen. Rust kann ohne die Standardbibliotheken kompiliert werden, dann ist die Laufzeitumgebung äquivalent zu der von C.

<h2 id="syntax">Syntax</h2>

<h3><a href="#why-curly-braces" name="why-curly-braces">
Warum geschwungene Klammern? Warum kann die Syntax von Rust nicht Haskell oder Python ähnlicher sein?
</a></h3>

Die Benutzung geschweifter Klammern ist eine Entwurfsentscheidung, welche eine Vielzahl von Programmiersprachen getroffen haben. Da Rust hier konsistent bleibt, ist es für in anderen Sprachen erfahrene Programmierer einfacher zu lernen.

Geschweifte Klammern ermöglichen dem Programmierer eine flexible Syntax und dem Kompilierer einen einfacheren Parser.

<h3><a href="#why-brackets-around-blocks" name="why-brackets-around-blocks">
Ich kann die Klammern um <code>if</code>-Bedingungen weglassen, warum muss ich sie dann um einzeilige Blöcke setzen? Warum ist der C-Stil nicht erlaubt?
</a></h3>

Während C Klammerung um ein `if`-Statement aber keine Klammern für einzeilige Blöcke erfordert, trifft Rust die genau entgegengesetzte Wahl. Das trennt die Bedingung klar vom Block und vermeidet die Gefahren der optionalen Klammern, welche zu leicht übersehbaren Fehlern wie Apples [goto-fail-Bug](https://gotofail.com/) führen können.

<h3><a href="#why-no-literal-syntax-for-dictionaries" name="why-no-literal-syntax-for-dictionaries">
Warum gibt es keine Syntax für Dictionary-Literale?
</a></h3>

Die bevorzugte Vorgehensweise beim Entwurf von Rust war es, die *Sprache* selbst relativ klein zu halten und dafür mächtige *Bibliotheken* anzubieten. Rust bietet zwar Literale zur Initialisierung von Arrays und Strings an, aber diese sind die einzigen in die Sprache eingebauten Collection-Typen. Andere, in Bibliotheken definierte Typen, wie zum Beispiel der häufig genutzte [`Vec`][Vec] Collection-Typ erlauben die Initialisierung durch Makros wie [`vec!`][VecMacro].

In der Zukunft wird die Design-Entscheidung, Makros zum Initialisieren von Datenstrukturen zu verwenden, wahrscheinlich auf weitere Datentypen erweitert werden. Zusätzlich zu [`HashMap`][HashMap] und [`Vec`][Vec] sollen Typen wie [`BTreeMap`][BTreeMap] unterstützt werden. Wenn du jetzt schon komfortablere Syntax zur Initialisierung von Datenstrukturen benötigst, kannst du dafür [dein eigenes Makro definieren](https://stackoverflow.com/questions/27582739/how-do-i-create-a-hashmap-literal).

<h3><a href="#when-should-i-use-an-implicit-return" name="when-should-i-use-an-implicit-return">
Wann sollte ich ein implizites Return verwenden?
</a></h3>
[comment]: <> (Ich finde, hier wird zu wenig auf den Unterschied, den ein Semikolon macht (insbesondere im Ggs. zu anderen Sprachen) eingegangen. Allgemein könnte man hier den Unterschied zwischen Ausdruck und Statement erklären. Upstream?)
Rust ist eine stark Ausdruck-orientierte Sprache, und _implizite Returns_ gehören zu diesem Design. Konstrukte wie `if`, `match`, oder normale Blöcke sind in Rust Ausdrücke. Der folgende Programmcode testet zum Beispiel, ob ein [`i64`][i64] ungerade ist und liefert das Ergebnis mit einem impliziten Return zurück.

```rust
fn is_odd(x: i64) -> bool {
    if x % 2 != 0 { true } else { false }
}
```

Das kann natürlich weiter vereinfacht werden:

```rust
fn is_odd(x: i64) -> bool {
    x % 2 != 0
}
```

In beiden Beispielen ist die letzte Zeile der Rückgabewert der Funktion. Ein wichtiges Detail ist, dass der Rückgabetyp einer Funktion, welche mit einem Semikolon endet, `()` ist. Dies deutet an, dass kein Wert zurückgegeben wird. Implizite Rückgaben funktionieren nur ohne abschließendes Semikolon, da sonst der Wert des Ausdrucks unterdrückt wird.

Explizite Rückgaben müssen dann benutzt werden, wenn implizite unmöglich sind, zum Beispiel wenn man vor dem Ende des Funktionskörpers einen Wert zurückgeben will. Beide Funktionen im obigen Beispiel hätten mit einem `return` und einem Semikolon geschrieben werden können, aber das wäre unnötig ausführlich und gegen die Konventionen von gutem Rust-Code.

<h3><a href="#why-arent-function-signatures-inferred" name="why-arent-function-signatures-inferred">
Warum werden Funktionssignaturtypen nicht vom Compiler hergeleitet?
</a></h3>

Deklarationen in Rust werden normalerweise mit expliziten Typannotationen versehen, während der eigentliche Programmcode mit inferierten Typen arbeitet. Diese Entscheidung ist folgendermaßen begründet:

- Erforderliche Signaturdeklarationen helfen, die Stabilität von Schnittstellen in Modulen und Crates zu gewährleisten.
- Signaturtypen erleichtern dem Programmierer das Verständnis des Programms. Dadurch, dass die Signaturtypen immer explizit lokal im Programm definiert sind, muss eine IDE keinen Inferenzalgorithmus über den gesamten Crate laufen lassen, um den Typ eines Argumentes herauszufinden.
- Da die Argumenttypen auf Funktionsebene festgelegt sind, kann der Inferenzalgorithmus stark vereinfacht werden.

<h3><a href="#why-does-match-have-to-be-exhaustive" name="why-does-match-have-to-be-exhaustive">
Warum muss ein <code>match</code> alle Fälle abdecken?
</a></h3>

Um Klarheit zu schaffen und Refactoring zu vereinfachen.

Erstens: wenn ein `match` jeden Fall eines `enum`s abdeckt, führt das zukünftige Hinzufügen einer Variante zu einem Kompilierfehler und nicht einem Fehler zur Laufzeit. Diese Hilfe durch den Kompilierer ermöglicht es dem Rust-Programmierer, zu Refakturisieren, ohne neue Fehler befürchten zu müssen.

Zweitens: abdeckende Prüfung aller Fälle expliziert die Semantik des default-Falles. Allgemein wäre ein nicht-abdeckendes `match` nur sicher, wenn der Thread im Fall einer unvorhergesehenen Variante ein `panic` auslösen würde. In frühen Versionen von Rust, in denen `match` nicht zwingend alle Fälle abdecken musste, wurde dies als Ursache für viele Fehler befunden.

Mit `_` kannst du einfach alle weiteren, unspezifizierten Fälle ignorieren:

```rust
match val.do_something() {
    Cat(a) => { /* ... */ }
    _      => { /* ... */ }
}
```

<h2 id="numerics">Arithmetik</h2>

<h3><a href="#which-type-of-float-should-i-use" name="which-type-of-float-should-i-use">
Soll ich für mathematische Operationen mit Gleitkommazahlen <code>f32</code> oder <code>f64</code> verwenden?
</a></h3>

Diese Wahl hängt vom Zweck des Programmes ab.

Wenn du für deine Gleitkommazahlen die größtmögliche Genauigkeit benötigst, dann nutze [`f64`][f64]. Wenn du Speicher- oder Rechenzeiteffizienz benötigst und dafür Abstriche in der Genauigkeit machen kannst (da du pro Wert weniger Bits zur Darstellung zur Verfügung hast), dann ist [`f32`][f32] besser. Auch auf 64-Bit Hardware sind Operationen mit [`f32`][f32] normalerweise schneller. Ein häufiges Beispiel findet man in der Grafikprogrammierung: hier wird typischerweise [`f32`][f32] verwendet, da hohe Leistung nötig ist und 32-Bit Gleitkommazahlen zur Darstellung von Pixeln auf dem Bildschirm ausreichen.

Wähle im Zweifel [`f64`][f64], um bessere Präzision zu erreichen.

<h3><a href="#why-cant-i-compare-floats" name="why-cant-i-compare-floats">
Warum kann ich keine Gleitkommazahlen vergleichen oder sie als Schlüsseltypen für <code>HashMap</code> oder <code>BTreeMap</code> verwenden?
</a></h3>

Gleitkommazahlen können mit den Operatoren `==`, `!=`, `<`, `<=`, `>`, `>=`, sowie mit der Funktion `partial_cmp()` verglichen werden. `==` and `!=` sind Teil des [`PartialEq`][PartialEq]-Traits, während `<`, `<=`, `>`, `>=`, und `partial_cmp()` Teil des [`PartialOrd`][PartialOrd]-Traits sind.

Gleitkommazahlen können nicht mit der Funktion `cmp()` verglichen werden, welche Teil des [`Ord`][Ord] Traits ist, da es für Gleitkommazahlen keine totale Ordnung gibt. Genauso gibt es keine totale Gleichheitsrelation, weswegen Gleitkommazahlen auch nicht den [`Eq`][Eq]-Trait implementieren. Der Grund ist, dass der Gleitkommawert [`NaN`](https://en.wikipedia.org/wiki/NaN) nicht gleich, kleiner als oder größer als irgendeine anderen Gleitkommazahl (oder sogar `NaN` selbst) ist.

Da Gleitkommazahlen weder [`Eq`][Eq] noch [`Ord`][Ord] implementieren, können sie nicht für Typen verwendet werden, deren Trait-Bounds diese Eigenschaften verlangen, wie zum Beispiel [`BTreeMap`][BTreeMap] oder [`HashMap`][HashMap]. Das ist wichtig, da diese Typen *annehmen*, dass ihre Schlüssel totale Ordnung oder totale Gleichheit anbieten - anderenfalls würden sie nicht richtig funktionieren.

[Es gibt einen Crate](https://crates.io/crates/ordered-float), welche [`f32`][f32] und [`f64`][f64] um eine Implementierung von [`Ord`][Ord] und [`Eq`][Eq] erweitert, die in manchen Fällen nützlich sein kann.

<h3><a href="#how-can-i-convert-between-numeric-types" name="how-can-i-convert-between-numeric-types">
Wie kann ich zwischen numerischen Typen umwandeln?
</a></h3>

Es gibt zwei Möglichkeiten: Das `as` Schlüsselwort, welches einfache Typumwandlung für primitive Typen vollzieht, und die Traits [`Into`][Into] und [`From`][From], welche für einige Typkonversionen implementiert sind (und welche du für eigene Typen selbst implementieren kannst). Die [`Into`][Into] und [`From`][From]-Traits sind nur in Fällen implementiert, in denen eine verlustfreie Umwandlung möglich ist. Zum Beispiel wird `f64::from(0f32)` kompilieren, `f32::from(0f64)` aber nicht. Das Schlüsselwort `as` hingegen wandelt alle primitiven Typen untereinander um und schneidet wenn nötig deren Werte ab.

<h3><a href="#why-doesnt-rust-have-increment-and-decrement-operators" name="why-doesnt-rust-have-increment-and-decrement-operators">
Warum kennt Rust keine Inkrement- und Dekrementoperatoren?
</a></h3>
[comment]: <> (Unterschied Expression/Statement?)
Präinkrement und Postinkrement sowie ihre Gegenstücke für Dekrement sind zwar komfortabel, aber auch relativ komplex. Sie erfordern Kenntnis der Ausführungsreihenfolge und können in C und C++ leicht zu subtilen Fehlern oder undefiniertem Verhalten führen. `x = x + 1` oder `x += 1` sind nur geringfügig länger und dafür eindeutig.

<h2 id="strings">Strings</h2>

<h3><a href="#how-to-convert-string-or-vec-to-slice" name="how-to-convert-string-or-vec-to-slice">
Wie kann ich einen <code>String</code> oder <code>Vec&lt;T&gt;</code> in einen Slice konvertieren (<code>&amp;str</code> und <code>&amp;[T]</code>)?
</a></h3>

Normalerweise kannst du eine Referenz zu einem `String` oder `Vec<T>` immer dort übergeben, wo ein Slice passend wäre.
Mithilfe von [Deref Coercions](https://doc.rust-lang.org/stable/book/deref-coercions.html) können [`String`s][String] und [`Vec`s][Vec] automatisch in ihren jeweiligen Slice-Typen „zerfallen“, wenn man eine Referenz darauf mit `&` oder `&mut` übergibt.

Methoden, die auf `&str` oder `&[T]` implementiert wurden, können auf `String` und `Vec<T>` direkt aufgerufen werden. Der Aufruf `some_string.char_at(0)` zum Beispiel funktioniert, obwohl `char_at` eine Methode von `&str` und `some_string` ein `String` ist.

Manchmal, zum Beispiel in generischem Code, wird manuelle Konversion notwendig. Diese kann man mit dem _Slicing-Operator_ `&my_vec[..]` erreicht werden.

<h3><a href="#how-to-convert-between-str-and-string" name="how-to-convert-between-str-and-string">
Wie kann man <code>&amp;str</code> in <code>String</code> und umgekehrt umwandeln?
</a></h3>

Die Methode [`to_string()`][to_string] wandelt einen [`&str`][str] zu einem [`String`][String] um, und [`String`s][String] werden automatisch zu [`&str`][str] umgewandelt, wenn man per _Borrowing_ eine Referenz auf sie übergibt. Folgendes Beispiel soll beide Fälle veranschaulichen:

```rust
fn main() {
    let s = "Jane Doe".to_string();
    say_hello(&s);
}

fn say_hello(name: &str) {
    println!("Hello {}!", name);
}
```

<h3><a href="#what-are-the-differences-between-str-and-string" name="what-are-the-differences-between-str-and-string">
Worin unterscheiden sich die beiden String-Typen?
</a></h3>

[`String`][String] ist ein auf dem Heap allokierter Buffer von UTF-8 Bytes, welcher durch eine 'owned'-Referenz gebunden ist. Veränderbare (mutable) [`String`s][String] können modifiziert werden, wobei ihre Kapazität angepasst wird. [`&str`][str] ist lediglich eine "Einsicht" mit fester Größe in einen [`String`][String]. Dieser [`String`][String] ist normalerweise auf dem Heap allokiert, wenn die Slice durch Dereferenzierung eines [`String`s][String] entstanden ist. Er kann auch im statischen Speicher liegen, wenn der String ein Literal aus dem Programmcode ist.

[`&str`][str] ist ein primitiver Typ der Sprache Rust, während [`String`][String] in der Standardbibliothek definiert ist.

<h3><a href="#how-do-i-do-o1-character-access-in-a-string" name="how-do-i-do-o1-character-access-in-a-string">
Wie kann ich in O(1) auf ein Zeichen in einem <code>String</code> zugreifen?
</a></h3>

Das ist unmöglich, außer wenn über den String im Vornherein so viel bekannt ist, dass der Index eines Zeichens genau berechnet werden kann.
[comment] <> (You cannot. At least not without a firm understanding of what you mean by "character", and preprocessing the string to find the index of the desired character)

In Rust sind Strings in UTF-8-kodiert. In ASCII wäre ein einzelnes Zeichen auch genau ein Byte, aber dies ist in UTF-8 nicht zwingend der Fall. Ein Byte wird _Code Unit_ genannt (in UTF-16 sind Code Units 2 Bytes lang; in UTF-32 sind es 4 Bytes). Ein _Code Point_ besteht aus einem oder mehreren Code Units, und mehrere Code Points formen _Grapheme Cluster_, welche am ehesten als Zeichen interpretiert werden können.

Obwohl man einfach auf den Bytes eines UTF-8 Strings indizieren könnte, kann man nicht in konstanter Zeit den `i`ten Code Point oder Grapheme Cluster erreichen, da sie alle unterschiedlich lang sein können. Es ist allerdings möglich, einen konkreten Grapheme Cluster oder Code Point zu erreichen, wenn man genau weiß, wo er beginnt.
Funktionen wie [`str::find()`][str__find] und Regex-Matches geben Byte-Indizes an, wodurch dieser Zugriff auf Byte-Ebene ermöglicht wird.

<h3><a href="#why-are-strings-utf-8" name="why-are-strings-utf-8">
Warum sind Strings standardmäßig UTF-8-kodiert?
</a></h3>

Der [`str`][str]-Typ ist UTF-8-kodiert, weil Text (vor allem in Endian-agnostischen Netzwerkübertragungen) sehr häufig in dieser Form vorkommt. Wir sind der Meinung, dass Eingabe/Ausgabe standardmäßig nicht das Neukodieren von Text erfordern sollte.

Das bedeutet, dass auf einen spezifischen Code Point in einem String nur durch eine O(n)-Operation zugegriffen werden kann (wobei ein Byte an bekannter Position natürlich weiterhin nur O(1) kostet). Einerseits ist das ein unerwünschter Nachteil; Andererseits ist dieses Problem voller Abwägungen und Trade-Offs. Ein paar wichtige Merkmale:

Einen [`str`][str] nach ASCII-Codepoints zu durchsuchen kann immer noch sicher Byte für Byte geschehen. Mit [`.as_bytes()`][str__as_bytes] kann man mit O(1)-Kosten einen [`u8`][u8] gewinnen, welcher zu einem ASCII-[`char`][char] umgewandelt oder mit einem ASCII-[`char`][char] verglichen werden kann. Durch das gute Design von UTF-8 kann zum Beispiel ein `'\n'`-Byte weiterhin als Zeilenumbruch interpretiert werden.

Die meisten „zeichenorientierten“ Operationen auf Text funktionieren nur bei sehr einschränkenden Annahmen, wie etwa dass der Text nur ASCII-Bytes enthält. Außerhalb des ASCII-Bereiches wird häufig sowieso ein komplexerer (nicht laufzeitkonstanter) Algorithmus zur Ermittlung der linguistischen Einheit (Glyph, Wort, Abschnitt) verwendet. Wir empfehlen, einen „ehrlichen“, linguistisch korrekten, anerkannten Unicode-Algorithmus zu verwenden.

Der [`char`][char]-Typ ist UTF-32-kodiert. Wenn du sicher bist, dass du einen Algorithmus verwenden musst, welcher jeden Codepoint einzeln betrachtet, kannst du einfach einen `type wstr = [char]` definieren. Dann kannst du in einem einen [`str`][str] in ihn entpacken, mit welchem du dann als `wstr` arbeiten kannst. In anderen Worten: Die Tatsache, das die Sprache standardmäßig nicht in UTF-32 enkodiert, soll dich nicht daran hindern, Strings in irgendeinem anderen Enkoding zu verarbeiten.

Eine detailliertere Erklärung, warum UTF-8 üblicherweise UTF-16 oder UTF-32 vorgezogen werden sollte, findet sich im [UTF-8 Everywhere Manifesto](http://utf8everywhere.org/).

<h3><a href="#what-string-type-should-i-use" name="what-string-type-should-i-use">
Welchen String-Typ sollte ich verwenden?
</a></h3>

Rust hat vier Paare von Stringtypen, von welchen [jeder einen bestimmten Sinn hat](http://www.suspectsemantics.com/blog/2016/03/27/string-types-in-rust/). In jedem dieser Paare befindet sich ein _Owned_-Typ sowie ein _Slice_-Typ. Die Typen sind folgendermaßen organisiert:

|               | _Slice_-Typ | _Owned_-Typ |
|:--------------|:-------------|:-------------|
| UTF-8         | `str`        | `String`     |
| Betriebssystemkompatibel | `OsStr`      | `OsString`   |
| C-kompatibel  | `CStr`       | `CString`    |
| Systempfad   | `Path`       | `PathBuf`    |

Jeder String-Typen dient einem anderen Zweck. `String` und `str` sind UTF-8-kodierte, allgemein verwendbare Strings. `OsString` und `OsStr` sind nach den Vorgaben der jeweiligen Plattform enkodiert und sollten benutzt werden, um mit dem Betriebssystem zu interagieren. `CString` und `CStr` sind Rusts Gegenstück zu C-Strings und werden in FFI (Foreign Function Interface)-Code genutzt. `PathBuf` und `Path` sind bequeme Wrapper um `OsString` und `OsStr`, welche Methoden zur Dateipfad-Manipulation anbieten.

<h3><a href="#why-are-there-multiple-types-of-strings" name="why-are-there-multiple-types-of-strings">
Wie kann ich eine Funktion schreiben, welche sowohl <code>&str</code> als auch <code>String</code> annimmt?
</a></h3>

Es gibt je nach Verwendung der Funktion verschiedene Möglichkeiten:

- Wenn die Funktion einen _Owned_-String benötigt, aber jede Art von String empfangen soll, dann nutze einen generischen Typen mit `Into<String>`-Bound.
- Wenn die Funktion einen String-Slice benötigt, aber jede Art von String empfangen soll, dann nutze einen generischen Typen mit `AsRef<str>`-Bound.
- Wenn es keine Rolle spielt, welchen String-Typ die Funktion bekommt, und du mit beiden Möglichkeiten einheitlich umgehen willst, dann nutze `Cow<str>` als Parametertyp.

__Nutzung von `Into<String>`__

In diesem Beispiel wird die Funktion sowohl _Owned_-Strings als auch String-Slices akzeptieren und dann entweder nichts tun oder die Eingabe innerhalb des Funktionskörpers zu einem _Owned_-String umwandeln. Diese Konversion muss explizit geschehen und wird ansonsten nicht stattfinden.

```rust
fn accepts_both<S: Into<String>>(s: S) {
    let s = s.into();   // This will convert s into a `String`.
    // ... the rest of the function
}
```

__Nutzung von `AsRef<str>`__

In diesem Beispiel wird die Funktion sowohl _Owned_-Strings als auch String-Slices akzeptieren und dann entweder nichts tun oder die Eingabe innerhalb der Funktion zu einem String-Slice umwandeln. Dies kann automatisch geschehen, indem die Eingabe als Referenz betrachtet wird:

```rust
fn accepts_both<S: AsRef<str>>(s: &S) {
    // ... the body of the function
}
```

__Nutzung von `Cow<str>`__

In diesem Beispiel akzeptiert die Funktion einen `Cow<str>`, der nicht ein generischer Typ ist, sondern ein Container, welcher je nach Nutzung einen _Owned_-String oder String-Slice enthält.

```rust
fn accepts_cow(s: Cow<str>) {
    // ... the body of the function
}
```


<h2 id="collections">Collections</h2>

<h3><a href="#can-i-implement-linked-lists-in-rust" name="can-i-implement-linked-lists-in-rust">
Ist es möglich, Datenstrukturen wie Vektoren oder verkettete Listen in Rust effizient zu implementieren?
</a></h3>

Es ist unnötig, diese Datenstrukturen zu Nutzung in deinem eigenen Programm selber zu schreiben, da effiziente Implementierungen durch die Standardbibliothek angeboten werden.

Wenn du aber [einfach nur lernen willst](http://cglab.ca/~abeinges/blah/too-many-lists/book/), dann wirst du wahrscheinlich _unsafe_-Code verwenden müssen. Diese Datenstrukturen _können_ zwar direkt in sicherem Rust implementiert werden, aber die Leistung einer _unsafe_-Implementierung wird wahrscheinlich besser sein. Der einfache Grund dafür ist, dass die Implementierung von Vektoren und verketteten Listen Zeigermanipulationen und Speicherzugriffe erfordert, welche in sicherem Rust verboten sind.

Doppelt verkettete Listen erfordern zum Beispiel, dass auf jeden Knoten zwei veränderliche Referenzen verweisen. Dies verletzt aber die Aliasing-Regeln für veränderliche Referenzen (es darf nur höchstens eine auf ein Objekt bestehen). Du kannst dieses Problem umgehen, indem du [`Weak<T>`][Weak] nutzt, aber darunter wird die Leistung stark leiden. Mit unsicherem Code kannst du die _Mutable-Aliasing_-Regel umgehen, aber dann musst du manuell sicherstellen, dass dein Code die Speichersicherheit nicht verletzt.

<h3><a href="#how-can-i-iterate-over-a-collection-without-consuming-it" name="how-can-i-iterate-over-a-collection-without-consuming-it">
Wie kann ich über eine Collection iterieren ohne sie zu konsumieren / zu verschieben?
</a></h3>

Die einfachste Art ist, die [`IntoIterator`][IntoIterator]-Implementierung der Referenzen auf Collections zu nutzen. Zum Beispiel für [`&Vec`][Vec]:

```rust
let v = vec![1,2,3,4,5];
for item in &v {
    print!("{} ", item);
}
println!("\nLength: {}", v.len());
```

In Rust nutzen die `for`-Schleifen die `into_iter()`-Funktion aus dem [`IntoIterator`][IntoIterator]-Trait der zu iterierenden Collection. Über alle Typen, die den [`IntoIterator`][IntoIterator]-Trait anbieten, kann mit einer `for`-Schleife iteriert werden. Für [`&Vec`][Vec] und [`&mut Vec`][Vec] ist [`IntoIterator`][IntoIterator] ebenfalls implementiert. Das bedeutet, dass ein Iterator durch `into_iter()` den Inhalt der Collection als Referenz betrachtet, statt ihn durch einen _Move_ zu konsumieren. Dies gilt auch für die anderen Standard-Collections.

Wenn du einen konsumierenden Iterator benötigst, dann schreibe die `for`-Schleife ohne `&` oder `&mut` in der Iteration.

Direkten Zugriff auf einen Iterator, welcher Referenzen auf den Inhalt anbietet, erhältst du normalerweise durch den Aufruf der `iter()`-Methode.

<h3><a href="#why-do-i-need-to-type-the-array-size-in-the-array-declaration" name="why-do-i-need-to-type-the-array-size-in-the-array-declaration">
Warum muss ich die Größe eines Arrays in der Deklaration angeben?
</a></h3>

Du musst das nicht zwingend tun. Wenn du ein Array direkt deklarierst, wird die Größe durch die Anzahl der Elemente bestimmt. Aber wenn du eine Funktion deklarierst, welche ein Array fester Größe annimmt, muss der Compiler wissen wie groß dieses Array sein wird.

Anzumerken ist, dass Rust momentan keine Generics für Arrays verschiedener Größe anbietet. Wenn du einen zusammenhängenden Container einer Variablen Anzahl von Werten annehmen willst, nutze einen [`Vec`][Vec] oder ein Slice (abhängig davon, ob du Ownership benötigst).

<h2 id="ownership">Ownership</h2>

<h3><a href="#how-can-i-implement-a-data-structure-that-contains-cycles" name="how-can-i-implement-a-data-structure-that-contains-cycles">
Wie kann ich eine Datenstruktur mit Zyklen implementieren?
</a></h3>

Es gibt mindestens vier Möglichkeiten, welche ausführlich in [Too Many Linked Lists](http://cglab.ca/~abeinges/blah/too-many-lists/book/)) beschrieben werden:

- Du kannst [`Rc`][Rc] und [`Weak`][Weak] nutzen, um geteilte Ownership zu Knoten zu erhalten.
Dann musst du die Kosten von Memory Management in Kauf nehmen.
- Du kannst `unsafe`-Code mit rohen Pointern nutzen.
Dies wird effizient sein, aber es umgeht die Sicherheitsgarantien.
- Du kannst Vektoren und Indexe in diesen Vektoren benutzen. Hier sind einige Beispiele und Erklärungen für diese Herangehensweise: [several](http://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/) [available](https://featherweightmusings.blogspot.com/2015/04/graphs-in-rust.html).
- Du kannst 'borrowed' Referenzen mit [`UnsafeCell`][UnsafeCell] nutzen. Es gibt für diese Herangehensweise [Erklärungen und Beispielcode](https://github.com/nrc/r4cppp/blob/master/graphs/README.md#node-and-unsafecell).

<h3><a href="#how-can-i-define-a-struct-that-contains-a-reference-to-one-of-its-own-fields" name="how-can-i-define-a-struct-that-contains-a-reference-to-one-of-its-own-fields">
Wie kann ich ein Struct mit Referenzen zu seinen eigenen Feldern definieren?
</a></h3>

Das ist möglich, aber nutzlos. Das Struct ist dann permanent von sich selbst „ausgeliehen“ und kann nicht bewegt werden. Hier ein Beispiel:

```rust
use std::cell::Cell;

#[derive(Debug)]
struct Unmovable<'a> {
    x: u32,
    y: Cell<Option<&'a u32>>,
}


fn main() {
    let test = Unmovable { x: 42, y: Cell::new(None) };
    test.y.set(Some(&test.x));

    println!("{:?}", test);
}
```

<h3><a href="#what-is-the-difference-between-consuming-and-moving" name="what-is-the-difference-between-consuming-and-moving">
Was ist der Unterschied zwischen Pass-By-Value, Konsumieren, Verschieben (Moving), und der Übertragung von Ownership?
</a></h3>

Alle diese Begriffe sind äquivalent. In jedem Fall bedeuten sie, dass der Wert zu einem neuen Besitzer übertragen und aus dem Besitz des vorherigen entfernt wurde. Der vorherige Besitzer kann den Wert nicht mehr nutzen. Wenn ein Typ den `Copy`-Trait anbietet, dann wird der Wert des ursprünglichen Besitzers nicht invalidiert und kann weiterhin benutzt werden.

<h3><a href="#why-can-values-of-some-types-by-reused-while-others-are-consumed" name="why-can-values-of-some-types-by-reused-while-others-are-consumed">
Warum können Werte eines Typs nach dem übergeben an eine Funktion wiederverwendet werden, während die Wiederverwendung von Werten anderen Typs zu einem Fehler führt?
</a></h3>

Wenn ein Typ den [`Copy`][Copy]-Trait anbietet, dann wird ein Wert dieses Typs bei der Übergabe an eine Funktion kopiert. Alle numerischen Typen in Rust implementieren [`Copy`][Copy], aber Struct-Typen implementieren den Trait nicht standardmäßig. Sie werden also standardmäßig mit _Ownership_ übergeben. Das bedeutet, dass ein Struct nach der Übergabe nicht mehr benutzt werden kann, wenn es nicht (samt Ownership) am Ende der aufgerufenen Funktion wieder zurückgegeben wird.

<h3><a href="#how-do-you-deal-with-a-use-of-moved-value-error" name="how-do-you-deal-with-a-use-of-moved-value-error">
Wie gehe ich mit einem „Use of moved Value“-Fehler um?
</a></h3>

Diese Fehlermeldung bedeutet, dass du versuchst auf einen Wert zuzugreifen, der den Besitzer gewechselt hat. Die erste Frage ist, ob die Übergabe des Besitzes nötig war: Wenn der Wert einer Funktion übergeben wurde, dann könnte es möglich sein die Funktion so umzuschreiben, dass sie nur eine Referenz entgegennimmt.
Wenn der übergebene Typ den [`Clone`][Clone]-Trait implementiert, dann wird ein Aufruf zu `clone()` vor der Übergabe eine Kopie des Wertes erstellen und das Original zur weiteren Verwendung freigeben. Da durch Klonen weitere Allokationen notwendig werden, sollte dies der letzte Ausweg sein.

Wenn der Übergebene Wert dein eigener Typ ist, dann könntest du [`Copy`][Copy] (für implizites Kopieren anstelle von Klonen) oder [`Clone`][Clone] (explizite Kopie) selbst implementieren. [`Copy`][Copy] kann häufig durch `#[derive(Copy, Clone)]` erhalten werden ([`Copy`][Copy] erfordert [`Clone`][Clone]), und [`Clone`][Clone] mit `#[derive(Clone)]`.

Wenn diese Möglichkeiten nicht gegeben sind, dann könntest du die Funktion, welche den Besitz des Wertes erfordert so modifizieren, dass sie den Besitz am Ende wieder zurückgibt.

<h3><a href="#what-are-the-rules-for-different-self-types-in-methods" name="what-are-the-rules-for-different-self-types-in-methods">
Nach welchen Regeln richtet sich die Verwendung von <code>self</code>, <code>&amp;self</code>, oder <code>&amp;mut self</code> in Methodendeklarationen?
</a></h3>

- Nutze `self`, wenn eine Methode Besitz vom Wertes ergreifen soll.
- Nutze `&self`, wenn eine Methode lediglich eine Nur-Lese-Referenz auf den Wert benötigt.
- Nutze `&mut self`, wenn eine Methode den Wert verändern, aber nicht davon Besitz ergreifen soll.

<h3><a href="#how-can-i-understand-the-borrow-checker" name="how-can-i-understand-the-borrow-checker">
Wie kann ich den Borrow-Checker verstehen?
</a></h3>

Zum Evaluieren von Rust-Code nutzt der Borrow Checker nur ein paar wenige Regeln, welche im [Abschnitt über Borrowing](https://doc.rust-lang.org/stable/book/references-and-borrowing.html#the-rules) erklärt werden. Diese Regeln sind:

> Erstens darf ein _Borrow_ höchstens so lange bestehen wie der eigentliche Besitzer. Zweitens darfst du zu jedem Zeitpunnkt nur eine dieser beiden Arten von Borrow haben:
>
> - Eine oder mehr Referenzen (&T) zu einer Ressource.
> - Genau eine veränderbare Referenz (&mut T).

Diese Regeln sind zwar einfach, sie konsistent zu befolgen aber nicht, vor allem wenn man nicht gewohnt ist, über Lifetimes und Ownership nachzudenken.

Der erste Schritt zum Verständnis des Borrow Checkers ist es, seine Fehlermeldungen zu studieren. Es wurde viel Arbeit investiert, um die Qualität der Hilfestellungen des Borrow Checkers zu erhöhen. Wenn du ein Problem mit dem Borrow Checker hast, ist der erste Schritt, langsam und vorsichtig die Fehlermeldung zu lesen. Verändere deinen Code erst, nachdem du den Fehler verstanden hast.

Der zweite Schritt ist, die Container-Typen der Standardbibliothek, welche mit Ownership und Mutability zu tun haben, kennenzulernen. Dies sind zum Beispiel [`Cell`][Cell], [`RefCell`][RefCell], und [`Cow`][Cow]. Diese Typen sind nützliche und notwendige Werkzeuge, um bestimmte Ownership- und Mutabilityverhältnisse auszudrücken, und wurden für minimale Leistungskosten entworfen.

Der wichtigste Bestandteil, um den Borrow Checker zu verstehen, ist Übung. Die starke Garantien der statischen Analyse von Rust sind streng und von anderen Programmiersprachen verschieden. Es wird einige Zeit dauern, mit Allem vollständig zurechtzukommen.

Wenn du dich mit dem Borrow Checker allzu sehr abmühen musst und dir die Geduld ausgeht, ist die [Rust-Community](community.html) jederzeit für dich da.

<h3><a href="#when-is-rc-useful" name="when-is-rc-useful">
Wann sollte ich <code>Rc</code> verwenden?
</a></h3>

Die Funktion des nichtatomaren, referenzzählenden Containers [`Rc`][Rc] wird in der offiziellen Dokumentation erläutert. Kurz gesagt kann man [`Rc`][Rc] und seinen threadsicheren Cousin [`Arc`][Arc] verwenden, um gemeinsamen Besitz einer Ressource auszudrücken und diese Ressource automatisch freizugeben, wenn kein Besitzer mehr Zugriff darauf hat.

<h3><a href="#how-do-i-return-a-closure-from-a-function" name="how-do-i-return-a-closure-from-a-function">
Wie gebe ich eine Closure aus einer Funktion zurück?
</a></h3>

Um eine Closure aus einer Funktion herausreichen zu können, muss sie eine _Move Closure_ sein, welche mit dem Schlüsselwort `move` deklariert wird. Wie [im Buch zu Rust erklärt](https://doc.rust-lang.org/book/closures.html#move-closures), gibt dies der Closure eine eigene Kopie ihrer eingefangenen Variablen, die vom Stack Frame der Elternfunktion unabhängig sind. Eine andere Rückgabe von Closures wäre unsicher, da dies Zugriff auf nicht mehr gültige Variablen gewähren würde. In anderen Worten: Es würde das Auslesen potentiell ungültigen Speichers ermöglichen. Die Closure muss außerdem von einer [`Box`][Box] umgeben werden, damit sie auf dem Heap allokiert wird. [Im Buch](https://doc.rust-lang.org/book/closures.html#returning-closures) kannst du mehr darüber lesen.

<h3><a href="#what-are-deref-coercions" name="what-are-deref-coercions">
Was sind <i>Deref Coercions</i> und wie funktionieren sie?
</a></h3>

Eine [Deref Coercion](https://doc.rust-lang.org/book/deref-coercions.html) ist eine nützliche Umwandlung, die automatisch Referenzen auf Zeigertypen (also zum Beispiel `&Rc<T>` or `&Box<T>`) zu Referenzen auf ihren Inhalt konvertiert.
Deref Coercions existieren, um einen ergonomischeren Umgang mit Rust zu ermöglichen und werden durch den [`Deref`][Deref]-Trait implementiert.

Eine Implementierung von Deref gibt an, dass der implementierende Typ durch den Aufruf der `deref`-Methode zu einem Zieltyp konvertiert werden kann. Dabei nimmt die Methode eine unveränderliche Referenz zum aufrufenden Typ an und gibt eine Referenz mit derselben Lifetime zum Zieltyp zurück. Der `*`-Präfix ist eine Kurznotation für die `deref`-Methode.

Der Name „Coercion“ kommt aus der [hier im Buch erklärten Regel](https://doc.rust-lang.org/stable/book/deref-coercions.html):

> Wenn du einen Typ `U` hast, welcher `Deref<Target=T>` implementiert, dann können Werte von `&U` automatisch in `&T` konvertiert (_coerced_) werden.

Wenn du beispielsweise einen `&Rc<String>` hast, wird er nach dieser Regel automatisch zu einem `&String`, welcher dann auf die gleiche Weise zu einem `&str` wird. Wenn also eine Funktion einen `&str`-Parameter entgegennimmt, kannst du einen `&Rc<String>` direkt übergeben, woraufhin alle Umwandlungen automatisch durch den `Deref`-Trait geschehen.

Die häufigsten Arten der Deref Coercion sind:

- `&Rc<T>` zu `&T`
- `&Box<T>` zu `&T`
- `&Arc<T>` zu `&T`
- `&Vec<T>` zu `&[T]`
- `&String` zu `&str`

<h2 id="lifetimes">Lifetimes</h2>

<h3><a href="#why-lifetimes" name="why-lifetimes">
Welchen Zweck haben Lifetimes?
</a></h3>

Lifetimes sind Rusts Antwort auf die Frage der Speichersicherheit. Sie erlauben es Rust, Speichersicherheit ohne die Laufzeitkosten von Garbage-Collection zu erlangen. Sie basieren auf einer Vielzahl akademischer Arbeiten, welche im [Rust book](https://doc.rust-lang.org/stable/book/bibliography.html#type-system) nachgeschlagen werden können.

<h3><a href="#why-is-the-lifetime-syntax-the-way-it-is" name="why-is-the-lifetime-syntax-the-way-it-is">
Warum ist die Syntax für Lifetimes so, wie sie ist?
</a></h3>

Die `'a`-Syntax kommt aus der ML-Familie der Programmiersprachen, wo `'a` benutzt wird, um einen generischen Parameter zu markieren. In Rust sollte der Syntax eindeutig und auffällig sein und neben Traits und Referenzen in eine Typdeklaraion (Signatur) passen. Alternative syntaktische Repräsentationen wurden diskutiert, von denen aber keine eindeutige Vorteile vorwiesen.

<h3><a href="#how-do-i-return-a-borrow-to-something-i-created-from-a-function" name="how-do-i-return-a-borrow-to-something-i-created-from-a-function">
Wie kann ich in einer Funktion einen Wert alloziieren, um dann eine Referenz darauf zurückzugeben?
</a></h3>

Du musst sicherstellen, dass das alloziierte Objekt länger lebt als die Funktion. Das kannst du erreichen, indem du die Ausgabe-Lifetime an eine Eingabe-Lifetime bindest:

```rust
type Pool = TypedArena<Thing>;

// (Die Lifetime 'a ist nur zur Erklärung explizit angegeben.
// Du kannst sie nach den in einem späteren FAQ-Eintrag
// erklärten Elision-Regeln weglassen.)
fn create_borrowed<'a>(pool: &'a Pool,
                       x: i32,
                       y: i32) -> &'a Thing {
    pool.alloc(Thing { x: x, y: y })
}
```

Eine Alternative Vorgehensweise wäre, die Referenzen vollständig wegzulassen und einen Typ mit _Ownership_ wie zum Beispiel [`String`][String] zurückzugeben:

```rust
fn happy_birthday(name: &str, age: i64) -> String {
    format!("Hello {}! You're {} years old!", name, age)
}
```

Diese Vorgehensweise ist einfacher, aber hat oft unnötige Allokationen zufolge.

<h3><a href="#when-are-lifetimes-required-to-be-explicit" name="when-are-lifetimes-required-to-be-explicit">
Warum haben manche Referenzen Lifetimes, wie <code>&amp;'a T</code>, und manche anderen wie <code>&amp;T</code> nicht?
</a></h3>

Eigentlich haben *alle* Referenzen eine Lifetime, aber meistens musst du sie nicht explizit angeben.
Die Regeln sind wie folgt:

1. Innerhalb eines Funktionskörpers musst du nie explizit
   eine Lifetime angeben;
   Der korrekte Wert sollte immer inferiert werden.
2. Innerhalb einer Funktions*signatur* (zum Beispiel für die Typen der
   Parameter oder den Rückgabetyp), sind explizite Lifetimes manchmal notwendig.
   Hier nutzen Lifetimes ein einfaches Vorgabeschema namens
   [_Lifetime elision_](https://doc.rust-lang.org/book/lifetimes.html#lifetime-elision),
   welches aus den drei folgenden Regeln besteht:
  - Jede ausgelassene Lifetime in den Funktionsparametern wird ein eigener Lifetime-Parameter.
  - Wenn es genau eine explizite oder ausgelassene Eingabe-Lifetime gibt, dann
    wird diese Lifetime allen ausgelassenen Lifetimes der Rückgabetypen
    dieser Funktion zugewiesen.
  - Wenn es mehrere Eingabe-Lifetimes gibt, von denen eine `&self` oder `&mut self` ist,
    dann wird die Lifetime von `self` allen ausgelassenen Rückgabe-Lifetimes zugeordnet.
3. In einer `struct`- oder `enum`-Definition müssen alle Lifetimes explizit angegeben werden.

Wenn diese Regeln nicht anwendbar sind, wird der Rust-Compiler eine Fehlermeldung zusammen mit einer potenziellen Lösung ausgegeben. Diese Lösung hängt vom konkreten Schritt des Inferenzvorganges ab,
in dem der Fehler aufgetreten ist.

<h3><a href="#how-can-rust-guarantee-no-null-pointers" name="how-can-rust-guarantee-no-null-pointers">
Wie kann Rust Freiheit von Nullzeigern und „hängenden Zeigern“ garantieren?
</a></h3>

Der einzige Weg, einen Wert vom Typ `&Foo` oder `&mut Foo` zu konstruieren ist, einen existierenden Wert vom Typ `Foo` anzugeben, auf den die Referenz zeigt. Die Referenz „borgt“ sich den originalen Wert für einen gegeben Abschnitt des Codes (nämlich der Lifetime der Referenz) aus. Während der Dauer der „Ausborgung“ kann der Wert nicht an einen neuen Besitzer übergeben, verändert oder freigegeben werden.

<h3><a href="#how-do-i-express-the-absense-of-a-value-without-null" name="how-do-i-express-the-absense-of-a-value-without-null">
Wie drücke ich die Abwesenheit eines Wertes aus, ohne <code>null</code> zu verwenden?
</a></h3>

Das kannst du mit dem [`Option`][Option]-Typ erreichen, welcher entweder ein `Some(T)` oder `None` sein kann. `Some(T)` zeigt an, dass ein Wert vom Typ `T` in der Option vorhanden ist, während `None` die Abwesenheit anzeigt.

<h2 id="generics">Generics</h2>

<h3><a href="#what-is-monomorphisation" name="what-is-monomorphisation">
Was ist Monomorphisierung?
</a></h3>

Monomorphisierung spezialisiert jeden Aufruf einer generischen Funktion oder Struktur mit einer spezifischen Instanz
basierend auf den Parametertypen der Funktionaufrufe (oder Verwendungen der Struktur).

Für jede einzigartige Menge von Typen, mit welcher eine generische Funktion instanziiert wird, erstellt der Compiler eine neue Kopie der Funktion. Diese Strategie wird auch von C++ genutzt und resultiert in schnellem Code, welcher für jeden Aufruf spezialisiert ist und frühe Bindung nutzen kann. Allerdings kann sie bei vielen voneinander verschiedenen Aufruftypen auch dazu führen, dass die Größe der generierten Ausführbaren Datei größer ist als bei anderen Aufrufstrategien.

Für Funktionen, welche anstelle von Typparametern [Trait Objects](https://doc.rust-lang.org/book/trait-objects.html) annehmen, wird keine Monomorphisierung durchgeführt. Stattdessen werden Methoden für Trait Objects dynamisch zur Laufzeit entschieden.

<h3><a href="#whats-the-difference-between-a-function-and-a-closure-that-doesnt-capture" name="whats-the-difference-between-a-function-and-a-closure-that-doesnt-capture">
Was ist der Unterschied zwischen einer Funktion und einer Closure, welche keine Variablen einfängt?
</a></h3>

Funktionen und Closures sind äquivalente Operationen, haben aber zur Laufzeit aufgrund ihrer verschiedenen Implementierungen unterschiedliche Repräsentationen.

Funktionen sind eingebaute Primitive der Sprache, während Closures eigentlich syntaktischer Zucker für einen von drei Traits sind: [`Fn`][Fn], [`FnMut`][FnMut], und [`FnOnce`][FnOnce]. Beim Kompilieren einer Closure wird der Rust-Compiler automatisch ein Struct erstellen, für das der entsprechende Trait implementiert ist und der die entgegengenommenen Variablen als Attribute enthält. der Trait ermöglicht es, dieses Struct wie eine Funktion aufzurufen. Reguläre Funktionsdefinitionen können keine Variablen aus ihrer Umgebung einfangen.

Der große Unterschied zwischen diesen Traits ist, wie sie mit dem `self`-Parameter umgehen. [`Fn`][Fn] nimmt `&self`, [`FnMut`][FnMut] nimmt `&mut self`, und [`FnOnce`][FnOnce] nimmt `self`.

Sogar wenn eine Closure gar keine Umgebungsvariablen entgegennimmt, wird sie zur Laufzeit als zwei Zeiger repräsentiert, genau wie jede andere Closure.

<h3><a href="#what-are-higher-kinded-types" name="what-are-higher-kinded-types">
Was sind Typen höherer Ordnung, wozu brauche ich sie, und warum hat Rust sie nicht?
</a></h3>

Typen höherer Ordnung sind Typen mit noch ausstehenden Parametern. Typkonstruktoren wie [`Vec`][Vec], [`Result`][Result], und [`HashMap`][HashMap] sind Beispiele für Typen höherer Ordnung: Sie erfordern einige weitere Typparameter um einen tatsächlichen Typ wie `Vec<u32>` darzustellen. Unterstützung für Typen höherer Ordnung würde bedeuten, dass diese „unvollständigen“ Typen überall dort benutzt werden können, wo sonst auch „vollständige“ Typen verwendet werden können, also auch als generische Parameter für Funktionen.

Jeder vollständige Typ wie [`i32`][i32], [`bool`][bool], oder [`char`][char] ist von der Art `*` (diese Notation kommt aus der Typtheorie). Ein Typ mit einem Parameter, wie [`Vec<T>`][Vec] ist von der Art `* -> *`, was bedeutet, dass [`Vec<T>`][Vec] einen vollständigen Typ wie [`i32`][i32] nimmt und einen vollständigen Typ wie `Vec<i32>` zurückgibt.
Ein Typ mit drei Parametern, wie [`HashMap<K, V, S>`][HashMap] ist von der Art `* -> * -> * -> *`, und nimmt drei vollständige Typen (wie [`i32`][i32], [`String`][String], and [`RandomState`][RandomState]) entgegen, um einen neuen vollständigen Typ `HashMap<i32, String, RandomState>` zu konstruieren.

Zusätzlich zu diesen Beispielen können Typkonstruktoren auch *Lifetime*-Argumente entgegennehmen, welche wir mit `Lt` bezeichnen wollen. Der Typ `slice::Iter` hat zum Beispiel die Art `Lt -> * -> *`, weil er mit einer Lifetime und einem vollständigen Typ konstruiert werden muss: `Iter<'a, u32>`.

Die fehlende Unterstützung von Typen höherer Ordnung erschwert das Schreiben von bestimmten Arten generischen Codes. Es ist insbesondere problematisch, Konzepte wie Iteratoren zu abstrahieren, da diese oft mit mindestens einer Lifetime parametrisiert sind. Dies hat die Erstellung von Traits verhindert, die über Collections abstrahieren.

Ein weiteres häufiges Beispiel sind Konzepte wie Funktoren oder Monaden. Beide sind Typkonstruktoren statt einfacher Typen.

Bisher hat Rust keine Typen höherer Ordnung weil ihnen gegenüber anderen Verbesserungen keine Priorität zugewiesen wurde. Weil der Entwurf eine große, querschneidende Veränderung ist, wollen wir auch vorsichtig damit umgehen. Aber es gibt keinen tiefgreifenden Grund, der Typen höherer Ordnung unmöglich machen würde.

<h3><a href="#what-do-named-type-parameters-in-generic-types-mean" name="what-do-named-type-parameters-in-generic-types-mean">
Was bedeuten benannte Parameter wie <code>&lt;T=Foo&gt;</code> in generischen Typen?
</a></h3>

Diese werden [Assoziierte Typen](https://doc.rust-lang.org/stable/book/associated-types.html) genannt und erlauben es, Trait-Bounds auszudrücken, für die eine `where`-Klausel nicht ausreicht.
Eine generische Einschränkung `X: Bar<T=Foo>` bedeutet: „`X` muss den Trait `Bar` implementieren, und die Implementierung von `Bar` muss für den assoziierten Typ `T` den Typ `Foo` annehmen.“. Beispiele für Typ-Einschränkungen, welche nicht mit einer `where`-Klausel ausgedrückt werden können, sind zum Beispiel Trait-Objekte wie `Box<Bar<T=Foo>>`.

Assoziierte Typen existieren, weil Generics oft mit Familien von Typen umgehen müssen, wobei ein Typ alle anderen in der Familie bestimmt. Ein Trait eines Graphen zum Beispiel könnte als `Self`-Typ den Graphen selber haben, sowie assoziierte Typen für Knoten und Kanten. Jeder Graph-Typ bestimmt dann eindeutig die assoziierten Typen. Die Verwendung von assoziierten Typen vereinfacht die Arbeit mit solchen Typfamilien stark und bietet in vielen Fällen auch bessere Typinferenz.

<h3><a href="#how-do-i-overload-operators" name="how-do-i-overload-operators">
Kann ich Operatoren überladen? Welche und wie?
</a></h3>

Du kannst eigene Implementierung bestimmte Operatoren über die zugehörigen Traits definieren: [`Add`][Add] für `+`, [`Mul`][Mul] für `*` und so weiter. Das Ganze sieht folgendermaßen aus:

```rust
use std::ops::Add;

struct Foo;

impl Add for Foo {
    type Output = Foo;
    fn add(self, rhs: Foo) -> Self::Output {
        println!("Addition");
        self
    }
}
```

Die folgenden Operatoren können überladen werden:

| Operation            | Trait                          |
|:---------------------|:-------------------------------|
| `+`                  | [`Add`][Add]                   |
| `+=`                 | [`AddAssign`][AddAssign]       |
| `binary -`           | [`Sub`][Sub]                   |
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
Warum gibt es die Unterscheidung zwischen <code>Eq</code>/<code>PartialEq</code> und <code>Ord</code>/<code>PartialOrd</code>?
</a></h3>

Die Werte mancher Typen in Rust sind nur partiell geordnet oder kennen nur partielle Gleichheit. In einer partiellen Ordnung kann es vorkommen, dass bei zwei verschiedenen Werte eines Typs der eine weder kleiner noch größer als der andere ist. Partielle Gleichheit bedeutet, dass Werte des Typs ungleich sich selbst sein können.

Gleitkommazahlen ([`f32`][f32] and [`f64`][f64]) sind gute Beispiele für beide Fälle. Gleitkommatypen können den Wert `NaN` ("Not a Number") annehmen. `NaN` ist ungleich sich selbst (`NaN == NaN` ist `false`) und nicht kleiner oder größer als jeder beliebige Gleitkommawert. Deshalb implementieren sowohl [`f32`][f32] als auch [`f64`][f64] [`PartialOrd`][PartialOrd] und [`PartialEq`][PartialEq], nicht aber [`Ord`][Ord] oder [`Eq`][Eq].

Wie in der obigen [Frage zu Gleitkommazahlen](#why-cant-i-compare-floats) erklärt, ist diese Unterscheidung wichtig, da manche Collection-Typen eine totale Ordnung oder Vergleichbarkeit benötigen, um korrekt zu funktionieren.

<h2 id="input-output">Ein- und Ausgabe</h2>

<h3><a href="#how-do-i-read-a-file-into-a-string" name="how-do-i-read-a-file-into-a-string">
Wie lese ich eine Datei in einen <code>String</code> ein?
</a></h3>

Mit der [`read_to_string()`][read__read_to_string]-Methode, die auf dem [`Read`][Read]-Trait in [`std::io`][std-io] definiert ist.

```rust
use std::io::Read;
use std::fs::File;

fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut f = try!(File::open(path));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));  // `s` enthält den Inhalt von "foo.txt"
    Ok(s)
}

fn main() {
    match read_file("foo.txt") {
        Ok(_) => println!("Datei gelesen!"),
        Err(err) => println!("Fehler beim Lesen der Datei: {}", err)
    };
}
```

<h3><a href="#how-do-i-read-file-input-efficiently" name="how-do-i-read-file-input-efficiently">
Wie lese ich effektiv aus einer Datei?
</a></h3>

Der Typ [`File`][File] implementiert den [`Read`][Read]-Trait, der eine Reihe an Funktionen zum Lesen und Schreiben von Daten bereitstellt, etwa [`read()`][read__read], [`read_to_end()`][read__read_to_end], [`bytes()`][read__bytes], [`chars()`][read__chars], und [`take()`][read__take]. Jede dieser Funktionen liest einen bestimmte Menge an Daten aus der jeweiligen Datei. [`read()`][read__read] liest so viele Daten, wie das Eingabe-Ausgabe-System in einem einzigen Aufruf zur Verfügung stellt. [`read_to_end()`][read__read_to_end] liest den gesamten Puffer in einen Vektor ein und fordert dabei so viel Speicher an wie notwendig. [`bytes()`][read__bytes] und [`chars()`][read__chars] liefern Iteratoren über die Bytes bzw. Zeichen einer Datei. Zu guter Letzt ermöglicht es [`take()`][read__take], eine beliebige Anzahl Bytes aus der Datei zu lesen. Zusammen genommen sollten diese Funktionen ausreichen, um effektiv aus jeder beliebigen Datei zu lesen.

Für gepufferte Eingabe gibt es den [`BufReader`][BufReader]-Struct, der hilft, die Anzahl der Systemaufrufe während des Lesens zu verringern.

<h3><a href="#how-do-i-do-asynchronous-input-output-in-rust" name="how-do-i-do-asynchronous-input-output-in-rust">
Wie setze ich asynchrone Ein- und Ausgabe in Rust um?
</a></h3>

Es gibt mehrere aktive Bibliotheken zu asynchronem I/O in Rust, wie etwa [mio](https://github.com/carllerche/mio), [tokio](https://github.com/tokio-rs/tokio-core), [mioco](https://github.com/dpc/mioco), [coio-rs](https://github.com/zonyitoo/coio-rs), und [rotor](https://github.com/tailhook/rotor).

<h3><a href="#how-do-i-get-command-line-arguments" name="how-do-i-get-command-line-arguments">
Wie kann ich auf die Kommandozeilenargumente meines Programms zugreifen?
</a></h3>

Die einfachste Möglichkeit ist [`Args`][Args], das einen Iterator über die Argumente zur Verfügung stellt.

Wenn du nach etwas mächtigeren suchst, gibt es eine [Reihe an Optionen auf crates.io](https://crates.io/keywords/argument).

<h2 id="error-handling">Fehlerbehandlung</h2>

<h3><a href="#why-doesnt-rust-have-exceptions" name="why-doesnt-rust-have-exceptions">
Warum kennt Rust keine Exceptions?
</a></h3>

Exceptions erschweren das Verständnis von Kontrollfluss, drücken Gültigkeit und Ungültigkeit außerhalb des Typsystems aus und spielen schlecht mit Multithreading zusammen (ein wichtiges Ziel von Rust).

Rust zieht einen typbasierten Ansatz zur Fehlerbehandlung vor, der [ausführlich im Buch](https://doc.rust-lang.org/stable/book/error-handling.html) beschrieben wird. Dieser passt besser zum Kontrollfluss, der Nebenläufigkeit und dem Rest der Sprache.

<h3><a href="#whats-the-deal-with-unwrap" name="whats-the-deal-with-unwrap">
Was hat es mit den dauernden <code>unwrap()</code>-Aufrufen auf sich?
</a></h3>

`unwrap()` ist eine Funktion, die den Wert aus einer [`Option`][Option] oder einem [`Result`][Result] entpackt und eine Panic auslöst, wenn dieser Wert nicht vorhanden ist.

`unwrap()` ist kein guter Weg, um Fehlersituationen wie falsche Benutzereingaben abzufangen. In Produktionscode dient `unwrap()` eher als Assertion, um das Vorhandensein eines Werts als Invariante sicherzustellen, deren Verletzung einen Bug darstellt und das Programm sofort abbrechen soll.

Die Funktion ist auch für Prototypen geeignet, in denen noch keine Fehlerbehandlung implementiert werden soll. Außerdem ist sie für Codebeispiele praktisch, in denen die Fehlerbehandlung vom Ziel des Programms ablenken würde.

<h3><a href="#why-do-i-get-errors-with-try" name="why-do-i-get-errors-with-try">
Warum bekomme ich einen Compilerfehler in Beispielcode, der das <code>try!</code>-Makro benutzt?
</a></h3>

Das liegt wahrscheinlich am Rückgabetyp der Funktion. Das [`try!`][TryMacro]-Makro entpackt im Erfolgsfall den Wert aus einem [`Result`][Result] oder kehrt aus der aufrufenden Funktion mit dem Fehler zurück, den das [`Result`][Result] beschreibt. Damit funktioniert `try!` nur in Funktionen die ihrerseits ein `Result` zurückgeben. Dessen `Err`-Wert muss außerdem mit `From::from(err)` aus dem Fehlertyp des `try!`-Arguments konstruiert werden können. Insbesondere kann `try!` somit auch nicht in der `main`-Funktion verwendet werden.

<h3><a href="#error-handling-without-result" name="error-handling-without-result">
Gibt es einen einfacheren Fehlerbehandlungsmechanismus, als überall <code>Result</code> zu verwenden?
</a></h3>

[`Result`s][Result]-Werte wirst du immer mit einem [`unwrap()`][unwrap] los, nur ist das meistens nicht das was du möchtest. [`Result`][Result] ist ein Anzeichen dafür, dass eine Operation möglicherweise fehlschlagen kann. Rust zwingt dich diese Fehlerpfade explizit zu behandeln, um die Robustheit von Programmen gegenüber Fehlersituationen zu fördern. Es gibt Helfer wie das [`try!`-Makro][TryMacro], das das Propagieren von Fehlern angenehmer macht.

Wenn du einen Fehler wirklich nicht behandeln möchtest kannst du auf [`unwrap()`][unwrap] zurückgreifen. Das bedeutet aber, dass dein Code im Fehlerfall eine Panic erzeugt, die im Normalfall den Prozess sofort beendet.

<h2 id="concurrency">Nebenläufigkeit</h2>

<h3><a href="#can-i-use-static-values-across-threads-without-an-unsafe-block" name="can-i-use-static-values-across-threads-without-an-unsafe-block">
Kann ich statische Werte in mehreren Threads benutzen, ohne auf <code>unsafe</code>-Code zurückzugreifen?
</a></h3>

Schreibende Zugriffe sind sicher, solange sie synchronisiert erfolgen. Dazu kann etwa ein [`Mutex`][Mutex] (spät initialisiert über [lazy-static](https://crates.io/crates/lazy_static/)) oder ein [`AtomicUsize`][AtomicUsize] (regulär initialisiert) verwendet werden.

Allgemein gesprochen können all die Typen in einer [statischen Variablen benutzt werden](https://doc.rust-lang.org/book/const-and-static.html#static), die [`Sync`][Sync] aber nicht [`Drop`][Drop] implementieren.

<h2 id="macros">Makros</h2>

<h3><a href="#can-i-write-a-macro-to-generate-identifiers" name="can-i-write-a-macro-to-generate-identifiers">
Kann ich ein Makro schreiben, das neue Bezeichner erzeugt?
</a></h3>

Momentan nicht. Rust-Makros sind [&„hygiensich“](https://en.wikipedia.org/wiki/Hygienic_macro) und vermeiden damit absichtlich das Reservieren oder Erzeugen von Bezeichnern, die unerwartete Kollisionen mit anderem Code auslösen können. Ihre Fähigkeiten unterscheiden sich grundsätzlich von Makros, wie sie vom C-Präprozessor bekannt sind. Makro-Aufrufe können nur an explizit erlaubten Positionen im Code auftauchen: An Stelle von Definitionen (_items_) auf Modulebene, Methodendeklarationen, Statements, Ausdrücken, und Patterns. Sie können nicht benutzt werden, um eine partielle Methodendeklaration oder Variablendeklaration zu vervollständigen.

<h2 id="debugging">Fehlersuche und Werkzeuge</h2>

<h3><a href="#how-do-i-debug-rust-programs" name="how-do-i-debug-rust-programs">
Wie finde ich Fehler in meinem Rust-Programm?
</a></h3>

Rust-Programme können mit [gdb](https://sourceware.org/gdb/current/onlinedocs/gdb/) oder [lldb](http://lldb.llvm.org/tutorial.html) debuggt werden - genau wie C und C++. Jede Rust-Installation kommt mit rust-gdb, rust-lldb oder beidem, je nachdem was die Plattform unterstützt. Dabei handelt es sich um Wrapper um gdb und lldb, die Rust-Datenstrukturen lesbar ausgeben können (_pretty printing_).

<h3><a href="#how-do-i-locate-a-panic" name="how-do-i-locate-a-panic">
<code>rustc</code> behauptet, dass eine Panic im Code er Standardbibliothek aufgetreten ist. Wie kann ich den Fehler in meinem Code finden?
</a></h3>

Dieser Fehler tritt meistens auf, wenn aus Anwendungscode heraus [`unwrap()`][unwrap] auf einem `None`- oder `Err`-Wert aufgerufen wird. Ist beim Start des Programms die Umgebungsvariable `RUST_BACKTRACE=1` gesetzt, wird zusammen mit der Panic der Aufrufstapel ausgegeben, der zum Fehler geführt hat. Es hilft auch, das Projekt im Debug-Modus zu übersetzen (Standardverhalten für `cargo build`) und einen Debugger wie `rust-gdb` oder `rust-lldb` einzusetzen.

<h3><a href="#what-ide-should-i-use" name="what-ide-should-i-use">
Welche IDE sollte ich benutzen?
</a></h3>

Es gibt eine ganze Reihe an Entwicklungsumgebungen für Rust, die alle auf der [Seite zu IDEs](https://forge.rust-lang.org/ides.html) vorgestellt werden.

<h3><a href="#wheres-rustfmt" name="wheres-rustfmt">
<code>gofmt</code> ist toll! Wo ist <code>rustfmt</code>?
</a></h3>

`rustfmt` findest du [hier](https://github.com/rust-lang-nursery/rustfmt). Das Tool wird aktiv weiterentwickelt, um Rust-Code so leserlich und vorhersagbar wie möglich zu gestalten.

<h2 id="low-level">Low-Level</h2>

<h3><a href="#how-do-i-memcpy-bytes" name="how-do-i-memcpy-bytes">
Kann ich Bytes wie mit <code>memcpy</code> kopieren?
</a></h3>

Wenn du nur einen Slice klonen möchtest, kannst du dafür [`clone_from_slice`][clone_from_slice] verwenden.

Bytesequenzen, die sich möglicherweise überlappen, können mit [`copy`][copy] kopiert werden. Bist du dir sicher, dass sich Quelle und Ziel nicht überlappen, funktioniert auch das (etwas schnellere) [`copy_nonoverlapping`][copy_nonoverlapping]. Beide Funktionen sind `unsafe`, da sie die Sicherheitsgarantien von Rust verletzen können. Sie sollten nur mit Vorsicht eingesetzt werden.

<h3><a href="#does-rust-work-without-the-standard-library" name="does-rust-work-without-the-standard-library">
Kann Rust auch ohne Standardbibliothek vernünftig funktionieren?
</a></h3>

Absolut. Rust-Programme können die Standardbibliothek mit dem `#![no_std]`-Attribut abwählen. Damit kann weiterhin die Rust-Core-Bibliothek verwendet werden, die nur die plattformunabhängigen Primitive der Standardbibliothek enthält. Damit enthält sie keine Funktionalität zu I/O, Nebenläufigkeit, Heap-Allokation oder ähnlichem.

<h3><a href="#can-i-write-an-operating-system-in-rust" name="can-i-write-an-operating-system-in-rust">
Ist es möglich, ein Betriebssystem in Rust schreiben?
</a></h3>

Ja! Es gibt tatsächlich bereits [mehrere Projekte, die genau dieses Ziel verfolgen](http://wiki.osdev.org/Rust).

<h3><a href="#how-can-i-write-endian-independent-values" name="how-can-i-write-endian-independent-values">
Wie kann ich numerische Datentypen wie <code>i32</code> oder <code>f64</code> in Little- oder Big-Endian-Kodierung von einem Stream lesen oder in einen Stream schreiben?
</a></h3>

Diese Funktionalität wird vom [byteorder-Crate](http://burntsushi.net/rustdoc/byteorder/) zur Verfügung gestellt.

<h3><a href="#does-rust-guarantee-data-layout" name="does-rust-guarantee-data-layout">
Garantiert Rust ein spezifisches Layout von Datenstrukturen?
</a></h3>

Standardmäßig nicht. Allgemein gesprochen ist das Layout von `enum`s und `struct`s undefiniert. Das erlaubt Compileroptimierungen wie das Wiederverwenden von Padding für Enum-Diskriminanten, Kombinieren von Varianten verschachtelter Enums und Umordnen von Struct-Feldern, um Padding zu verringern. C-ähnlichen Enums (solchen ohne Daten innerhalb der Varianten) kann eine definierte Repräsentation zugewiesen werden:

```rust
enum CLike {
    A,
    B = 32,
    C = 34,
    D
}
```

Solche Enums dürfen das Attribut `#[repr(C)]` tragen, um sie auf die Repräsentation festzulegen, die sie in äquivalentem C-Code hätten. Das erlaubt die Verwendung von Rust-Enums in FFI-Code, der nach C-Enums übersetzt. Das Attribut kann auch auf Structs angewendet werden, um ihnen das Layout des entsprechenden C-Structs zu verpassen.

<h2 id="cross-platform">Plattformübergreifende Programmierung</h2>

<!--
### How do I build a Windows binary that doesn't display the console window?

TODO: Write this answer.
-->

<!--
### How do I make the console-less binary not crash on panic!?

TODO: Write this answer.
-->

<h3><a href="#how-do-i-express-platform-specific-behavior" name="how-do-i-express-platform-specific-behavior">
Was ist der idiomatische Weg, plattformspezifisches Verhalten in Rust auszudrücken?
</a></h3>

Plattformspezifisches Verhalten kann mit [Attributen zur bedingten Übersetzung](https://doc.rust-lang.org/reference/attributes.html#conditional-compilation) wie `target_os`, `target_family` oder `target_endian` beschrieben werden.

<h3><a href="#can-rust-be-used-for-android-ios-programs" name="can-rust-be-used-for-android-ios-programs">
Ist Rust für die Android/iOS-Programmierung tauglich?
</a></h3>

Ja! Es gibt bereits einige Beispiele für funktionierende Rust-Programme unter [Android](https://github.com/tomaka/android-rs-glue) und [iOS](https://www.bignerdranch.com/blog/building-an-ios-app-in-rust-part-1/). Das Aufsetzen erfordert etwas Arbeit, Rust kommt mit beiden Plattformen aber wunderbar zurecht.

<h3><a href="#can-i-run-my-rust-program-in-a-web-browser" name="can-i-run-my-rust-program-in-a-web-browser">
Kann ich mein Rust-Programm in einem Webbrowser ausführen?
</a></h3>

Wahrscheinlich. Rust hat [experimentelle Unterstützung][wasm] für [asm.js] und [WebAssembly].

[wasm]: https://davidmcneil.gitbooks.io/the-rusty-web/
[asm.js]: http://asmjs.org/
[WebAssembly]: http://webassembly.org/

<h3><a href="#how-do-i-cross-compile-rust" name="how-do-i-cross-compile-rust">
Wie funktioniert Cross-Compilation in Rust?
</a></h3>

Rust kann Code für andere Systeme übersetzen, erfordert dafür aber [etwas Vorbereitsungsarbeit](https://github.com/japaric/rust-cross/blob/master/README.md). Jeder Rust-Compiler kann als Cross-Compiler arbeiten, Bibliotheken müssen aber zunächst für die Zielplattform übersetzt werden.

Rust vertreibt [Kopien der Standardbibliothek](https://static.rust-lang.org/dist/index.html) für jede unterstützte Plattform; sie liegen in den `rust-std-*`-Dateien der Distribution für die jeweilige Plattform. Es existiert jedoch noch kein Automatismus, um diese Bibliotheken für Cross-Compilation zu installieren.

<h2 id="modules-and-crates">Module und Crates</h2>

<h3><a href="#what-is-the-relationship-between-a-module-and-a-crate" name="what-is-the-relationship-between-a-module-and-a-crate">
Wie verhalten sich Crates und Module zueinander?
</a></h3>

- Ein Crate ist eine Übersetzungseinheit, also die kleinste Einheit, auf der der Rust-Compiler arbeiten kann.
- Ein Modul ist eine (möglicherweise verschachtelte) Organisationseinheit innerhalb eines Crates.
- Ein Crate enthält ein implizites, unbenanntes Modul auf Wurzelebene.
- Rekursive Definitionen können sich über mehrere Module erstrecken, nicht aber über mehrere Crates.

<h3><a href="#why-cant-the-rust-compiler-find-a-library-im-using" name="why-cant-the-rust-compiler-find-a-library-im-using">
Warum kann der Rust-Compiler die Bibliothek nicht finden, die ich eingebunden habe?
</a></h3>

Hier gibt es eine Reihe von Möglichkeiten, aber ein häufiger Grund ist, dass die `use`-Deklaration nicht relativ zur Crate-Wurzel angegeben wurde. Versuche deine Deklarationen so zu schreiben, dass sie den gleichen Pfad haben wie wenn sie in der Wurzeldatei deines Projekts importiert würden.

Es gibt außerdem noch die Schlüsselwörter `self` und `super`, die es erlauben, Pfade relativ zum aktuellen oder zum Elternmodul anzugeben.

Mehr Informationen findest du im Kapitel ["Crates and Modules"](https://doc.rust-lang.org/stable/book/crates-and-modules.html) des Rust Book.

<h3><a href="#why-do-i-have-to-declare-modules-with-mod" name="why-do-i-have-to-declare-modules-with-mod">
Warum muss ich Moduldateien mit <code>mod</code> im Crate deklarieren, anstatt sie einfach mit <code>use</code> einzubinden?
</a></h3>

Module können in Rust auf zwei Arten deklariert werden: Direkt im Code, oder in einer separaten Datei. Hier ein Beispiel beider Varianten:

```rust
// In main.rs
mod hello {
    pub fn f() {
        println!("Hallo!");
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
    println!("Hallo!");
}
```

Im ersten Beispiel wird das Modul in der Datei definiert, in der es auch benutzt wird. Im zweiten Beispiel weist die Moduldeklaration den Compiler darauf hin, die Definition aus `hello.rs` oder `hello/mod.rs` zu laden.

`mod` deklariert also ein neues Modul, wogegen sich `use` auf ein anderswo existierendes Modul bezieht und dessen Inhalte in den aktuellen Gültigkeitsbereich übernimmt.

<h3><a href="#how-do-i-configure-cargo-to-use-a-proxy" name="how-do-i-configure-cargo-to-use-a-proxy">
Wie kann ich Cargo dazu bringen, einen Proxy zu verwenden?
</a></h3>

Wie in der [Cargo-Dokumentation](http://doc.crates.io/config.html) (englisch) beschrieben, kann ein Proxyserver eingerichtet werden, indem die `proxy`-Variable in der `[http]`-Sektion der Konfigurationsdatei gesetzt wird.

<h3><a href="#why-cant-the-compile-find-method-implementations" name="why-cant-the-compile-find-method-implementations">
Warum kann der Compiler eine Implementierung nicht finden, obwohl ich das Modul des Typs bereits importiert habe?
</a></h3>

Für Methoden, die über einen Trait definiert werden, muss die Trait-Deklaration explizit mit importiert werden. Es genügt also nicht, das Modul zu importieren, in dem der Trait für den Struct implementiert wird, sondern der Trait selbst muss zusätzlich eingebunden werden.

<h3><a href="#why-cant-the-compiler-infer-use-statements" name="why-cant-the-compiler-infer-use-statements">
Warum kann der Compiler <code>use</code>-Deklarationen nicht einfach herleiten?
</a></h3>

Er könnte wahrscheinlich, aber das willst du wahrscheinlich gar nicht. In einfachen Fällen könnte der Compiler wohl das korrekte Modul finden, indem er nach passenden Deklarationen zu einem Bezeichner sucht, das klappt im Allgemeinen aber nicht. Jede Entscheidungsregel bei Namenskonflikten würde in einigen Fällen für Überraschung sorgen, und Rust zieht es vor die Herkunft von Symbolen explizit zu benennen.

So könnte der Compiler etwa festlegen, dass bei einem Namenskonflikt das erste importierte Modul Vorrang hat. Definieren also die beiden Module `foo` und `bar` jeweils den Bezeichner `baz`, `foo` ist aber das erste registrierte Modul, so würde der Compiler ein `use foo::baz;` einfügen.

```rust
mod foo;
mod bar;

// use foo::baz  // wird vom Compiler eingefügt

fn main() {
  baz();
}
```

Wenn du sicher weißt, dass das passieren wird, kann das einige wenige Tastenanschläge einsparen. Dieser Gewinn wird aber mit deutlich höheren Wahrscheinlichkeit an überraschenden Fehlermeldungen erkauft, falls mit `baz()` eigentlich `bar::baz` gemeint war. Auch reduziert diese Logik die Lesbarkeit des Codes, weil ein Funktionsaufruf plötzlich von der Importreihenfolge abhängt. Diesen Trade-Off wollen wir nicht eingehen.

Zukünftig könnten jedoch IDEs die Auflösung von Deklarationen erleichtern: Hilfestellung beim Finden des richtigen Imports, aber explizite Modulpfade im Code.

<!--
### How do I package and archive crates from [https://crates.io](https://crates.io)?

TODO: Write this answer.
-->

<h3><a href="#how-do-i-do-dynamic-rust-library-loading" name="how-do-i-do-dynamic-rust-library-loading">
Wie kann ich in Rust dynamische Bibliotheken laden?
</a></h3>

Dynamische Bibliotheken können mit [libloading](https://crates.io/crates/libloading) importiert werden, das ein plattformübergreifendes System für dynamisches Linken bereitstellt.

<h3><a href="#why-doesnt-crates-io-have-namespaces" name="why-doesnt-crates-io-have-namespaces">
Warum hat crates.io keine Namensräume?
</a></h3>

Übersetzung der [offiziellen Erklärung](https://internals.rust-lang.org/t/crates-io-package-policies/1041) zum Design von [https://crates.io](https://crates.io):

> Im ersten Monat nach der Veröffentlichung von crates.io wurden wir von mehreren Nutzern nach der Möglichkeit gefragt, [Pakete mit Namensräumen](https://github.com/rust-lang/crates.io/issues/58) einzuführen.<br><br>
>
> Auch wenn Namensräume es Autoren einfacher macht, generische Namen für Pakete zu wählen, erhöhen sie die Komplexität, mit der Crates im Rust-Code und in der Kommunikation zwischen Entwicklern referenziert werden. Auf den ersten Blick erlauben sie mehreren Entwicklern, Namen wie `http` zu wählen, das sorgt aber nur dafür, dass diese Pakete als `wycats' http` oder `reem's http` bekannt werden, was keine wirkliche Verbesserung gegenüber längeren Namen wie `wycats-http` oder `reem-http` bringt.<br><br>
>
> Es hat sich herausgestellt, dass Entwickler in Ökosystemen ohne Namensräume dazu tendieren, kreativere Namen (wie `nokogiri` statt `tenderlove's libxml2`) zu wählen. Diese Namen sind meist kurz und gut zu merken, gerade aufgrund der fehlenden Hierarchie. Das macht es einfacher, unmissverständlich über Pakete zu sprechen und erschafft spannende Markennamen. Auch haben wir den Erfolg von Umgebungen mit zigtausenden Paketen wie NPM und RubyGems gesehen, die wunderbar mit einem einheitlichen Namensraum zurechtkommen.<br><br>
>
> Kurz gesagt sind wir nicht der Meinung, dass das das Cargo-Ökosystem davon profitieren würde, wenn Piston einen Namen wie `bvssvni/game-engine` statt dem einfach zu merkenden `piston` bekommen hätte (mit der Möglichkeit, dass ein Anderer Entwickler `wycats/game-engine` veröffentlicht).
>
> Da Namensräume auf verschiedene Arten strikt komplexer ist und sie bei bedarf später hinzugefügt werden können, sollten sie irgendwann nötig werden, werden wir vorerst bei einem gemeinsamen Namensraum bleiben.<br><br>

<h2 id="libraries">Bibliotheken</h2>

<h3><a href="#how-can-i-make-an-http-request" name="how-can-i-make-an-http-request">
Wie kann ich eine HTTP-Anfrage absetzen?
</a></h3>

Die Standardbibliothek stellt keine HTTP-Implementierung zur Verfügung, deshalb musst du dafür einen externen Crate bemühen.
[reqwest](http://docs.rs/reqwest) ist einer der einfachsten.  Er setzt auf [hyper](https://github.com/hyperium/hyper) auf und ist in Rust geschrieben,
es gibt jedoch [eine Vielzahl an Alternativen](https://crates.io/keywords/http).
Der [curl](https://docs.rs/curl)-Crate ist beispielsweise weit verbreitet und bildet eine Anbindung an die curl-Bibliothek.

<h3><a href="#how-can-i-write-a-gui-application" name="how-can-i-write-a-gui-application">
Wie kann ich in Rust eine grafische Benutzeroberfläche schreiben?
</a></h3>

Es gibt eine große Auswahl von [GUI-Frameworks](https://github.com/kud1ing/awesome-rust#gui) für Rust.

<h3><a href="#how-can-i-parse-json-xml" name="how-can-i-parse-json-xml">
Wie kann ich JSON oder XML parsen?
</a></h3>

[Serde](https://github.com/serde-rs/serde) ist die empfohlene Bibliothek für (De-)Serialisierung von Rust-Datenstrukturen von und in eine Reihe von Datenformaten.

<h3><a href="#is-there-a-standard-2d-vector-crate" name="is-there-a-standard-2d-vector-crate">
Gibt es einen Standard-Crate für 2D-Vektorgrafik?
</a></h3>

Noch nicht! Lust einen zu bauen?

<h3><a href="#how-do-i-write-an-opengl-app" name="how-do-i-write-an-opengl-app">
Wie kann ich in Rust ein OpenGL-Programm schreiben?
</a></h3>

[Glium](https://github.com/tomaka/glium) ist die größte Bibliothek für OpenGL-Programmierung in Rust. [GLFW](https://github.com/bjz/glfw-rs) ist auch eine solide Option.

<h3><a href="#can-i-write-a-video-game-in-rust" name="can-i-write-a-video-game-in-rust">
Kann ich in Rust ein Computerspiel schreiben?
</a></h3>

Ja! Die wichtigste Bibliothek für Spieleprogrammierung in Rust ist [Piston](http://www.piston.rs/), und es gibt sowohl ein [Subreddit für Spieleprogrammierung in Rust](https://www.reddit.com/r/rust_gamedev/), als auch einen IRC-Kanal (`#rust-gamedev` im [Mozilla-IRC](https://wiki.mozilla.org/IRC)) zu diesem Thema.

<h2 id="design-patterns">Design Patterns</h2>

<h3><a href="#is-rust-object-oriented" name="is-rust-object-oriented">
Ist Rust objektorientiert?
</a></h3>

Rust ist eine Mehrparadigmensprache. Viele Konzepte aus OOP-Sprachen können nach Rust übernommen werden, aber nicht alle, und nicht immer mit dem gewohnten Abstraktionsmechanismus.

<h3><a href="#how-do-i-map-object-oriented-concepts-to-rust" name="how-do-i-map-object-oriented-concepts-to-rust">
Wie kann ich objektorientierte Konzepte in Rust abbilden?
</a></h3>

Kommt darauf an. Es _gibt_ Möglichkeiten, um objektorientierte Konzepte wie [Mehrfachvererbung](https://www.reddit.com/r/rust/comments/2sryuw/ideaquestion_about_multiple_inheritence/) nach Rust zu übersetzen, das Ergebnis kann aber aufgrund der nicht objektorientierten Natur von Rust deutlich von seinem Äquivalent in einer OOP-Sprache abweichen.

<h3><a href="#how-do-i-configure-a-struct-with-optional-parameters" name="how-do-i-configure-a-struct-with-optional-parameters">
Wie kann ich einen Struct mit optionalen Parametern konfigurieren?
</a></h3>

Die einfachste Möglichkeit ist es, den Typ [`Option`][Option] als Parameter für Funktionen wie `new` zu verwenden. Alternativ kannst du das [Builder Pattern](https://aturon.github.io/ownership/builders.html) implementieren, mit dem einzelne Attribute durch Funktionsaufrufe an einen Builder belegt werden, bevor der eigentliche Struct konstruiert wird.

<h3><a href="#how-do-i-do-global-variables" name="how-do-i-do-global-variables">
Wie benutze ich globale Variablen in Rust?
</a></h3>

Globale Konstanten werden mit dem Schlüsselwort `const` deklariert, globale Variablen mit `static`. Beachte, dass das Verändern einer `static mut`-Variablen `unsafe`-Code erfordert, da es Data Races erlaubt, deren Absenz von Safe Rust garantiert wird. Ein wichtiger Unterschied zwischen `const` und `static` ist, dass auf `static`-Variablen Referenzen gebildet werden können - `const`-Werte haben keine definierte Speicheradresse. Für weitere Informationen zum Thema `const` vs. `static` sei auf das [Buch](https://doc.rust-lang.org/book/const-and-static.html) verwiesen.

<h3><a href="#how-can-i-set-compile-time-constants-that-are-defined-procedurally" name="how-can-i-set-compile-time-constants-that-are-defined-procedurally">
Wie kann ich Compilezeit-Konstanten definieren, die prozedural berechnet werden?
</a></h3>

Rust hat momentan nur eingeschränkte Unterstützung für Compilezeit-Konstanten. Du kannst Werte eines primitiven Typs mittels `const`-Deklarationen definieren (ähnlich zu `static`, aber unveränderlich und ohne eine feste Speicheradresse). Funktionen können ebenfalls `const` sein.

Konstanten, die sich mit diesen Mechanismen nicht beschreiben lassen, können mit dem [`lazy-static`](https://github.com/rust-lang-nursery/lazy-static.rs)-Crate erzeugt werden, der Compilezeit-Berechnung durch Auswertung bei der ersten Benutzung einer globalen Variablen emuliert.

<h3><a href="#can-i-run-code-before-main" name="can-i-run-code-before-main">
Kann ich vor Betreten der `main`-Funktion Initialisierungscode ausführen?
</a></h3>

Rust kennt keine Programmausführung vor `main`. Am nächsten kommt dem Konzept der [`lazy-static`](https://github.com/Kimundi/lazy-static.rs)-Crate, der dieses Verhalten nachbildet, indem globale Variablen bei ihrer ersten Benutzung initialisiert werden.

<!--

This answer needs significant work. Let's revise after the initial posting. --aturon

<h3><a href="#why-doesnt-rust-have-inheritance" name="why-doesnt-rust-have-inheritance">
Warum hat Rust keine Vererbung?
</a></h3>

Es gibt für das Wort "Vererbung" (eng.: "Inheritance") zwei verschiedene Bedeutungen: _subtyping_ und _interface sharing_. Beide Zwecke werden durch Traits bereits erfüllt.

Traits erlauben es, Subtypen zu definieren, was Polymorphismus ermöglicht.

Interface sharing wird durch Trait-Methoden ermöglicht. Diese definieren eine Menge von zusammengehörigen Funktionen, welche für jede Implementierung dieses Traits vorhanden sein müssen.

Rust hat stets daran gearbeitet, keine Features zu haben, deren Zwecke sich überlappen - dadurch bleiben Features orthogonal. Aus diesem Grund, und weil die nützlichen Aspekte von Vererbung durch Traits abgedeckt werden, hat man sich entschieden, Vererbung in Rust nicht zu unterstützen.

-->

<h3><a href="#does-rust-allow-non-constant-expression-values-for-globals" name="does-rust-allow-non-constant-expression-values-for-globals">
Erlaubt Rust Werte für globale Variablen, die keine Compilezeit-Konstanten sind?
</a></h3>

Nein. Globale Variablen können keine nicht-compilezeit-konstanten Konstruktoren und überhaupt keine Destruktoren besitzen. Statische Konstruktoren sind unerwünscht, da bei gegenseitigen Abhängigkeiten eine Initialisierungsreihenfolge nicht ohne weiteres garantiert werden kann. Codeausführung vor Beginn der `main`-Funktion wird weithin als Misfeature gesehen, weswegen es in Rust nicht umgesetzt ist.

Das [C++ FQA](http://yosefk.com/c++fqa/ctors.html#fqa-10.12) hat einen Eintrag zum "static initialization order fiasco", und [Eric Lipperts Blog](https://ericlippert.com/2013/02/06/static-constructors-part-one/) schreibt über die Herausforderungen, die das Feature in C# mit sich gebracht hat.

Nicht-konstante Initialisierer in globalen Variablen können mit dem [lazy-static](https://crates.io/crates/lazy_static/) nachgebildet werden.

<h2 id="other-languages">Andere Sprachen</h2>

<h3><a href="#how-can-i-use-static-fields" name="how-can-i-use-static-fields">
Wie kann ich ein C-Konstrukt wie <code>struct X { static int X; }</code> in Rust implementieren?
</a></h3>

Rust kennt keine `static`-Attribute wie im obigen Programmausschnitt. Stattdessen kannst du eine `static`-Variable auf Modulebene deklarieren, die nur im umgebenden Modul sichtbar ist.

<h3><a href="#how-can-i-convert-a-c-style-enum-to-an-integer" name="how-can-i-convert-a-c-style-enum-to-an-integer">
Wie kann ich einen C-ähnlichen Enum in einen Integer konvertieren und umgekehrt?
</a></h3>

Ein Enum kann mit einem `as`-Cast in einen Integer überführt werden, wie etwa `e as i64` (wobei `e` ein Enum ist).

Die Gegenrichtung kann mit einem `match`-Statement erreicht werden, das Zahlenwerte auf Enum-Werte abbildet.

<h3><a href="#why-do-rust-programs-have-larger-binary-sizes-than-C-programs" name="why-do-rust-programs-have-larger-binary-sizes-than-C-programs">
Warum erzeugen Rust-Programme größere Binaries als C?
</a></h3>

Mehrere Faktoren tragen dazu bei, dass Rust-Programme standardmäßig größere Binaries erzeugen als funktionell äquivalente C-Programme. Grundsätzlich optimiert Rust die Performance komplexer Anwendungen, nicht die Größe kleiner Beispielprogramme.

__Monomorphisierung__

Rust monomorphisiert seine Generics, was bedeutet, dass eine generische Funktion oder ein generischer Typ für jeden konkreten Typ, mit dem er instanziiert wird, neu übersetzt wird. Das ist ähnlich zum Verhalten von Templates in C++. Zum Beispiel werden im folgenden Programm

```rust
fn foo<T>(t: T) {
    // ... tu irgendwas
}

fn main() {
    foo(10);       // i32
    foo("hello");  // &str
}
```

zwei unabhängige Versionen von `foo` im fertigen Programm auftauchen, jeweils für `i32` und `&str` spezialisiert. Das erlaubt effiziente frühe Bindung der generischen Funktion, sorgt aber für größere Binaries.

__Debug-Symbole__

Rust-Programme beinhalten auch im Release-Modus einige Debug-Symbole. Diese ermöglichen Backtraces bei Panics und können mit `strip` oder einem ähnlichen Tool aus dem Binary entfernt werden. Der Release-Modus in Cargo ist zum Optimierungslevel 3 in rustc äquivalent. [Mittlerweile](https://github.com/rust-lang/rust/pull/32386) kennt Rust eine alternative Optimierungsstrategie (Level `s` oder `z`),  mit der der Compiler versucht, die Größe statt der Performance des Programms zu optimieren.

__Jemalloc__

Rust benutzt standardmäßig jemalloc als Allokator, was die erzeugten Binaries etwas vergrößert. Jemalloc bietet einen konsistentere Allokation mit günstigerer Performance als viele systemeigenen Allokatoren. In Zukunft wird es auch einfacher sein, [benutzerdefinierte Allokatoren](https://github.com/rust-lang/rust/issues/32838) in Rust einzubinden.

__Link-Time Optimization__

Rust benutzt standardmäßig keine Link-Time-Optimization, kann aber so eingerichtet werden. Das erhöht das Optimierungspotential des Rust-Compilers und kann einen kleinen Effekt auf die Größe der Binaries haben. Dieser Effekt wird vor allem in Kombination mit den oben genannten Optimierungsstrategien sichtbar.

__Standardbibliothek__

Die Standardbibliothek von Rust beinhaltet libbacktrace und libunwind, die in manchen Programmen unerwünscht sein können. Mit dem Crate-Attribut `#![no_std]` können ohne diese Bibliotheken kleinere Binaries erzeugt werden, die aber üblicherweise wesentliche Änderungen am Code nach sich ziehen. Rust-Code, der ohne die Standardbibliothek geschrieben wurde, ist funktionell äquivalentem C-Code ähnlich.

Als Beispiel liest das folgende C-Programm einen Namen ein und begrüßt den Benutzer mit diesem Namen.

```c
#include <stdio.h>

int main(void) {
    printf("Wie heißt du?\n");
    char input[100] = {0};
    scanf("%s", input);
    printf("Hallo %s!\n", input);
    return 0;
}
```

In Rust würde man dieses Programm womöglich wie folgt schreiben:

```rust
use std::io;

fn main() {
    println!("Wie heißt du?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("Hallo {}!", input);
}
```

Dieses Programm wird übersetzt ein größeres Binary produzieren und mehr Speicher in Anspruch nehmen als das C-Programm. Es ist aber nicht wirklich gleichwertig zum obigen C-Code. Das äquivalente Rust-Programm würde eher so aussehen:

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
        printf(b"Wie heißt du?\n\0".as_ptr());
        let mut input = [0u8; 100];
        scanf(b"%s\0".as_ptr(), &mut input);
        printf(b"Hallo %s!\n\0".as_ptr(), &input);
        0
    }
}

#[lang="eh_personality"] extern fn eh_personality() {}
#[lang="panic_fmt"] fn panic_fmt() -> ! { loop {} }
#[lang="stack_exhausted"] extern fn stack_exhausted() {}
```

Dieser Code sollte in seinem Speicherverbrauch grob der C-Version entsprechen. Diese Reduktion wird mit zusätzlicher Komplexität und dem Fehlen statischer Garantien erkauft, die Rust üblicherweise gewährt (und die hier mit dem Schlüsselwort `unsafe` umgangen wurden).

<h3><a href="#why-no-stable-abi" name="why-no-stable-abi">
Warum hat Rust keine stabile ABI wie C, und warum muss ich Symbole mit extern annotieren?
</a></h3>

Das Festlegen einer ABI ist eine große Entscheidung, die zukünftige Änderungen an der Sprache behindern könnte. Da Rust erst im Mai 2015 Version 1.0 erreicht hat, ist es noch zu früh, sich an dieser Stelle festzulegen. Das bedeutet nicht, dass es nie eine stabile ABI geben wird (auch wenn C++ es viele Jahre ohne Spezifikation einer ABI geschafft hat).

Über das Rust-Schlüsselwort `extern` kann mit spezifischen ABIs wie der C-ABI interagiert werden.

<h3><a href="#can-rust-code-call-c-code" name="can-rust-code-call-c-code">
Kann Rust-Code C-Code aufrufen?
</a></h3>

Ja. Rust wurde so entworfen, dass C-Code genauso effizient aufgerufen kann wie aus C++.

<h3><a href="#can-c-code-call-rust-code" name="can-c-code-call-rust-code">
Kann C-Code Rust-Code Aufrufen?
</a></h3>

Ja. Der Rust-Code muss mit einer `extern`-Deklaration versehen werden, die ihn C-ABI-kompatibel macht. Eine solche Funktion kann als Funktionszeiger an C übergeben werden oder sogar direkt aus C aufgerufen werden, wenn sie mit dem `#[no_mangle]`-Attribut erhält, um Symbol-Mangling zu unterdrücken.

<h3><a href="#why-rust-vs-cxx" name="why-rust-vs-cxx">
Ich kann bereits perfektes C++ schreiben. Welche Vorteile bietet mir Rust?
</a></h3>

Modernes C++ implementiert viele Features, die das Schreiben sicheren und korrekten Codes weniger fehleranfällig macht. Es ist jedoch immer noch sehr einfach, Speicherfehler zu verursachen. Die C++-Hauptentwickler arbeiten daran die Prävalenz dieser Problematik zu verringern, die Sprache lässt aber durch ihre lange Geschichte und die notwendige Rückwärtskompatibilität nur eingeschränkt Änderungen zu.

Rust wurde vom ersten Tag an mit dem Ziel entworfen, eine sichere Systemprogrammiersprache zu sein. Sie ist damit nicht von historischen Entscheidungen belastet, die das Entwickeln sicheren Codes in C++ so kompliziert machen. In C++ wird Sicherheit durch strenge Selbstdisziplin erreicht und kann leicht verletzt werden. In Rust ist Sicherheit die Vorgabe. Die Sprache eröffnet so die Möglichkeit, mit weniger erfahrenen Entwicklern zusammenzuarbeiten, ohne den Code wieder und wieder auf Sicherheitslücken prüfen zu müssen.

<h3><a href="#how-to-get-cxx-style-template-specialization" name="how-to-get-cxx-style-template-specialization">
Wie lässt sich die Templatespezialisierung aus C++ in Rust umsetzen?
</a></h3>

Rust hat zur Zeit noch kein Äquivalent zu Templatespezialisierung, daran [wird jedoch gearbeitet](https://github.com/rust-lang/rfcs/pull/1210). Ähnliche Effekte können aber mittels [Assoziierter Typen](https://doc.rust-lang.org/stable/book/associated-types.html) erzielt werden.

<h3><a href="#how-does-ownership-relate-to-cxx-move-semantics" name="how-does-ownership-relate-to-cxx-move-semantics">
Was hat das Ownership-System von Rust mit Move Semantics aus C++ zu tun?
</a></h3>

Die zugrundeliegenden Konzepte sind ähnlich, in der Praxis funktionieren die beiden Systeme jedoch grundsätzlich verschieden. In beiden Fällen fungiert das Verschieben (Move) eines Werts als Möglichkeit, den Besitz zugrundeliegender Ressourcen zu übertragen. So überträgt der Move eines Strings etwa den Stringpuffer, anstatt ihn zu kopieren.

In Rust ist Ownership Transfer das Standardverhalten. So wird eine Funktion, die einen `String` als Argument nimmt, den Besitz am übergebenen String-Wert übernehmen:

```rust
fn process(s: String) { }

fn caller() {
    let s = String::from("Hello, world!");
    process(s); // Überträgt den Besitz von `s` an `process`
    process(s); // Fehler: `caller` besitzt `s` an dieser Stelle nicht mehr
}
```

Wie im Codebeispiel oben zu sehen, überträgt der erste Aufruf an `process` den Besitz an der Variablen `s`. Der Compiler führt über den Besitz von Werten Buch, sodass der zweite Aufruf an `process` einen Fehler zur Folge hat - ein gültiges Programm darf den Besitz eines Werts nicht zweimal aufgeben. Rust wird das Verschieben eines Werts auch verhindern, solange noch eine aktive Referenz darauf existiert.

C++ verfolgt einen anderen Ansatz. In C++ werden Werte per Vorgabe kopiert, in dem ihr Kopierkonstruktor aufgerufen wird. Es ist allerdings möglich, Funktionen zu deklarieren, die ihre Argumente als "rvalue-Referenz", wie etwa `string&&` übernehmen. Dies deutet darauf hin, dass die aufgerufene Funktion den Besitz an Ressourcen des Werts übernimmt. Der Aufrufer muss dazu entweder einen temporären Wert übergeben oder einen gebundenen Wert explizit mit `std::move` verschieben. Das obige Beispiel würde in C++ etwa wie folgt aussehen:

```cpp
void process(string&& s) { }

void caller() {
    string s("Hello, world!");
    process(std::move(s));
    process(std::move(s));
}
```

C++-Compiler müssen über Ownership nicht Buch führen, sodass der obige Code ohne Warnungen oder Fehler übersetzt. In C++ bleibt der Besitz der Variablen `s` weiterhin bei `caller` (nicht aber der interne Puffer des Strings), sodass trotzdem der Destruktor von `s` ausgeführt wird, sobald `caller` zurückkehrt. In Rust wird `drop` dagegen nur durch den neuen Besitzer des Werts aufgerufen.

<h3><a href="#how-to-interoperate-with-cxx" name="how-to-interoperate-with-cxx">
Wie kann ich aus Rust C++-Funktionen aufrufen und umgekehrt?
</a></h3>

Rust und C++ können C als gemeinsame Schnittstelle benutzen. Sowohl Rust als auch C++ besitzen ein [Foreign Function Interface](https://doc.rust-lang.org/book/ffi.html) nach C über das sie miteinander kommunizieren können. Falls das Schreiben von C-Bindings zu langwierig wird, kannst du jederzeit auf [rust-bindgen](https://github.com/servo/rust-bindgen) zurückgreifen, ein Tool, das dir hilft automatisch funktionierende C-Bindings für Rust zu bauen.

<h3><a href="#does-rust-have-cxx-style-constructors" name="does-rust-have-cxx-style-constructors">
Hat Rust Konstruktoren, wie sie aus C++ bekannt sind?
</a></h3>

Nein. Funktionen erfüllen den gleichen Zweck wie Konstruktoren, ohne die Sprachkomplexität zu erhöhen. Der übliche Name für eine Konstrukorfunktion in Rust ist `new()`, dabei handelt es sich aber lediglich um eine Namenskonvention. `new()` ist tatsächlich eine Funktion wie jede andere. Hier ein Beispiel:

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
Hat Rust Kopierkonstruktoren?
</a></h3>

Nicht direkt. Typen, die den `Copy`-Trait implementieren, führen eine C-artige, "flache Kopie" ohne zusätzliche Arbeit oder Funktionsaufrufe durch. Das entspricht dem Konzept der *trivially copyable types* aus C++. Es ist nicht möglich, `Copy` mit abweichendem Verhalten zu implementieren. Dazu dient der `Clone`-Trait, dessen Funktionalität durch einen expliziten Aufruf an die `clone`-Methode angesprochen wird. Die benutzerdefininerte Kopieroperation wird absichtlich explizit gehalten um die zugrundeliegende Komplexität und potentiell teure Operationen sichtbar zu machen.

<h3><a href="#does-rust-have-move-constructors" name="does-rust-have-move-constructors">
Hat Rust Move-Konstruktoren?
</a></h3>

Nein. Die Werte aller Typen werden Byte für Byte via `memcpy` verschoben. Das macht das Schreiben generischen `unsafe`-Codes deutlich einfacher, da das Zuweisen, Übergeben und Zurückgeben eines Werts nie Seiteneffekte wie Unwinding zur Folge haben kann.

<h3><a href="#compare-go-and-rust" name="compare-go-and-rust">
Was haben Go und Rust gemeinsam, und wo unterscheiden sich die Sprachen?
</a></h3>

Rust und Go haben grundsätzlich verschiedene Designziele. Alle Unterschiede aufzuzählen wäre zu umfangreich, im Folgenden werden einige der wichtigsten genannt.

- Rust arbeitet auf einer niedrigeren Ebene als Go. So setzt Rust im Gegensatz zu Go beispielsweise keinen Garbage Collector voraus. Allgemein bietet Rust ähnlich viel Kontrolle über das Verhalten eines Programms wie C oder C++.
- Der Fokus von Rust liegt darin, Sicherheit und Effizienz zu gewährleisten, gleichzeitig aber für High-Level-Abstraktionen zugänglich zu sein. Go konzentriert sich darauf, eine kleine, einfache Sprache zu sein, die schnell kompiliert und gut mit einer großen Auswahl an Tools zusammenzuarbeiten.
- Rust hat im Gegensatz zu Go gute Unterstützung für Generics.
- Rust ist stark von der Welt der Funktionalprogrammierung beeinflusst, was sich etwa im Typsystem äußert, das die Typklassen aus Haskell in der Form von Traits übernimmt. Go hat ein einfacheres Typsystem, das mit Interfaces einfache generische Programmierung ermöglicht.

<h3><a href="#how-do-rust-traits-compare-to-haskell-typeclasses" name="how-do-rust-traits-compare-to-haskell-typeclasses">
Wie lassen sich Traits in Rust mit den Typklassen von Haskell vergleichen?
</a></h3>

Rust-Traits sind ähnlich, aber weniger mächtig als Haskell-Typklassen, da Rust keine typen höherer Ordnung unterstützt. Assoziierte Typen in Rust entsprechen Typfamilien in Haskell.

Einige spezifische Unterschiede zwischen Haskell und Rust:

- Rust-Traits haben einen impliziten ersten Parameter `Self`. `trait Bar` in Rust entspricht damit `class Bar self` in Haskell, und `trait Bar<Foo>` entspricht `class Bar foo self`.
- "Supertraits" oder "Superklassen-Constraints" werden in Rust `trait Sub: Super` geschrieben, in Haskell `class Super self => Sub self`.
- Rust erlaubt keine verwaisten Instanzen (orphan instances), d.h. Trait-Implementierungen, die weder im Modul des Traits noch im Modul des Typs liegen. Haskell erlaubt dies, was zu abweichenden Kohärenzregeln führt.
- Die Auflösung von `impl`-Blöcken berücksichtigt die relevanten `where`-Klauseln, um zwischen `impl`-Instanzen zu wählen oder Überlappungen festzustellen. Haskell berücksichtigt dabei nur die Bedingungen der `instance`-Deklaration und vernachlässigt Einschränkungen, die an anderen Stellen angegeben wurden.
- Eine Teilmenge der Traits in Rust ([Object Safe Traits](https://github.com/rust-lang/rfcs/blob/master/text/0255-object-safety.md)) können für späte Bindung mittels Traitobjekten benutzt werden. Das selbe ist in Haskell mit dem `ExistentialQuantification`-Feature von GHC verfügbar.

<h2 id="documentation">Dokumentation</h2>

<h3><a href="#why-are-so-many-rust-answers-on-stackoverflow-wrong" name="why-are-so-many-rust-answers-on-stackoverflow-wrong">
Warum gibt es so viele falsche Antworten zu Rust auf Stack Overflow?
</a></h3>

Rust hatte bis zur Veröffentlichung von Version 1.0 im Mai 2015 eine lange Entwicklungsgeschichte hinter sich. Währenddessen hat sich die Sprache signifikant geändert, und einige Antworten auf Stack Overflow beziehen sich auf Sprach- und Bibliothekskonstrukte, die sich mittlerweile geändert haben.

Mit der Zeit werden immer mehr Antworten für die aktuelle Version von Rust entstehen, sodass der Anteil der veralteten Antworten abnehmen wird.

<h3><a href="#where-do-i-report-issues-in-the-rust-documentation" name="where-do-i-report-issues-in-the-rust-documentation">
Wie kann ich die Entwickler auf Fehler in der Rust-Dokumentation hinweisen?
</a></h3>

Probleme in der Dokumentation kannst du im Rust-Compiler [Issue Tracker](https://github.com/rust-lang/rust/issues) melden. Lies vorab unsere [Richtlinien](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#writing-documentation), um effektiv mitwirken zu können.

<h3><a href="#how-do-i-view-rustdoc-documentation-for-a-library-my-project-depends-on" name="how-do-i-view-rustdoc-documentation-for-a-library-my-project-depends-on">
Wo finde ich die rustdoc-Dokumentation für eine Bibliothek, von der mein Projekt abhängt?
</a></h3>

Wenn du `cargo doc` aufrufst, um Dokumentation für dein Projekt zu generieren, wird automatisch auch die Dokumentation für aktiven Versionen aller Abhängigkeiten erzeugt. Diese landet im Unterverzeichnis `target/doc` deines Projekts. Um die Dokumentation nach der Erstellung zu öffnen, kannst du `cargo doc --open` benutzen oder einfach direkt `target/doc/index.html` in deinem Browser öffnen.

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
