---
layout: faq
title: Frequently Asked Questions &middot; The Rust Programming Language
---

# Frequently Asked Questions

This page exists to answer common questions about the Rust programming language. It is not a complete guide to the language, nor is it a tool for teaching the language. It is a reference to answer oft-repeated questions people in the Rust community encounter, and to clarify the reasoning behind some of Rust's design decisions.

If there is some common or important question you feel is wrongly left unanswered here, feel free to [help us fix it](https://github.com/rust-lang/rust-www/blob/master/CONTRIBUTING.md).

<div id="toc">
    <h2>Table of Contents</h2><a href="#toggle-toc"></a>
    <ol id="toc-contents">
        <li><a href="#performance">Performance</a></li>
        <li><a href="#syntax">Syntax</a></li>
        <li><a href="#numerics">Numerics</a></li>
        <li><a href="#strings">Strings</a></li>
        <li><a href="#collections">Collections</a></li>
        <li><a href="#ownership">Ownership</a></li>
        <li><a href="#lifetimes">Lifetimes</a></li>
        <li><a href="#generics">Generics</a></li>
        <li><a href="#input-output">Input / Output</a></li>
        <li><a href="#error-handling">Error Handling</a></li>
        <li><a href="#concurrency">Concurrency</a></li>
        <li><a href="#macros">Macros</a></li>
        <li><a href="#debugging">Debugging</a></li>
        <li><a href="#low-level">Low-Level</a></li>
        <li><a href="#cross-platform">Cross-Platform</a></li>
        <li><a href="#modules-and-crates">Modules and Crates</a></li>
        <li><a href="#libraries">Libraries</a></li>
        <li><a href="#design-patterns">Design Patterns</a></li>
        <li><a href="#other-languages">Other Languages</a></li>
        <li><a href="#documentation">Documentation</a></li>
        <li><a href="#project">The Rust Project</a></li>

    </ol>
</div>

<h2 id="performance">Performance</h2>

### How fast is Rust?

Fast! Rust is already competitive with idiomatic C and C++ in a number of benchmarks (like the [Benchmarks Game](http://benchmarksgame.alioth.debian.org/u64q/compare.php?lang=rust&lang2=gpp) and [others](https://github.com/kostya/benchmarks)).

It is an explicit goal of Rust to be at least as fast as C++. Language decisions are made with performance in mind, and given that Rust is built on LLVM and strives to resemble Clang from LLVM's perspective, any LLVM performance improvements also help Rust.

### Is Rust garbage collected?

No. A language that requires a GC is a language that opts into a larger, more complex runtime than Rust cares for. Rust is usable on bare metal with no extra runtime.

Additionally, garbage collection is frequently a source of non-deterministic behavior. Rust provides tools to make using third-party garbage collectors [possible](http://manishearth.github.io/blog/2015/09/01/designing-a-gc-in-rust/), but it is not part of the language as provided.

### Why is my program slow?

The Rust compiler doesn't compile with optimizations unless asked to, [as optimizations slow down compilation and are usually undesirable during development](https://users.rust-lang.org/t/why-does-cargo-build-not-optimise-by-default/4150/3).

If you compile with `cargo`, use the `--release` flag. If you compile with `rustc` directly, use the `-O` flag. Either of these will turn on optimizations.

### Why is Rust compilation slow?

Code translation and optimizations. Rust provides high level abstractions that compile down into efficient machine code, and those translations take time to run, especially when optimizing.

But Rust's compilation time is not as bad as it may seem, and there is reason to believe it will improve. When comparing projects of similar size between C++ and Rust, compilation time of the entire project is generally believed to be comparable. The common perception that Rust compilation is slow is in large part due to the differences in the *compilation model* between C++ and Rust: C++'s compilation unit is the file, while Rust's is the crate, composed of many files. Thus, during development, modifying a single C++ file can result in much less recompilation than in Rust. There is a major effort underway to refactor the compiler to introduce [incremental compilation](https://github.com/rust-lang/rfcs/blob/master/text/1298-incremental-compilation.md), which will provide Rust the compile time benefits of C++'s model.

Aside from the compilation model, there are several other aspects of Rust's language design and compiler implementation that affect compile-time performance. First, Rust has a moderately-complex type system, and must spend a non-negligible amount of compile time enforcing the constraints that make Rust safe at runtime. Secondly, Rust's use of LLVM for code generation is a double-edged sword: while it enables Rust to have world-class runtime performance, LLVM is a large framework that is not focused on compile-time performance. Thirdly, the Rust compiler suffers from long-standing technical debt, and notably generates poor-quality LLVM IR which LLVM must spend time "fixing". There is hope that future [MIR-based](https://github.com/rust-lang/rfcs/blob/master/text/1211-mir.md) optimization and translation passes will ease the burden the Rust compiler places on LLVM. Finally, while Rust's preferred strategy of monomorphising generics (ala C++) produces fast code, it demands that significantly more code be generated than other translation strategies.

### Why are Rust's `HashMap`s slow?

By default, Rust's `HashMap` uses the [SipHash](https://131002.net/siphash/) hashing algorithm, which is designed to prevent [hash table collision attacks](http://programmingisterrible.com/post/40620375793/hash-table-denial-of-service-attacks-revisited) while providing [reasonable performance on a variety of workloads](https://www.reddit.com/r/rust/comments/3hw9zf/rust_hasher_comparisons/cub4oh6).

While SipHash [demonstrates competitive performance](http://cglab.ca/%7Eabeinges/blah/hash-rs/) in many cases, one case where it is notably slower than other hashing algorithms is with short keys, such as integers. This is why Rust programmers often observe slow performance with `HashMap`. The [FNV hasher](https://crates.io/crates/fnv) is frequently recommended for these cases, but be aware that it does not have the same collision-resistence properties as SipHash.

### Why can't I run benchmarks?

You can run benchmarks, but only on the nightly channel. Rust's benchmarking mechanism is currently unstable, as the API has not been deemed ready for stabilization. This [may change in the future](https://github.com/rust-lang/rust/issues/29553), but until then benchmarking can only be used on nightly.

### Does Rust do tail-call optimization?

Not generally, no. Tail-call optimization may be done in [limited circumstances](http://llvm.org/docs/CodeGenerator.html#sibling-call-optimization), but is [not guaranteed](https://mail.mozilla.org/pipermail/rust-dev/2013-April/003557.html). As the feature has always been desired, Rust has a keyword (`become`) reserved, though it is not clear yet whether it is technically possible, nor whether it will be implemented. There was a [proposed extension](https://github.com/rust-lang/rfcs/pull/81) that would allow tail-call elimination in certain contexts, but it is currently postponed. 

### Does Rust have a runtime?

Not in the typical sense used by languages such as Java, but parts of the Rust standard library can be considered a "runtime", providing a heap, backtraces, unwinding, and stack guards. There is a [small amount of initialization code](https://github.com/rust-lang/rust/blob/33916307780495fe311fe9c080b330d266f35bfb/src/libstd/rt.rs#L43) that runs before the user's `main` function. The Rust standard library additionally links to the the C standard library, which does similar [runtime initialization](http://www.embecosm.com/appnotes/ean9/html/ch05s02.html). Rust code can be compiled without the standard library, in which case there is no runtime at all.

<h2 id="syntax">Syntax</h2>

### Why curly braces? Why can't everything be like Haskell/Python?

Use of curly braces to denote blocks is a common design choice in a variety of programming languages, and Rust's consistency is useful for people already familiar with the style.

Curly braces also allow for more flexible syntax for the programmer, a simpler parser in the compiler, and help reduce the possibility of logic mistakes caused by incorrect indentation, like Apple's [goto fail](https://gotofail.com/) bug.

### I can leave out parentheses on `if` conditions, so why do I have to put brackets around single line blocks? Why is the C style not allowed?

Whereas C requires mandatory parantheses for `if`-statements but leaves brackets optional, Rust makes the opposite choice. Rust's `if`-expressions thus require strictly fewer delimiters than their C counterparts. Furthermore, the optional brackets on C's `if`-statements are well-understood as a hazard to maintenance and refactoring.

### Why is there no literal syntax for dictionaries?

The reason Rust does not have syntax for initializing dictionaries &mdash; or collections in general &mdash; is due to Rust's overall design preference for limiting the size of the *language* while enabling powerful *libraries*. The only type of collection that Rust has direct syntax for initializing is the array type, which is also the only type of collection built into the language. Note that Rust does not even have syntax for initializing the common `Vec` collection type, instead the standard library defins the [`vec!`](http://doc.rust-lang.org/std/macro.vec!.html) macro.

This design choice of using Rust's macro facilities to initialize collections will likely be extended generically to other collections in the future, enabling simple initialization of not only `HashMap` and `Vec`, but also other collection types such as [`BTreeMap`](http://doc.rust-lang.org/collections/struct.BTreeMap.html). In the meantime, if you want a more convenient syntax for initializing collections, you can [create your own macro](http://stackoverflow.com/questions/27582739/how-do-i-create-a-hashmap-literal) to provide it.

### When should I use an implicit return?

Implicit returns are simply a coding style option, and can be used anywhere they make sense. While early returns require an explicit `return`, any other return can be made implicit according to your preferences or the preferences of your project. In the Rust project itself, the style guidelines lean toward preferring implicit returns.

### Why aren't function signatures inferred?

- Mechanically, it simplifies the inference algorithm, as inference only requires looking at one function at a time.
- Mandatory function signatures help enforce interface stability at both the module and crate level.
- It improves code comprehension for the programmer, eliminating the need for an IDE running an inference algorithm across an entire crate to be able to guess at a function's argument types; it's always explicit and nearby.

### Why does `match` have to be exhaustive?

`match` being exhaustive has some useful properties. First, if every possibility is covered by the `match`, adding variants to the `enum` in the future will cause a compilation failure, rather than an error at runtime. Second, it makes the semantics of the default case explicit: in general, the only safe way to have a non-exhaustive `match` would be to panic the thread if nothing is matched. Early versions of Rust did not require `match` cases to be exhaustive and it was found to be a great source of bugs.

It is easy to ignore all unspecified cases by using the `_` wildcard:

```rust
match val.do_something() {
    Cat(a) => { /* ... */ }
    _      => { /* ... */ }
}
```

<h2 id="numerics">Numerics</h2>

### Which of `f32` and `f64` should I prefer for floating-point math?

The choice of which to use is dependent on the purpose of the program.

If you are interested in the greatest degree of precision with your floating point numbers, then prefer `f64`. If you are more interested in keeping the size of the value small or being maximally efficient, and are not concerned about the associated innacuracy of having fewer bits per value, then `f32` is better. Operations on `f32` are usually faster, even on 64-bit hardware. As a common example, graphics programming typically uses `f32` because it requires high performance, and 32-bit floats are sufficient for representing pixels on the screen.

If in doubt, choose `f64` for the greater precision.

### Why can't I use `f32` or `f64` as `HashMap` keys?

In order to be used as a key in a `HashMap`, a type must implement the `Eq` and `Hash` traits. `f32` and `f64` implement `PartialEq`, but not `Eq`, because one of the potential values for floating-point types is `NaN` (or "not a number"). `NaN` values are [not equal to any float, and are not equal to each other](https://en.wikipedia.org/wiki/NaN). This means that `f32` and `f64` can't be used as keys in a HashMap.

### Why can't I compare floats?

Floats can be compared in some circumstances, but not others. Floats do implement the `PartialOrd` trait, which means that `>`, `<`, `<=`, and `>=` are defined for them. But they do _not_ implement the `Ord` trait, because (thanks to `NaN`) [there is no total ordering for floating point numbers](https://en.wikipedia.org/wiki/IEEE_floating_point#Total-ordering_predicate). The practical effect of this is that data structures that require a total ordering, such as [`BTreeMap`](http://doc.rust-lang.org/std/collections/struct.BTreeMap.html) do not work with floating point numbers.

There [is a crate](https://crates.io/crates/ordered-float) that provides a total ordering on floats by wrapping them, which may be desirable depending on your use-case. You can still do all of the normal comparison operations you would expect, but you can't use the `cmp` function.

### How can I convert between numeric types?

There are two ways: the `as` keyword, which does simple casting for primitive types, and the `Into` and `From` traits, which are implemented for a number of type conversions (and which you can implement for your own types). The `Into` and `From` traits are only implemented in cases where conversions are lossless, so for example, `f64::from(0f32)` will compile while `f32:from(0f64)` will not. On the other hand, `as` will convert between any two primitive types, truncating values as necessary.

<h2 id="strings">Strings</h2>

### How can I convert a `String` or `Vec<T>` to a slice (`&str` and `&[T]`)?

Using [Deref coercions](https://doc.rust-lang.org/stable/book/deref-coercions.html), `Strings` and `Vec`s will automatically coerce to their respective slices when passed by reference with `&` or `& mut`.

### How can I convert from `&str` to `String` or the other way around?

The `to_owned()` method converts from a `&str` into a `String`, and `String`s are automatically converted into `&str` when you borrow a reference to them. Both are demonstrated in the following example:

```rust
fn main() {
    let s = "Jane Doe".to_owned();
    say_hello(&s);
}

fn say_hello(name: &str) {
    println!("Hello {}!", name);
}
```

### What are the differences between the two different string types?

`String` is an owned buffer of UTF-8 bytes allocated on the heap. Mutable `String`s can be modified, growing their capacity as needed. `&str` is a fixed-capacity "view" into a `String` allocated elsewhere, commonly on the heap, in the case of slices dereferenced from `String`s, or in static memory, in the case of string literals.

`&str` is a primitive type implemented by the Rust language, while `String` is implemented in the standard library.

### How do I do O(1) character access in a `String`?

You cannot. At least not without a firm understanding of what you mean by "character", and preprocessing the string to find the index of the desired character.

Rust strings are UTF-8 encoded. A single visual character in UTF-8 is not necessarily a single byte as it would be in an ASCII-encoded string. Each byte is called a "code unit" (in UTF-16, code units are 2 bytes; in UTF-32 they are 4 bytes). "Code points" are composed of one or more code units, and combine in "grapheme clusters" which most closely approximate characters.

Thus, even though you may index on bytes in a UTF-8 string, you can't access the `i`th code point or grapheme cluster in constant time. However, if you know at which byte that desired code point or grapheme cluster begins, then you _can_ access it in constant time. Functions including `str::find()` and regex matches return byte indices, facilitating this sort of access.

### Why are strings UTF-8 by default?

The `str` type is UTF-8 because we observe more text in the wild in this encoding – particularly in network transmissions, which are endian-agnostic – and we think it's best that the default treatment of I/O not involve having to recode codepoints in each direction.

This does mean that locating a particular Unicode codepoint inside a string is an O(n) operation, although if the starting byte index is already known then they can be accessed in O(1) as expected. On the one hand, this is clearly undesirable; on the other hand, this problem is full of trade-offs and we'd like to point out a few important qualifications:

Scanning a `str` for ASCII-range codepoints can still be done safely byte-at-a-time. If you use `.as_bytes()`, pulling out a `u8` costs only `O(1)` and produces a value that can be cast and compared to an ASCII-range `char`. So if you're (say) line-breaking on `'\n'`, byte-based treatment still works. UTF8 was well-designed this way.

Most "character oriented" operations on text only work under very restricted language assumptions such as "ASCII-range codepoints only". Outside ASCII-range, you tend to have to use a complex (non-constant-time) algorithm for determining linguistic-unit (glyph, word, paragraph) boundaries anyway. We recommend using an "honest" linguistically-aware, Unicode-approved algorithm.

The `char` type is UTF32. If you are sure you need to do a codepoint-at-a-time algorithm, it's trivial to write a `type wstr = [char]`, and unpack a `str` into it in a single pass, then work with the `wstr`. In other words: the fact that the language is not "decoding to UTF32 by default" shouldn't stop you from decoding (or re-encoding any other way) if you need to work with that encoding.

For a more in-depth explanation of why UTF-8 is usually preferable over UTF-16 or UTF-32, read the [UTF-8 Everywhere manifesto](http://utf8everywhere.org/).

<h2 id="collections">Collections</h2>

### Can I implement data structures like vectors and linked lists efficiently in Rust?

If your reason for implementing these data structures is to use them for other programs, there's no need, as efficient implementations of these data structures are provided by the standard library.

If, however, [your reason is simply to learn](http://cglab.ca/~abeinges/blah/too-many-lists/book/), then you will likely need to dip into unsafe code. While these data structures _can_ be implemented entirely in safe Rust, the performance is likely to be worse than it would be with the use of unsafe code. The simple reason for this is that data structures like vectors and linked lists rely on pointer and memory operations that are disallowed in safe Rust.

For example, a doubly-linked list requires that there be two mutable references to each node, but this violates Rust's mutable reference aliasing rules. You can solve this using `Weak<T>`, but the performance will be poorer than you likely want. With unsafe code you can bypass the mutable reference aliasing rule restriction, but must manually verify that your code introduces no memory safety violations.

### How can I iterate over a collection without moving/consuming it?

The easiest way is by using the collection's `IntoIterator` implementation. Here is an example for `Vec`:

```rust
let v = vec![1,2,3,4,5];
for item in &v {
    print!("{} ", item);
}
println!("\nLength: {}", v.len());
```

The way Rust `for` loops work, they actually call `into_iter()` (which is defined in the `IntoIterator` trait) for whatever you are trying to iterate over. `IntoIterator` is implemented for `&Vec<T>` and `&mut Vec<T>`, meaning you can iterate over a vector without consuming it by using `&v` or `&mut v` (for some vector `v`). The same is true for the other standard collections as well.

### Why do I need to type the array size in the array declaration?

You don't necessarily have to. If you're declaring an array directly, the size is inferred based on the number of elements. But if you're declaring a function that takes an array, the compiler has to know how big that array will be.

One thing to note is that currently Rust doesn't offer generics over arrays of different size. If you'd like to accept a contiguous container of a variable number of values, use a `Vec` or slice (depending on whether you need ownership).

<h2 id="ownership">Ownership</h2>

### How can I implement a graph or other data structure that contains cycles?

There are three major options:

- You can implement it using `Rc` and `Weak` to allow shared ownership of nodes,
although this approach pays the cost of memory management.
- You can implement it using `unsafe` code using raw pointers. This will be
more efficient, but bypasses Rust's safety guarantees.
- Using vectors and indices into those vectors. There are [several](http://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/) [available](http://featherweightmusings.blogspot.com/2015/04/graphs-in-rust.html) examples and explanations of this approach.

### How can I define a struct that contains a pointer to one of its own fields?

It's possible, but useless to do so. The struct becomes permanently borrowed by itself and therefore can't be moved. Here is some code illustrating this:

```rust
use std::cell::Cell;

#[derive(Debug)]
struct Unmovable<'a> {
    x: u32,
    y: Cell<Option<&'a u32>>,
}


fn main() {
    let test = Unmovable { x: 42, y: Cell::new(None) };
    test.y.set(Some(&test.x));

    println!("{:?}", test);
}
```

### What does it mean to "consume a value"?

"Consuming a value" means taking ownership of a value. When this is done, the value can't be used elsewhere.

### What is the difference between consuming and moving/taking ownership?

These are different terms for the same thing. In both cases, it means the value has been moved to another owner, and moved out of the calling owner.

### Why can values of some types be used after passing them to a function, while reuse of values of other types results in an error?

If a type implements the `Copy` trait, then it will be copied when passed to a function. All numeric types in Rust implement `Copy`, but struct types do not implement `Copy` by default, so they are moved instead. This means that the struct can no longer be used elsewhere, unless it is moved back out of the function via the return.

### How do you deal with a "use of moved value" error?

This error means that the value you're trying to use has been moved to a new owner. The first thing to check is whether the move in question was necessary: if it moved into a function, it may be possible to rewrite the function to use a reference, rather than moving. Otherwise if the type being moved implements `Clone`, then calling `clone()` on it before moving will move a copy of it, leaving the original still available for further use. Note though that cloning a value should typically be the last resort since cloning can be expensive, causing further allocations.

If the moved value is of your own custom type, consider implementing `Copy` (for implicit copying, rather than moving) or `Clone` (explicit copying). `Copy` is most commonly implemented with `#[derive(Copy, Clone)]` (`Copy` requires `Clone`), and `Clone` with `#[derive(Clone)]`.

If none of these are possible, you may want to modify the function that acquired ownership to return ownership of the value when the function exits.

### What are the rules for using `self`, `&self`, or `&mut self` in a method declaration?

- Use `self` when a function needs to consume the type
- Use `&self` when a function only needs a read-only reference to the type
- Use `&mut self` when a function needs to mutate the type without consuming it

### How can I understand the borrow checker?

The borrow checker applies only a few rules, which can be found in the Rust book's [section on borrowing](https://doc.rust-lang.org/stable/book/references-and-borrowing.html#the-rules), when evaluating Rust code. These rules are:

> First, any borrow must last for a scope no greater than that of the owner. Second, you may have one or the other of these two kinds of borrows, but not both at the same time:
>
> - one or more references (&T) to a resource.
> - exactly one mutable reference (&mut T)

While the rules themselves are simple, making sure your code conforms to them can be complicated. It is easy to attempt to borrow a value for longer than its lifetime, or to introduce two mutable references to the same value. The borrow checker is useful because it verifies that your code follows these rules, and helps guide corrections when the code doesn't. Many new Rust programmers struggle to satisfy the borrow checker at first, but over time become more skilled at writing memory safe code with minimal borrow checker intervention.

### How do deref coercions work?

[Deref coercions](https://doc.rust-lang.org/book/deref-coercions.html) exist to make using Rust more ergonomic, and are implemented via the [`Deref`](https://doc.rust-lang.org/stable/std/ops/trait.Deref.html) trait, which looks like

```rust
pub trait Deref {
    type Target: ?Sized;
    fn deref(&self) -> &Self::Target;
}
```

A Deref implementation indicates that the implementing type may be converted into `Target` by a call to the `deref` function, which takes an immutable reference of a certain lifetime to the calling struct, and returns a reference of the same lifetime to the target. The `*` prefix operator is shorthand for the `deref` method.

You can see a [full list of `Deref` implementations](https://doc.rust-lang.org/stable/std/ops/trait.Deref.html#implementors) for the standard library in the documentation.

<h2 id="lifetimes">Lifetimes</h2>

### Why lifetimes?

Lifetimes are Rust's answer to the question of memory safety. They allow Rust to ensure memory safety without the performance costs of garbage collection. They are based on a variety of academic work, which can be found in the [Rust book](https://doc.rust-lang.org/stable/book/academic-research.html#type-system).

### Why is the lifetime syntax the way it is?

The `'a` syntax comes from the ML family of programming languages, where `'a` is used to indicate a generic type parameter. For Rust, the syntax had to be something that was unambiguous, noticeable, and fit nicely in a type declaration right alonside traits and references. Alternative syntaxes have been discussed, but this seems to work fine.

### When is `Rc` useful?

This is covered in the [official documentation for `Rc`](https://doc.rust-lang.org/stable/std/rc/), Rust's non-atomically reference-counted pointer type. In short, `Rc` and its thread-safe cousin `Arc` are useful to express shared ownership, and have the system automatically deallocate the associated memory when no one has access to it.

### How do I return a borrow to something I created from a function?

You need to ensure that the borrowed item will outlive the function. This can be done by binding the output lifetime to some input lifetime. An alternative is to return an owning type like `String`. This eliminates the reference issues entirely, at the cost of possibly unnecessary allocations. There is also the `Cow` ("copy on write") type, which will only do the extra allocation if you attempt to mutate the contained value.

### How do I return a closure from a function?

To return a closure from a function, it must be a "move closure", meaning that the closure is declared with the `move` keyword. As [explained in the Rust book](https://doc.rust-lang.org/book/closures.html#move-closures), this gives the closure its own copy of the captured variables, independent of its parent stack frame. Otherwise, returning a closure would be unsafe, as it would allow access to variables that are no longer valid; put another way: it would allow reading potentially invalid memory. The closure must also be wrapped in a `Box`, so that it is allocated on the heap. Read more about this [in the book](https://doc.rust-lang.org/book/closures.html#returning-closures).

### When are lifetimes required to be defined?

Lifetimes can often be elided, as explained in the ["Lifetime elision" section](https://doc.rust-lang.org/book/lifetimes.html#lifetime-elision) of the Rust book. "Elided lifetimes" are those lifetimes which are implicit in any code containing references. They are automatically inserted by the compiler with the three following rules:

- Each elided lifetime in a function’s arguments becomes a distinct lifetime parameter.
- If there is exactly one input lifetime, elided or not, that lifetime is assigned to all elided lifetimes in the return values of that function.
- If there are multiple input lifetimes, but one of them is &self or &mut self, the lifetime of self is assigned to all elided output lifetimes.

If these rules would result in incorrect code elsewhere, then the Rust compiler will provide errors, and you will need to define the relevant lifetimes to correct that error.

### How can Rust guarantee "no null pointers"?

The only way to construct a value of type `&Foo` or `&mut Foo` is to specify an existing value of type `Foo` that the reference points to. In this way Rust makes sure no null references are introduced.

### How do I express the absence of a value without `null`?

You can do that with the [`Option`](https://doc.rust-lang.org/stable/std/option/index.html) type, which can either be `Some(T)` or `None`. `Some(T)` indicates that a value of type `T` is contained within, while `None` indicates the absence of a value.

<h2 id="generics">Generics</h2>

### What is "monomorphisation"?

Monomorphisation is the process by which Rust translates to machine code specific instances of a generic function based on the parameter types of calls to that function. During monomorphisisation a new copy of the generic function is translated for each unique set of types the function is instantiated with. This is the same strategy used by C++. It results in fast code that is specialized for every call-site and statically dispatched, with the tradeoff that functions instantiated with many different types can cause "code bloat", where multiple function instances result in larger binaries than would be created with other translation strategies.

Functions that accept [trait objects](http://doc.rust-lang.org/book/trait-objects.html) instead of type parameters do not undergo monomorphisation. Instead, methods on the trait objects are dispatched dynamically at runtime.

### What's the difference between a function and a closure that doesn't capture any variables?

Functions are a built-in primitive of the language, while closures are essentially syntactic sugar for one of three traits: `Fn`, `FnMut`, and `FnOnce`. When you make a closure, the Rust compiler automatically creates a struct implementing the appropriate trait of those three and containing the captured environment variables as members, and makes it so the struct can be called as a function. Bare functions can not capture an environment.

The big difference between these traits is how they take the `self` parameter. `Fn` takes `&self`, `FnMut` takes `&mut self`, and `FnOnce` takes `self`.

In the end, functions and closures are operationally equivalent, but have different runtime representations due to their different implementations (as a language primitive and a syntactic sugar over traits, respectively).

### What are higher-kinded types, why would I want them, and why doesn't Rust have them?

Higher-kinded types are types with unfilled parameters. Support for higher-kinded types means "incomplete" types may be used anywhere "complete" types can be used, such as in a trait `impl`.

The lack of support for higher-kinded types makes expression of certain ideas more tedious than it would otherwise be. For example, implementing a `Functor` trait (a term for a container which can be mapped over, obeying certain rules) without higher-kinded types would be difficult, as a `Functor` should be implemented for the container regardless of the type of values it contains. With higher-kinded types, the Functor `impl` for some container could leave the container's type parameter unfilled, with no concrete type.

Rust doesn't currently have them simply because they haven't been a priority. There is nothing inherent to the language that stops us from implementing support for higher-kinded types, it just hasn't been done yet.

### What do named type parameters like `<T=Foo>` in generic types mean?

These are called [associated types](https://doc.rust-lang.org/stable/book/associated-types.html), and they allow for the expression of trait bounds that can't be expressed with a `where` clause. In essence, for a generic type with some type parameters, it is often unecessary to include those type parameters in a function taking that generic type as a parameter. The function shouldn't have to care about being generic over the types which make up the generic type (say, the node and edge types in a graph), but only about being generic over the type itself.

### Can I override operators? Which ones and how?

You can provide custom implementations for a variety of operators using their associated traits: `Add` for `+`, `Mul` for `*`. It looks like this:

```rust
use std::ops::Add;

struct Foo;

impl Add for Foo {
    type Output = Foo;
    fn add(self, rhs: Foo) -> Self::Output {
        println!("Adding!");
        self
    }
}
```

The following operators can be overloaded:

| Operation           | Trait        |
|:--------------------|:-------------|
| `+`                 | [`Add`]      |
| `binary -`          | [`Sub`]      |
| `*`                 | [`Mul`]      |
| `/`                 | [`Div`]      |
| `unary -`           | [`Neg`]      |
| `%`                 | [`Rem`]      |
| `&`                 | [`BitAnd`]   |
| <code>&#124;</code> | [`BitOr`]    |
| `^`                 | [`BitXor`]   |
| `!`                 | [`Not`]      |
| `<<`                | [`Shl`]      |
| `>>`                | [`Shr`]      |
| `*`                 | [`Deref`]    |
| `mut *`             | [`DerefMut`] |
| `[]`                | [`Index`]    |
| `mut []`            | [`IndexMut`] |

[`Add`]: https://doc.rust-lang.org/stable/std/ops/trait.Add.html
[`Sub`]: https://doc.rust-lang.org/stable/std/ops/trait.Sub.html
[`Mul`]: https://doc.rust-lang.org/stable/std/ops/trait.Mul.html
[`Div`]: https://doc.rust-lang.org/stable/std/ops/trait.Div.html
[`Neg`]: https://doc.rust-lang.org/stable/std/ops/trait.Neg.html
[`Rem`]: https://doc.rust-lang.org/stable/std/ops/trait.Rem.html
[`BitAnd`]: https://doc.rust-lang.org/stable/std/ops/trait.BitAnd.html
[`BitOr`]: https://doc.rust-lang.org/stable/std/ops/trait.BitOr.html
[`BitXor`]: https://doc.rust-lang.org/stable/std/ops/trait.BitXor.html
[`Not`]: https://doc.rust-lang.org/stable/std/ops/trait.Not.html
[`Shl`]: https://doc.rust-lang.org/stable/std/ops/trait.Shl.html
[`Shr`]: https://doc.rust-lang.org/stable/std/ops/trait.Shr.html
[`Deref`]: https://doc.rust-lang.org/stable/std/ops/trait.Deref.html
[`DerefMut`]: https://doc.rust-lang.org/stable/std/ops/trait.DerefMut.html
[`Index`]: https://doc.rust-lang.org/stable/std/ops/trait.Index.html
[`IndexMut`]: https://doc.rust-lang.org/stable/std/ops/trait.IndexMut.html

### Why the split between `Eq`/`PartialEq` and `Ord`/`PartialOrd`?

There are some types in Rust whose values are only partially ordered, or have only partial equality. Partial ordering means that there may be values of the given type that are neither less than nor greater than each other. Partial equality means that there may be values of the given type that are not equal to themselves.

Floating point types (`f32` and `f64`) are good examples of each. Any floating point type may have the value `NaN` (meaning "not a number"). `NaN` is not equal to itself (`NaN == Nan` is false), and not less than or greater than any other floating point value. As such, both `f32` and `f64` implement `PartialOrd` and `PartialEq` but not `Ord` and not `Eq`.

<h2 id="input-output">Input / Output</h2>

### How do I read a file into a `String`?

Using the `read_to_string()` method, which is defined on the `Read` trait in `std::io`.

```rust
fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut f = try!(File::open(path));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));  // `s` contains the contents of "foo.txt"
    s
}

fn main() {
    match read_file("foo.txt") {
        Ok(_) => println!("Got file contents!"),
        Err(err) => println!("Getting file contents failed with error: {}", err)
    };
}
```

### How do I read file input efficiently?

The [`File` type](https://doc.rust-lang.org/stable/std/fs/struct.File.html) implements the `Read` trait, which has a variety of functions for reading and writing data, including `read()`, `read_to_end()`, `bytes()`, `chars()`, and `take()`. Each of these functions reads a certain amount of input from a given file. `read()` reads as much input as the underlying system will provide in a single call. `read_to_end()` reads the entire buffer into a vector, allocating as much space as is needed. `bytes()` and `chars()` allow you to iterate over the bytes and characters of the file, respectively. Finally, `take()` allows you to read up to an arbitrary number of bytes from the file. Collectively, these should allow you to efficiently read in any data you need.

For buffered reads, use the [`BufReader`](http://doc.rust-lang.org/stable/std/io/struct.BufReader.html) struct, which helps to reduce the number of system calls when reading.

### How do I get command line arguments in Rust?

The easiest way is to use `std::env::Args`, which provides an iterator over the input arguments.

If you're looking for something more powerful, there are a [number of options on crates.io](https://crates.io/keywords/argument).

<h2 id="error-handling">Error Handling</h2>

### Why doesn't Rust have exceptions?

Exceptions complicate understanding of control-flow, they express validity/invalidity outside of the type system, and they interoperate poorly with multithreaded code (a major focus of Rust).

Rust prefers a type-based approach to error handling, which is [covered at length in the book](https://doc.rust-lang.org/stable/book/error-handling.html). This fits more nicely with Rust's control flow, concurrency, and everything else.

### What's the deal with `unwrap()` everywhere?

`unwrap()` is a function that extracts the value inside an `Option` or `Result` and panics if no value is present. It is useful in the presence of truly unrecoverable errors, but is more useful for quick prototypes where you don't want to handle an error yet, or blog posts where error handling would distract from the main point. `unwrap()` shouldn't be your default way to handle errors, but it is a useful tool to have.

### Why do I get an error when I try to run example code that uses the `try!` macro?

It's probably an issue with the function's return type. The [`try!` macro](https://doc.rust-lang.org/stable/std/macro.try!.html) either extracts the value from a `Result`, or returns early with the error `Result` is carrying. This means that `try` only works for functions that return `Result` themselves, where the `Err`-constructed type implements `From::from(err)`. In particular, this means that the `try!` macro cannot work inside the `main` function.

### Is there an easier way to do error handling than having `Result`s everywhere?

If you're looking for a way to avoid handling `Result`s in other people's code, there's always `unwrap()`, but it's probably not what you want. `Result` is an indicator that some computation may or may not complete successfully. Requiring you to handle these failures explicitly is one of the ways that Rust encourages safety. If you really don't want to handle an error, use `unwrap()`, but know that doing so means that the given code will cause the entire process to panic on failure, which is usually undesirable.

<h2 id="concurrency">Concurrency</h2>

### Can I use static values across threads without an `unsafe` block?

Yes, if the type implements `Sync`, doesn't implement `Drop`, and you don't try to mutate the value.

<h2 id="macros">Macros</h2>

### Can I write a macro to generate identifiers?

Not currently. Rust macros are ["hygienic macros"](https://en.wikipedia.org/wiki/Hygienic_macro), which intentionally avoid capturing or creating identifiers that may cause unexpected collisions with other identifiers. Their capabilities are significantly different than the style of macros commonly associated with the C preprocessor. Macro invocations can only appear in places where they are explicitly supported: items, method declarations, statements, expressions, and patterns. Here, "method declarations" means a blank space where a method can be put. They can't be used to complete a partial method declaration. By the same logic, they can't be used to complete a partial variable declaration.

<h2 id="debugging">Debugging</h2>

### How do I debug Rust programs?

Rust programs can be debugged using [gdb](http://sourceware.org/gdb/current/onlinedocs/gdb/) or [lldb](http://lldb.llvm.org/tutorial.html), same as C and C++. In fact, every Rust installation comes with rust-gdb and rust-lldb, wrappers over gdb and lldb with Rust pretty-printing enabled.

### `rustc` said a panic occurred in standard library code. How do I locate the mistake in my code?

This error is usually caused by `unwrap()`ing a `None` or `Err` in client code. Enabling backtraces by setting the environment variable `RUST_BACKTRACE=1` helps with getting more information. Compiling in debug mode (the default for `cargo build`) is also helpful. Using a debugger like the provided `rust-gdb` or `rust-lldb` is also helpful.

<h2 id="low-level">Low-Level</h2>

### How do I `memcpy` bytes?

If you want to clone an existing slice safely, you can use `std::slice::clone_from_slice`. This function is currently unstable, but [should be stabilized soon](https://internals.rust-lang.org/t/stabilizing-basic-functions-on-arrays-and-slices/2868).

To copy potentially overlapping bytes, use `std::ptr::copy`. To copy nonoverlapping bytes, use `std::ptr::cpy_nonoverlapping`. Both of these functions are `unsafe`, as both can be used to subvert the language's safety guarantees. Take care when using them.

### Can Rust function reasonably without the standard library?

Absolutely. Rust programs can be set to not load the standard library using the `#![no_std]` attribute. With this attribute set, you can continue to use the Rust core library, which is nothing but the platform-agnostic primitives. As such, it doesn't include IO, concurrency, heap allocation, etc.

### Can I write an operating system in Rust?

Yes! In fact there are [several projects underway doing just that](http://wiki.osdev.org/Rust).

### How can I read or write numeric types like `i32` or `f64` in big-endian or little-endian format in a file or other byte stream?

You should check out the [byteorder crate](http://burntsushi.net/rustdoc/byteorder/), which provides utilities for exactly that.

### Does Rust guarantee a specific data layout?

Not by default. In the general case, `enum` and `struct` layouts are undefined. This allows the compiler to potentially do optimizations like re-using padding for the discriminant, compacting variants of nested `enum`s, reordering fields to remove padding, etc. `enums` which carry no data ("C-like") are eligible to have a defined representation. Such `enums` are easily distinguished in that they are simply a list of names that carry no data:

```rust
enum CLike {
    A,
    B = 32,
    C = 34,
    D
}
```

The `#[repr(C)]` attribute can be applied to such `enums` to give them the same representation they would have in equivalent C code. This allows using Rust `enum`s in FFI code where C `enum`s are also used, for most use cases. The attribute can also be applied to `struct`s to get the same layout as a C `struct` would.

<h2 id="cross-platform">Cross-Platform</h2>

<!--
### How do I build a Windows binary that doesn't display the console window?

TODO: Write this answer.
-->

<!--
### How do I make the console-less binary not crash on panic!?

TODO: Write this answer.
-->

### What's the idiomatic way to express platform-specific behavior in Rust?

Platform-specific behavior can be expressed using [conditional compilation attributes](https://doc.rust-lang.org/reference.html#conditional-compilation) such as `target_os`, `target_family`, `target_endian`, etc.

### Can Rust be used for Android/iOS programming?

Yes it can! There are already examples of using Rust for both [Android](https://github.com/tomaka/android-rs-glue) and [iOS](https://www.bignerdranch.com/blog/building-an-ios-app-in-rust-part-1/). It does require a bit of work to set up, but Rust functions fine on both platforms.

### Can I run my Rust program in a web browser?

Not yet, but there are efforts underway to make Rust compile to the web with Emscripten.

### How do I cross-compile in Rust?

Cross compilation is possible in Rust, but it requires [a bit of work](https://github.com/japaric/ruststrap/blob/master/1-how-to-cross-compile.md) to set up. Rust does distribute [copies of the standard library](http://static.rust-lang.org/dist/) for each of the supported platforms, which are contained in the `rust-std-*` files for each of the build directories found on the distribution page, but there are not yet automated ways to install them.

<h2 id="modules-and-crates">Modules and Crates</h2>

### What is the relationship between a module and a crate?

- A crate is a compilation unit, which is the smallest amount of code that the Rust compiler can operate on.
- A module is a (possibly nested) unit of name-management inside a crate.
- A crate contains an implicit, un-named top-level module.
- Recursive definitions can span modules, but not crates.
- Crates do not have global names, only a set of non-unique metadata tags.
- There is no global inter-crate namespace; all name management occurs within a crate.
- Using another crate binds the root of its namespace into the user's namespace.

### Why can't the Rust compiler find this library I'm `use`ing?

There are a number of possible answers, but a common mistake is not realizing that `use` declarations are relative to the crate root. Try rewriting your declarations to use the paths they would use if defined in the root file of your project and see if that fixes the problem.

There are also `self` and `super`, which disambiguate `use` paths as being relative to the current module or parent module, respectively.

For complete information on `use`ing libraries, read the Rust book's chapter ["Crates and Modules"](https://doc.rust-lang.org/stable/book/crates-and-modules.html).

### Why do I have to declare module files with `mod` at the top level of the crate, instead of just `use`ing them?

There are two ways to declare modules in Rust, inline or in another file. Here is an example of each:

```rust
// In main.rs
mod hello {
    pub fn f() {
        println!("hello!");
    }
}

fn main() {
    hello::f();
}
```

```rust
// In main.rs
mod hello;

fn main() {
    hello::f();
}

// In hello.rs
pub fn f() {
    println!("hello!");
}
```

In the first example, the module is defined in the same file it's used. In the second example, the module declaration in the main file tells the compiler to look for either `hello.rs` or `hello/mod.rs`, and to load that file.

A `use`ing declaration just tells the compiler to bring everything from a particular module into the current module. Without a `mod` declaration first, the compiler doesn't know if the `use`d module exists, and so can't import its contents into the current module.

### How do I configure Cargo to use a proxy?

As explained on the Cargo [configuration documentation](http://doc.crates.io/config.html), you can set Cargo to use a proxy by setting the "proxy" variable under `[http]` in the configuration file.

### Why can't the compiler find the method implementation even though I'm already `use`ing the crate?

For methods defined on a trait, you have to explicitly import the trait declaration. This means it's not enough to import a module where a struct implements the trait, you must also import the trait itself.

### Why can't the compiler infer `use` declarations for me?

It probably could, but you also don't want it to. While in many cases it is likely that the compiler could determine the correct module to import by simply looking for where a given identifier is defined, this may not be the case in general. Any decision rule in `rustc` for choosing between competing options would likely cause surprise and confusion, and not solve much of a problem.

For example, the compiler could say that in the case of competing identifier definitions the definition from the earliest imported module is chosen. So if both module `foo` and module `bar` define the identifier `baz`, but `foo` is the first registered module, the compiler would insert `use foo::baz;`.

```rust
mod foo;
mod bar;

// use foo::baz  // to be inserted by the compiler.

fn main() {
  baz();
}
```

If you know this is going to happen, perhaps it saves a small number of keystrokes, but it also greatly increases the possibility for surprising error messages when you actually meant for `baz()` to be `bar::baz()`, and it decreases the readability of the code by making the meaning of a function call dependent on module declaration. These are not tradeoffs we are willing to make.

<!--
### How do I package and archive crates from [http://crates.io](http://crates.io)?

TODO: Write this answer.
-->

### How do I do dynamic Rust library loading?

Import dynamic libraries in Rust with [libloading](https://crates.io/crates/libloading), which provides a cross-platform system for dynamic linking.

### Why doesn't [http://crates.io](http://crates.io) have namespaces?

Quoting the [official explanation](https://internals.rust-lang.org/t/crates-io-package-policies/1041) of [http://crates.io](http://crates.io)'s design:

> In the first month with crates.io, a number of people have asked us about the possibility of introducing [namespaced packages](https://github.com/rust-lang/crates.io/issues/58).<br><br>
>
> While namespaced packages allow multiple authors to use a single, generic name, they add complexity to how packages are referenced in Rust code and in human communication about packages. At first glance, they allow multiple authors to claim names like `http`, but that simply means that people will need to refer to those packages as `wycats' http` or `reem's http`, offering little benefit over package names like `wycats-http` or `reem-http`.<br><br>
>
> When we looked at package ecosystems without namespacing, we found that people tended to go with more creative names (like `nokogiri` instead of "tenderlove's libxml2"). These creative names tend to be short and memorable, in part because of the lack of any hierarchy. They make it easier to communicate concisely and unambiguously about packages. They create exciting brands. And we've seen the success of several 10,000+ package ecosystems like NPM and RubyGems whose communities are prospering within a single namespace.<br><br>
>
> In short, we don't think the Cargo ecosystem would be better off if Piston chose a name like `bvssvni/game-engine` (allowing other users to choose `wycats/game-engine`) instead of simply `piston`.<br><br>
>
> Because namespaces are strictly more complicated in a number of ways,and because they can be added compatibly in the future should they become necessary, we're going to stick with a single shared namespace.

### Why are so many Rust answers on StackOverflow wrong?

The Rust language has been around for a number of years, and only reached version 1.0 in May of 2015. In the time before then the language changed significantly, and a number of StackOverflow answers were given at the time of older versions of the language.

Over time more and more answers will be offered for the current version, thus improving this issue as the proportion of out-of-date answers is reduced.

<h2 id="libraries">Libraries</h2>

### How can I make an HTTP request?

[Hyper](https://github.com/hyperium/hyper) is the most popular, but there are [a number of others as well](https://crates.io/keywords/http).

### How can I write a GUI application in Rust?

There are a variety of ways to write GUI applications in Rust. Just check out [this list of GUI frameworks](https://github.com/kud1ing/awesome-rust#gui).

### How can I parse JSON/XML?

[Serde](https://github.com/serde-rs/serde) is the recommended library for serialization and deserialization of Rust data to and from a number of different formats.

### Is there a standard 2D+ dimensional vector and shape crate?

Not yet! Want to write one?

### How do I write an OpenGL app in Rust?

[Glium](https://github.com/tomaka/glium) is the major library for OpenGL programming in Rust. [GLFW](https://github.com/bjz/glfw-rs) is also a solid option.

### Can I write a video game in Rust?

Yes you can! The major game programming library for Rust is [Piston](http://www.piston.rs/), and there's both a [subreddit for game programming in Rust](https://www.reddit.com/r/rust_gamedev/) and an IRC channel (`#rust-gamedev` on [Mozilla IRC](https://wiki.mozilla.org/IRC))  as well.

<h2 id="design-patterns">Design Patterns</h2>

### Is Rust object oriented?

It is multi-paradigm. Many things you can do in OO languages you can do in Rust, but not everything, and not always using the same abstraction you're accustomed to.

### How do I handle configuration of a struct with optional parameters?

The easiest way is to use the `Option` type in whatever function you're using to construct instances of the struct (usually `new()`). Another way is to use the [builder pattern](https://aturon.github.io/ownership/builders.html), where only certain functions instantiating member variables must be called before the construction of the built type.

### How do I do global variables in Rust?

Globals in Rust can be done using `const` declarations for compile-time computed global constants, while `static` can be used for mutable globals. Note that modifying a `static` variable requires the use of `unsafe`, as it allows for data races, one of the things guaranteed not to happen in safe Rust. One important distinction between `const` and `static` values is that you can take references to `static` values, but not references to `const` values, which don't have a specified memory location. For more information on `const` vs. `static`, read [the Rust book](https://doc.rust-lang.org/book/const-and-static.html).

You can also use the `RefCell` and `Option` type to provide interior mutability of an optional global value. It is important to note that `RefCell`s are not thread-safe.

### How can I set compile-time constants that are defined procedurally?

Rust currently has limited support for compile time constants. You can define primitives using `const` declarations (similar to `static`, but immutable and without a specified location in memory) as well as define `const` functions and inherent methods.

To define procedural constants that can't be defined via these mechanisms, use the [`lazy-static`](https://github.com/rust-lang-nursery/lazy-static.rs) crate, which emulates compile-time evaluation by automatically evaluating the constant at first use.

### Can I run initialization code that happens before main?

Rust has no concept of "life before `main`". The closest you'll see can be done through the [`lazy_static`](https://github.com/Kimundi/lazy-static.rs) crate, which simulates a "before main" by lazily initializing static variables at their first usage.

### How do I map object-oriented concepts to Rust?

That depends. There _are_ ways of translating object-oriented concepts like [multiple inheritance](https://www.reddit.com/r/rust/comments/2sryuw/ideaquestion_about_multiple_inheritence/) to Rust, but as Rust is not object-oriented the result of the translation may look substantially different from its appearance in an OO language.

### Why doesn't Rust have inheritance?

There are two meanings for the word "inheritance": _subtyping_, and _interface sharing_. Both purposes are already handled by traits.

For the first, subtyping exists for polymorphism, which traits already provide.

For the second, interface sharing is handled via trait methods, which define a collection of related functions that must be implemented for any implementation of the trait.

Rust has consistently worked to avoid having features with overlapping purposes, preferring to keep features orthogonal. For this reason, and given that the two major purposes are already handled by traits, Rust has opted not to include inheritance.

### Does Rust allow non-constant-expression values for globals?

No. Globals cannot have a non-constant-expression constructor and cannot have a destructor at all. Static constructors are undesirable because portably ensuring a static initialization order is difficult. Life before main is often considered a misfeature, so Rust does not allow it.

See the [C++ FQA](http://yosefk.com/c++fqa/ctors.html#fqa-10.12) about the "static initialization order fiasco", and [Eric Lippert's blog](http://ericlippert.com/2013/02/06/static-constructors-part-one/) for the challenges in C#, which also has this feature.

<h2 id="other-languages">Other Languages</h2>

### How can I implement something like C's `struct X { static int X; };` in Rust?

Rust does not have `static` fields as shown in the code snippet above. Instead, you can declare a `static` variable in a given module, which is kept private to that module.

### How can I convert a C-style enum to an integer, or vice-versa?

Converting a C-style enum to an integer can be done with an `as` expression, like `e as i64` (where `e` is some enum).

Converting in the other direction can be done with a `match` statement, which maps different numeric values to different potential values for the enum.

### Why does Rust not have a stable ABI like C does, and why do I have to annotate things with extern?

Committing to an ABI is a big decision that can limit potentially advantageous language changes in the future. Given that Rust only hit 1.0 in May of 2015, it is still too early to make a commitment as big as a stable ABI. This does not mean that one won't happen in the future, though.

The `extern` keyword allows Rust to use specific ABI's, such as the well-defined C ABI, for interop with other languages.

### Can Rust code call C code?

Yes. Calling C code from Rust is designed to be as efficient as calling C code from C++.

### Can C code call Rust code?

Yes. The Rust code has to be exposed via an `extern` declaration, which makes it C-ABI compatible. Such a function can be passed to C code as a function pointer or, if given the `#[no_mangle]` attribute to disable symbol mangling, can be called directly from C code.

### I already write perfect C++. What does Rust give me?

Modern C++ includes many features that make writing safe and correct code less error-prone, but it's not perfect, and it's still easy to introduce unsafety. This is something the C++ core developers are working to overcome, but C++ is limited by a long history that predates a lot of the ideas they are now trying to implement.

Rust was designed from day one to be a safe systems programming language, which means it's not limited by historic design decisions that make getting safety right in C++ so complicated. In C++, safety is achieved by careful personal discipline, and is very easy to get wrong. In Rust, safety is the default.

### How do I do the equivalent of C++ template specialization in Rust?

Rust doesn't currently have an exact equivalent to template specialization, but it is [being worked on](https://github.com/rust-lang/rfcs/pull/1210) and will hopefully be added soon. However, similar effects can be achieved via [associated types](https://doc.rust-lang.org/stable/book/associated-types.html).

### How does Rust's ownership system relate to move semantics in C++?

The notions of a "move" in Rust and a "move" in C++ are quite different, owing to the different systems in which they exist.

In C++, R-value references come from a temporary value, or are explicitly created from a named value via `std::move`. In this way, C++ enforces that no mutable references exist to the moved-out value, so that the memory may be safely invalidated. In Rust, mutable aliases are statically eliminated by the borrow checker, and so the rough equivalent of C++'s R-values are found in Rust's mutable references (`&mut`), which can only be used if no other mutable alias already exists to the given memory location.

Rust "move"s are about transferring ownership, rather than eliminating mutable aliases (which are handled via the borrow checker). By default, the ownership of any function parameter is transferred into the function and out of the parameter at the call site, unless that parameter's type implements the `Copy` trait, in which case a shallow copy of the value is used to instantiate the actual parameter of the function. Rust's "move"s are often unecessary and undesirable. If the function you're writing does not require ownership of the value being passed in, that value should probably be borrowed (mutably or immutably, as necessary) rather than moved or copied.

### How can I interoperate with C++ from Rust, or with Rust from C++?

Rust and C++ can interoperate through C. Both Rust and C++ provide a [foreign function interface](https://doc.rust-lang.org/book/ffi.html) for C, and can use that to communicate between each other. If writing C bindings is too tedious, you can always use [rust-bindgen](https://github.com/crabtw/rust-bindgen) to help automatically generate workable C bindings.

### Does Rust have C++-style constructors?

No. Functions serve the same purpose as constructors without adding language complexity. The usual name for the constructor-equivalent function in Rust is `new()`, although this is just a convention rather than a language rule. The `new()` function in fact is just like any other function. An example of it looks like so:

```rust
struct Foo {
    a: i32,
    b: f64,
    c: bool,
}

impl Foo {
    fn new() -> Foo {
        Foo {
            a: 0,
            b: 0.0,
            c: false,
        }
    }
}
```

### Does Rust have copy constructors?

Not exactly. Types which implement `Copy` will do a standard C-like "shallow copy" with no extra work (similar to "plain old data" in C++). It is impossible to implement `Copy` types that require custom copy behavior. Instead, in Rust "copy constructors" are created by implementing the `Clone` trait, and explicitly calling the `clone` method. Making user-defined copy operators explicit surfaces the underlying complexity, making it easier for the developer to identify potentially expensive operations.

### Does Rust have move constructors?

No. Values of all types are moved via `memcpy`. This makes writing generic unsafe code much simpler since assignment, passing and returning are known to never have a side effect like unwinding.

### Why does Rust not have the ? and ! like in Swift?

In Swift, `?` is used to indicate an optional value. This is already done by `Option` in Rust, and so `?` is not needed. Similarly, `!` is used to "unwrap" an optional value in Swift, which is done by the `unwrap()` function in Rust. In both cases, Rust opted for slightly longer names which more clearly indicate intent. Also, in Rust the `!` symbol already indicates logical negation and macro calls, and does not need a third meaning.

### How are Go and Rust similar, and how are they different?

Rust and Go have substantially different design goals. The following differences are not the only ones (which are too numerous to list), but are a few of the more important ones:

- Rust is lower level than Go. For example, Rust does not require a garbage collector, whereas Go does. In general, Rust affords a level of control that is comparable to C or C++.
- Rust's focus is on ensuring safety and efficiency while also providing high-level affordances, while Go's is on being a small, simple language which compiles quickly and can work nicely with a variety of tools.
- Rust has strong support for generics, which Go does not.
- Rust has strong influences from the world of functional programming, including a type system which draws from Haskell's typeclasses. Go has a simpler type system, using interfaces for basic generic programming.

### `gofmt` is great. Where's `rustfmt`?

`rustfmt` is [right here](https://github.com/rust-lang-nursery/rustfmt), and is being actively developed to make reading Rust code as easy and predictable as possible.

### How do Rust traits compare to Haskell typeclasses?

Rust traits are similar to Haskell typeclasses, but are currently not as powerful, as Rust cannot express higher-kinded types. Rust's associated types are equivalent to Haskell type families.

Some specific difference between Haskell typeclasses and Rust traits include:

- Rust traits have an implicit first parameter called `Self`. `trait Bar` in Rust corresponds to `class Bar self` in Haskell, and `trait Bar<Foo>` in Rust corresponds to `class Boo foo self` in Haskell.
- "Supertraits" or "superclass constraints" in Rust are written `trait Sub: Super`, compared to `class Super self => Sub self` in Haskell.
- Rust forbids orphan instances, resulting in different coherence rules in Rust compared to Haskell.
- Rust's `impl` resolution considers the relevant `where` clauses and trait bounds when deciding whether two `impl`s overlap, or choosing between potential `impl`s. Haskell only considers the constraints in the `instance` declaration, disregarding any constraints provided elsewhere.
- A subset of Rust's traits (the ["object safe"](https://github.com/rust-lang/rfcs/blob/master/text/0255-object-safety.md) ones) can be used for dynamic dispatch via trait objects. The same feature is available in Haskell via GHC's `ExistentialQuantification`.

<h2 id="documentation">Documentation</h2>

### Where do I report issues in the Rust documentation?

You can report issues in the Rust documentation on the Rust compiler [issue tracker](https://github.com/rust-lang/rust/issues). Make sure to read the [contributing guidelines](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md) first.

### How do I view rustdoc documentation for a library my project depends on?

When you use `cargo doc` to generate documentation for your own project, it also generates docs for the active dependency versions. These are put into the `target/doc` directory of your project. Use `cargo doc --open` to open the docs after building them, or just open up `target/doc/index.html` yourself.

<h2 id="project">The Rust Project</h2>

### What is this project's goal?

To design and implement a safe, concurrent, practical systems language.

Rust exists because other languages at this level of abstraction and efficiency are unsatisfactory. In particular:

1. There is too little attention paid to safety.
2. They have poor concurrency support.
3. There is a lack of practical affordances.
4. They offer limited control over resources.

Rust exists as an alternative that provides both efficient code and a comfortable level of abstraction, while improving on all four of these points.

### Is this project controlled by Mozilla?

No. Rust started as Graydon Hoare's part-time side project in 2006 and remained so for over 3 years. Mozilla got involved in 2009 once the language was mature enough to run basic tests and demonstrate its core concepts. Though it remains sponsored by Mozilla, Rust is developed by a diverse community of enthusiasts from many different places around the world. The [Rust Team](https://www.rust-lang.org/team.html) is composed of both Mozilla and non-Mozilla members, and `rustc` (Rust's compiler) has had over [1,000 unique contributors](https://github.com/rust-lang/rust/) so far.

As far as [project governance](https://github.com/rust-lang/rfcs/blob/master/text/1068-rust-governance.md) goes, Rust is managed by a core team that sets the vision and priorities for the project, and accepts input on the design of the language via RFC proposals. There are also subteams to guide and foster development of particular areas of interest, including Rust libraries, Rust tools, and moderation of the official Rust communities. For changes which do not require an RFC, decisions are made through pull requests on the [`rustc` repository](https://github.com/rust-lang/rust).

### What are some design goals of Rust?

1. We do not employ any particularly cutting-edge technologies. Old, established techniques are better.
2. We do not prize expressiveness, minimalism or elegance above other goals. These are desirable but subordinate goals.
3. We do not intend to cover the complete feature-set of C++, or any other language. Rust should provide majority-case features.
4. We do not intend to be 100% static, 100% safe, 100% reflective, or too dogmatic in any other sense. Trade-offs exist.
5. We do not demand that Rust run on "every possible platform". It must eventually work without unnecessary compromises on widely-used hardware and software platforms.

### In which projects is Mozilla using Rust?

The main project is [Servo](https://github.com/servo/servo), an experimental browser engine Mozilla is working on. They are also working to [integrate Rust components](https://bugzilla.mozilla.org/show_bug.cgi?id=1135640) into Firefox.

### What examples are there of large Rust projects?

The two biggest open source Rust projects right now are [Servo](https://github.com/servo/servo) and the [Rust compiler](https://github.com/rust-lang/rust) itself.

<!--
### What projects are good examples of idiomatic Rust code?

TODO: Write this answer.
-->

### How can I try Rust easily?

The easiest way to try Rust is through the [playpen](https://play.rust-lang.org/), an online app for writing and running Rust code. If you want to try Rust on your system, [install it](https://www.rust-lang.org/install.html) and go through the ["Learn Rust"](https://doc.rust-lang.org/stable/book/learn-rust.html) section of the book.

### How do I get help with Rust issues?

There are several ways. You can:

- Post in [users.rust-lang.org](https://users.rust-lang.org/), the official Rust forum
- Ask in the official [Rust IRC channel](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust) (#rust on irc.mozilla.org)
- Ask on [StackOverflow](http://stackoverflow.com/questions/tagged/rust) with the "rust" tag
- Post in [/r/rust](https://www.reddit.com/r/rust), the official Rust subreddit

### Why has Rust changed so much over time?

Rust started with a goal of creating a safe but usable systems programming language. In pursuit of this goal it explored a lot of ideas, some of which it kept (lifetimes, traits) while others were discarded (the typestate system, green threading). Also, in the run up to 1.0 a lot of the standard library was rewritten as early designs were updated to best use Rust's features and provide quality, consistent cross-platform APIs.

### How does Rust language versioning work?

Rust's language versioning follows [SemVer](http://semver.org/), with backwards incompatible changes of stable APIs only allowed in minor versions if those changes fix compiler bugs, patch safety holes, or change type inference to require additional annotation.

Rust maintains three "release channels": stable, beta, and nightly. Stable and beta are updated every six weeks, with the current nightly becoming the new beta, and the current beta becoming the new stable. Language and standard library features marked unstable or hidden behind feature gates may only be used on the nightly channel.

For details, read the Rust blog post ["Stability as a Deliverable."](http://blog.rust-lang.org/2014/10/30/Stability.html)

### Can I use unstable features in the beta or stable channel?

No, you cannot. Rust works hard to provide strong guarantees about the stability of the features provided on the beta and stable channels. When something is unstable, it means that we can't provide those guarantees for it yet, and don't want people relying on it staying the same. This gives us the opportunity to try changes in the wild on the nightly channel, while still maintaining strong guarantees for people seeking stability.

Things stabilize all the time, and the beta and stable channels update every six weeks. If you're waiting for a feature to be available without using the nightly channel, you can locate its tracking issue by checking the [`B-unstable`](https://github.com/rust-lang/rust/issues?q=is%3Aissue+is%3Aopen+tracking+label%3AB-unstable) tag on the issue tracker.

### What IDE should I use?

There are a number of options for development environment with Rust, all of which are detailed on the official [IDE support page](https://www.rust-lang.org/ides.html).

### Why a dual MIT/ASL2 License?

The Apache license includes important protection against patent aggression, but it is not compatible with the GPL, version 2. To avoid problems using Rust with GPL2, it is alternately MIT licensed.

### Why a BSD-style permissive license rather than MPL or tri-license?

This is partly due to preference of the original developer (Graydon), and partly due to the fact that languages tend to have a wider audience and more diverse set of possible embeddings and end-uses than products such as web browsers. We'd like to appeal to as many of those potential contributors as possible.

