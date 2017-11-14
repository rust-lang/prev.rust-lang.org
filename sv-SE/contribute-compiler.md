---
layout: sv-SE/default
title: Att bidra till rust &mdash; språket, kompilatorn och standardbiblioteket &middot; Programmeringsspråket rust
---

# Att bidra till rust &mdash; språket, kompilatorn och standardbiblioteket

Källkoden för kompilatorn och standardbiblioteket är i huvud-repot och
deras underhåll är det primära målet för det repot. Många etiketter på
issue trackern är relaterade till det. Några av de mer fruktbara etiketterna
inbegriper [A-codegen], för översättning av rust-kod till LLVM IR;
[A-debuginfo], för generation av metadata använt av debuggers;
[A-diagnostics], feedbacken som kompilatorn ger vid fel;
[A-libs], problem i standardbiblioteket;
[A-macros] och [A-syntaxext], båda relaterade till syntax extensions
("syntax-utökningar"); och [A-typesystem], som handlar om typsystemet.

Det finns ingen välunderhållen guide till kompilatorns arkitektur men
[det finns en liten översikt i repot][rustc-guide].
[API-dokumentationen för de crates som utgör kompilatorn][internals-docs] kan 
hjälpa till med att navigera i koden, likväl som källkods-browsern [Rust DXR].
[Guiden till rusts testsvit][testsuite] kan lära dig hur du kan använda rusts
byggsystem effektivt likväl som att köra [`make tips`][tips] i terminalen.

Under överskådlig framtid är en av de initiativen i utvecklingen av
rust-kompilatorn att konvertera dess interna mekanismer från att
arbete direkt på det abstrakta syntax trädet (AST) till att arbeta med
och på [en mellanliggande representation vid namn MIR][mir].
Detta arbete förväntas uppna upp för många nya möjligheter genom att
förenkla kompilatorn, och hjälp behövs för att t.ex skapa ett MIR-baserat
översättningssteg, lägga till MIR-baserade optimeringar och implementera
inkrementell kompilering. Det finns tillsvidare ingen enskild resurs för
information om arbetet som krävs här, men fråga på [internals.rust-lang.org]
eller [#rust-internals] för att bli guidad.

[Det är pinsamt när vår kompilator kraschar][ice] &mdash; den
fruktade 'internt-kompilator-fel' (ICE). Etiketten [I-ICE]
håller koll på dessa och det finns ofta många sådana märkta problem.
Dessa är oftast bra buggar att börja med eftersom det är lätt att veta
när du har fixat dom och de är ofta relativt fristående.

Rustkods prestanda är en av dess stora fördelar; och prestandan på rustkompilatorn
är en av dess stora svagheter. Förbättringar av körsningstid eller speciellt
prestandan på kompileringstiden är mycket uppskattade. Etiketterna [I-slow] och
[A-optimization] är för körningstidsprestanda, och [I-compiletime] för
kompileringstid. Vi har en [sida som håller koll på kompileringstidsprestanda][rustc-perf]
på ett antal projekt. Kompilatorflaggan `-Z time-passes` kan hjälpa till att
debugga kompilatorprestanda, och rustkod kan profilas med standard-profilers såsom
`perf` på Linux.

Större ny funktionalitet genomgår en [Förfrågningar för kommentarer (RFC)][rfc]
process, igenom vilken en design för funktionaliteten bestäms. Denna process är
öppen för alla. Det är en social process utvecklare emellan som redan har olika
mängd erfarenhet av att arbeta tillsammas och det rekommenderas att bli involverad
sakta &mdash; att bidra en snabbskriven RFC utan att förstå ämnets historiska,
tekniska eller sociala kontext är ett lätt sätt att göra ett dåligt intryck och
att bli besviken på resultatet. Läs den tidigare nämnda readme-filen för att bättre
förstå hur det hela går till. Många idéer har debatterats under rusts historia.
Vissa har avvisats, några har skjutits upp på framtiden och [issue trackern][rfc-issues]
för RFCs katalogiserar vissa önskemål som ännu inte blivit en del av språket.
Strax innan en RFC blir accepterad för implementation går den in i en
'sista period för kommentarer'-fas, som indikeras av etiketten
[final-comment-period på rust-lang/rfcs repot][rfc-fcp]. Likaledes går ny
funktionalitet igenom en
[final-comment-period i rust-lang/rust repot][issue-fcp] innan den blir
aktiverad och användbar i den stabila kompilatorn (kallas för 'ungating').
Båda FCPs är kritiska stunder att bli involverad och uttrycka åsikter om
språkets utvecklingsriktning. FCPs påannonsers i de veckovisa sub-team
rapporterna på [internals.rust-lang.org].

Träffa andra rust-kompilator ingenjörer i [#rustc], språkdesigners i
[#rust-lang], och bibliotekdesigners i [#rust-libs].

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
