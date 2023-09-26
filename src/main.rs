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
//! ## Step by step on how this works
//! 1) *Take input string from stdin*
//!     Spaces in input are ignored and can be used for formatting purpose.
//!     For example, following two statement are same:
//!     - 100 a 200 a 50
//!     - 100a200a50
//! 2) *Iterate through the input string*
//!     - If it is a number, then it is a value

use std::borrow::Cow;

const BASE: u32 = 10;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum Operator {
    Addition = b"a"[0] as u8,
    Subtraction = b"b"[0] as u8,
    Multiplication = b"c"[0] as u8,
    Division = b"d"[0] as u8,
    OpenParen = b"e"[0] as u8,
    CloseParen = b"f"[0] as u8,
}

impl Operator {
    fn as_u8(self) -> u8 {
        self as u8
    }
}

macro_rules! impl_numeric {
    ( $type_name: ty ,? ) => {
        impl crate::Numeric for $type_name {}
    };
}
trait Numeric:
    std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
    + Copy
    + std::fmt::Debug
    + std::str::FromStr
    + Eq
{
}

/// Represent a single expression to be performed
#[derive(Debug, Clone)]
enum Expression<Number: Numeric> {
    Addition(Box<Self>, Box<Self>),
    Subtraction(Box<Self>, Box<Self>),
    Multiplication(Box<Self>, Box<Self>),
    Division(Box<Self>, Box<Self>),
    Value(Number),
}

/// Is thsi value or operator
pub enum Operand<Number> {
    Value(Number),
    Oper(Operator),
}

impl<Number: Numeric> Expression<Number> {
    fn get_numeric(self) -> Number {
        match self {
            Self::Addition(left, right) => (*left).get_numeric() + (*right).get_numeric(),
            Self::Subtraction(left, right) => (*left).get_numeric() - (*right).get_numeric(),
            Self::Multiplication(left, right) => (*left).get_numeric() * (*right).get_numeric(),
            Self::Division(left, right) => (*left).get_numeric() / (*right).get_numeric(),
            Self::Value(val) => val,
        }
    }

    fn split_once(expression: &str) -> (Option<Operand<Number>>, Cow<'_, str>) {
        let first_byte = match expression.as_bytes().first() {
            Some(byte) => *byte,
            None => return (None, "".into()),
        };

        if first_byte == Operator::Addition.as_u8() {
            (
                Some(Operand::Oper(Operator::Addition)),
                Cow::Borrowed(&expression[1..]),
            )
        } else if first_byte == Operator::Subtraction.as_u8() {
            (
                Some(Operand::Oper(Operator::Subtraction)),
                Cow::Borrowed(&expression[1..]),
            )
        } else if first_byte == Operator::Multiplication.as_u8() {
            (
                Some(Operand::Oper(Operator::Multiplication)),
                Cow::Borrowed(&expression[1..]),
            )
        } else if first_byte == Operator::Division.as_u8() {
            (
                Some(Operand::Oper(Operator::Division)),
                Cow::Borrowed(&expression[1..]),
            )
        } else if first_byte == Operator::OpenParen.as_u8() {
            (
                Some(Operand::Oper(Operator::OpenParen)),
                Cow::Borrowed(&expression[1..]),
            )
        } else if first_byte == Operator::CloseParen.as_u8() {
            (
                Some(Operand::Oper(Operator::CloseParen)),
                Cow::Borrowed(&expression[1..]),
            )
        } else {
            let (digit_str, remaining_str): (String, String) =
                expression.chars().partition(|c| c.is_digit(BASE));
            (
                digit_str.parse::<Number>().map(|a| Operand::Value(a)).ok(),
                remaining_str.into(),
            )
        }
    }

    fn make_one_expression(lhs: Box<Self>, expression: &str) -> (Option<Self>, Cow<'_, str>) {
        match Self::split_once(expression) {
            (None, rest) => (None, rest),

            (Some(Operand::Value(val)), rest) => (Some(Self::Value(val)), rest),

            (Some(Operand::Oper(operator)), rest) => match Self::split_once(&rest) {
                (None, rest) => (None, rest.to_owned()),

                (Some(Operand::Oper(_)), rest) => (None, rest.to_owned()),

                (Some(Operand::Value(val)), rest) => {
                    todo!()
                }
            },
        }
    }

    fn parse(mut expression: String) -> Option<Self> {
        expression = expression.replace(" ", "");
        let expression = expression.trim();

        match Self::split_once(expression) {
            // very first term is an expression
            (Some(Operand::Oper(_)), _rest) => None,
            // cannot parse
            (None, _rest) => None,

            // first term is a numeric value, we can continue
            (Some(Operand::Value(first_val)), rest) => {
                None
            }
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_non_paren() {
        // 100 + 300 * 2 - 50 / 2
        let statement = "100 a 300 c 2 b 50 d 2";
    }
}
