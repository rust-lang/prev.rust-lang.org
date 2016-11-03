---
layout: ru-RU/basic
title: Участие в разработке Rust &mdash; IDE, инструментарий и инфраструктура &middot; Язык программирования Rust
---

# Участие в разработке Rust &mdash; IDE, инструментарий и инфраструктура

Инструментарий играет огромную роль в успехе языков программирования,
и есть очень много вещей, которые нужно реализовать для Rust.
**Главная задача в разработке Rust сейчас - [улучшить работу с IDE][ides]**.
Это включает в себя работу со всей структурой языка - начиная с компилятора
и заканчивая вашим любимым IDE. Для получения подробной информации,
пройдите по ссылке.

Cargo, менеджер пакетов Rust, и rustdoc, генератор документации Rust,
хоть и реализованы в полной мере, но страдают от нехватки разработчиков. Для
rustdoc сейчас имеется много открытых задач, которые находятся в 
в главном репозитории с пометкой [A-rustdoc]. Большинство задач связаны 
с багами в коде, и участие в разработке сводится к исправлению этих багов и 
отправке pull request. У cargo есть [свой репозиторий с задачами][Cargo],
и тем, кто заинтересовался в разработке, было бы неплохо написать в 
IRC-канале [#cargo].

Не смотря на то что Rust может быть запущен как из gdb так и из lldb,
ещё реализован не весь функционал отладки, и не всё работает так как требуется.
Для отслеживания таких задач используется метка [A-debuginfo].

[Здесь][awesome-rust] указан список проектов, в которых вы можете принять участие.

Очень часто проекту просто не хватает правильных людей, которые могли бы
разработать тот или иной инструмент. Обсудить такие проекты с другими энтузиастами
можно в [#rust-tools].

[#cargo]: https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rustc
[#rust-tools]: https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-tools
[A-debuginfo]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-debuginfo
[A-rustdoc]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-rustdoc
[Cargo]: https://github.com/rust-lang/cargo/issues
[awesome-rust]: https://github.com/kud1ing/awesome-rust
[ides]: https://forge.rust-lang.org/ides.html
