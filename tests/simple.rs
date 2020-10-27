use postfix_macros::postfix_macros;

postfix_macros! {
	#[test]
	fn hi_equals_hi() {
		"hi".assert_eq!("hi".to_string());
	}
}

postfix_macros! {
	#[test]
	fn some_matches() {
		let v = Some(42);
		// Test that idents work
		let b = v.matches!(Some(42));
		// Test that Groups work
		let bb = b && (None::<()>).matches!(None);
	}
}
