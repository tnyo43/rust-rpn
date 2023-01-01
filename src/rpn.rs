use anyhow::{bail, ensure, Context, Result};

pub struct RpnCalculator(bool);

impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    pub fn eval(&self, formula: &str) -> Result<i32> {
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }

    fn eval_inner(&self, tokens: &mut Vec<&str>) -> Result<i32> {
        let mut stack = Vec::new();
        let mut pos = 0;

        while let Some(token) = tokens.pop() {
            pos += 1;

            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            } else {
                let y = stack.pop().context(format!("Invalid syntax at {}", pos))?;
                let x = stack.pop().context(format!("Invalid syntax at {}", pos))?;

                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => bail!("Invalid token at {}", pos),
                };

                stack.push(res);
            }

            if self.0 {
                println!("{:?} {:?}", tokens, stack);
            }
        }

        ensure!(stack.len() == 1, "Invalid syntax");

        Ok(stack[0])
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

            assert_eq!(calc.eval(raw).unwrap(), expected);
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

            assert_eq!(calc.eval(raw).unwrap(), expected);
        }
    }

    describe "シンタックスエラーがあるときパニックになる" {
        describe "空文字列が入力されたとき" {
            #[rstest]
            fn input_is_empty() {
                let calc = RpnCalculator::new(false);

                assert!(calc.eval("").is_err())
            }
        }

        describe "不適切な記号が紛れ込んでいるとき" {
            #[rstest]
            fn invalid_unknown_symbol() {
                let calc = RpnCalculator::new(false);

                assert!(calc.eval("1 1 &").is_err())
            }
        }

        describe "順番が不適切なとき" {
            #[rstest]
            fn invalid_unknown_symbol() {
                let calc = RpnCalculator::new(false);

                assert!(calc.eval("1 + 1").is_err())
            }
        }
    }
}
