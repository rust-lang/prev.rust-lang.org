---
layout: sv-SE/default
title: Att bidra till rust &mdash; att hitta, kategorisera, prioritera och fixa problem &middot; Programmeringsspråket Rust
---

# Att bidra till rust &mdash; att hitta, kategorisera, prioritera och fixa problem

Det dagliga underhållet av projetet kretsar kring rusts [issue tracker] och
[issue tracker] och [pull requests][PR], och mer hjälp behövs alltid.
Det enklaste sättet att komma igång med att bidra till rust är att leta efter
etiketterna [E-easy] eller [E-mentor]. Dessa är ämnade att kunna lösas av nya
rustprogrammerare.

På issues ("problem") märkta `E-mentor` har en erfaren rust-utvecklare anmält
sig frivilligt i kommentarerna för att handleda dig genom lösandet av problemet
och att [skicka in en fix via pull requests på GitHub][pull]. Kontakta dem om problemet
på issue trackern genom att [@nämna][@mentioning] deras namn i en kommentar, på IRC,
eller via email. Notera att rust-utvecklare får många notiser och att det därför är
lätt att missa några; tveka inte att hitta dem med alla till båds stående medel!

Andra projekt i rust tillhandahåller likande uppgifter på nybörjarnivå inklusive
webbläsaren [Servo], httpbiblioteket [hyper], källkodsformateraren [rustfmt], och 
projektet för unix-biblioteks-bindningar [nix], och källkodsförbättringsförslagsgivaren [clippy].

Medan rust har en [extensiv testsvit][test] finns det alltid mer att testa.
Etiketten [E-needstest] indikerar problem som tros ha fixats med saknar tester.
Att skriva testfall är ett mycket bra sätt att förstå ett nytt projekt och att
börja att bidra.

Rust är alltid i behov av folk som kan kategorisera och prioritera problem:
reproducera buggar, hitta mindre test-fall, etikettera, stänga lösta problem.
Notera att du behöver upphöjda GitHub-privilegier för att etikettera, men detta
är lätt att få för någon med lite erfarenhet i projektet. Fråga en [teammedlem][team]
för att få dessa privilegier.

När du har hittat dig runt projektet och skapat några
dra pull requests inom ett visst område, överväg då att granska andras förslag.
Bra granskningsförmåga är en sällsynt skicklighet och uppskattas alltid.

Inget tidigare tillstånd krävs &mdash; börja bara att ge konstruktiva och artiga
kommentarer på pull requests som intresserar dig. Om du behöver råd för hur man
ger bra kodgranskning [läs denna guide][reviews].

<!--
TODO: weekly triage email?
TODO: @nrc says suggesting everybody review w/o training is bad
-->

[@mentioning]: https://github.com/blog/821
[E-easy]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AE-easy
[E-mentor]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AE-easy+label%3AE-mentor
[E-needstest]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AE-needstest
[PR]: https://github.com/rust-lang/rust/pulls
[Servo]: https://github.com/servo/servo
[clippy]: https://github.com/Manishearth/rust-clippy
[hyper]: https://github.com/hyperium/hyper
[issue tracker]: https://github.com/rust-lang/rust/issues
[nix]: https://github.com/nix-rust/nix/
[pull]: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#pull-requests
[reviews]: http://blog.originate.com/blog/2014/09/29/effective-code-reviews/
[rustfmt]: https://github.com/rust-lang-nursery/rustfmt
[team]: team.html
[test]: https://github.com/rust-lang/rust-wiki-backup/blob/master/Note-testsuite.md
[triage]: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#issue-triage
