# postfix-macros

[![docs](https://docs.rs/postfix-macros/badge.svg)](https://docs.rs/crate/postfix-macros)
[![crates.io](https://img.shields.io/crates/v/postfix-macros.svg)](https://crates.io/crates/postfix-macros)

Postfix macros on stable Rust, today.

```Rust
arr.get(10).unwrap_or!(return Err(()));

"hello world".println!();
42.assert_ne!(4 + 2);

val.iter()
	.map(|v| v.1)
	.find(|z| z.matches!(Custom::Enum(_) | Custom::EnumOther))
	.dbg!();
```

WARNING: ALPHA QUALITY SOFTWARE. There are precedence bugs.
Please verify manually during usage that you aren't affected by them.

## Explanation

[RFC 2442] proposes to add postfix macros to the Rust language.
However, there are still very basic concerns from lang team members,
and it seems they won't get resolved quickly, so it's unlikely
to be merged any time soon, if at all.

The `postfix-macros` crate provides you with a proc macro `postfix_macros`
that checks for `possibly.chained.expression.macro_name!(params)` patterns
and then rewrites them in terms of traditional macro invocation, prepending
the expression to the passed parameters. This turns every "bang" macro
that's available to you into a potential postfix macro ([UFCS] style).
As if that wasn't enough, this crate additionally provides a set of
macros for use in a postfix context, for your greatest convenience.

As an example, the `unwrap_or!` macro enables something that needed 5 lines before:

```Rust
let v = if let Some(v) = something {
	v
} else {
	continue
};
```

to be written in a single line inside a `postfix-macros` context:

```Rust
let v = something.unwrap_or!(continue);
```

Furthermore, it also replaces the `unwrap_or_else` closure pattern with
something that's closer to the `unwrap_or` function in terms of cleanliness,
while being just as lazily evaluating as `unwrap_or_else`:

```Rust
let v = something.unwrap_or_else(|| some_expensive_fn_call(1, 2, 3));
```

```Rust
let v = something.unwrap_or!(some_expensive_fn_call(1, 2, 3));
```

[RFC 2442]: https://github.com/rust-lang/rfcs/pull/2442
[UFCS]: https://en.wikipedia.org/wiki/Uniform_Function_Call_Syntax

## Footprint

This crate has no dependencies beyond the one proc macro dependency,
which itself has no dependencies. There is no reliance on the extremely
slow to compile syn crate. The compile time is thus very short,
and thus the crate has little footprint.

This choice also has some downsides, as syn is actually quite a powerful
tool, namely that the expression precedence is different in some places
to the precedence of normal Rust. These differences are regarded as bugs
and users are welcome to file reports about them. Relying on the
non-Rust-specific behaviour is not supported by the semver guarantee of
this crate.

## MSRV

The MSRV of this crate is `1.42.0`.

## TODO

* Send a PR to rustc to syntactically permit postfix macros so that we can use an attribute macro.
  See [this PR](https://github.com/rust-lang/rust/pull/75857) for prior art.

## License
[license]: #license

This tool is distributed under the terms of both the MIT license
and the Apache License (Version 2.0), at your option.

See [LICENSE](LICENSE) for details.

### License of your contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for
inclusion in the work by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
