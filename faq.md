---
layout: faq
title: Frequently Asked Questions &middot; The Rust Programming Language
---

# Frequently Asked Questions

This document exists to answer some common questions about the Rust programming language. It is not a complete guide to the language, nor is it a tool for teaching the language. Rather, it exists as a reference to answer oft-repeated questions people in the Rust community encounter, and to clarify some of the design and history of the language.

If there is some common or important question you feel is wrongly left unanswered here, feel free to [help us fix it](https://github.com/rust-lang/rust-www/blob/master/CONTRIBUTING.md).

<div id="toc">
    <h2>Table of Contents</h2><a href="#toggle-toc"></a>
    <ol id="toc-contents">
        <li><a href="#performance">Performance</a></li>
        <li><a href="#concurrency">Concurrency</a></li>
        <li><a href="#error-handling">Error Handling</a></li>
        <li><a href="#numerics">Numerics</a></li>
        <li><a href="#syntax">Syntax</a></li>
        <li><a href="#lifetimes">Lifetimes</a></li>
        <li><a href="#ownership">Ownership</a></li>
        <li><a href="#stability">Stability</a></li>
        <li><a href="#strings">Strings</a></li>
        <li><a href="#collections">Collections</a></li>
        <li><a href="#documentation">Documentation</a></li>
        <li><a href="#input-output">Input / Output</a></li>
        <li><a href="#libraries">Libraries</a></li>
        <li><a href="#project">Project</a></li>
        <li><a href="#modules-and-crates">Modules and Crates</a></li>
        <li><a href="#generics">Generics</a></li>
        <li><a href="#debugging">Debugging</a></li>
        <li><a href="#low-level">Low-Level</a></li>
        <li><a href="#cross-platform">Cross-Platform</a></li>
        <li><a href="#design-patterns">Design Patterns</a></li>
        <li><a href="#macros">Macros</a></li>
        <li><a href="#other-languages">Other Languages</a></li>
        <li><a href="#licensing">Licensing</a></li>
        <li><a href="#naming">Naming</a></li>
    </ol>
</div>

## Performance

#### How fast is Rust?

As always, this question is difficult to answer. There's still a lot of work to do on speed, and depending on what you're benchmarking, Rust has variable performance.

That said, it is an explicit goal of Rust to be as fast as C++ for most things. Language decisions are made with performance in mind, and we want Rust to be as fast as possible. Given that Rust is built on top of LLVM, any performance improvements in it also help Rust become faster.

#### Is Rust garbage collected?

No. A language that requires a GC is a language that opts into a larger, more complex runtime than Rust cares for. Rust is usable on bare metal with no extra runtime. Additionally, garbage collection is frequently a source of non-deterministic behavior. Rust provides the tools to make using a GC possible and even pleasant, but it should not be a requirement for implementing the language.

#### Why is my program compiled with `cargo build` slow?

Did you compile with the `--release` flag? The Rust language uses a lot of optimizations in release mode that make the language competitive with C and C++, but you need to explicitly ask for them, as they also result in longer compilation times that may be undesirable during development.

#### Why is Rust compilation slow?

It's a combination of factors. Both the type inference and safety-related checks are fairly complex. They make life easier for the programmer, but they require some time to run. Add in the optimizations, and Rust's compilation is slower than a simpler language that doesn't provide Rust's safety guarantees.

But all is not lost. The Rust compiler has evolved significantly over a number of years, and a lot of good work is being done to make it work faster. So expect improvements in compilation speed over time.

#### Why is Rust's HashMap so slow?

Rust's default hashing algorithm is cryptographically secure, which means it's great if you need something that can't be easily guessed at, but you pay for that with slower performance relative to a non-cryptographically secure algorithm.

Even so, it's [actually not](https://www.reddit.com/r/rust/comments/3hw9zf/rust_hasher_comparisons/cub4oh6) too bad. As you can see from [these benchmark comparisons](http://cglab.ca/~abeinges/blah/hash-rs/) of different hashing implementations in Rust, Rust's SipHash implementation (the default) is a solid choice for a wide variety of purposes.

#### Can I run benchmarks using `test::Bencher` on the beta or stable channel?

Sadly, no. The `test` feature, which is necessary for testing and benchmarking in Rust, is restricted to the nightly channel.

#### Does Rust do tail-call optimization?

In general, tail-call optimization is not guaranteed: see [here](https://mail.mozilla.org/pipermail/rust-dev/2013-April/003557.html) for a detailed explanation with references. There was a [proposed extension](https://github.com/rust-lang/rfcs/pull/81) that would allow tail-call elimination in certain contexts, but it is currently postponed. The compiler is still free to optimize tail-calls [when it pleases](http://llvm.org/docs/CodeGenerator.html#sibling-call-optimization), however.

#### Does Rust have a runtime?

Rust has a [very small and limited runtime]() providing a heap, unwinding and backtrace support, and stack guards. This runtime is comparable to the [C runtime](http://www.embecosm.com/appnotes/ean9/html/ch05s02.html), and allows for the calling of Rust functions from C without setup.

## Concurrency

#### Can I use globals across threads without `unsafe`?

Yes, if the type implements `Sync`, doesn't implement `Drop`, and you don't try to mutate the global value.

## Error Handling

#### Why doesn't Rust have exceptions?

The design issues for exception handling in programming language have been covered at length elsewhere. Exceptions complicate understanding of control-flow, they express validity/invalidity outside of the type system, they interoperate poorly with multithreaded code (a major focus of Rust), the list goes on.

Rust opted instead for a type-based approach to error handling, which is [covered at length in the book](https://doc.rust-lang.org/nightly/book/error-handling.html#the-limits-of-combinators) (the improved chapter is only on nightly at the moment, but will be on stable soon. Everything it covers works on stable). This fits more nicely with Rust's control flow, concurrency, and everything else.

#### What's the deal with `unwrap()` everywhere?

`unwrap()` is a function that extracts the value inside an `Option` or `Result` and panics if its not there. It is useful both for causing catastrophic failure in certain cases, but it more useful for quick prototypes where you don't want to handle an error yet, or blog posts where error handling would distract from the main point. `unwrap()` shouldn't be your default way to handle errors, but it is a useful tool to have when you need it.

#### Why do I get an error when I try to run example code that uses the `try!` macro?

It's probably an issue with the function's return type. The [`try!` macro](https://doc.rust-lang.org/stable/std/macro.try!.html) either extracts the value from a `Result`, or returns early with the error `Result` is carrying. This means that try only works for functions that return `Result` themselves, where the `Err`-constructed type implements `From::from(err)`.

#### Is there an easier way to do error handling than having `Result`s everywhere?

If you're looking for a way to avoid handling `Result`s in other people's code, there's always `unwrap()`, but it's probably not what you actually want. `Result` is an indicator that some computation may or may not complete. The fact that some languages allow you to ignore failure cases is an anti-feature. Forcing you to handle them is one of the ways that Rust encourages safety. So, if you really don't want to handle error, use `unwrap()`, but you should probably just handle the error for real.

## Numerics

#### Should I default to using `f32` or `f64`?

The choice of which to use is dependent on the focus of the program.

If you're interested in the greatest degree of precision with your floating point numbers, then `f64` is the preferred choice. If you're more interested in keeping the size of the value small, then `f32` is better.

All things being equal, `f64` is likely a better default.

#### Why can't I compare floats?

You can! Floats implement the `PartialOrd` trait, which means that `>`, `<`, `<=`, and `>=` are defined for them. But they do _not_ implement the `Ord` trait, because (thanks to `NaN`) there is no total ordering for floating point numbers.

So you can do all the normal comparison operations you would expect, but you can't use the `cmp` function.

#### Why can't I use `f32` or `f64` as `HashMap` keys?

In order to be used as a key in a `HashMap`, a type must implement the `Eq` and `Hash` traits. `f32` and `f64` implement `PartialEq`, but not `Eq`, because both types include `NaN` (stands for "not a number"). `NaN` values are not equal to any float, and are not equal to each other, so there is no total ordering over floating point types in Rust. This means that `f32` and `f64` can't be used as keys in a HashMap.

#### How can I convert between numeric types?

There are three ways: the `as` keyword, which does simple casting for primitive types, the `Into` and `From` traits, which are implemented for a number of type conversions (and which you can implement for your own types), and `transmute`, which is an unsafe function that tells the compiler to treat the bits of one type as those of another type.

## Syntax

#### Why curly braces? Why can't everything be like Haskell/Python?

Flexibility, simplicity, and correctness. Curly braces allow for more flexible syntax for the programmer, a simpler parser in the compiler, and help reduce the possibility of logic mistakes caused by incorrect indentation, like Apple's classic [goto fail](https://gotofail.com/) bug.

#### I can leave out parentheses on if conditions, why do I have to put brackets around single line blocks? Why is the C style not allowed?

A single line block may not always be a single line block. Forgetting to add in the brackets when you add a line to an `if` block can be the cause of some serious programming errors (just look at Apple's [&ldquo;goto fail&rdquo;](https://gotofail.com/) bug). In the interest of safety, Rust makes you use the blocks every time.

#### Why is there no literal syntax for dictionaries?

Dictionaries (or `HashMap`s in Rust parlance) aren't a primitive data type, and so they don't get literal syntax. If you want to make a `HashMap`, use `HashMap::new()`.

#### When should I use an implicit return?

You should use an implicit return everywhere it can be used. Unless you're writing an early return (before the end of a function, where typing `return` is mandatory), it should be implicit.

#### Why aren't function signatures inferred?

- Mechanically, it simplifies the inference algorithm; inference only requires looking at one function at a time.
- The same simplification goes double for human readers. A reader does not need an IDE running an inference algorithm across an entire crate to be able to guess at a function's argument types; it's always explicit and nearby.

#### Why does `match` have to be exhaustive?

`match` being exhaustive has some useful properties. First, if every possibility is covered by the `match`, adding further variants to the `enum` in the future will cause a compilation failure, rather than an error at runtime. Second, it makes cost explicit. In general, the only safe way to have a non-exhaustive `match` would be to panic the thread if nothing is matched, though it could fall through if the type of the `match` expression is `()`. This sort of hidden cost and special casing is against the language's philosophy. It's easy to ignore all unspecified cases by using the `_` wildcard:

```rust
match val.do_something() {
    Cat(a) => { /* ... */ }
    _      => { /* ... */ }
}
```

## Lifetimes

#### Why lifetimes?

Lifetimes are Rust's answer to garbage collection. They are a way of ensuring memory safety without paying hefty performance costs. They are based on a variety of academic work, which can be found in the [Rust book](https://doc.rust-lang.org/stable/book/academic-research.html#type-system).

#### Why is the lifetime syntax the way it is?

The `'a` syntax comes from the ML family of programming languages, where `'a` is used to indicate a generic type parameter. For Rust, the syntax had to be something that was unambiguous, noticable, and fit nicely in a type declaration right alonside traits and references. Alternative syntaxes have been discussed, but this seems to work just fine.

#### When is `Rc` useful?

This is covered in the [official documentation for `Rc`](https://doc.rust-lang.org/stable/std/rc/). In short, `Rc` and its thread-safe cousin `Arc` are useful to express shared ownership of an immutable value, and have the system automatically deallocate the associated memory when no one has access to it.

#### How do I return a borrow to something I created from a function?

You need to ensure that the borrowed item will outlive the function. This can be done in two ways: by binding the output lifetime to some input lifetime, or by declaring the output lifetime as static. The first option is significantly better than the second.

Here is an example of the each:

```rust
// The first method
fn example1<'a>(s: &'a str) -> &'a str {
  // Do something...
}

// The second method
fn example2<'a>(s: &'a str) -> &'static str {
  // Do something else...
}
```

The problem with declaring the lifetime as `static` is that it's rarely what you actually mean, and is instead a way of escaping the lifetime system. References with a `static` lifetime outlive everything else, and this means that they can be used in ways that would otherwise be invalid with the first method.

An alternative is to return an owning type like `String`. This eliminates the reference issues entirely, at the cost of possibly unnecessary allocations.

There is also the `Cow` ("copy on write") type, which will only do the extra allocation if you attempt to mutate the contained value.

#### How do I return a closure from a function?

To return a closure from a function, it must be what's called a "move closure", meaning that the closure is declared with the `move` keyword. As [explained in the Rust book](https://doc.rust-lang.org/book/closures.html#move-closures), this gives the closure its own stack frame, so it is not dependent on its parent stack frame. Otherwise, returning a closure would be unsafe, as it would allow access to variables that are no longer defined (put another way, it would allow reading potentially invalid memory). The closure must also be wrapped in a `Box`, so that it is allocated on the heap. Read more about this [in the book](https://doc.rust-lang.org/book/closures.html#returning-closures).

#### When are lifetimes required to be defined?

Lifetimes can often be elided, as explained in the ["Lifetime elision" section](https://doc.rust-lang.org/book/lifetimes.html#lifetime-elision) of the Rust book. "Elided lifetimes" are those lifetimes with are implicit in any code containing references. They are automatically inserted by the compiler with the three following rules:

- Each elided lifetime in a function’s arguments becomes a distinct lifetime parameter.
- If there is exactly one input lifetime, elided or not, that lifetime is assigned to all elided lifetimes in the return values of that function.
- If there are multiple input lifetimes, but one of them is &self or &mut self, the lifetime of self is assigned to all elided output lifetimes.

If these rules would result in incorrect code elsewhere, then the Rust compiler will provide errors, and you will need to define the relevant lifetimes to correct that error.

#### How can Rust guarantee "no null pointers"?

Data values in the language can only be constructed through a fixed set of initializer forms. Each of those forms requires that its inputs already be initialized. A liveness analysis ensures that local variables are initialized before use.

## Ownership

#### How can I implement a graph or other data structure that contains cycles?

There are two major options:

- You can implement it using `Weak` or `Rc` to allow shared ownership of nodes,
although this approach pays the cost of memory management.
- You can implement it using `unsafe` code using raw pointers. This will be
more efficient, but bypasses Rust's safety guarantees.

#### How can I define a struct that contains a pointer to one of its own fields?

At the moment, you can't without `unsafe`. If you have a struct with a pointer to one of its own fields, the Rust compiler has no way to know if the pointed-to field will be destroyed before or after the containing struct, and so it can't guarantee that the pointer into the field won't potentially point to invalid memory.

#### What does it mean to "consume a value"?

"Consuming a value" means taking ownership of a value and then dropping it. When this is done, the value can't be used elsewhere.

#### What is the difference between consuming and moving/taking ownership?

These are different terms for the same thing. In both cases, it means the value has been moved into a function, and moved out of the calling owner.

#### Why when I pass a struct to a function the compiler says it's been moved and I can't use it anymore, when the same doesn't happen for integers?

If a type implements the `Copy` trait, then it will be copied when passed to a function. All numeric types in Rust implement `Copy`, but function types do not, so they are moved instead. This means that the function can no longer be used elsewhere, unless it is moved back out of the function via the return.

#### How do you deal with a "Use of moved value" error?

This error means that the value you're trying to use has been moved to a new owner. The first thing to check is whether the move in question was necessary. If it moved into a function, it may be possible to rewrite the function to use a reference, rather than moving. It may also be possible to implement `Copy` (for implicit copying, rather than moving) or `Clone` (explicit copying) for the type in question. If none of these are possible, you may want to return ownership of the value when the function is done, thus eliminating this error.

#### What are the rules for using `self`, `&self`, or `&mut self` in a method declaration?

If a method needs to consume the struct, declare it with `self` as the first parameter, if it only needs a reference use `&self`, and if it needs to mutate the struct without consuming it use `&mut self`.

#### How can I understand the borrow checker?

There is a certain desire to act as if the borrow checker is some mysterious zen master, doling out knocks on the head whenever its koans are misunderstood. The reality is a little different. In fact, the borrow checker is simply applying a couple simple rules, which can be found in the Rust book's [section on borrowing](https://doc.rust-lang.org/stable/book/references-and-borrowing.html#the-rules):

> First, any borrow must last for a scope no greater than that of the owner. Second, you may have one or the other of these two kinds of borrows, but not both at the same time:
>
> - one or more references (&T) to a resource.
> - exactly one mutable reference (&mut T)

Understanding these rules and understanding [lifetimes](https://doc.rust-lang.org/stable/book/lifetimes.html) is all you need to do to understand the borrow checker.

#### How do deref coercions work?

[Deref coercions](https://doc.rust-lang.org/book/deref-coercions.html) exist to make using Rust more ergonomic, and are implemented via the [`Deref`](https://doc.rust-lang.org/stable/std/ops/trait.Deref.html) trait, which looks like this:

```rust
pub trait Deref {
    type Target: ?Sized;
    fn deref(&'a self) -> &'a Self::Target;
}
```

A Deref implementation indicates that the implementing type may be converted into `Target` by a call to the `deref` function, which takes an immutable reference of a certain lifetime to the calling struct, and returns a reference of the same lifetime to the target.

You can see a [full list of `Deref` implementations](https://doc.rust-lang.org/stable/std/ops/trait.Deref.html#implementors) for the standard library in the documentation.

## Stability

#### Can I use unstable features in the beta or stable channel?

No, you cannot. Rust works hard to provide strong guarantees about the stability of the APIs provided on the beta and stable channels. When something is unstable, it means that we can't provide those guarantees for it yet, and don't want people relying on it staying the same. This gives us the opportunity to try changes in the wild on the nightly channel, while still maintaining strong guarantees for people seeking stability.

Things stabilize all the time, and the beta and stable channels update every six weeks, so if you're waiting for a feature to be available without using the nightly channel, it should hopefully happen quickly.

## Strings

#### How can I convert a `String` or `Vec<T>` to a slice (`&str` and `&[T]`)?

Using Deref coercions `Strings` and `Vec`s will automatically coerce to their respective slices when passed by reference with `&` or `& mut`.

#### How can I convert from `&str` to `String` or the other way around?

`String`s are automatically converted into `&str` when you take a reference of them. For example, the following code works:

```rust
fn say_hello(name: &str) {
  println!("Hello {}!", name);
}

fn main() {
  let s = "Jane Doe".to_owned();
  say_hello(&s);
}
```

The `to_owned()` is how you convert from a `&str` into a `String`. This is necessary because string literals in Rust are of type `&str`. In this particular example, making `s` into a `String` isn't actually necessary, and the code could be rewritten as:

```rust
fn say_hello(name: &str) {
  println!("Hello {}!", name);
}

fn main() {
  let s = "Jane Doe";
  say_hello(s);
}
```

#### How do I split a `String` into lines?

The [`lines()`](https://doc.rust-lang.org/stable/std/string/struct.String.html#method.lines) function for `String`s provides an iterator over the lines of a string, which can then be `collect()`ed into a container of lines like so:

```rust
let s = "This\nis\na\ntest";
let v: Vec<&str> = s.lines().collect();
```

#### What are the differences between the different string types?

`String` is an owned string type, while `&str` is a string slice. For a more detailed explanation, [check out the Rust book](https://doc.rust-lang.org/stable/book/).

#### How do I do O(1) character access in a `String`?

`str` implements both the `Index` and `IndexMut` traits, but because Rust strings are all UTF-8, and so it can be indexed exactly as you would expect. But if you do that, it comes with some serious caveats. Strings in Rust are all UTF-8 encoded, and O(1) access to characters is impossible in UTF-8. If you index by bytes as you normally would in ASCII strings you'll get a UTF-8 codepoint, which may or may not be an actual character. If you try to index a location that is not a valid UTF-8 boundary, the operation with panic, which makes it doubly unlikely that indexing is what you're looking for.

If you are absolutely certain your string is in fact ASCII, you can get O(1) access by indexing the underlying buffer like so:

```rust
let s = "This is a test.";
let bytes = s.into_bytes();
let c = bytes[2];  // 'i' character
```

But remember that this operation is almost certainly wrong most of the time.

#### Why are strings UTF-8 by default?

The `str` type is UTF-8 because we observe more text in the wild in this encoding – particularly in network transmissions, which are endian-agnostic – and we think it's best that the default treatment of I/O not involve having to recode codepoints in each direction.

This does mean that indexed access to a Unicode codepoint inside a `str` value is an O(n) operation. On the one hand, this is clearly undesirable; on the other hand, this problem is full of trade-offs and we'd like to point a few important qualifications:

Scanning a `str` for ASCII-range codepoints can still be done safely octet-at-a-time. If you use `.as_bytes()`, pulling out a `u8` costs only `O(1)` and produces a value that can be cast and compared to an ASCII-range `char`. So if you're (say) line-breaking on `'\n'`, octet-based treatment still works. UTF8 was well-designed this way.

Most "character oriented" operations on text only work under very restricted language assumptions sets such as "ASCII-range codepoints only". Outside ASCII-range, you tend to have to use a complex (non-constant-time) algorithm for determining linguistic-unit (glyph, word, paragraph) boundaries anyways. We recommend using an "honest" linguistically-aware, Unicode-approved algorithm.

The `char` type is UTF32. If you honestly need to do a codepoint-at-a-time algorithm, it's trivial to write a `type wstr = [char]`, and unpack a `str` into it in a single pass, then work with the `wstr`. In other words: the fact that the language is not "decoding to UTF32 by default" shouldn't stop you from decoding (or re-encoding any other way) if you need to work with that encoding.

## Collections

#### Can I implement data structures like vectors and linked lists efficiently in Rust?

If your reason for implementing these data structures is to use them for other programs, there's no need, as all the common data structures are provided with the standard library.

If, however, your reason is simply to learn, then you will likely need to dip into unsafe code. While these data structures _can_ be implemented entirely in safe Rust, the performance is likely to be worse than they would be with the use of unsafe code. The simple reason for this is that data structures like vectors and link lists rely on pointer and memory operations that are disallowed by safe Rust.

For example, a doubly-linked list requires that there be two mutable references to each node, but this violates Rust's mutable reference aliasing rules. You can solve this using `Weak<T>`, but the performance will be poorer than you likely want.

#### How can I join a `Vec` (or an array) of strings into a single string?

Whether the strings are owned strings (`String`) or string slices (`&str`), you can accomplish this with a fold, like so:

```rust
fn main() {
  // For vector of `&str`
  let v1 = vec!["This ", "is ", "a ", "sentence."];
  let s1 = v1.iter.fold(String::new(), |acc, s| acc.push_str(s));
  println!("{}", s1);  // >> This is a sentence.

  // For vector of `String`
  let v2 = vec![
    "This ".to_owned(),
    "is ".to_owned(),
    "another ".to_owned(),
    "sentence.".to_owned()
  ];
  let s2 = v2.iter.fold(String::new(), |acc, &s| acc.push_str(s));
  println!("{}", s2);  // >> This is another sentence.
}
```

#### How can I iterate over a `Vec<T>` without moving/consuming it?

The easiest way is by using `Vec`'s `IntoIterator` implementation, like so:

```rust
let v = vec![1,2,3,4,5];
for item in &v {
    print!("{} ", item);
}
println!("\nLength: {}", v.len());
```

The way Rust `for` loops work, they actually call `into_iter()` (which is defined in the `IntoIterator` trait) for whatever you are trying to iterate over. `IntoIterator` is implemented for `&'a Vec<T>` and `&'a mut Vec<T>`, meaning you can iterate over a vector without consuming it just be using `&v` or `&mut v` (for some vector `v`).

#### How do you write a linked list in Rust?

You probably don't need to. The Rust standard library provides a doubly-linked list for free. But if you're looking to do it just for personal edification, there are three ways of doing it:

- Write it as a classic algebraic data-type (also called a "cons list"). This is easiest, but the performance characteristics aren't good at all
- Write it using `Weak` or `Rc` to allow shared ownership of nodes. This can work, but is also a pain. If you're using `Rc`, you need some way to break cycles, otherwise no memory will ever be freed.
- Use `unsafe`. This is the most performant, but also the toughest, because you can't rely on Rust's safety guarantees.

#### Why do I need to type the array size in the array declaration?

You don't necessarily have to. If you're declaring an array directly, the size is inferred based on the number of elements. But if you're declaring a function that takes an array, the compiler has to know how big that array will be.

One thing to note is that currently Rust doesn't offer generics over arrays of different size. This is being worked on, but isn't available currently. If you'd like to accept a contiguous container of a variable number of values, use a Vec or slice (depending on whether you need ownership).

## Documentation

#### Where do I report issues in the Rust documentation?

You can report issues in the Rust documentation on the Rust compiler [issue tracker](https://github.com/rust-lang/rust/issues). Make sure to read the [contributing guidelines](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md) first.

#### How do I view rustdoc documentation for a library my project depends on?

When you use `cargo doc` to generate documentation for your own project, it also generates docs for the active dependency versions. These are put into the `target/doc` directory of your project. Use `cargo doc --open` to open the docs after building them, or just open up `target/doc/index.html` yourself after building the docs.

## Input / Output

#### How do I read a file into a `String`?

Using the `read_to_string()` method, which is defined on the `Read` trait in `std::io`.

```rust
fn main() {
  let mut f = try!(File::open("foo.txt"));
  let mut s = String::new();
  try!(f.read_to_string(&mut s));  // `s` contains the contents of "foo.txt"
}
```

#### How do I read file input efficiently?

The [`File` type](https://doc.rust-lang.org/stable/std/fs/struct.File.html) implements the `Read` trait, which has a variety of functions for reading and writing data, including `read()`, `read_to_end()`, `bytes()`, `chars()`, and `take()`. Each of these functions reads in a certain amount of input from a given file. `read()` reads input until the provided buffer is full. `read_to_end()` reads the entire buffer into a vector, allocating as much space as is needed. `bytes()` and `chars()` allow you to iterate over the bytes and characters of the file, respectively. Finally, `take()` allows you to read up to an arbitrary number of bytes from the file. Collectively, these should allow you to efficiently read in any data you need.

#### How do I get command line arguments in Rust?

The easiest way is to use `std::env::Args`, which provides an iterator over the input arguments.

If you're looking for something more powerful, the [getopt](https://doc.rust-lang.org/getopts/getopts/index.html) and [docopt](https://github.com/docopt/docopt.rs) crates are both solid options.

## Libraries

#### How can I make an HTTP request?

[Hyper](https://github.com/hyperium/hyper) is the most popular, but there are [a number of others as well](https://crates.io/search?q=http).

#### How can I write a GUI application in Rust?

There are a variety of ways to write GUI applications in Rust. Just check out [this list of GUI frameworks](https://github.com/kud1ing/awesome-rust#gui).

#### How can I parse JSON/XML?

[Serde](https://github.com/serde-rs/serde) is the recommended library for serialization and deserialization of Rust data to and from a number of different formats.

#### Is there a standard 2D+ dimensional vector and shape crate?

Not yet! Want to write one?

#### How do I write an OpenGL app in Rust?

[Glium](https://github.com/tomaka/glium) is the major library for OpenGL programming in Rust. [GLFW](https://github.com/bjz/glfw-rs) is also a solid option.

#### Can I write a video game in Rust?

Yes you can! The major game programming library for Rust is [Piston](http://www.piston.rs/), and there's a whole [community for game programming in Rust](https://www.reddit.com/r/rust_gamedev/) as well.

## Project

#### What is this project's goal?

To design and implement a safe, concurrent, practical, static systems language.

Rust exists because other languages at this level of abstraction and efficiency are unsatisfactory. In particular:

1. There is too little attention paid to safety.
2. They have poor concurrency support.
3. There is a lack of practical affordances.

Rust exists as alternative language that provides both efficient code and a comfortable level of abstraction, while improving on all three of these points.

#### Is this project controlled by Mozilla?

No. Rust started as Graydon Hoare's part-time side project in 2006 and remained so for over 3 years. Mozilla got involved in 2009 once the language was mature enough to run basic tests and demonstrate its core concepts. Though it remains sponsored by Mozilla, Rust is developed by a diverse community of enthusiasts from many different places around the world.

#### What are explicit non-goals of Rust?

1. To employ any particularly cutting-edge technologies. Old, established techniques are better.
2. To prize expressiveness, minimalism or elegance above other goals. These are desirable but subordinate goals.
3. To cover the complete feature-set of C++, or any other language. It should provide majority-case features.
4. To be 100% static, 100% safe, 100% reflective, or too dogmatic in any other sense. Trade-offs exist.
5. To run on "every possible platform". It must eventually work without unnecessary compromises on widely-used hardware and software platforms.

#### Who is using Rust?

There are a number of companies using Rust, including:

- [OpenDNS](https://labs.opendns.com/2013/10/04/zeromq-helping-us-block-malicious-domains/)
- [Skylight](https://www.skylight.io/)
- [Wit.ai](https://github.com/wit-ai/witd)
- [Codius](https://codius.org/blog/codius-rust/)
- [MaidSafe](http://maidsafe.net/)
- [Terminal.com](https://www.terminal.com/)

#### In which projects is Mozilla using Rust?

The main project is [Servo](https://github.com/servo/servo), an experimental browser engine Mozilla is working on. They are also working to integrate Rust components into Firefox.

#### What examples are there of large Rust projects?

The two biggest open source Rust projects right now are [Servo](https://github.com/servo/servo) and the [Rust compiler](https://github.com/rust-lang/rust) itself.

<!--
#### What projects are good examples of idiomatic Rust code?

TODO: Write this answer.
-->

#### How can I try Rust easily?

The easiest way to try Rust is through the [Playground](https://play.rust-lang.org/), an online app for writing and running Rust code. If you want to try it on your system, [install it](https://www.rust-lang.org/install.html) and go through the ["Learn Rust"](https://doc.rust-lang.org/stable/book/learn-rust.html) section of the book.

#### How do I get help with Rust issues?

There are several ways. You can:

- Post in [/r/rust](https://www.reddit.com/r/rust), the official Rust subreddit
- Post in [users.rust-lang.org](https://users.rust-lang.org/), the official Rust forum
- Ask in the official [Rust IRC channel](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust) (#rust on irc.mozilla.org)

#### Why has Rust changed so much over time?

Rust started with a goal of creating a safe but usable systems programming language. In pursuit of this goal it explored a lot of ideas, some of which it kept (lifetimes, traits) others which it discarded (the typestate system). Also, in the run up to 1.0 a lot of the standard library was rewritten. Some of this was out of a desire to improve the old versions of the APIs. Some of it was out of a desire for improved Windows support.

#### What IDE should I use?

There are a couple developing options: [RustDT](https://github.com/RustDT/RustDT) is a Rust plugin for Eclipse, while [SolidOak](https://github.com/oakes/SolidOak) is a from-scratch Rust IDE. Both are solid options for a Rust IDE.

## Modules and Crates

#### What is the relationship between a module and a crate?

- A crate is a top-level compilation unit that corresponds to a single loadable object.
- A module is a (possibly nested) unit of name-management inside a crate.
- A crate contains an implicit, un-named top-level module.
- Recursive definitions can span modules, but not crates.
- Crates do not have global names, only a set of non-unique metadata tags.
- There is no global inter-crate namespace; all name management occurs within a crate.
- Using another crate binds the root of its namespace into the user's namespace.

#### Why can't the Rust compiler find this library I'm `use`ing?

There are a number of possible answers, but a common mistake is not realizing that `use` declarations are _always_ relative to the crate root. Try rewriting your declarations to use the paths they would use if defined in the root file of your project and see if that fixes the problem.

#### Why do I have to declare module files with mod at the top level of the crate, instead of just `use`ing them?

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

#### How do I configure Cargo to use a proxy?

As explained on the Cargo [configuration documentation](http://doc.crates.io/config.html), you can set Cargo to use a proxy by setting the "proxy" variable under `[http]` in the configuration file.

#### Why can't the compiler find the method implementation even through I'm already `use`ing the crate?

For methods defined on a trait, you have to explicitly import the trait declaration. This means it's not enough to import a module where a struct implements the trait, you must also important the module containing the trait itself.

#### Why can't the compiler infer `use` declarations for me?

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
#### How do I package and archive crates from [http://crates.io](http://crates.io)?

TODO: Write this answer.
-->

#### How do I do dynamic Rust library loading?

Importing dynamic libraries in Rust is done using [`std::dynamic_lib`](https://doc.rust-lang.org/std/dynamic_lib/index.html). Note that this entire module is currently marked unstable, and that a number of things may change in the current implementation.

#### Why doesn't [http://crates.io](http://crates.io) have namespaces?

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

#### Why are so many Rust answers on StackOverflow wrong?

The Rust language has been around for a number of years, and only reached version 1.0 in May of 2015. In the time before then the language changed significantly, and a number of StackOverflow answers were given at the time of older versions of the language.

Over time more and more answers will be offered for the current version, this improving this issue as the proportion of out-of-date answers is reduced.

## Generics

#### What is "monomorphisation"?

Monomorphisation is the process by which Rust generates specific instances of a generic function based on the types of the various calls to that function. This is used to provide static dispatch for generic functions. For functions using trait objects for generics, dynamic dispatch is used instead, with calls to the function going through a vtable to identify specific function calls for the provided type implementing the given trait.

In C++ people would likely know this as "template instantiation." But unlike C++, Rust's monomorphisation is an implementation detail, and not a language feature.

#### What's the different between a function and a closure that doesn't capture any variables?

Functions are a built-in primitive of the language, while closures are essentially syntactic sugar for one of three traits: `Fn`, `FnMut`, and `FnOnce`. When you make a closure, the Rust compiler automatically creates a struct implementing the appropriate trait of those three and containing the captured environment variables as members, and makes it so the the struct can be called as a function.

The big difference between these traits is how they take the `self` parameter. `Fn` takes `&self`, `FnMut` takes `&mut self`, and `FnOnce` takes `self`.

#### What are higher-kinded types, why should I want them, and why doesn't Rust have them?

Let's go through these one by one:

First, higher-kinded types are types that take other types as parameters. Type constructors, like `Vec` are examples. What support for higher-kinded types means is that you can use these type constructors wherever you can use types, such as in a trait `impl`.

Next, the lack of higher-kinded types makes expression of certain ideas far more tedious than it would otherwise be. For example, implementing a `Functor` trait (a math-y term for something which can be mapped over, obeying certain rules) without higher-kinded types is a pain. With higher-kinded types, it's as simple as (note: theoretical syntax, this does not actually work, and has no guarantee to be how higher-kinded types look when and if they're implemented):

```rust
trait Functor {
    fn fmap<A, B, F: Fn(&A) -> B>(&self, f: F) -> Self<B>;
}
```

The part that Rust currently can't do is that `Self` above is a type constructor parameterized by another type constructor. This is what support for higher-kinded types lets you express.

Finally, Rust doesn't currently have them simply because they haven't been a priority. There is nothing inherent to the language that stops us from implementing higher-kinded types, it just hasn't been done yet. There is an open RFC for implementing them, but no real proposal exists yet. If one comes around, it will definitely be considered.

(Credit to anasaziwochi for his [useful explanation](https://www.reddit.com/r/rust/comments/2av5tv/why_does_rust_not_have_a_functor_trait/ciz6iwm) of higher-kinded types in Rust.)

#### What do named type parameters in generic types mean?

These are called [associated types](https://doc.rust-lang.org/stable/book/associated-types.html), and they allow for the expression of trait bounds that can't be expressed with a simple `where` clause. In essence, for a generic type with some type parameters, it is often unecessary to include those type parameters in a function taking that generic type as a parameter. The function shouldn't have to care about being generic over the types which make up the generic type (say, the node and edge types in a graph), but only about being generic over the type itself.

#### Does Rust have type reflection?

Rust does have limited type reflection through the `Reflect` and `Any` traits. `Reflect` is a marker trait (meaning it has no functions associated with it) that indicates a function expects a type which can be reflected over. It is implemented for all types. `Any` is automatically implemented for any type that is both `Reflect` and `'static`, and is used for basic dynamic typing, as in the following example:

```rust
use std::any::Any;
fn foo<T: Any>(x: &T) {
    let any: &Any = x;
    if any.is::<u32>() { println!("u32"); }
}
```

Note that the above could be equivalently written as:

```rust
#![feature(reflect_marker)]
use std::marker::Reflect;
use std::any::Any;
fn foo<T: Reflect + 'static>(x: &T) {
    let any: &Any = x;
    if any.is::<u32>() { println!("u32"); }
}
```

#### Can I override operators? Which ones and how?

You can provide custom implementations for a variety of operators using their associated traits: `Add` for `+`, `Mult` for `*`. It looks like this:

```rust
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

| Operation           | Trait      |
|:--------------------|:-----------|
| `+`                 | `Add`      |
| `binary -`          | `Sub`      |
| `*`                 | `Mul`      |
| `/`                 | `Div`      |
| `unary -`           | `Neg`      |
| `%`                 | `Rem`      |
| `&`                 | `BitAnd`   |
| <code>&#124;</code> | `BitOr`    |
| `^`                 | `BitXor`   |
| `!`                 | `Not`      |
| `<<`                | `Shl`      |
| `>>`                | `Shr`      |
| `*`                 | `Deref`    |
| `mut *`             | `DerefMut` |
| `[]`                | `Index`    |
| `mut []`            | `IndexMut` |

#### Why the split between `Eq`/`PartialEq` and `Ord`/`PartialOrd`?

There are some types in Rust that have a partial ordering, or partial equality, but no total ordering or total equality. The floating point types `f32` and `f64` are examples of this. Because a floating point value may be `NaN`, and because `NaN`s aren't equal to any other floating point type, nor less than or greater to any other floating point type, nor equal to each other, these types _can't_ implement `Eq` and `Ord`, although they _can_ implement `PartialEq` and `PartialOrd`.

## Debugging

#### How do I debug Rust programs?

Rust programs can be debugged using gdb or lldb, same as C and C++. In fact, every Rust installation comes with rust-gdb and rust-lldb, wrappers over gdb and lldb with Rust pretty-printing enabled.

#### `rustc` said a panic occurred in standard library code. How do I locate the mistake in my code?

This error is usually caused by `unwrap()`ing a `None` or `Err`. Enabling backtraces by setting the environment variable `RUST_BACKTRACE=1` helps with getting more information. Compiling in debug mode (the default for `cargo build` is also helpful). Using a debugger like the provided `rust-gdb` or `rust-lldb` is also helpful.

## Low-Level

#### How do I `memcpy` bytes?

To copy potentially overlapping bytes, use `std::ptr::copy`. To copy nonoverlapping bytes, use `std::ptr::cpy_nonoverlapping`. Both of these functions are `unsafe`, as both can be used to subvert the language's safety guarantees. Use caution when using them.

#### Can Rust reasonably function without the standard library?

Absolutely. Rust programs can be set to not load the standard library using the `#![no_std]` attribute. With this attribute set, you can continue to use the Rust core, which is nothing but the platform-agnostic primitives. As such, it doesn't include IO, concurrency, heap allocation, etc. Note however that `libcore` has not been stabilized.

#### Can I write an operating system in Rust?

Yes! In fact there are several projects underway doing just that, including [reenix](https://github.com/scialex/reenix) and [RustOS](https://github.com/ryanra/RustOS).

#### How can I read or write numeric types like `i32` or `f64` in big-endian or little-endian format in a file or other byte stream?

You should check out the [byteorder crate](http://burntsushi.net/rustdoc/byteorder/), which provides utilities for exactly that.

#### Does Rust specify data layout?

No. In the general case, `enum` and `struct` layout is undefined. This allows the compiler to potentially do optimizations like re-using padding for the discriminant, compacting variants of nested `enum`s, reordering fields to remove padding, etc. `enums` which carry no data ("C-like") are eligible to have a defined representation. Such `enums` are easily distinguished in that they are simply a list of names that carry no data:

```rust
enum CLike {
    A,
    B = 32,
    C = 34,
    D
}
```

The `repr` attribute can be applied to such `enums` to give them the same representation as a primitive. This allows using Rust `enum`s in FFI where C `enum`s are also used, for most use cases. The attribute can also be applied to `struct`s to get the same layout as a C `struct` would.

## Cross-Platform

<!--
#### How do I build a Windows binary that doesn't display the console window?

TODO: Write this answer.
-->

<!--
#### How I make the console-less binary not crash on panic!?

TODO: Write this answer.
-->

#### What's the idiomatic way to express platform-specific behavior in Rust?

Platform-specific behavior can be expressed using [conditional compilation attributes](https://doc.rust-lang.org/reference.html#conditional-compilation) such as `target_os`, `target_family`, `target_endian`, etc.

#### Can Rust be used for Android/iOS programming?

Yes it can! There are already examples of using Rust for both [Android](https://github.com/tomaka/android-rs-glue) and [iOS](https://www.bignerdranch.com/blog/building-an-ios-app-in-rust-part-1/). It does require a bit of work to set up, but Rust functions fine on both platforms.

#### Can I run my Rust program in a web browser?

There [are efforts](https://www.bignerdranch.com/blog/building-an-ios-app-in-rust-part-1/) to make Rust run in the web browser using Emscripten. Given that Rust is implemented on top of LLVM, this should be possible, but Rust currently runs on a special patched version of LLVM, and so it is tougher than it would otherwise be. This will likely improve over time.

#### How do I cross-compile in Rust?

Cross compilation is possible in Rust, but it requires a bit of work to set up, which is covered here: https://github.com/japaric/ruststrap/blob/master/1-how-to-cross-compile.md

## Design Patterns

#### Is Rust object oriented?

It is multi-paradigm. Not everything is shoe-horned into a single abstraction. Many things you can do in OO languages you can do in Rust, but not everything, and not always using the same abstraction you're accustomed to.

#### How do I handle configuration of a struct with optional parameters?

The easiest way is to use the `Option` type in whatever function you're using to construct instances of the struct (usually `new()`). Another way is to use the builder pattern, where only certain functions instantiating member variables must be called before the construction of the built type.

#### How do I do global variables in Rust?

Globals in Rust can be done using `const` declarations for compile-time computed global constants, while `static` can be used for mutable globals. Note that modifying a `static` variable requires the use of `unsafe`, as it allows for data races, one of the things guaranteed not to happen in safe Rust.

#### How can I set compile-time constants that are defined procedurally (equivalent to constexpr in C++)?

Rust currently has limited support for compile time constants. You can define primitives using `const` declarations (similar to `static`, but immutable). You can also define some compile-time constants using compiler plugins. The closest thing you have to full Compile-Time Function Evaluation (CTFE) is the [`lazy-static`](https://github.com/rust-lang-nursery/lazy-static.rs) crate, which lazily evaluates constants (guaranteed to be evaluated once) at runtime, rather than evaluating them at compile time. CTFE is open for consideration in the future, it simply hasn't been a priority thus far.

#### Can I run initialization code that happens before main?

Rust has no concept of "life before `main`". The closest you'll see can be done through the ["lazy_static"](https://github.com/Kimundi/lazy-static.rs) crate, which simulates a "before main" by lazily initializing static variables at their first usage.

#### How do I map OO concepts to Rust?

Trying to directly map object oriented programming concepts to Rust is unlikely to be fruitful. Rust has its own features and its own way of doing things, and you are better off learning how to use them than you are trying to simulate another style the language wasn't designed for.

#### Why doesn't Rust have inheritance?

There are two meanings for the word "inheritance": _subtyping_, and _interface sharing_. Both purposes are already handled by traits.

For the first, subtyping exists for polymorphism, which traits already provide.

For the second, interface sharing is handled via trait methods, which define a collection of related functions that must be implemented for any implementation of the trait.

Rust has consistently worked to avoid having features with overlapping purposes, preferring to keep features orthogonal. For this reason, and given that the two major purposes are already handled by traits, Rust has opted not to include inheritance.

#### Does Rust allow non-constant-expression values for globals?

No. Globals can not have a non-constant-expression constructor and cannot have a destructor at all. This is an opinion of the language. Static constructors are undesirable because they can slow down program startup. Life before main is often considered a misfeature, never to be used. Rust helps this along by just not having the feature.

See the [C++ FQA](http://yosefk.com/c++fqa/ctors.html#fqa-10.12) about the "static initialization order fiasco", and [Eric Lippert's blog](http://ericlippert.com/2013/02/06/static-constructors-part-one/) for the challenges in C#, which also has this feature.

A nice replacement is the [lazy-static](https://github.com/rust-lang-nursery/lazy-static.rs) library by Marvin Löbel.

## Macros

#### Can I write a macro to generate identifiers?

Not currently. Rust macros are so-called hygienic macros, not the unlimited compile-time syntax editing you see in C. Macro invocations can only appear in places where they are explicitly supported: items, methods declarations, statements, expressions, and patterns. Here, "method declarations" means a blank space where a method can be put. They can't be used to complete a partial method declaration. By the same logic, they can't be used to complete a partial variable declaration.

## Other Languages

### C

#### How can I implement something like `struct X { static int X; };` in Rust?

Rust does not have `static` fields as shown in the code snippet above. Instead, you can declare a `static` variable in a given module, which is kept private to that module.

#### How can I convert a C-style enum to an integer, or vice-versa?

Converting a C-style enum to an integer to an integer can be done with a simple `as` expression, like `e as i64` (where `e` is some enum).

Converting in the other direction is a little tougher, as Rust can't statically ensure that the conversion is valid. As such, it requires `unsafe` via `unsafe::reinterpret_cast()`.

#### Why does Rust not have an ABI like C does, and why do I have to annotate things with extern?

Committing to an ABI is a big thing, and can limit potentially advantageous language changes in the future. Given that Rust only hit 1.0 in May of 2015, it is still too early to make a commitment as big as a stable ABI. This does not mean that one won't happen in the future, though.

#### Can Rust code call C code?

Yes. Calling C code from Rust is simple and exactly as efficient as calling C code from C.

#### Can C code call Rust code?

Yes. The Rust code has to be exposed via an `extern` declaration, which makes it C-ABI compatible. Such a function can be passed to C code as a function pointer or, if given the `#[no_mangle]` attribute to disable symbol mangling, can be called directly from C code.

### C++

#### I already write perfect C++. What does Rust give me?

Modern C++ has made a lot of strides to being a safe and comfortable programming language, but it's not perfect, and it's still very easy to do things unsafely. This is something the C++ core developers are working to overcome, but C++ is limited by a long history that predates a lot of the ideas they are now trying to implement.

Rust was designed from day one to be a safe systems programming language, which means it's not limited by historic design decisions that make getting safety right in C++ so complicated. In C++, safety is achieved by careful personal discipline, and is very easy to get wrong. In Rust, safety is the default.

#### How do I do the equivalent of C++ template specialization in Rust?

Rust doesn't currently have an equivalent to template specialization, but it is [being worked on](https://github.com/rust-lang/rfcs/pull/1210) and will hopefully be added soon.

#### How does Rust's ownership system related to move semantics in C++?

In C++, moving vs copying was added on late with C++11. With Rust the concept of moving vs copying has been around from the beginning. In C++ something can be moved into a function or out of a function using r-value references and either `std::move` or `std::forward`. In Rust, moves happen for anything that does not implement the `Copy` trait (which will cause the value of the type to be copied, rather than moved). This means that moves are the default operation, and that copies must be opted into explicitly. It's also important to know that moves in Rust are destructive copies, which call `Drop` (equivalent to a C++ destructor) on the moved value.

Moves are often not necessary or desirable in Rust. If the function you're writing does not require ownership of the value being passed in, it should probably be borrowed (mutably or immutably, as necessary) rather than moved or copied.

It's also useful to note that functions can explicitly require that an input parameter be copied like so:

```rust
fn must_copy<T: Copy>(t: T) {
    // Do something in here...
}
```

#### How can I interoperate with C++ from Rust, or with Rust from C++?

The simplest way is to interoperate through C. Both Rust and C++ provide a [foreign function interface](https://doc.rust-lang.org/book/ffi.html) for C, and can use that to communicate between each other. If writing C bindings is too tedious, you can always use [rust-bindgen](https://github.com/crabtw/rust-bindgen) to help automatically generate workable C bindings.

#### Does Rust have C++-style constructors?

No. Functions can serve the same purpose as constructors without adding any language complexity. The usual name for the constructor-equivalent function in Rust is `new()`, although this is just a social norm rather than a language rule. The `new()` function in fact is just like any other function. An example of it looks like so:

```rust
struct Foo {
    a: i32,
    b: f64,
    c: bool
}

impl Foo {
    fn new() -> Foo {
        Foo {
            a: 0,
            b: 0.0,
            c: false
        }
    }
}
```

#### Does Rust have copy constructors?

Not exactly. Types which implement `Copy` will do a standard C-like "shallow copy" with no extra work (similar to "plain old data" in C++). It is impossible to implement `Copy` types that require custom copy behavior. Instead, in Rust "copy constructors" are created by implementing the `Clone` trait, and explicitly calling the `clone` method. Making user-defined copy operators explicit surfaces the underlying complexity, forcing the developer to opt-in to potentially expensive operations.

#### Does Rust have move constructors?

No. Values of all types are moved via `memcpy`. This makes writing generic unsafe code much simpler since assignment, passing and returning are known to never have a side effect like unwinding.

### Swift

#### Why does Rust not have the ? and ! like in Swift?

In Swift, `?` is used to indicate an optional value. This is already done by `Option` in Rust, and so `?` is not needed. Similarly, `!` is used to "unwrap" an optional value in Swift, which is done by the `unwrap()` function in Rust. In both cases, Rust opted for slightly longer names which more clearly indicate intent. Also, in Rust the `!` symbol already indicates logical negation and macro calls, and does not need a third meaning.

### Go

#### How are Go and Rust similar, and how are they different?

Rust and Go have substantially different design goals, and so differ substantially. The following differences are not the only ones (which are too numerous to list), but are a few of the more important ones:

- Rust is lower level than Go, comparable with C, C++, D, and Nim. It provides access to memory management primitives that do not exist in Go (which has a garbage collector)
- Rust's focus is on ensuring safety and efficiency while also providing high-level affordances, while Go's is on being a small, simple language which compiles quickly and can work nicely with a variety of tools.
- Rust has strong support for generics, which Go does not
- Rust has strong influences from the world of functional programming, including a type system which draws from Haskell's typeclasses. Go has a simpler type system, using interfaces for basic generic programming

Rust is probably not the best choice in every situation. If you're considering using either Rust or Go for a project it's probably best to try out both languages a bit and make an informed decision rather than base your decision on a small comparison of the two.

#### `gofmt` is great. Where's `rustfmt`?

`rustfmt` is [right here](https://github.com/nrc/rustfmt/), and is being actively developed to make reading Rust code as easy and predictable as possible.

### Haskell

#### How do Rust traits compare to Haskell typeclasses?

Rust traits are similar to Haskell typeclasses, but are currently not as powerful. Rust traits cannot express functional dependencies or type families, nor does Rust have full support for higher-kinded types. Some of these may be added in the future, but are not provided currently.

## Licensing

#### Why a dual MIT/ASL2 License?

The Apache license includes important protection against patent aggression, but it is not compatible with the GPL, version 2. To avoid problems using Rust with GPL2, it is alternately MIT licensed.

#### Why a BSD-style permissive license rather than MPL or tri-license?

This is partly due to preference of the original developer (Graydon), and partly due to the fact that languages tend to have a wider audience and more diverse set of possible embeddings and end-uses than products such as web browsers. We'd like to appeal to as many of those potential contributors as possible.

## Naming

#### Why is the language called Rust?

As [stated by Graydon Hoare](https://www.reddit.com/r/rust/comments/27jvdt/internet_archaeology_the_definitive_endall_source/), original developer of the Rust language, the name "Rust" comes from his personal interest in fungi, and because it evoked the feeling he was looking for in a programming language name. But truly, as Graydon himself put it:

> &lt;graydon&gt; IOW I don't have a really good explanation. it seemed like a good name. (also a substring of "trust", "frustrating", "rustic" and ... "thrust"?)<br>
> &lt;graydon&gt; I think I named it after fungi. rusts are amazing creatures.

