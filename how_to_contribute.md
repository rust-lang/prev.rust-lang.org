---
layout: default
title: How To Contribute
---

# Contributing to Rust

If you're learning Rust, you've no doubt been amazed at how awesome
the language is and will be keen to write more Rust code. Hopefully,
you'll also want to make a meaningful contribution to the community.
If you're not sure how best to do that, then hopefully this page will
help.

Most of this page is filled with links to lists of issues to help you
find something to work on. There are issues on the Rust project
itself, a number of supporting projects, and some projects which are
large users of Rust.

Writing code isn't the only way to make a meaningful contribution.
Writing tests and documentation is how many people get started and is
really useful. There are links to these kind of issues too. Likewise
finding bugs and filing issues in any of the projects linked here is
very much appreciated.

As a reminder, all contributors are expected to follow our [Code of
Conduct](https://www.rust-lang.org/conduct.html).

## Finding Something to Work On

#### Issues

If you're looking for
[issues](https://github.com/rust-lang/rust/issues) on the Rust repo
and want something easy-ish to start with, look for the
[E-easy](https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AE-easy)
or
[E-mentor](https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AE-easy+label%3AE-mentor)
labels. If no one is specifically mentioned as a mentor, try pinging
the author or assignee on IRC to see if they can mentor or recommend
someone.

Contributors with sufficient permissions on the Rust repo can help by
adding labels to triage issues:

Yellow
A-prefixed labels state which area of the project an issue relates to.

Magenta
B-prefixed labels identify bugs which belong elsewhere.

Green
E-prefixed labels explain the level of experience necessary to fix the
issue.

Red
I-prefixed labels indicate the importance of the issue. The
I-nominated label indicates that an issue has been nominated for
prioritizing at the next triage meeting.

Orange
P-prefixed labels indicate a bug's priority. These labels are only
assigned during triage meetings, and replace the I-nominated label.

Blue
T-prefixed bugs denote which team the issue belongs to.

Dark blue
beta- labels track changes which need to be backported into the beta branches.

Purple
The purple metabug label marks lists of bugs collected by other categories.

#### Checking Older Bugs

Sometimes, an issue will stay open, even though the bug has been
fixed. And sometimes, the original bug may go stale because something
has changed in the meantime. It can be helpful to go through older bug
reports and make sure that they are still valid. Load up an older
issue, double check that it's still true, and leave a comment letting
us know if it is or is not. The [least recently
updated](https://github.com/rust-lang/rust/issues?q=is%3Aissue+is%3Aopen+sort%3Aupdated-asc)
sort is good for finding issues like this.

#### The Rust compiler

The compiler is part of the [main repo](https://github.com/rust-lang),
which also includes the standard library crates and a whole bunch of
supporting code. For questions about the compiler, there is the #rustc
irc channel. Compiler errors (ICE for 'internal compiler errors') can
be searched for in issues using the
[I-ICE](https://github.com/rust-lang/rust/labels/I-ICE) label. These
are usually good bugs to start with because it's easy to know when
you've fixed them, and they're often relatively self-contained. If
you're interested in parsing, macros, syntactic stuff, the
[parsing](https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-parser)
label and the
[macro](https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-parser+label%3AA-macros)
label are a good places to start.

    Backend/codegen Some knowledge of LLVM is helpful here.

#### Standard Libraries

[Libstd](https://github.com/rust-lang/rust/tree/master/src/libstd),
[libcollections](https://github.com/rust-lang/rust/tree/master/src/libcollections),
[liballoc](https://github.com/rust-lang/rust/tree/master/src/liballoc)
and
[libcore](https://github.com/rust-lang/rust/tree/master/src/libcore)
are the main library crates.  These all appear to users as if they are
part of libstd, the standard library. These tend to be very
fundamental libraries - built-in types, low level io and concurrency,
allocation, essential collections, and so forth. You should join
\#rust-libs if you are interested in contributing to the Rust
libraries.

#### Cargo

Cargo [issues](https://github.com/rust-lang/cargo/issues)

[TODO]

#### Tests

Contributing tests is extremely valuable to the Rust project. For the
compiler and standard libraries, there are unit tests (usually)
embedded in the source code and regression tests in the
[src/tests](https://github.com/rust-lang/rust/tree/master/src/test)
directory. There is a list of issues which have (probably) been fixed,
but still need a test. Nearly any other project will be extremely
welcoming of new tests too.  Writing test cases is a great way to
begin to understand a new project and get started contributing.

#### Documentation

Documentation is never good enough and there's never enough of it.
Writing docs is a really valuable way to contribute to open source
projects. Many aspects of the docs don't require super-deep knowledge,
especially if they're aimed at newcomers. It's also a great way to
learn more about the language or specific libraries.

[The Book](https://doc.rust-lang.org/book/)
The main guide to learning Rust

Documentation
Better documentation throughout the Rust project - mostly for libraries.

[Rust by Example](http://rustbyexample.com/)

[Rust for C++ Programmers](https://www.gitbook.com/book/aminb/rust-for-c/details)
A tutorial aimed at experienced C++ programmers coming to Rust.

#### Tools

Tools play a huge part in the success of a language. Rust has some
great tool support (in particular with debugging and package
management), but we need much more.

    Cargo Rust's package manager and build system (#cargo is an irc channel dedicated to Cargo).
    Rustdoc Produces documentation for the official libraries and user projects.
    Debugging Rust support in GDB and LLDB. Mostly compiler issues.
    DXR Source code exploration and cross-referencing.
    Racer Code completion.
    rustfmt Code formatting.

#### Infrastructure

These projects support the Rust project. They're mostly not written in
Rust, but if you have experience with GitHub, Python, or other such
technologies you can make really helpful impact here.

    Homu (bors) Continuous integration and queue management for the Rust project.
    Highfive Generally useful bot.
    Rustaceans.org Rust community phonebook.
    Rustbook Used for making the book/guide.

#### Other projects

Try [Github Trending](https://github.com/trending?l=rust) for
currently active Rust projects.

## Feature Requests

To request a change to the way that the Rust language works, please open an
issue in the [RFCs repository](https://github.com/rust-lang/rfcs/issues/new)
rather than this one. New features and other significant language changes
must go through the RFC process.

## Bug Reports

While bugs are unfortunate, they're a reality in software. We can't fix what we
don't know about, so please report liberally. If you're not sure if something
is a bug or not, feel free to file a bug anyway.

If you have the chance, before reporting a bug, please [search existing
issues](https://github.com/rust-lang/rust/search?q=&type=Issues&utf8=%E2%9C%93),
as it's possible that someone else has already reported your error. This doesn't
always work, and sometimes it's hard to know what to search for, so consider this
extra credit. We won't mind if you accidentally file a duplicate report.

Opening an issue is as easy as following [this
link](https://github.com/rust-lang/rust/issues/new) and filling out the fields.
Here's a template that you can use to file a bug, though it's not necessary to
use it exactly:

    <short summary of the bug>

    I tried this code:

    <code sample that causes the bug>

    I expected to see this happen: <explanation>

    Instead, this happened: <explanation>

    ## Meta

    `rustc --version --verbose`:

    Backtrace:

All three components are important: what you did, what you expected, what
happened instead. Please include the output of `rustc --version --verbose`,
which includes important information about what platform you're on, what
version of Rust you're using, etc.

Sometimes, a backtrace is helpful, and so including that is nice. To get
a backtrace, set the `RUST_BACKTRACE` environment variable. The easiest way
to do this is to invoke `rustc` like this:

```bash
$ RUST_BACKTRACE=1 rustc ...
```
## How to Make a Pull Request

Rust and all the projects listed here use Git and GitHub for version
control and to manage contributing. If you're new to Git a good
starting point for git is [Scott Chacon's
book](https://git-scm.com/book/en/v2).

Pull requests are the primary mechanism we use to change Rust. GitHub
itself has some great documentation such as the [Github
Guides](https://guides.github.com/) and more specifically on using
[pull-requests](https://help.github.com/articles/using-pull-requests/).
We use the 'fork and pull' model described there.



Please make pull requests against the `master` branch.

Compiling all of `make check` can take a while. When testing your pull request,
consider using one of the more specialized `make` targets to cut down on the
amount of time you have to wait. You need to have built the compiler at least
once before running these will work, but that’s only one full build rather than
one each time.

    $ make -j8 rustc-stage1 && make check-stage1

is one such example, which builds just `rustc`, and then runs the tests. If
you’re adding something to the standard library, try

    $ make -j8 check-stage1-std NO_REBUILD=1

This will not rebuild the compiler, but will run the tests.

All pull requests are reviewed by another person. We have a bot,
@rust-highfive, that will automatically assign a random person to review your
request.

If you want to request that a specific person reviews your pull request,
you can add an `r?` to the message. For example, Steve usually reviews
documentation changes. So if you were to make a documentation change, add

    r? @steveklabnik

to the end of the message, and @rust-highfive will assign @steveklabnik instead
of a random person. This is entirely optional.

After someone has reviewed your pull request, they will leave an annotation
on the pull request with an `r+`. It will look something like this:

    @bors: r+ 38fe8d2

This tells @bors, our lovable integration bot, that your pull request has
been approved. The PR then enters the [merge queue][merge-queue], where @bors
will run all the tests on every platform we support. If it all works out,
@bors will merge your code into `master` and close the pull request.

[merge-queue]: http://buildbot.rust-lang.org/homu/queue/rust

Speaking of tests, Rust has a comprehensive test suite. More information about
it can be found
[here](https://github.com/rust-lang/rust-wiki-backup/blob/master/Note-testsuite.md).

## Writing Documentation

Documentation improvements are very welcome. The source of `doc.rust-lang.org`
is located in `src/doc` in the tree, and standard API documentation is generated
from the source code itself.

Documentation pull requests function in the same way as other pull requests,
though you may see a slightly different form of `r+`:

    @bors: r+ 38fe8d2 rollup

That additional `rollup` tells @bors that this change is eligible for a 'rollup'.
To save @bors some work, and to get small changes through more quickly, when
@bors attempts to merge a commit that's rollup-eligible, it will also merge
the other rollup-eligible patches too, and they'll get tested and merged at
the same time.

To find documentation-related issues, sort by the [A-docs label][adocs].

[adocs]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-docs

In many cases, you don't need a full `make doc`. You can use `rustdoc` directly
to check small fixes. For example, `rustdoc src/doc/reference.md` will render
reference to `doc/reference.html`. The CSS might be messed up, but you can
verify that HTML is right.

## Other Contributions

There are a number of other ways to contribute to Rust that don't deal
directly with the Rust repository.

Answer questions in
[#rust](http://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust),
on the [Rust Forum](https://users.rust-lang.org/) or on [Stack
Overflow](http://stackoverflow.com/questions/tagged/rust).

Participate in the [RFC process](https://github.com/rust-lang/rfcs).

Find a [requested community
library](https://github.com/rust-lang/rfcs/labels/A-community-library)
, build it, and publish it to [Crates.io](http://crates.io). Easier
said than done, but very, very valuable!

## Helpful Links and Information

For people new to Rust, and just starting to contribute, or even for
more seasoned developers, some useful places to look for information
are:

* The [Rust Internals forum][rif], a place to ask questions and
  discuss Rust's internals
* The [generated documentation for rust's compiler][gdfrustc]
* The [rust reference][rr], even though it doesn't specifically talk about Rust's internals, it's a great resource nonetheless
* Although out of date, [Tom Lee's great blog article][tlgba] is very helpful
* [rustaceans.org][ro] is helpful, but mostly dedicated to IRC
* The [Rust Compiler Testing Docs][rctd]
* For @bors, [this cheat sheet][cheatsheet] is helpful (Remember to replace `@homu` with `@bors` in the commands that you use.)
* **Google!** ([search only in Rust Documentation][gsearchdocs] to find types, traits, etc. quickly)
* Don't be afraid to ask! The Rust community is friendly and helpful.

[gdfrustc]: http://manishearth.github.io/rust-internals-docs/rustc/
[gsearchdocs]: https://www.google.de/search?q=site:doc.rust-lang.org+your+query+here
[rif]: http://internals.rust-lang.org
[rr]: https://doc.rust-lang.org/book/README.html
[tlgba]: http://tomlee.co/2014/04/03/a-more-detailed-tour-of-the-rust-compiler/
[ro]: http://www.rustaceans.org/
[rctd]: ./COMPILER_TESTS.md
[cheatsheet]: http://buildbot.rust-lang.org/homu/
