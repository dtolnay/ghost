## Define your own PhantomData
# https://tea.xyz/what-is-this-file
---
version: 1.0.0
codeOwners:
  - '0x8fF370dc6d6Bf42C25562E9d842c914d411E4109'
quorum: 1

[<img alt="github" src="https://img.shields.io/badge/github-dtolnay/ghost-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/dtolnay/ghost)
[<img alt="crates.io" src="https://img.shields.io/crates/v/ghost.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/ghost)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-ghost-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/ghost)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/dtolnay/ghost/ci.yml?branch=master&style=for-the-badge" height="20">](https://github.com/dtolnay/ghost/actions?query=branch%3Amaster)

This crate makes it possible to define your own PhantomData and similarly
behaved unit types with generic parameters, which is not permitted in ordinary
Rust.

```toml
[dependencies]
ghost = "0.1"
```

*Supports rustc 1.31+*

### Background

[`PhantomData`] as defined by the Rust standard library is magical in that the
same type is impossible to define in ordinary Rust code. It is defined in the
standard library like this:

[`PhantomData`]: https://doc.rust-lang.org/std/marker/struct.PhantomData.html

```rust
#[lang = "phantom_data"]
pub struct PhantomData<T: ?Sized>;
```

The `#[lang = "..."]` attribute indicates that this is a [lang item], a special
case known to the compiler. It is the only type permitted to carry an unused
type parameter.

[lang item]: https://manishearth.github.io/blog/2017/01/11/rust-tidbits-what-is-a-lang-item/

If we try to define an equivalent unit struct with type parameter, the compiler
rejects that.

```rust
struct MyPhantom<T: ?Sized>;
```

```console
error[E0392]: parameter `T` is never used
 --> src/main.rs:1:18
  |
1 | struct MyPhantom<T: ?Sized>;
  |                  ^ unused type parameter
  |
  = help: consider removing `T` or using a marker such as `std::marker::PhantomData`
```

This crate provides a `#[phantom]` attribute that makes it possible to define
unit structs with generic parameters.

### Examples

```rust
use ghost::phantom;

#[phantom]
struct MyPhantom<T: ?Sized>;

fn main() {
    // Proof that MyPhantom behaves like PhantomData.
    let _: MyPhantom<u8> = MyPhantom::<u8>;
    assert_eq!(0, std::mem::size_of::<MyPhantom<u8>>());
}

// Proof that MyPhantom is not just a re-export of PhantomData.
// If it were a re-export, these would be conflicting impls.
trait Trait {}
impl<T> Trait for std::marker::PhantomData<T> {}
impl<T> Trait for MyPhantom<T> {}

// Proof that MyPhantom is local to the current crate.
impl<T> MyPhantom<T> {
}
```

The implementation accepts where-clauses, lifetimes, multiple generic
parameters, and derives. Here is a contrived invocation that demonstrates
everything at once:

```rust
use ghost::phantom;

#[phantom]
#[derive(Copy, Clone, Default, Hash, PartialOrd, Ord, PartialEq, Eq, Debug)]
struct Crazy<'a, V: 'a, T> where &'a V: IntoIterator<Item = T>;

fn main() {
    let _ = Crazy::<'static, Vec<String>, &'static String>;

    // Lifetime elision.
    let crazy = Crazy::<Vec<String>, &String>;
    println!("{:?}", crazy);
}
```

### Variance

The `#[phantom]` attribute accepts attributes on individual generic parameters
(both lifetime and type parameters) to make them contravariant or invariant. The
default is covariance.

- `#[contra]` — contravariant generic parameter
- `#[invariant]` — invariant generic parameter

The implications of variance are explained in more detail by the [Subtyping
chapter] of the Rustonomicon.

[Subtyping chapter]: https://doc.rust-lang.org/nomicon/subtyping.html

```rust
use ghost::phantom;

#[phantom]
struct ContravariantLifetime<#[contra] 'a>;

fn f<'a>(arg: ContravariantLifetime<'a>) -> ContravariantLifetime<'static> {
    // This coercion is only legal because the lifetime parameter is
    // contravariant. If it were covariant (the default) or invariant,
    // this would not compile.
    arg
}

#[phantom]
struct Demo<A, #[contra] B, #[invariant] C>;
```

### Documentation

There are two alternatives for how to handle Rustdoc documentation on publicly
exposed phantom types.

You may provide documentation directly on the phantom struct in the obvious way,
but Rustdoc will blithely display the somewhat distracting implementation
details of the mechanism emitted by the `#[phantom]` macro. This way should be
preferred if you need to document any public methods, as methods will not be
visible in the other alternative.

```rust
use ghost::phantom;

/// Documentation.
#[phantom]
pub struct MyPhantom<T: ?Sized>;

impl<T: ?Sized> MyPhantom<T> {
    /// Documentation on methods.
    pub fn foo() {}
}
```

If you aren't adding methods or don't need methods to be rendered in the
documentation, the recommended idiom is as follows. Rustdoc will show a much
less distracting type signature and all of your trait impls, but will not show
inherent methods.

```rust
mod private {
    use ghost::phantom;

    #[phantom]
    pub struct MyPhantom<T: ?Sized>;
}

/// Documentation goes here.
#[allow(type_alias_bounds)]
pub type MyPhantom<T: ?Sized> = private::MyPhantom<T>;

#[doc(hidden)]
pub use self::private::*;
```

### Use cases

Entirely up to your imagination. Just to name one, how about a typed registry
library that admits the following syntax for iterating over values registered of
a particular type:

```rust
for flag in Registry::<Flag> {
    /* ... */
}
```

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
