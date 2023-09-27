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
const END_STATEMENT: char = ';';

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
        } else if ch == END_STATEMENT {
            Some(Operand::Operator(END_STATEMENT))
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

pub struct State {
    pub result: Number,
    pub digits_buf: Vec<u32>,
    pub last_operator: char,
}

fn process_char<I: Iterator<Item = char>>(
    expression_chars: &mut I,
    input_char: char,
    state: &mut State,
) {
    let operand =
        Operand::parse(input_char).expect(&format!("Unexpected character: {input_char:?}",));

    match operand {
        // Do nothing on whitespace
        Operand::Whitespace => return,

        // it's a digit.
        // Just push it into digits buffer
        Operand::Digit(digit) => {
            state.digits_buf.push(digit);
        }

        Operand::Operator(OPEN_PAREN) => {
            assert!(
                state.digits_buf.is_empty(),
                "Open parenthesis without operator"
            );

            let mut parenthesis_depth = 1;
            let mut parenthesis_expr = String::new();
            while parenthesis_depth != 0 {
                let next_char = expression_chars.next().expect("Unlosed parenthesis");
                parenthesis_expr.push(next_char);
                if next_char == OPEN_PAREN {
                    parenthesis_depth += 1;
                } else if next_char == CLOSE_PAREN {
                    parenthesis_depth -= 1;
                }
            }

            let paren_res = compute(parenthesis_expr);
            state.digits_buf = paren_res
                .to_string()
                .chars()
                .map(|c| c.to_digit(RADIX).unwrap())
                .collect();
        }

        Operand::Operator(CLOSE_PAREN) => {}

        // It's a operator
        // make a number from digits_buffer and apply last_operator
        // to result
        Operand::Operator(operator) => {
            let last_digit = combine_digit(&state.digits_buf);
            let last_operator = state.last_operator;

            state.digits_buf.clear();
            state.last_operator = operator;

            match last_operator {
                ADDITION => state.result += last_digit,
                SUBTRACTION => state.result -= last_digit,
                MULTIPLICATION => state.result *= last_digit,
                DIVISION => state.result /= last_digit,
                OPEN_PAREN | CLOSE_PAREN => unreachable!(),
                unknown_operator => panic!("Unknown operator: {unknown_operator:?}"),
            }
        }
    }
}

fn compute(raw_expression: String) -> Number {
    let mut state = State {
        result: 0,
        digits_buf: vec![],
        last_operator: ADDITION,
    };

    let mut expression_chars = raw_expression.chars();
    while let Some(input_char) = expression_chars.next() {
        process_char(&mut expression_chars, input_char, &mut state);
    }
    process_char(&mut expression_chars, END_STATEMENT, &mut state);

    state.result
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

    // NOTE:
    // this is the test given in assigment file
    #[test]
    fn assignment_test() {
        // 3 + 2 * 4 = 5*4 = 20
        assert_eq!(compute("3a2c4".to_string()), 20);
        // 32 + 2 / 2 = 34/2 = 17
        assert_eq!(compute("32a2d2".to_string()), 17);
        // 500 + 10 - 66 * 32 = 510-66*32 = 444*32 = 14208
        assert_eq!(compute("500a10b66c32".to_string()), 14208);
        // 3 + (4 * 66) - 32 = 3+264-32 = 267-32 = 235
        assert_eq!(compute("3ae4c66fb32".to_string()), 235);

        // 3 * 4 / 2 + ((2 + 4 * 41) * 4)
        // = 12/2+((2+4*41)*4) = 6+(246*4)
        // = 6+984 = 990
        assert_eq!(compute("3c4d2aee2a4c41fc4f".to_string()), 990);
    }

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
    fn parenthesis_emphasize() {
        // 10 + 5 * 3 - 1 = 15*3-1 = 45-1 = 44
        assert_eq!(compute("10 a 5 c 3 b 1".to_string()), 44);
        // 10 + (5*3) - 1 = 10+15-1 = 25-1 = 24
        assert_eq!(compute("10 a e 5 c 3 b 1 f".to_string()), 24);
        // 10 + ( 5 * (3 - 1) ) - (10 - 5) + 5 = 10+(5*2)-(10-5)+5 = 10+10-(10-5)+5
        // = 10+10-5+5 = 20-5+5 = 15+5 = 20
        assert_eq!(
            compute("10 a e5 c e3 b 1 ff b e 10 b 5f a 5".to_string()),
            20
        );
    }

    #[test]
    fn test_combine_digit() {
        assert_eq!(combine_digit(&[]), 0);
        assert_eq!(combine_digit(&[9]), 9);
        assert_eq!(combine_digit(&[1, 2]), 12);
        assert_eq!(combine_digit(&[9, 8, 6, 6]), 9866);
    }
}
