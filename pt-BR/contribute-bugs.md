---
layout: pt-BR/default
title: Contribuindo para Rust &mdash; encontrando, fazendo triagem e solucionando bugs &middot; A linguagem de programação Rust
---

# Contribuindo para Rust &mdash; encontrando, fazendo triagem e solucionando bugs

A manutenção diária do projeto gira em torno do [seguidor de problemas][issue tracker]
e das [pull requests][PR] de Rust, e mais ajuda é sempre necessária. O jeito mais
básico de se começar a contribuir para Rust é olhar para rótulos [E-easy] e [E-mentor].
Esses são pensados para novos programadores Rust.

Em problemas `E-mentor`, um desenvolvedor Rust experiente se voluntariou nos comentários
para aconselhar/acompanhar você em resolver o problema e enviar sua [solução via pull requests do GitHub][pull].
Contate-os sobre o problema no seguidor de problemas [@mencionando][@mentioning] seus nomes num comentário,
no IRC ou por e-mail. Note que os desenvolvedores Rust recebem muitas notificações e é
fácil escapar uma; não hesite em insistir se for necessário!

Outros projetos em Rust mantêm tarefas de cunho similar, incluindo o navegador web [Servo],
a biblioteca HTTP [hyper], o formatador de código [rustfmt], as *bindings* para bibliotecas unix [nix],
e a coleção de lints, [clippy].

Enquanto Rust tem uma [suite de testes intensiva][tests], há sempre mais para testar.
Os rótulos [E-needstest] indica que problemas estão quase solucionados mas não foram completamente
testados ainda. Escrever test cases é um ótimo jeito de entender um novo projeto
e começar a contribuir.

Rust está sempre necessitando de pessoas para realizar a triagem de problemas:
reproduzir bugs, minimizar as test cases, aplicar rótulos, fechar problemas já
solucionados. Note que você precisará de permissões elevadas no GitHub para aplicar
rótulos, mas isso é fácil de se obter para alguém com um pouco de experiência no projeto.
Pergunte um [membro do time][team].

Assim que você encontrar seu lugar no projeto e já tiver criado algumas pull requests
numa área particular, considere revisar pull requests de outros contribuidores:
boa liderança é uma habilidade rara e é sempre bem-vinda. Nenhuma permissão é necessária para tal &mdash;
apenas comece a comentar construtivamente e politicamente nas pull requests que lhe interessam. Se
você deseja treinamento em como realizar boas revisões de código, [leia este guia (inglês)][reviews].

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
