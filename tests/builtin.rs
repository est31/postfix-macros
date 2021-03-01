use postfix_macros::{match_or, postfix_macros, then, then_else, unwrap_or};

postfix_macros! {
    #[test]
    fn builtin_unwrap_or() {
        let mut check = false;

        None.unwrap_or!(check = true);
        assert!(check);

        Some(()).unwrap_or!(check = false);
        assert!(check);
    }
}

postfix_macros! {
    #[test]
    fn builtin_match_or() {
        let mut check_match = false;
        let mut check_default = false;

        true.match_or!(true => check_match = true; check_default = true);
        assert!(check_match);
        assert!(!check_default);

        check_match = false;
        check_default = false;

        false.match_or!(true => check_match = true; check_default = true);
        assert!(!check_match);
        assert!(check_default);
    }
}

postfix_macros! {
    #[test]
    fn builtin_then() {
        let mut check = false;

        true.then!(check = true);
        assert!(check);

        false.then!(check = false);
        assert!(check);
    }
}

postfix_macros! {
    #[test]
    fn builtin_then_else() {
        let mut check_then = false;
        let mut check_else = false;

        true.then!(check_then = true).then_else!(check_else = true);
        assert!(check_then);
        assert!(!check_else);

        check_then = false;
        check_else = false;

        false.then!(check_then = true).then_else!(check_else = true);
        assert!(!check_then);
        assert!(check_else);
    }
}
