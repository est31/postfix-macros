/*!
I wondered about whether to add a `pass_to` macro or not,
but ultimately I decided not to as people usually don't
want to chain beyond something that calls
return/break/continue, which would be the main benefit
of doing somehting in postfix manner.
Yes, there is also the benefit that it allows easier
editing, but I didn't feel it was worth it. I'm adding
these tests to keep the code+idea around should
I reconsider.

The macro is a good test for postfix_macros
anyways.
*/

#![allow(unused_parens)]

use postfix_macros::postfix_macros;

postfix_macros! {
	fn is_prime(v: &u32) -> bool {
		(!(2..*v).any(|w| *v % w == 0))
			.pass_to! { return };
	}
	fn find_largest_prime(v: u32) -> Option<u32> {
		(2..=v).rev()
			.find(is_prime)
			.pass_to! { return };
	}
	#[test]
	fn primes_test() {
		find_largest_prime(10).assert_eq!(Some(7));
		find_largest_prime(20).assert_eq!(Some(19));
		find_largest_prime(100).assert_eq!(Some(97));
	}
}

/**
Pass a value to `return`, `break`, or `continue`
*/
#[macro_export]
macro_rules! pass_to {
	// We can't use the ? repetition indicator here because
	// of https://github.com/rust-lang/rust/issues/35853
	($v:expr, return $l:lifetime) => {
		return $l $v;
	};
	($v:expr, return) => {
		return $v;
	};
	($v:expr, break $l:lifetime) => {
		break $l? $v;
	};
	($v:expr, break) => {
		break $v;
	};
	($v:expr, continue $(l:lifetime)?) => {
		continue $l $v;
	};
	($v:expr, continue) => {
		continue $v;
	};
}
