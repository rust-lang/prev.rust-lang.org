---
layout: default
title: Working Groups &middot; The Rust Programming Language
---

# Rust working groups

First, what *is* a working group and why should you care? It’s a group appointed to study a particular question and make recommendations based on its findings. In Rust terms, working groups focus on improving the **end-to-end user experience** of using Rust in each domain. They also provide a space for discussion and coordination between various stakeholders.

Each working group sets its own goals and reports directly to the core team and determines how often to meet. Successful groups encourage community input and support by regularly seeking feedback.

## Current working groups


- **Networking services**: focusing on the end-to-end experience for both sync and async networking code, in coordination with the growing ecosystem in this space.
  - POC: [@withoutboats](https://internals.rust-lang.org/u/withoutboats) and [@cramertj](https://internals.rust-lang.org/u/cramertj)
  - Contributing: [channel](https://discord.gg/rust-lang) #wg-net | [repository](https://github.com/rust-lang-nursery/wg-net)
- **WebAssembly**: focusing on the end-to-end experience of embedding Rust code in JS libraries and apps via WebAssembly.
  - POC: [@fitzgen](https://internals.rust-lang.org/u/fitzgen)
  - Contributing: [channel](https://client00.chat.mibbit.com/?channel=%23rust-wasm&server=irc.mozilla.org) | [repository](https://github.com/rustwasm/team)
- **CLI apps**:  focusing on the end-to-end experience of writing CLI apps, both large and small, in Rust.
  - POC: [@killercup](https://internals.rust-lang.org/u/killercup)
  - Contributing: [channel](https://gitter.im/rust-lang/WG-CLI) | [repository](https://github.com/rust-lang-nursery/cli-wg)
- **Embedded devices**: focusing on the end-to-end experience of using Rust in resource-constrained environments and non-traditional platforms.
  - POC:[@japaric](https://internals.rust-lang.org/u/japaric)
  - Contributing: [channel](https://client00.chat.mibbit.com/?channel=%23rust-embedded&server=irc.mozilla.org) | [repository](https://github.com/rust-embedded/wg)
- **Unsafe code guidelines**: focusing on producing a “reference guide” for writing unsafe code
  - POC: [@avadacatavra](https://internals.rust-lang.org/u/avadacatavra) and [@nikomatsakis](https://internals.rust-lang.org/u/nikomatsakis)
  - Contributing: [channel](https://rust-lang.zulipchat.com/#narrow/stream/136281-wg-unsafe-code-guidelines) | [repository](https://github.com/rust-rfcs/unsafe-code-guidelines)
- **Formal verification**: focusing on enabling the use of formal methods and verification tools with Rust programs
  - POC: [@avadacatavra](https://internals.rust-lang.org/u/avadacatavra)
  - Contributing: [channel](https://gitter.im/rust-lang/wg-verification) | [repository](https://github.com/rust-lang-nursery/wg-verification)


## Joining a working group

Joining a working group is as easy as looking at what’s going on in the working group repository, and seeing how you can contribute. You can also pop into the chat or reach out to the group leads for more specific questions.

## Starting a new working group

Propose your working group on [Internals](https://internals.rust-lang.org/) to get feedback. Once the working group is official, set up a repository and channel (and open up a PR [here](https://github.com/rust-lang/rust-www)!) — a number of groups have their repositories in the [Rust lang nursery](https://github.com/rust-lang-nursery) organization, but what really matters is that people can *find* the information.

Suggestions:

- Consider the WG goals—what question are you exploring? What will success look like?
- Who can lead/coordinate?
- Is there overlap with other WGs? How will this be coordinated?
- What is an appropriate starting point?

