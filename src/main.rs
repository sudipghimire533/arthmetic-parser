//! # Parser-rs
//! build up a numeric result from given expression
//!
//! Allowed expression are numeric values, operators and parenthesis.
//! Unless in parenthesis, computation occuer from left to right
//!
//! Allowed operators are: +, -, *, /
//! represented by a,b,c,d respectively
//! Likewise, Open and close parenthesis are represented by e, f respectively
//!

const ADDITION: char = 'a';
const SUBTRACTION: char = 'b';
const MULTIPLICATION: char = 'c';
const DIVISION: char = 'd';
const OPEN_PAREN: char = 'e';
const CLOSE_PAREN: char = 'f';

const RADIX: u32 = 10;

type Number = i128;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Operand {
    Operator(char),
    Digit(u32),
    Whitespace,
}

impl Operand {
    fn parse(ch: char) -> Option<Self> {
        if let Some(digit) = ch.to_digit(RADIX) {
            Some(Operand::Digit(digit))
        } else if ch == ADDITION {
            Some(Operand::Operator(ADDITION))
        } else if ch == SUBTRACTION {
            Some(Operand::Operator(SUBTRACTION))
        } else if ch == MULTIPLICATION {
            Some(Operand::Operator(MULTIPLICATION))
        } else if ch == DIVISION {
            Some(Operand::Operator(DIVISION))
        } else if ch == OPEN_PAREN {
            Some(Operand::Operator(OPEN_PAREN))
        } else if ch == CLOSE_PAREN {
            Some(Operand::Operator(CLOSE_PAREN))
        } else if ch.is_whitespace() {
            Some(Operand::Whitespace)
        } else {
            None
        }
    }
}

/// Convert array of digits to number
/// Example:
/// input: &Vec::new([9, 8, 6, 6])
/// output: Number::from(9866)
fn combine_digit(digits: &[u32]) -> Number {
    let mut res: Number = 0;
    for (digit_index, digit) in digits.iter().rev().enumerate() {
        let digit_value = digit * (10_u32.pow(digit_index as u32));
        res += digit_value as i128;
    }
    res
}

fn compute(raw_expression: String) -> Number {
    let mut result = 0;
    let mut digits_buf = vec![];
    let mut last_operator = ADDITION;

    for (char_index, input_char) in raw_expression.chars().enumerate() {
        let operand = Operand::parse(input_char).expect(&format!(
            "Unexpected character: {input_char:?} at index: {char_index}",
        ));

        match operand {
            // Do nothing on whitespace
            Operand::Whitespace => continue,

            // it's a digit.
            // Just push it into digits buffer
            Operand::Digit(digit) => {
                digits_buf.push(digit);
            }

            // It's a operator
            // make a number from digits_buffer and apply last_operator
            // to result
            Operand::Operator(operator) => {
                let last_digit = combine_digit(&digits_buf);
                match last_operator {
                    ADDITION => result += last_digit,
                    SUBTRACTION => result -= last_digit,
                    MULTIPLICATION => result *= last_digit,
                    DIVISION => result /= last_digit,
                    OPEN_PAREN => todo!(),
                    CLOSE_PAREN => todo!(),
                    unknown_operator => panic!("Unknown operator: {unknown_operator:?}"),
                }

                digits_buf.clear();
                last_operator = operator;
            }
        }
    }

    let last_digit = combine_digit(&digits_buf);
    match last_operator {
        ADDITION => result += last_digit,
        SUBTRACTION => result -= last_digit,
        MULTIPLICATION => result *= last_digit,
        DIVISION => result /= last_digit,
        OPEN_PAREN => todo!(),
        CLOSE_PAREN => todo!(),
        unknown_operator => panic!("Unknown operator: {unknown_operator:?}"),
    }

    result
}

pub fn main() {
    // read the cli argument passed into this binary
    let maybe_equation = std::env::args()
        .collect::<Vec<_>>()
        .get(1)
        .iter()
        .filter_map(|s| {
            let s = s.trim().to_string();
            if s.is_empty() {
                None
            } else {
                Some(s)
            }
        })
        .collect::<Vec<_>>();

    if let Some(equation) = maybe_equation.first() {
        println!("Your equation: {equation:?}");
        println!("=== Computing... ====");
        let result = compute(equation.to_string());
        println!("Result came out to be: {result}");

        return;
    }

    println!("Write your equation:");
    let input = std::io::stdin().lines().next().unwrap().unwrap();
    println!("=== Computing... ====");

    let result = compute(input);
    println!("Result came out to be: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        assert_eq!(compute("".to_string()), 0);
    }

    #[test]
    fn single_expression() {
        assert_eq!(compute("9".to_string()), 9);
        assert_eq!(compute(" 0 ".to_string()), 0);
    }

    #[test]
    fn two_expression() {
        // 9 - 9 = 0
        assert_eq!(compute("9 b 9".to_string()), 0);
        // 9 + 9 = 18
        assert_eq!(compute("9 a 9".to_string()), 18);
        // 5 * 4 = 20
        assert_eq!(compute("5 c 4".to_string()), 20);
        // 100 / 10 10
        assert_eq!(compute("100 d 10".to_string()), 10);
    }

    #[test]
    fn multi_expression() {
        // 9 - 9 * 10 = 0 * 10 = 0
        assert_eq!(compute("9 b 9 c 10".to_string()), 0);
        // 10 + 10 - 10 * 10 / 10 = 20-10*10/10 = 10*10/10 = 100/10 = 10
        assert_eq!(compute("10 a 10 b 10 c 10 d 10".to_string()), 10);
    }

    #[test]
    fn can_start_with_operator() {
        // - 10 + 50 = 0 - 10 + 50 = -10 + 50 = 40
        assert_eq!(compute("b 10 a 50".to_string()), 40);
    }

    #[test]
    fn test_combine_digit() {
        assert_eq!(combine_digit(&[]), 0);
        assert_eq!(combine_digit(&[9]), 9);
        assert_eq!(combine_digit(&[1, 2]), 12);
        assert_eq!(combine_digit(&[9, 8, 6, 6]), 9866);
    }
}
