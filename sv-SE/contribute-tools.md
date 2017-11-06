---
layout: sv-SE/default
title: Att bidra till rust &mdash; verktyg, IDEs och infrastruktur &middot; Programmeringsspråket Rust
---

# Att bidra till rust &mdash; verktyg, IDEs och infrastruktur

Verktyg spelar en väldigt stor roll i ett programmeringsspråks framgång,
och det finns en hel del kvar att implementera.
***Ett stort fokus i rustutveckling är nu att [förbättra IDE-upplevelsen][ides]***.
Där i ingår arbete genom hela rusts stack från kompilatorn själv till din
favorit-IDE. Följ länken för mer information.

Både Cargo, rusts pakethanterare och rustdoc,
rusts dokumentationsgenerator, medan den är fullfjädrad och fuktionell,
lider av en brist på utvecklare. Rustdoc har många öppna problem (issues) märkta
med etiketten [A-rustdoc] i huvudrepot. De är för det mesta buggar och att bidra
handlar om att fixa en bug och att skicka in en pull-request. Cargo har
[dess egna repo och problem/issues][Cargo]. De som är intresserade av att bidra
kan vilja introducera sig själva i kanalen [#cargo].

Medan rust kan köras både under debuggers gdb och lldb med begränsan framgång
finns det fortfarande många fall där debugging inte fungerar som förväntat.
Issues/problem för dessa sorterar under etiketten [A-debuginfo].

För idéer till fler verktygsprojekt att bidra till, se [awesome-rust].

Det fimms pfta fler intressanta verktygsprojekt som bara vänta på rätt personer
att dyka upp och implementera dem. Diskutera med andra rustverktygsentusiaster i
[#rust-tools].

[#cargo]: https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rustc
[#rust-tools]: https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-tools
[A-debuginfo]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-debuginfo
[A-rustdoc]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-rustdoc
[Cargo]: https://github.com/rust-lang/cargo/issues
[awesome-rust]: https://github.com/kud1ing/awesome-rust
[ides]: https://forge.rust-lang.org/ides.html
