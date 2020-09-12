// https://arzg.github.io/lang/0/
// https://arzg.github.io/lang/1/

mod utils;

#[derive(Debug, PartialEq)]
pub struct Number(pub i32);

impl Number {
    pub fn new(s: &str) -> Self {
        Self(s.parse().unwrap())
    }
}
#[derive(Debug, PartialEq)]
pub struct Expr {
    pub lhs: Number,
    pub rhs: Number,
    pub op: Op,
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Expr {
    pub fn new(s: &str) -> Self {
        let (s, lhs) = utils::extract_digits(s);
        let lhs = Number::new(lhs);

        let (s, rhs) = utils::extract_digits(s);
        let rhs = Number::new(rhs);

        let op = Op::new(s);

        Self { lhs, rhs, op }
    }
}

impl Op {
    pub fn new(s: &str) -> Self {
        match s {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            _ => panic!("bad operator"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // primitives
    #[test]
    fn parse_number() {
        assert_eq!(Number::new("123"), Number(123));
    }

    // arithmetic
    #[test]
    fn parse_add_op() {
        assert_eq!(Op::new("+"), Op::Add);
    }
    #[test]
    fn parse_sub_op() {
        assert_eq!(Op::new("-"), Op::Sub);
    }
    #[test]
    fn parse_mul_op() {
        assert_eq!(Op::new("*"), Op::Mul);
    }
    #[test]
    fn parse_div_op() {
        assert_eq!(Op::new("/"), Op::Div);
    }

    // expressions
    #[test]
    fn parse_one_plus_two() {
        assert_eq!(
            Expr::new("1+2"),
            Expr {
                lhs: Number(1),
                rhs: Number(2),
                op: Op::Add,
            }
        );
    }
}
