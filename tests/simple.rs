use postfix_macros::postfix_macros;

postfix_macros! {
	#[test]
	fn hi_equals_hi() {
		"hi".assert_eq!("hi".to_string());
	}
}

postfix_macros! {
	#[test]
	fn conditional() {
		let v = 40i32.checked_add(2);
		// Test that postfix macros work
		// in conditional contexts
		if v.matches!(Some(42)) {
			return;
		}
		panic!();
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
		assert!(bb);
	}
}

postfix_macros! {
	#[test]
	fn chaining() {
		let v = Some(40);
		let b = v.map(|v| v + 2).matches!(Some(42));
		assert!(b);
		(None::<()>).matches!(None).assert!();
	}
	#[test]
	#[should_panic]
	fn chaining_panic() {
		(None::<()>).matches!(Some(_)).assert!();
	}
}

postfix_macros! {
	macro_rules! macro_declaration {
		($v:tt) => {
			$v
		};
	}
	#[test]
	fn macro_declaration() {
		fn foo(_v :&mut ()) {}
		// Test that &mut doesn't terminate the expression
		let v = &mut ().macro_declaration!();
		foo(v);
	}
}

// If there is no args in the macro, don't require commas
postfix_macros! {
	macro_rules! no_comma_pattern_macro {
		($v:expr,) => {
			compile_error!("comma in pattern!");
		};
		($v:expr) => {
			$v
		};
	}
	#[test]
	fn no_comma_pattern_macro() {
		"hello".no_comma_pattern_macro!();
	}
}
