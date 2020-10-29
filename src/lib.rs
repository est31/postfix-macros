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

*/
pub use postfix_macros_impl::postfix_macros;

/**
Either unwraps the content passed to the macro,
or executes the code block passed as second argument.

The macro is very similar to functions like
[`Option::unwrap_or`](std::option::Option::unwrap_or),
in that it tries to attain the content contained inside,
and if that's not possible, evaluates to the alternative
provided by the user.
Unlike `unwrap_or` though, the macro is lazily evaluated,
so only if there is actually the need to return the
alternative.

The `unwrap_or_else` function provides
lazy evaluation through a closure that you pass to it.

A code block is way more powerful though, as it
allows controlling the outside control flow,
like issuing `continue`, `return`, or `break`.

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
Postfix `match` macro with a default case shorthand

The macro is the postfix analog of `match` and `if let`
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

