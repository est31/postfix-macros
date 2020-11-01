//! Tests for when the expression ends
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

// Test a group without punctuation terminating the
// expression.
postfix_macros! {
	#[test]
	fn nested_fn_1() {
		fn foo() {}
		42.stringify_eq!(42);
	}
	#[test]
	fn nested_fn_2() {
		let hello = 42;
		fn foo() {}
		hello.stringify_eq!(hello);
	}
	#[test]
	fn nested_fn_3() {
		fn foo() {}
		(20..42)
			.find(|v| v % 13 == 7)
			.stringify_eq!((20..42).find(|v| v % 13 == 7));
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
}
