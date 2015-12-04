---
layout: default
title: Contributing to Rust &middot; The Rust Programming Language
---

# Contributing to Rust

You've started learning Rust. You love it, and you want to be a part
of it. If you're not sure how to get involved, then this page
will help.

**Found a bug and need to report it?** [Follow the bug reporting
guide][bugs]. Thanks in advance!

Rust is an expansive system of projects, the most prominent of which
are maintained by [The Rust Project Developers][devs] in [the
rust-lang organization on GitHub][rust-lang]. Newcomers may be
interested in [an overview of the organization, processes, and
policies of The Rust Project][dev_proc] and the project's
[CONTRIBUTING.md] file, which explains the specifics of contributing
to [rust-lang/rust].

There are many ways to contribute to the success of Rust.
This guide focuses on a few avenues for the new contributor:

* [Finding, triaging and fixing issues](#bugs). The basic work
  of maintaining a large and active project like Rust.
* [Documentation](#docs). Not just official documentation, but also
  for crates, blog posts, and other unofficial sources.
* [Community building](#community). Helping your fellow Rustacean,
  and expanding the reach of Rust.
* [Tooling, IDEs and infrastructure](#tools). There's so much
  to do here.
* [Libraries](#libs). Rust's suitability for any particular task
  is mostly dependent on the quality libraries that exist.
* [Language, compiler and the standard library](#compiler). Language design, feature implementation, performance improvement.

If you need additional guidance ask on [#rust-internals] or
[internals.rust-lang.org].

We pride ourselves on maintaining civilized discourse, and to that end
contributors are expected to follow our [Code of Conduct][coc]. If you
have questions about this please inquire with the [community team].

<!--
TODO: The overview link doesn't go somewhere enlightening. Write it.
TODO: List of active initiatives
TODO: Write guide to advertising Rust projects to link from
libs / community building
-->

<a name="bugs"></a>
## Finding, triaging and fixing issues

The day-to-day maintenance of the project revolves around Rust's
[issue tracker] and [pull requests][PR], and more help is always
needed. The most basic way to get started contributing to Rust is to
look for the [E-easy] or [E-mentor] labels. These are meant to be
approachable for new Rust programmers.

On `E-mentor` issues an experienced Rust developer has volunteered in
the comments to mentor you through solving the issue and [submitting
the fix via GitHub pull request][pull]. Contact them about the issue,
on the issue tracker by [@mentioning] their name in a comment, on IRC,
or through email. Note that Rust developers get a lot of notifications
and it is easy to miss some; don't hesitate to hunt them down by
whatever means necessary!

Other projects in Rust maintain similar entry-level tasks, including
the web browser [Servo], the HTTP library [hyper], the
source-formatter [rustfmt], and the lint collection [clippy].

While Rust has an [extensive test suite][test] there is always more to
test. The [E-needstest] label indicates issues that are thought to be
fixed but don't have tests. Writing test cases is a great way to
understand a new project and get started contributing.

Rust is always in need of people to [triage] issues: reproduce bugs,
minimize test cases, apply labels, close resolved issues. Note that
you'll need elevated GitHub permissions to apply labels, but this is
easy to obtain for somebody with a bit of experience in the
project. Ask a [team member][team].

Once you've found your way around the project and have created a few
pull requests in a particular area, consider reviewing others' pull
requests: good reviewership is a rare skill and always appreciated. No
prior permission is needed &mdash; just start constructively and politely
commenting on pull requests that interest you. If you want training
on conducting good code reviews [read this guide][reviews].

<!--
TODO: weekly triage email?
TODO: @nrc says suggesting everybody review w/o training is bad
-->

<a name="docs"></a>
## Documentation

Documentation is never good enough and there's never enough of it.
Many aspects of Rust's documentation don't require deep knowledge to
improve, and writing, reviewing, and editing documentation are great
ways to learn Rust. Furthermore, improvements to documentation are
easy to identify and limitless. Don't like the way something reads?
Discover some information that wasn't documented?  Your pull request
will be gleefully embraced.

***The most impactful documentation you can write is [for the crates
that make up the Rust ecosystem][crate_docs]***. While the in-tree
documentation is relatively complete, the same is not yet true for
[many of the popular crates and tools][awesome-rust] that Rust
programmers interact with every day. Contributing API documentation to
a popular Rust project will earn you the enduring love of its maintainer.

[The Book] is the primary documentation for Rust, maintained in the
main repository. It has its own issue label, [A-book][book_issues] and
is continually being refined. Other documentation in the main
repository include [The Rust Reference], the [standard library API
documentation][std], [The Rustonomicon] (a guide to using `unsafe`
correctly). The [Rust Style Guidelines] are so incomplete they are not
linked prominently; an ambitious contributor can make much headway
there. The [error index][err] provides extended explanations of the
errors produced by the compiler. As new errors are added this
documentation [must be maintained][err-issue], so there always
errors not reflected in the index to be added. Most in-tree
documentation lives in the [src/doc] directory. To contribute simply
edit it and submit a pull request. These are all covered by the
[A-docs] label on the issue tracker.

A great deal of important Rust documentation does not live in the main
repository, or is not maintained by the project, but is still
critically important to Rust's success. Examples of excellent Rust
documentation that is actively developed and in need of contributors
include [Rust By Example], [Rust Design Patterns], and [rust-rosetta].
For other existing documentation projects to contribute to see [rust-learning].

Meet other Rust documentarians in [#rust-docs].

<!--
TODO: blogging, translation
-->

<a name="community"></a>
## Community building

Help newbies, spread the word, meet interesting people. Make Rust the
shining example of open source development that we all want it to be.

Keep an eye on the [#rust-beginners] channel. This is where we direct
new Rust programmers to ask for help, and it is vital when they do
that they receive prompt, accurate, and courteous responses. Likewise,
[Stack Overflow], [users.rust-lang.org], and [/r/rust], are all forums
where Rust programers commonly look for assistance. If you want
training on answering programmers' questions [read this
guide][helpful].

If you are already experienced in some area of the project, please
look out for potential [E-easy] bugs. When you see an
easy issue on the bug tracker that you know how to fix, write up a
description of the fix and tag it with E-easy. Note that what is
obvious to you is not obvious to a new Rust contributor, and its
important to describe the problem and the solution clearly. It is
thus also helpful to triage E-easy bugs for poor descrptions and
improve them.

Experienced developers who are patient and communicate clearly should
consider [mentoring new contributors][mentor]. Tag easy issues with
[E-mentor] and mention in a comment that you will mentor. Expect people
to contact you about the issue, and attempt to respond promptly.

Maintaining entry-level tasks is good not only for The Rust Project
itself but all projects. If your project has a consistent supply of
entry-level tasks you might institute such a program
yourself. ***Curating entry-level tasks is one of the most effective
methods of bringing new programmers into the project***.

Talk about what you are working on in the weekly "what's everyone
working on this week" threads on [/r/rust] and [users.rust-lang.org],
and indicate what you need help with. These are great starting points
for collaboration.

Advocate Rust in your own local community. Rust [user groups] and [events]
are a unique and exciting part of the Rust experience: there are so
many, and they are everywhere! If you haven't been yet, go and enjoy
new experiences. If there is nothing Rusty going on near you then
consider organizing something. You can poll for interest and announce
events on [/r/rust] or [users.rust-lang.org]. Contact the [community
team] to put events on the calendar, and thus be announced on [This
Week in Rust].

Remember as you are advocating Rust though to be considerate of
others' views &mdash; not everybody is going to be receptive to Rust, and
that's just fine.

Meet other Rust community builders in [#rust-community].

<!--
Other ideas:
TWIR, podcasts.

experience reports
conf talks

Conduct training on Rust. (link to training material).
-->

<a name="tools"></a>
## Tooling, IDEs, and infrastructure

Tools play a huge part in the success of a language, and there is a
great deal left to implement. ***A major focus of Rust development now
is [improving the IDE experience][ides]***. This involves work
throughout the Rust stack, from the compiler itself through your
favorite IDE. Follow the link for more information.

Both Cargo, the Rust package manager, and rustdoc,
the Rust documentation generator, while full-featured and functional,
suffer from a lack of developers. Rustdoc has many open issues, under
the main repository's [A-rustdoc] label. They are mostly bugs and
contributing is a matter of fixing the bug and submitting a pull
request. Cargo has [its own repository and issues][Cargo], and those
interested in contributing might want to introduce themselves in
[#cargo].

Although Rust can be run under both the gdb and lldb debuggers with
limited success, there are still many cases where debugging does not
work as expected.  The [A-debuginfo] issue tracks these.

For ideas for more tooling projects to contributo to see
[awesome-rust].

There are often other tooling projects of interest just waiting for
the right people to come along and implement them. Discuss with other
Rust tooling enthusiasts in [#rust-tools].

<a name="libs"></a>
## Libraries

If you want to contribute to Rust by writing volumes of Rust code,
then libraries are where it's at: since Rust is a young language,
there are many types of libraries that either do not exist yet
or are incomplete and in need of improvement or competition.

Deciding what to write that will have impact and be fun is a common
difficulty. Here are some ideas:

* Read and participate in the weekly "what's everyone working on
  this week" threads on [/r/rust] and [users.rust-lang.org]. These are
  packed with exciting announcements from other Rust programmers in
  need of collaborators.
* Familiarize yourself with the best Rust libraries through
  [awesome-rust] and [libs.rs].
* Some larger projects, including the web browser [Servo], the HTTP
  library [hyper], the source-formatter [rustfmt], and the lint
  collection [clippy], tag issues with 'easy' labels for new
  contributors.
* Get involved with one of the active Rust-oriented GitHub
  organizations, such as [PistonDevelopers], [servo], [redox-os],
  [iron], [contain-rs], [hyperium]. It's often easier to find a place
  to fit in with these subcommunities, they are in greater need of
  help than rust-lang itself, and they are filled with experienced
  Rust developers to guide you.
* Help guide libraries from [rust-lang-nursery] into rust-lang proper.
  Unfortunately there is not much documentation on what needs to be
  done here; ask on [#rust-libs].
* Inspect the RFC issue tracker for a [requested community
  library][requested] and build it.
* Watch [Github Trending][trending] for currently active Rust projects.

As a library author you will want to be aware of the [best practices
for Rust libraries][lib-prac].

Meet other Rust library designers in [#rust-libs].

<!--
TODO: Not sure #rust-libs is the place to direct people
-->

<a name="compiler"></a>
## Language, compiler, and the standard library

The source code to the compiler and standard library are in the main
repository, and as their maintenance is the primary objective of that
repository, many labels on the issue tracker relate to it. Some of
the more fruitful labels include [A-codegen], for translation of
Rust to LLVM IR; [A-debuginfo], generation of metadata used by debuggers;
[A-diagnostics], the feedback the compiler provides on errors; [A-libs],
issues with the standard library; [A-macros] and [A-syntaxext], both
related to syntax extensions; and [A-typesystem], on the topic of types.

There is no well-maintained guide to the architecture of the compiler,
but [there is a small overview in-tree][rustc-guide]. The [API
documentation for the crates that make up the
compiler][internals-docs] can help with navigating the code, as can
the source code browser [Rust DXR]. The [guide to the Rust test
suite][testsuite] will teach you how to exercise the Rust build system
effectively, as will running [`make tips`][tips] at the command line.

For the foreseable future, one of the major thrusts of Rust compiler
development is converting its internals from operating directly off
the AST to working with an [intermediate representation called
MIR][mir]. This work is expected to open up many new possibilities by
simplifying the compiler and help is needed to e.g. create a MIR-based
translation pass, add MIR-based optimizations, and implement
incremental compilation. There is yet no single source for information
on work needed here, but ask on [internals.rust-lang.org] or
[#rust-internals] for guidance.

[It's embarrasing when our compiler crashes][ice] &mdash; the
dreaded 'internal compiler error' (ICE). The [I-ICE] label
tracks these, and they are often plentiful. These are usually
good bugs to start with because it's easy to know when you've fixed
them, and they're often relatively self-contained.

The performance of Rust code is one of its great advantages; and the
performance of the Rust compiler one of its great weaknesses. Any
improvements to either runtime or &mdash; especially &mdash; compiletime performance
are widely celebrated. The [I-slow] and [A-optimization] labels deal
with runtime performance, and [I-compiletime] with compiletime. We have
a [site that tracks compiletime performance][rustc-perf] on a number
of workloads. The `-Z time-passes` compiler flag can help debug
compiler performance, and Rust code can be profiled with standard
profilers like `perf` on Linux.

Major new features go through a [Request for Comments (RFC)][rfc]
process, by which the design is agreed upon. Though it is open to all,
it is a social process between developers who already have various
amounts of experience working together, and it is recommended to get
involved slowly &mdash; submitting a hasty RFC without understanding
the historical, technical, or social context is an easy way
to make a poor impression and come away disappointed. Read the
aforelinkd readme file to understand best how it all works. Many
ideas have been debated in Rust's history, some rejected, some
postponed until the future, and the RFC [issue tracker][rfc-issues]
catalogs some wishlist ideas that have yet to make headway into the
language. Shortly before an RFC is accepted for implementation it
enters 'final commemnt period', indicated by the [final-comment-period
label on the rust-lang/rfcs repository][rfc-fcp]. Likewise, before a
feature is enabled in the stable compiler (called 'ungating') it
enters [final-comment-period in the rust-lang/rust
repository][issue-fcp]. Both FCPs are critical moments to get involved
and express opinions on the direction of the language, and are
advertised in the weekly subteam reports on [internals.rust-lang.org].

Meet other Rust compiler engineers in [#rustc], language
designers in [#rust-lang], and library designers in [#rust-libs].

<!--
TODO: guide to compile-time benchmarking
TODO: using the triage bot?
TODO: some of this RFC description could probably go in the RFC readme
-->


[#cargo]: https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rustc
[#rust-beginners]: https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-beginners
[#rust-community]: https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community
[#rust-docs]: https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs
[#rust-internals]: https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-internals
[#rust-lang]: https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-lang
[#rust-libs]: https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-libs
[#rust-tools]: https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-tools
[#rustc]: https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rustc
[/r/rust]: https://reddit.com/r/rust
[@mentioning]: https://github.com/blog/821
[A-codegen]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-codegen
[A-debuginfo]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-debuginfo
[A-debuginfo]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-debuginfo
[A-diagnostics]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-diagnostics
[A-docs]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-docs
[A-libs]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-libs
[A-macros]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-macros
[A-optimization]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-optimization
[A-rustdoc]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-rustdoc
[A-syntaxext]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-syntaxext
[A-typesystem]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-typesystem
[CONTRIBUTING.md]: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md
[Cargo]: https://github.com/rust-lang/cargo/issues
[E-easy]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AE-easy
[E-mentor]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AE-easy+label%3AE-mentor
[E-needstest]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AE-needstest
[I-ICE]: https://github.com/rust-lang/rust/labels/I-ICE
[I-compiletime]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AI-compiletime
[I-slow]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AI-slow
[PR]: https://github.com/rust-lang/rust/pulls
[PistonDevelopers]: https://github.com/PistonDevelopers
[Rust By Example]: https://github.com/rust-lang/rust-by-example
[Rust DXR]: https://dxr.mozilla.org/rust/source/src
[Rust Design Patterns]: https://github.com/nrc/patterns
[Rust Style Guidelines]: http://doc.rust-lang.org/style/index.html
[Servo]: https://github.com/servo/servo
[Stack Overflow]: http://stackoverflow.com/questions/tagged/rust
[The Book]: http://doc.rust-lang.org/book/index.html
[The Rust Reference]: http://doc.rust-lang.org/reference.html
[The Rustonomicon]: http://doc.rust-lang.org/nomicon/index.html
[This Week in Rust]: http://www.this-week-in-rust.org
[awesome-rust]: https://github.com/kud1ing/awesome-rust
[awesome-rust]: https://github.com/kud1ing/awesome-rust
[book_issues]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-book
[bugs]: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[clippy]: https://github.com/Manishearth/rust-clippy
[coc]: https://www.rust-lang.org/conduct.html
[community team]: https://www.rust-lang.org/team.html#Community
[contain-rs]: https://github.com/contain-rs
[crate_docs]: https://users.rust-lang.org/t/lets-talk-about-ecosystem-documentation/2791
[dev_proc]: community.html#rust-development
[devs]: https://github.com/rust-lang/rust/graphs/contributors
[err-issue]: https://github.com/rust-lang/rust/issues/24407
[err]: http://doc.rust-lang.org/error-index.html
[events]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc@group.calendar.google.com
[helpful]: http://blogs.msmvps.com/jonskeet/2009/02/17/answering-technical-questions-helpfully/
[hyper]: https://github.com/hyperium/hyper
[hyperium]: https://github.com/hyperium
[ice]: https://users.rust-lang.org/t/glacier-a-big-ol-pile-of-ice/3380
[ides]: ides.html
[internals-docs]: http://manishearth.github.com/rust-internals-docs
[internals.rust-lang.org]: https://internals.rust-lang.org/
[iron]: https://github.com/iron
[issue tracker]: https://github.com/rust-lang/rust/issues
[issue-fcp]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AB-unstable+label%3Afinal-comment-period
[lib-prac]: https://pascalhertleif.de/artikel/good-practices-for-writing-rust-libraries/
[libs.rs]: http://www.libs.rs
[lru_issues]: https://github.com/rust-lang/rust/issues?q=is%3Aissue+is%3Aopen+sort%3Aupdated-asc
[main_repo]: https://github.com/rust-lang
[mentor]: https://users.rust-lang.org/t/mentoring-newcomers-to-the-rust-ecosystem/3088
[mir]: https://github.com/rust-lang/rust/issues/27840
[pull]: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#pull-requests
[redox-os]: https://github.com/redox-os
[requested]: https://github.com/rust-lang/rfcs/labels/A-community-library
[reviews]: http://blog.originate.com/blog/2014/09/29/effective-code-reviews/
[rfc-fcp]: https://github.com/rust-lang/rfcs/pulls?q=is%3Aopen+is%3Apr+label%3Afinal-comment-period
[rfc-issues]: https://github.com/rust-lang/rfcs/issues
[rfc]: https://github.com/rust-lang/rfcs#table-of-contents
[rust-lang-nursery]: https://github.com/rust-lang-nursery
[rust-lang/rust]: https://github.com/rust-lang/rust
[rust-lang]: https://github.com/rust-lang
[rust-learning]: https://github.com/ctjhoa/rust-learning
[rust-rosetta]: https://github.com/Hoverbear/rust-rosetta
[rustc-guide]: https://github.com/rust-lang/rust/blob/master/src/librustc/README.md
[rustc-perf]: http://ncameron.org/perf-rustc/
[rustfmt]: https://github.com/nrc/rustfmt
[src/doc]: https://github.com/rust-lang/rust/tree/master/src/doc
[std]: http://doc.rust-lang.org/std/index.html
[team]: team.html
[test]: https://github.com/rust-lang/rust-wiki-backup/blob/master/Note-testsuite.md
[testsuite]: https://github.com/rust-lang/rust-wiki-backup/blob/master/Note-testsuite.md
[tips]: https://github.com/rust-lang/rust/blob/3d1f3c9d389d46607ae28c51cc94c1f43d65f3f9/Makefile.in#L48
[trending]: https://github.com/trending?l=rust
[triage]: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#issue-triage
[user groups]: user_groups.html
[users.rust-lang.org]: https://users.rust-lang.org

