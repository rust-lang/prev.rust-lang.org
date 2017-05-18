---
layout: pt-BR/default
title: Contribuindo para Rust &mdash; documentação &middot; A linguagem de programação Rust
---

# Contribuindo para Rust &mdash; documentação

Documentação nunca é boa o suficiente e nunca terá o bastante dela. Muitos
aspectos da documentação do Rust não requere um conhecimento mais profundo para melhorar,
escrever, revisar e editar a documentação, que são ótimas formas de aprender Rust. Além disso,
melhoras na documentação são fáceis de identificar e não têm limites. Não gosta da forma da qual
algo está escrito? Descobriu alguma informação não documentada? Sua pull request será muito apreciada.

***A documentação mais impactante que você pode escrever é [para pacotes que fazem parte
do ecossistema Rust][crate_docs]***. Enquanto a documentação de dentro é relativamente completa, o mesmo
não é verdade para [a maioria dos pacotes e ferramentas populares][awesome-rust] que
programadores Rust interagem todos os dias. Contribuindo documentação de APIs para um projeto
Rust popular vai te recompensar com eterno amor de seu responsável.

[O livro][The Book] é a documentação primária para Rust, mantido no repositório
principal. Ele tem seu próprio rótulo, [A-book] e está continuamente sendo refinado.
Outras documentações no repositório principal incluem [A referência para Rust][The Rust Reference],
a [Documentação da Biblioteca Padrão][std], O [Rustonomicon][The Rustonomicon] (um guia para
código `inseguro` em Rust, corretamente). O [Guia de Estilo de Rust][Rust Style Guide] está tão
incompleto que ele não é referenciado com frequência; um contribuidor ambicioso pode se dar
muito bem nele. O [índice de erros][err] fornece explicações extendidas para os erros
que o compilador emite. Ao mesmo tempo que novos erros são adicionados, essa documentação
precisa [ser atualizada][err-issue], então há sempre erros não indexados no índice prontos
para serem adicionados. A maior parte da documentação interna vive na pasta [src/doc].
Tudo lá é regido pelo rótulo [A-docs] no seguidor de problemas. Finalmente, esse documento
e outros materiais do website são mantidos no [Repositório Git do website Rust][Rust website Git repository].
Para contribuir, simplesmente edite e mande uma pull request.

Uma grande quantidade de documentação importante para Rust não fica no repositório
principal, ou não é mantido pelo projeto, mas ainda sim tem importância crítica para
o sucesso de Rust. Exemplos de excelente documentação que está sendo ativamente desenvolvida e
com necessidade de contribuidores inclui [Rust por Exemplos (inglês)][Rust By Example], [Padrões de Design para Rust][Rust Design Patterns]
e o [rust-rosetta]. Para outros projetos existentes de documentação para contribuir
veja [rust-learning].

Conheça outros documentaristas de Rust em [#rust-docs].

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
