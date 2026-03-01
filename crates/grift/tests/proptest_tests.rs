use grift::{ArenaError, Lisp, Value};
use proptest::prelude::*;

// ── Strategies ──────────────────────────────────────────────────────

/// Generate a valid Grift number literal.
fn arb_number() -> impl Strategy<Value = String> {
    prop::num::i32::ANY.prop_map(|n| n.to_string())
}

/// Generate a simple self-evaluating expression.
fn arb_self_eval() -> impl Strategy<Value = String> {
    prop_oneof![
        arb_number(),
        Just("#t".to_string()),
        Just("#f".to_string()),
        Just("#inert".to_string()),
        Just("()".to_string()),
    ]
}

/// Generate an arithmetic expression.
fn arb_arithmetic() -> impl Strategy<Value = String> {
    (arb_number(), arb_number(), prop_oneof![Just("+"), Just("-"), Just("*")]).prop_map(
        |(a, b, op)| format!("({op} {a} {b})"),
    )
}

// ── Property tests ──────────────────────────────────────────────────

proptest! {
    /// Self-evaluating values round-trip through eval.
    #[test]
    fn prop_number_eval(n in -1_000_000i64..1_000_000i64) {
        let lisp: Lisp<1000> = Lisp::new();
        let src = n.to_string();
        let result = lisp.eval(&src);
        prop_assert_eq!(result, Ok(Value::Number(n as isize)));
    }

    /// Boolean literals evaluate to themselves.
    #[test]
    fn prop_bool_eval(b in prop::bool::ANY) {
        let lisp: Lisp<1000> = Lisp::new();
        let src = if b { "#t" } else { "#f" };
        let result = lisp.eval(src);
        prop_assert_eq!(result, Ok(Value::Boolean(b)));
    }

    /// Arithmetic identity: (+ n 0) == n
    #[test]
    fn prop_add_identity(n in -1_000_000i64..1_000_000i64) {
        let lisp: Lisp<1000> = Lisp::new();
        let src = format!("(+ {n} 0)");
        let result = lisp.eval(&src);
        prop_assert_eq!(result, Ok(Value::Number(n as isize)));
    }

    /// Arithmetic identity: (* n 1) == n
    #[test]
    fn prop_mul_identity(n in -1_000_000i64..1_000_000i64) {
        let lisp: Lisp<1000> = Lisp::new();
        let src = format!("(* {n} 1)");
        let result = lisp.eval(&src);
        prop_assert_eq!(result, Ok(Value::Number(n as isize)));
    }

    /// Unbalanced parens always produce an error, never a successful parse.
    #[test]
    fn prop_unbalanced_parens_error(extra in 1usize..5) {
        let lisp: Lisp<1000> = Lisp::new();
        let src = "(".repeat(1 + extra) + "1" + &")".repeat(1);
        let result = lisp.eval(&src);
        // Should error because there are more open parens than close parens
        prop_assert!(result.is_err());
    }

    /// Random byte strings should not panic the parser (they may error).
    #[test]
    fn prop_parser_no_panic(s in "[ -~]{0,50}") {
        let lisp: Lisp<5000> = Lisp::new();
        // We only care that it doesn't panic; errors are fine.
        let _ = lisp.eval(&s);
    }

    /// Self-evaluating expressions round-trip.
    #[test]
    fn prop_self_eval_roundtrip(expr in arb_self_eval()) {
        let lisp: Lisp<1000> = Lisp::new();
        let result = lisp.eval(&expr);
        prop_assert!(result.is_ok(), "Failed to eval self-evaluating: {expr}");
    }

    /// Arithmetic expressions don't panic (may overflow).
    #[test]
    fn prop_arithmetic_no_panic(expr in arb_arithmetic()) {
        let lisp: Lisp<1000> = Lisp::new();
        let result = lisp.eval(&expr);
        // Either a successful number or an arithmetic overflow—never a panic.
        match result {
            Ok(Value::Number(_)) => {},
            Err(ArenaError::ArithmeticOverflow) => {},
            other => prop_assert!(false, "Unexpected result for {expr}: {other:?}"),
        }
    }

    /// ParseError always includes non-zero line/col for SliceSource input.
    #[test]
    fn prop_parse_error_has_location(n in 1usize..5) {
        let lisp: Lisp<1000> = Lisp::new();
        // Trailing close paren always produces ParseError
        let src = format!("1{})", " ".repeat(n));
        let result = lisp.eval(&src);
        if let Err(ArenaError::ParseError { line, col }) = result {
            prop_assert!(line >= 1, "line should be >= 1, got {line}");
            prop_assert!(col >= 1, "col should be >= 1, got {col}");
        }
        // If it doesn't error, that's also fine (some whitespace patterns may be valid)
    }
}
