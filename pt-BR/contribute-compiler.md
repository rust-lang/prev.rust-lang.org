---
layout: pt-BR/default
title: Contribuindo para Rust &mdash; a linguagem, o compilador, e a biblioteca padrão &middot; A linguagem de programação Rust
---

# Contribuindo para Rust &mdash; a linguagem, o compilador, e a biblioteca padrão

O código fonte para o compilador e da biblioteca padrão estão no repositório
principal, e como sua manutenção é o objetivo primário desse repositório, muitos
rótulos são relacionados a eles. Alguns dos rótulos mais férteis são [A-codegen],
da tradução do código Rust para LLVM IR; [A-debuginfo], geração de metadatas usada
por debugadores; [A-diagnostics], o feedback que o compilador produz em casos
de erros; [A-libs], problemas com a biblioteca padrão; [A-macros] e [A-syntaxext],
relacionados à extensões de sintaxe; e [A-typesystem], no tópico de tipos.

Não há um guia bem mantido para a arquitetura do compilador, mas há [uma pequena revisão interna][rustc-guide].
A [documentação para a API para as crates que envolvem com o compilador][internals-docs]
podem ajudar na navegação no código, assim como o navegador de código [Rust DXR]. O
[guia aos test suites de Rust][testsuite] vai te ensinar a exercitar o sistema de compilação
de Rust efetivamente, assim como rodando [`make tips`][tips] no terminal.

Para o previsível futuro, uma das maiores previsões do desenvolvimento
do compilador Rust é convertendo sua representação interna para operar
diretamente com uma [representação intermediária chamada MIR][mir], ao invés
de usar AST. Essa tarefa é esperada para abrir muitas portas para simplificar o
compilador e ajuda é necessária para, por exemplo, criar o passe de tradução
já baseado em MIR, adicionar optimizações usando MIR e implementar a compilação
incremental. Ainda não há uma única fonte de informação das tarefas necessárias aqui,
mas pergunte no [internals.rust-lang.org] ou [#rust-internals] para ajuda.

[É estranho quando nosso compilador crasha][ice] &mdash; o falecido 'internal compiler error'
(ICE). O rótulo [I-ICE] segue esses, que são muitas vezes abundantes. Esses são
normalmente bons bugs para começar, porquê é fácil saber quando você o consertou, e eles
são muitas vezes independentes.

A performance do código Rust é uma das suas melhores vantagens; e a performance
do compilador Rust uma de suas maiores fraquezas. Qualquer melhora, seja no runtime
ou &mdash; especialmente &mdash; performance no tempo de compilação são extremamente
celebradas. Os rótulos [I-slow] e [A-optimization] lidam com problemas na performance
de execução e [I-compiletime] com tempo de compilação. Nos temos um [site que segue a performance de compilação][rustc-perf]
numa série de testes. A flag `-Z time-passes` do compilador pode ajudar a debugar
performance do compilador, e o código Rust pode ser medida com profilers como `perf`, no Linux.

As maiores novas features em Rust passam por um processo de [Pedido por Comentários (RFC)][rfc],
que por design é de consenso público. Embora seja aberto à todos, isso é um processo social
entre desenvolvedores que já possuem muita experiência trabalhando juntos, e é recomendado
se envolver de forma lenta &mdash; submeter um RFC precicpitado, sem entendimento
histórico, técnico ou contexto social é um jeito fácil e rápido de fazer uma má impressão
e sair desapontado. Leia o arquivo leiame (readme) mecionado anteriormente para entender
melhor como tudo funciona. Mutias ideias já foram debatidas na história de Rust, algumas
rejeitadas, algumas deixadas pro futuro, e o [seguidor de problemas RFC][rfc-issues]
cataloga algumas ideias que ainda não fizeram seu caminho para dentro da linguagem.
Pouco antes de um RFC ser aceito para implementação, ele entra no 'perído de comentário final',
indicado pelo rótulo [final-comment-period][rfc-fcp] no repositório rust-lang/rfcs.
Da mesma forma, antes de uma feature ser implementada no compilador estável (chamado de 'libertação')
ele entra no [final-comment-period][issue-fcp] no repositório rust-lang/rust. Ambos
FCPs são momentos críticos para se envolver e expressar opniões na direção que a linguagem
está tomando, e são anunciados nos relatórios semanais do sub-time em [internals.rust-lang.org].

Encontre outros engenheiros do compilador rust em [#rustc], designers da linguagem
em [#rust-lang] e designers de bibliotecas em [#rust-libs].

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
