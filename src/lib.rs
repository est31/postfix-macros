/*!
Postfix macros on stable Rust, today.

```
# use postfix_macros::{postfix_macros, unwrap_or};
# #[derive(Debug, Clone, Copy)] enum Custom { Enum(()), EnumOther}
# let val = [((),Custom::EnumOther,)];
# postfix_macros! {
"hello".assert_ne!("world");

val.iter()
	.map(|v| v.1)
	.find(|z| z.matches!(Custom::Enum(_) | Custom::EnumOther))
	.dbg!()
	.unwrap_or!{ return };
# }
```

*/
pub use postfix_macros_impl::postfix_macros;

/**
Either unwraps the content passed to the macro,
or executes the code block passed as second argument.

The macro is very similar to functions like [Option::unwrap_or](std::Option::unwrap_or),
Unlike functions like [Option::unwrap_or], he code block is lazily
The macro is built for evaluation in postfix contexts,
to facilitate quick

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
