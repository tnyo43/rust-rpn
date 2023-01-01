
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
mod tests {
    use super::*;

    #[test]
    fn parse_just_number() {
        let calc = RpnCalculator::new(false);

        assert_eq!(calc.eval("5"), 5);
        assert_eq!(calc.eval("10"), 10);
    }

    #[test]
    fn calculate() {
        let calc = RpnCalculator::new(false);

        assert_eq!(calc.eval("5 3 +"), 8);
        assert_eq!(calc.eval("10 14 -"), -4);
        assert_eq!(calc.eval("9 5 *"), 45);
        assert_eq!(calc.eval("40 3 /"), 13);
        assert_eq!(calc.eval("40 3 %"), 1);
    }

    #[test]
    #[should_panic]
    fn invalid_syntax_make_panic() {
        let calc = RpnCalculator::new(false);

        calc.eval("1 1 &");
    }
}
