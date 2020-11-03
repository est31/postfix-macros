//! Tests for when the expression ends
#![allow(unused_parens, unused_braces)]

use postfix_macros::postfix_macros;

macro_rules! stringify_eq {
	($v:tt, $w:ident) => {{
		assert_eq!(stringify!($v), stringify!($w));
		$v
	}};
	($v:tt, $w:literal) => {{
		assert_eq!(stringify!($v), stringify!($w));
		$v
	}};
	($v:tt, $($w:tt)*) => {{
		assert_eq!(stringify!($v), stringify!({$($w)*}));
		$v
	}};
}

postfix_macros! {
	#[test]
	fn mut_doesnt_end_expr() {
		let _ = &mut ().stringify_eq!(&mut ());
		let _ = 0 -(0).stringify_eq!((0));
		// && and & & are actually two different things
		let _ = &&(0).stringify_eq!(&& (0));
		let _ = & &(0).stringify_eq!(& & (0));
		let _ = (0, &().stringify_eq!(&()));
		// TODO add more weird details of the expression parsing code
	}
}

// Test that array lookups work
postfix_macros! {
	#[test]
	fn arrays() {
		let arr = ["hello", "world"];
		arr[0].stringify_eq!(arr[0]);
		[0, 1, 2, 3, 4,].stringify_eq!([0, 1, 2, 3, 4,]);
	}
}

// Tests for colons
postfix_macros! {
	#[test]
	fn colon() {
		let _ :String = Default::default().stringify_eq!(Default::default());
		// TODO this doesn't work
		//None::<()>.stringify_eq!(None::<()>);
	}
}

// Test a group without punctuation terminating the
// expression.
postfix_macros! {
	#[test]
	fn nested_fn_1() {
		fn _foo() {}
		42.stringify_eq!(42);
	}
	#[test]
	fn nested_fn_2() {
		let hello = 42;
		fn _foo() {}
		hello.stringify_eq!(hello);
	}
	#[test]
	fn nested_fn_3() {
		fn _foo() {}
		(20..42)
			.find(|v| v % 13 == 7)
			.stringify_eq!((20..42).find(|v| v % 13 == 7));
	}
	#[test]
	fn nested_fn_4() {
		fn _foo() {}
		&-0.stringify_eq!(&-0);
	}

	#[test]
	fn if_clause_1() {
		if false {}
		42.stringify_eq!(42);
	}
	#[test]
	fn if_clause_2() {
		let hello = 42;
		if false {}
		hello.stringify_eq!(hello);
	}
	#[test]
	fn if_clause_3() {
		if false {}
		(20..42)
			.find(|v| v % 13 == 7)
			.stringify_eq!((20..42).find(|v| v % 13 == 7));
	}
	#[test]
	fn if_clause_4() {
		if false {}
		&-0.stringify_eq!(&-0);
	}
}

// Test that braced expressions work
postfix_macros! {
	#[test]
	fn braced() {
		dbg!{ 42 }.stringify_eq!(dbg!{42});
		{ 42 }.stringify_eq!({42});
		(0, { "hello" }.stringify_eq!({ "hello" }));
	}
}

// Test that if, else, or match don't terminate the expression
postfix_macros! {
	#[test]
	fn if_else_belongs() {
		if false { "hello" } else { "hi" }
			.to_string()
			.stringify_eq!(if false { "hello" } else { "hi" }.to_string());
	}
	#[test]
	fn if_if_belongs() {
		if if false { true } else { true } { "hello" } else { "hi" }
			.to_string()
			.stringify_eq!(if if false { true } else { true } {
				"hello"
			} else {
				"hi"
			}.to_string());
	}
	#[test]
	fn match_belongs() {
		match false { _ => "hi" }
			.to_string()
			.stringify_eq!(match false { _ => "hi" }.to_string());
	}
}

// Postfix macros inside if clauses
postfix_macros! {
	#[test]
	fn inside_if_clause() {
		let hello = true;
		// TODO this doesn't work
		//if &false.stringify_eq!(&false) == &false {}
		if hello.stringify_eq!(hello) {}
		if true.stringify_eq!(true) {}
	}
}
