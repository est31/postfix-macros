# postfix-macros

Postfix macros on stable Rust, today.

```Rust
"hello".assert_ne!("world");

val.iter()
	.map(|v| v.1)
	.find(|z| z.matches!(Custom::Enum(_) | Custom::EnumOther))
	.dbg!();
```

## Explanation

[RFC 2442] proposes to add postfix macros to the Rust language.
However, there are still very basic concerns from lang team members, and it seems they won't get resolved quickly, so it's unlikely to be merged any time soon.

Until then, this crate will help you to call postfix macros in [UFCS] like manner:
every macro can be called in postfix form, there is no need for a special `$self` parameter or anything like it.

[RFC 2442]: https://github.com/rust-lang/rfcs/pull/2442
[UFCS]: https://en.wikipedia.org/wiki/Uniform_Function_Call_Syntax

## TODO

* Allow chaining
* Add builtin macros: `unwrap_or!`, `do! { return }`, ...
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
