# Contributing to the Rust website

Thank you for your interest in contributing to the Rust site!

As a reminder, all contributors are expected to follow our [Code of Conduct][coc].

[coc]: https://www.rust-lang.org/conduct.html

## What this repo isn't

The documentation, `doc.rust-lang.org`, is built from the [docs in
tree][rustdocs]. 

The [playpen][playpen] source lives in [its own repo][playsrc].

The [crates.io][crates] source lives [on github][cratessrc].

[crates]: https://crates.io/
[cratessrc]: https://github.com/rust-lang/crates.io
[playpen]: https://play.rust-lang.org/
[playsrc]: https://github.com/rust-lang/rust-playpen/
[rustdocs]: https://github.com/rust-lang/rust/tree/master/src/doc

## Troubleshooting this site

Every page has the timestamp at which it was last generated embedded in the
source code. You can view a page's source by right clicking and selecting
"view source". The timestamp will look like `<!-- Page last generated
2015-08-04 12:00:42 -0700 -->`. 

## Bug Reports

Opening an issue is as easy as following [this
link](https://github.com/rust-lang/rust-www/issues/new) and filling out the fields.

## Pull Requests

Pull requests are the primary mechanism we use to change Rust. GitHub itself
has some [great documentation][pull-requests] on using the Pull Request
feature. We use the 'fork and pull' model described there.

[pull-requests]: https://help.github.com/articles/using-pull-requests/

Please make pull requests against the `master` branch.

All pull requests are reviewed by another person. We have a bot,
@rust-highfive, that will automatically assign a random person to review your
request.

If you want to request that a specific person reviews your pull request,
you can add an `r?` to the message. For example, Steve usually reviews
documentation changes. So if you were to make a documentation change, add

    r? @steveklabnik

to the end of the message, and @rust-highfive will assign @steveklabnik instead
of a random person. This is entirely optional.
