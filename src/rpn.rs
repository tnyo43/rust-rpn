
pub struct RpnCalculator(bool);

impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    pub fn eval(&self, formula: &str) -> i32 {
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }

    fn eval_inner(&self, tokens: &mut Vec<&str>) -> i32 {
        let mut stack = Vec::new();

        while let Some(token) = tokens.pop() {
            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            } else {
                let y = stack.pop().expect("Invalid syntax");
                let x = stack.pop().expect("Invalid syntax");

                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => panic!("Invalid syntax"),
                };

                stack.push(res);
            }

            if self.0 {
                println!("{:?} {:?}", tokens, stack);
            }
        }

        if stack.len() == 1 {
            stack[0]
        } else {
            panic!("invalid syntax")
        }
    }
}

#[cfg(test)]
extern crate speculate;
extern crate rstest;

use speculate::speculate;
use rstest::*;

speculate! {
    describe "数字はそのまま parse される" {
        #[rstest(raw, expected,
            case("5", 5),
            case("10", 10),
        )]
        fn parse_just_number(raw: &str, expected: i32) {
            let calc = RpnCalculator::new(false);

            assert_eq!(calc.eval(raw), expected);
        }
    }

    describe "四則演算と余りの県債ができる" {
        #[rstest(raw, expected,
            case("5 3 +", 8),
            case("10 14 -", -4),
            case("9 5 *", 45),
            case("40 3 /", 13),
            case("40 3 %", 1),
        )]
        fn calculate(raw: &str, expected: i32) {
            let calc = RpnCalculator::new(false);

            assert_eq!(calc.eval(raw), expected);
        }
    }

    describe "シンタックスエラーがあるときパニックになる" {
        describe "空文字列が入力されたとき" {
            #[rstest]
            #[should_panic]
            fn input_is_empty() {
                let calc = RpnCalculator::new(false);

                calc.eval("");
            }
        }

        describe "不適切な記号が紛れ込んでいるとき" {
            #[rstest]
            #[should_panic]
            fn invalid_unknown_symbol() {
                let calc = RpnCalculator::new(false);

                calc.eval("1 1 &");
            }
        }

        describe "順番が不適切なとき" {
            #[rstest]
            #[should_panic]
            fn invalid_unknown_symbol() {
                let calc = RpnCalculator::new(false);

                calc.eval("1 + 1");
            }
        }
    }
}
