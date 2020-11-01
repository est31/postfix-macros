/*!
Postfix macros on stable Rust, today.

```
# use postfix_macros::{postfix_macros, unwrap_or};
# #[derive(Debug, Clone, Copy)] enum Custom { Enum(()), EnumOther}
# let val = [((),Custom::EnumOther,)];
postfix_macros! {
	"hello world".println!();
	42.assert_ne!(4 + 2);

	val.iter()
		.map(|v| v.1)
		.find(|z| z.matches!(Custom::Enum(_) | Custom::EnumOther))
		.dbg!()
		.unwrap_or!{ return };
}
```

The crate provides the [`postfix_macros!`] macro,
as well as some helpful macros for use in a postfix context,
for your greatest convenience.

| Rust construct | postfix replacement macro |
| - | - |
| `unwrap_or`, `unwrap_or_else` | [`unwrap_or!`] |
| **`if let`** with else clause | [`match_or!`] |
| **`match`** with default case | [`match_or!`] |
| **`if`** `<bool>`, `bool::then` | [`then!`] |
| **`else`** | [`then_else!`] |
*/

/**
Proc macro to parse code containing postfix macros,
to rewrite it to use traditional macro invocations.

The main macro of this crate.

The macro scans for `expr.macro_invoc!(params)` patterns
and changes them to `macro_invoc!(expr, params)` patterns.

If no parameters are passed to the postfix macro,
then no trailing `,` is emitted.
*/
pub use postfix_macros_impl::postfix_macros;

/**
Either unwraps the content passed to the macro,
or executes the passed code block.

The macro is very similar to functions like
[`Option::unwrap_or`](std::option::Option::unwrap_or),
in that it tries to attain the content contained inside,
and if that's not possible, evaluates to the alternative
provided by the user.

Unlike the function though, the body of the macro is lazily
evaluated, so only if there is actually the need to return
the alternative, similar to the `unwrap_or_else` function.

A code block is way more powerful as `unwrap_or_else`,
though, as it allows issuing commands like `continue`,
`return`, or `break` in the body that can bring control
flow outside of the body.

As such, the `unwrap_or` macro combines the benefits of
both the `unwrap_or` and `unwrap_or_else` functions.

The macro requires the presence of two functions on the
underlying type: `map` and `unwrap_or`. Maybe in the future
when the `Try` trait is stable, it will be used instead.

If you want to do more powerful matching, you can
use the [`match_or!`] macro instead.

# Examples

```
# use postfix_macros::{postfix_macros, unwrap_or};
# postfix_macros! {
let v = Err(());
let mut w = 0;
for i in 0..3 {
	w += i;
	v.unwrap_or!{ continue };
	break
}
assert_eq!(w, 3);
# }
```
*/
#[macro_export]
macro_rules! unwrap_or {
	($v:expr, $($w:tt)*) => {
		if let Some(inner) = $v.map(|v| Some(v)).unwrap_or(None) {
			inner
		} else {
			$($w)*
		}
	};
}

/**
**`match`** macro with a default case shorthand

Meant to be used in a postfix context, as
the postfix analog of **`match`** and **`if let`**
Rust constructs.

```
# use postfix_macros::{postfix_macros, match_or};
# postfix_macros! {
#[derive(Copy, Clone)]
enum Foo {
	Bar(u8),
	Baz,
}
let v = Foo::Bar(42);
let mut w = 0;
for i in 0..3 {
	w += i;
	v.match_or!{ Foo::Bar(x) => x; break };
}
assert_eq!(w, 3);
# }
```
*/
#[macro_export]
macro_rules! match_or {
	($v:expr, $($pat:pat => $e:expr)+ ; $($else:tt)*) => {
		match $v {
			$($pat => $e)*,
			_ => {
				$($else)*
			},
		}
	};
}


/**
Executes the body if the argument is `true`

Meant to be used in a postfix context, as
the postfix analog of **`if`**.

With the `bool::then` function, there is a
[currently unstable](https://github.com/rust-lang/rust/issues/64260)
equivalent in the standard library, but
the macro doesn't put the body into a
closure, and is thus more powerful.

Evaluates the first argument as a boolean,
and if it's `true`, executes the body.

```
# use postfix_macros::{postfix_macros, then};
# postfix_macros! {
let mut w = 0;
for i in 1..10 {
	w += i;
	(w % i == 0).then!{ w += i * i };
}
assert_eq!(w, 75);
# }
```
*/
#[macro_export]
macro_rules! then {
	($v:expr, $($body:tt)*) => {
		if $v {
			$($body)*
		}
	};
}

/**
**`else`** clauses for the [`then!`] macro

Meant to be used in a postfix context.
The [`then!`] macro would serve as
the postfix analog of **`if`**, while
this macro would be the postfix analog
of **`else`**.

```
# use postfix_macros::{postfix_macros, then, then_else};
# postfix_macros! {
let mut w = 0;
for i in 1..10 {
	w += i;
	(w % i == 0)
		.then!{ w += i * i }
		.then_else!{ w += 1 };
}
assert_eq!(w, 181);
# }
```
*/
#[macro_export]
macro_rules! then_else {
	($v:tt, $($body:tt)*) => {
		$v {
			$($body)*
		}
	};
}
