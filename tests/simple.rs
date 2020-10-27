use postfix_macros::postfix_macros;

postfix_macros!(
#[test]
fn check_hi_equals_hi() {
	"hi".assert_eq!("hi".to_string());
}
);
