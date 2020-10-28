//! Tests for when the expression ends
use postfix_macros::postfix_macros;

macro_rules! stringify_eq {
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
