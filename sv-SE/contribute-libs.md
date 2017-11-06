---
layout: sv-SE/default
title: Att bidra till rust &mdash; bibliotek &middot; Programmeringsspråket Rust
---

# Att bidra till rust &mdash; bibliotek

Om du vill bidra till rust genom att skriva volymer av rustkod så är 
bibliotek där du kan göra det. Eftersom rust är ett nytt språk finns
det många typer av bibliotek som antingen inte finns än eller är
ofullständiga och är i behov av förbättring eller andra konkurrens.

Att bestämma vad du ska skriva som kommer ha effekt på ekosystemet och vara
kul är en vanlig svårighet. Här är ett antal idéer som kan hjälpa:

* Läs och delta i de veckovisa "vad jobbar alla med denna veckan"-trådar
  ("what's everyone working on this week") på subredditen [/r/rust] och
  [users.rust-lang.org]. Dessa är fyllda med spännande nyheter från andra
  rustprogrammerare som är i behov av medarbetare.
* Bekanta dig med de bästa rust-biblioteken via [awesome-rust] och [libs.rs].
* Vissa större projekt, inklusive webbläsaren [Servo], http-biblioteket [hyper],
  källkodsformateraren [rustfmt], och unix-bindnings-biblioteket [nix], och
  kodförbättringstipsaren [clippy] märker problem med 'easy' etiketter för nya
  medarbetare.
* Bli involverad med en av de mest aktiva rust-orienterade GitHub organisationerna
  såsom: [PistonDevelopers], [servo], [redox-os], [iron], [contain-rs] och [hyperium].
  Det är ofta enklare att hitta ett ställe att passa in i i dessa del-communities.
  De är också i större behov av hjälp än rust-lang organisationen självt och de är
  fyllda med erfarna rustutvecklare som kan vägleda dig.
* Hjälp till att guida bibliotek från [rust-lang-nursery] till huvudorganisationen rust-lang.
  Dessvärre finns det inte mycket dokumentation om vad som behövs göras här. Du
  kan dock fråga på kanalen [#rust-libs].
* Ta en titt på RFC-issue-trackern för ett [efterfrågat bibliotek][requested] och bygg det.
* Håll koll på [Github Trending][trending] för rustprojekt som är aktiva just nu.

Som biblioteksförfattare behöver du vara medveten om
[bästa praxis för rustbibliotek][lib-prac].

Träffa andra rustbibliotekdesigners i [#rust-libs].

<!--
TODO: Not sure #rust-libs is the place to direct people
-->

[#rust-libs]: https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-libs
[/r/rust]: https://reddit.com/r/rust
[PistonDevelopers]: https://github.com/PistonDevelopers
[Servo]: https://github.com/servo/servo
[Servo]: https://github.com/servo/servo
[awesome-rust]: https://github.com/kud1ing/awesome-rust
[clippy]: https://github.com/Manishearth/rust-clippy
[contain-rs]: https://github.com/contain-rs
[hyper]: https://github.com/hyperium/hyper
[hyperium]: https://github.com/hyperium
[iron]: https://github.com/iron
[lib-prac]: https://pascalhertleif.de/artikel/good-practices-for-writing-rust-libraries/
[libs.rs]: http://libs.rs
[nix]: https://github.com/nix-rust/nix/
[redox-os]: https://github.com/redox-os
[requested]: https://github.com/rust-lang/rfcs/labels/A-community-library
[rust-lang-nursery]: https://github.com/rust-lang-nursery
[rustfmt]: https://github.com/rust-lang-nursery/rustfmt
[trending]: https://github.com/trending?l=rust
[users.rust-lang.org]: https://users.rust-lang.org
