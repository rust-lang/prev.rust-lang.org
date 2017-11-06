---
layout: sv-SE/default
title: Att bidra till rust &mdash; dokumentation &middot; Programmeringsspråket Rust
---

# Att bidra till rust &mdash; dokumentation

Dokumentation är aldrig bra nog, och det finns aldrig tillräckligt av det.
Många aspekter av rusts dokumentaton kräver ej djup kunskap för att förbättra.
Att skriva, granska, och redigera dokumentation är bra sätt att lära sig rust på.
Därutöver är förbättringar på dokumentationen lättidentifierbara och obegränsade.
Gillar du inte hur något läses? Har du hittat någon odokumenterad information?
Dina pull requests kommer bli glatt välkomnade.

***Den dokumentation du kan skriva som hjälper ekosystemet mest är
[för crates som utgör rusts "ekosystem"][crate_docs]***. Medan standardbibliotekets
dokumentation är relativt utförlig är detsamma ej sant för
[många populära crates och verktyg][awesome-rust] som rustprogrammerare
använder varje dag. Att bidra med API-dokumentation till ett populärt rustprojekt
kommer ge dig evig uppskattning från de som underhåller projektet.

[Boken (om rust)][The Book] är den primära källan för dokumentation om rust,
den underhålls i huvudrepot. Den har sig egna problem-etikett, [A-book] och
förbättras ständigt. Annan dokumentation i huvudrepot inbegriper
[rusts referenshandbok][The Rust Reference],
[standardbibliotekets API-dokumentation][std],
[den sk. Rustonomicon][The Rustonomicon] (en guide till att använda `unsafe`
på ett korrekt sätt; begreppet härör ur kombinationen "rust" och "necronomicon").
[Rusts stilguide][Rust Style Guidelines] är så ofullständig att den inte länkas
till ofta; den som är ambitiös kan bidra mycket där. [Indexet över fel][err]
tillhandahåller utförliga beskrivningar av diverse felmeddelanden som kompilatorn
kan ge. Allteftersom nya felmeddelanden läggs till i kompilatorn
[måste dokumentationen underhållas][err-issue]. Därför finns det alltid
felmeddelanden som inte beskrivs i indexet att lägga till och beskriva.
Den större delen av dokumentationen i huvudrepot finns i mappen [src/doc].
I issue-trackern märks problem relaterade till denna dokumentation med etiketten
[A-docs]. Till sist är detta dokument och andra webbsidematerial underhållna i
[rustwebbsidans gitrepo][Rust website Git repository]. För att bidra till den
behöver du bara redigera och skicka in en pull request.

En hel del viktig rustdokumentation finns ej i huvudrepot eller underhålls inte
av projektet men är fortfarande kritiskt viktig för rusts framgång.
Exempel på utmärkt rustdokumentation som utvecklas aktivt och behöver folk som
bidrar är [Rust By Example], [Rust Design Patterns], och [rust-rosetta].
För annan existerande dokumentationsprojekt att bidra till, se [rust-learning].

Träffa annat folk som dokumenterar rust i [#rust-docs].

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
