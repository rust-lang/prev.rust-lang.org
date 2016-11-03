---
layout: ru-RU/basic
title: Участие в разработке Rust &mdash; поиск, классификация и решение задач &middot; Язык программирования Rust
---

# Участие в разработке Rust &mdash; поиск, классификация и решение задач

Ежедневная поддержка проекта строится вокруг
[трекера задач][issue tracker] и [pull request][PR], и для этого
нам всегда требуется помощь. Самый простой путь начать участие в
разработке Rust &mdash; искать задачи с метками [E-easy] и [E-mentor].
Данные задачи вполне решаемы новичками в Rust.

Метка `E-mentor` означает что опытный Rust разработчик, который своими
комментариями поможет решить задачу и подготовить 
[GitHub pull request][pull]. Свяжитесь с ним по поводу этой задачи,
[упомянув][@mentioning] его в комментарии, ипользуя IRC либо e-mail.
Нужно отметить, что у опытных разработчиков полным полно уведомлений,
и не отчаивайтесь, если на ваше сообщение не придет ответ. 
Не стесняйтесь выходить с ними на контакт любыми способами.

Другие проекты в Rust так же имеют задачи начального уровня, включая
веб-браузер [Servo], HTTP библиотеку [hyper], форматировщик исходных
кодов [rustfmt], привязки к Unix библиотеке [nix] и коллекцию средств
синтаксического контроля [clippy].

Так как Rust имеет [обширные средства для тестирования][test], всегда
найдется что протестировать. Метка [E-needstest] означает что для
задачи есть решение но нет тестов. Написание тестов это хороший путь к
пониманию нового проекта, и позволяет начать участие в разработке.

Rust всегда нужны люди, которые будут [классифицировать][triage] задачи:
воспроизводить баги, минимизировать тестов, назначать метки, закрывать
решенные задачи. Нужно отметить, что необходимы соответствующие 
разрешения, чтобы GitHub позволил вам назначать метки, но это не проблема,
так как их можно получить у более опытных разработчиков. 
Спрашивайте [членов команды разработки][team].

Как только вы найдете себе проект, и создадите пару pull request, 
начните участвовать в обсуждении pull request других разработчиков: 
хороший обзор pull request всегда высоко ценится. Для этого не требуется 
никаких дополнительных разрешений – просто начните вежливо и конструктивно 
комментировать pull request который вам покажется интересным. Если вы не 
уверены что сможете написать грамотный обзор кода (code review), то вы 
можете прочитать это [руководство][reviews].

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
