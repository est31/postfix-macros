/*!
Postfix macros on stable Rust, today.

```Rust
"hello".assert_ne!("world");

val.iter()
	.map(|v| v.1)
	.find(|z| z.matches!(Custom::Enum(_) | Custom::EnumOther))
	.dbg!();
```

*/
pub use postfix_macros_impl::postfix_macros;
