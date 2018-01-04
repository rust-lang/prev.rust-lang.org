---
layout: sv-SE/default
title:  Dokumentation om rust &middot; Programmeringsspråket Rust
---

# Dokumentation om rust

Om du hitintills inte sett rust alls så är det första du borde läsa
introduktionen till boken [Programmeringsspråket Rust][book]. Den kommer ge dig
en god uppfattning om vad rust är, hur du installerar det, samt förklara språkets
syntax och koncept. När du har läst klart boken kommer du vara en novis
rustutvecklare och kommer ha ett bra grepp om de idéer som ligger till
grund för rust.

## Att lära sig rust

[Programmeringsspråket Rust][book]. Boken "Programmeringsspråket Rust", även känd
som "Boken" kommer introducera huvudämnena som är viktiga för att lära sig rust
och ta dig till den punkt då du kan vara produktiv. Boken är språkets primära
officiella dokument.

[Rust by Example][rbe]. En samling av fristående exempel skrivna i rust
om en rad olika ämnen som är körbara i webbläsaren.

[Vanliga frågor][faq].

[Den sk. Rustonomicon][nomicon]. En hel bok ägnad åt att förklara hur
man skriver osäker (`unsafe`) rustkod. Den är till för rustprogrammerare på
en avancerad nivå.

[rust-learning]. Ett kollektiv av resurser för att lära sig rust som underhålls
av communityt.

[book]: https://doc.rust-lang.org/book/
[rbe]: http://rustbyexample.com
[faq]: faq.html
[nomicon]: https://doc.rust-lang.org/nomicon/
[rust-learning]: https://github.com/ctjhoa/rust-learning

## Referenser

[Standardbibliotekets API-referenshandbok][api]. Dokumentation för
standardbiblioteket.

[docs.rs]. Dokumentation för alla crates publicerade på [crates.io].

[Rusts referenshandbok][ref]. Även om rust inte har en formell specifikation
försöker referensen att beskriva hur rust fungerar i detalj. Den brukar
vara föråldrad.

[Indexet över syntax][syn]. Detta appendix från Boken innehåller diverse exempel
för all syntax i rust korsrefererad med det kapitel i boken som beskriver exemplet.

[Guiden till Cargo][cargo]. Dokumentation för cargo, rusts pakethanterare.

[Index över kompilatorns felmeddelande][err]. Utförliga beskrivningar av
felmeddelande som kompilatorn genererar.

[Release Notes][release_notes]. En lista på förändringar som skett under varje release.

[Plattformsstöd][platform_support]. Lista av plattformar i varje tier ("stödnivå").

[api]: https://doc.rust-lang.org/std/
[syn]: https://doc.rust-lang.org/book/syntax-index.html
[ref]: https://doc.rust-lang.org/reference
[cargo]: http://doc.crates.io/guide.html
[err]: https://doc.rust-lang.org/error-index.html
[release_notes]: https://github.com/rust-lang/rust/blob/stable/RELEASES.md
[docs.rs]: https://docs.rs
[crates.io]: https://crates.io
[platform_support]: https://forge.rust-lang.org/platform-support.html

## Projektets policyer

[Rusts säkerhetspolicy][security]. Projektets policyer för att rapportera,
fixa och avslöja säkerhetsrelaterade buggar.

[Rusts policy kring upphovsrätt och varumärke][legal]. Rusts upphovsrätter
ägs av rustprojektets utvecklare ("The Rust Project Developers"), och dess
varumärken ägs av Mozilla. Lämplig användning av Rusts varumärken beskrivs här.

[Uppförandekod][coc]. Gäller för organisationen rust-lang på GitHub,
de officiella forumen, IRC-kanalerna och olika delar av rust-världen.

[security]: security.html
[legal]: legal.html
[coc]: https://www.rust-lang.org/sv-SE/conduct.html

## Nattlig och beta-dokumentation

Mycket av den officiella dokumentationen om rust är också tillgänglig för
[nattliga][nightly] och [beta-][beta]releaser utöver den dokumentationen
för den stabila versionen av rust länkad ovan.

[nightly]: https://doc.rust-lang.org/nightly/
[beta]: https://doc.rust-lang.org/beta/

## Icke-engelska resurser

För resurser på andra språk än engelska, se de
[språkspecifika länkarna i rust-learning][locale].

[locale]: https://github.com/ctjhoa/rust-learning#locale-links
