---
layout: pt-BR/default
title: Contribuindo para Rust &mdash; ferramentas, IDEs e infraestrutura &middot; A linguagem de programação Rust
---

# Contribuindo para Rust &mdash; ferramentas, IDEs e infraestrutura

Ferramentas tem uma enorme participação no sucesso de uma linguagem, e é
uma grande coisa a se implementar. ***O maior foco do desenvolvimento de Rust no
momento é [melhorar a experiência com IDEs][ides]***. Isso envolve trabalho
na pilha de Rust, desde o compilador em si até sua IDE favorita. Dê uma olhada
no link para mais informações.

Ambos Cargo, o gerenciador de pacotes do Rust, e o rustdoc, o gerador de Documentação
do Rust, enquanto cheio de funcionalidades e funcional, sofre da falta de desenvolvedores.
Rustdoc tem muitos problemas em aberto, no repositório principal no rótulo [A-rustdoc].
Eles são, na sua maioria, bugs e contribuir é apenas sanar o problema e mandar uma pull
request. Cargo tem [seu próprio repositório e seção de problemas][Cargo], e interessados
em contribuir talvez queiram se introduzir em [#cargo].

Mesmo que Rust possa rodar em ambos os debugadores gdb e lldb com sucesso limitado,
há casos em que a debugging não corre como o esperado. Os problemas [A-debuginfo] seguem esses.

Para ideias de mais projetos de ferramentas a se contribuir, veja [awesome-rust].

Esses são, muitas vezes, outros projetos de ferramentas de seu interesse, apenas esperando
pelas pessoas certas para chegarem e implementarem eles. Discuta com outros entusiastas
de ferramentas para Rust em [#rust-tools].

[#cargo]: https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rustc
[#rust-tools]: https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-tools
[A-debuginfo]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-debuginfo
[A-rustdoc]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-rustdoc
[Cargo]: https://github.com/rust-lang/cargo/issues
[awesome-rust]: https://github.com/kud1ing/awesome-rust
[ides]: https://forge.rust-lang.org/ides.html
