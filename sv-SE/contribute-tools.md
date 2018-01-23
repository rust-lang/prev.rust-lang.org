---
layout: sv-SE/default
title: Att Bidra till Rust &mdash; Verktyg, IDEs och Infrastruktur &middot; Programmeringsspråket Rust
---

# Att Bidra till Rust &mdash; Verktyg, IDEs och Infrastruktur

Verktyg spelar en väldigt stor roll i ett programmeringsspråks framgång,
och det finns en hel del kvar att implementera.
***Ett stort fokus i Rust-utveckling är nu att [förbättra IDE-upplevelsen][ides]***.
Där i ingår arbete genom hela Rusts stack från kompilatorn själv till din
favorit-IDE. Följ länken för mer information.

Både Cargo, Rusts pakethanterare och Rustdoc,
Rusts dokumentationsgenerator, saknar utvecklare trots att de är funktionella.
Rustdoc har många öppna problem (issues) märkta med etiketten [A-rustdoc]
i huvudrepot. De är för det mesta buggar och att bidra handlar om att fixa
en bugg och att skicka in en pull request. Cargo har
[sitt egna repo och problem/issues][Cargo]. De som är intresserade av att bidra
kan introducera sig själva i kanalen [#cargo].

Medan Rust kan köras både under debuggers gdb och lldb med begränsad framgång
finns det fortfarande många fall där debugging inte fungerar som förväntat.
Issues/problem för dessa sorterar under etiketten [A-debuginfo].

För idéer till fler verktygsprojekt att bidra till, se [awesome-rust].

Det finns ofta flera intressanta verktygsprojekt som bara väntar på att rätt personer
ska dyka upp och implementera dem. Diskutera med andra Rust-verktygsentusiaster i
[#rust-tools].

[#cargo]: https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rustc
[#rust-tools]: https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-tools
[A-debuginfo]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-debuginfo
[A-rustdoc]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-rustdoc
[Cargo]: https://github.com/rust-lang/cargo/issues
[awesome-rust]: https://github.com/kud1ing/awesome-rust
[ides]: https://forge.rust-lang.org/ides.html
