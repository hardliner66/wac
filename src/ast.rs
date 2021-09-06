#![allow(dead_code)]

use crate::numbers::RandomNumbers;
use rand::rngs::ThreadRng as RNG;
use std::fmt::{Debug, Error, Formatter};

use rand::prelude::*;

pub enum Help {
    Opcode(Opcode),
    PrefixOpcode(PrefixOpcode),
    PostfixOpcode(PostfixOpcode),
    Number(u64),
    Constant(Constant),
    Show,
    Help,
}

#[derive(Copy, Clone)]
pub enum Constant {
    Min,
    Max,
    C,
}

pub enum Expr {
    Number(u64),
    Show(Box<Expr>),
    Help(Help),
    Op(Box<Expr>, Opcode, Box<Expr>),
    Prefix(PrefixOpcode, Box<Expr>),
    Postfix(Box<Expr>, PostfixOpcode),
    Error,
}

fn x<F: Fn(&mut RNG, u64) -> u64>(
    rng: &mut RNG,
    success_rate: f32,
    min: u64,
    depth: u64,
    v: u64,
    f: F,
) -> u64 {
    if depth > 0 && v > min {
        if rng.gen::<f32>() <= success_rate {
            v
        } else {
            f(rng, v)
        }
    } else {
        v
    }
}

fn inc_dec(rng: &mut RNG, success_rate: f32, min: u64, depth: u64, v: u64) -> u64 {
    x(rng, success_rate, min, depth, v, |rng, v| {
        if rng.gen() {
            v.wrapping_add(1)
        } else {
            v.wrapping_sub(1)
        }
    })
}

fn bit_flip(rng: &mut RNG, success_rate: f32, min: u64, depth: u64, v: u64) -> u64 {
    x(rng, success_rate, min, depth, v, |_rng, v| {
        let numbers = RandomNumbers::new(0, 8);
        v ^ numbers
            .map(|n| 2_u64.pow(n as u32))
            .skip_while(|n| *n > v)
            .nth(1)
            .unwrap()
    })
}

#[inline(always)]
fn factorial(rng: &mut RNG, success_rate: f32, depth: u64, n: u64) -> u64 {
    let mut acc = 1u64;
    let mut i = 1u64 + 1u64;
    while i <= n {
        if acc == 0 {
            acc = x(rng, success_rate, 0, depth, i % 9, |rng, v| {
                if rng.gen() {
                    v.wrapping_add(1)
                } else {
                    v.wrapping_sub(1)
                }
            });
        }
        let acc_i = acc.wrapping_mul(i);
        acc = acc_i;
        i = i + 1u64;
    }
    acc
}

impl Expr {
    pub fn evaluate(&self, rng: &mut RNG, success_rate: f32, depth: u64) -> Result<u64, String> {
        match self {
            Self::Number(n) => {
                let n = *n;
                let res = inc_dec(rng, success_rate, 100, depth, n);
                Ok(res)
            }
            Self::Op(e1, o, e2) => {
                let v1 = e1.evaluate(rng, success_rate, depth + 1)?;
                let v2 = e2.evaluate(rng, success_rate, depth + 1)?;
                let res = match o {
                    Opcode::Mul => v1 * v2,
                    Opcode::Div => {
                        if v2 == 0 {
                            inc_dec(rng, success_rate, 0, depth, 1)
                        } else {
                            v1 / v2
                        }
                    }
                    Opcode::Mod => {
                        if v2 == 0 {
                            inc_dec(rng, success_rate, 0, depth, 1)
                        } else {
                            v1 % v2
                        }
                    }
                    Opcode::Add => v1.wrapping_add(v2),
                    Opcode::Sub => v1.wrapping_sub(v2),
                    Opcode::LShift => v1 << inc_dec(rng, success_rate, 0, depth, v2),
                    Opcode::RShift => v1 >> inc_dec(rng, success_rate, 0, depth, v2),
                    Opcode::Or => {
                        bit_flip(rng, success_rate, 1000, depth, v1)
                            | bit_flip(rng, success_rate, 1000, depth, v2)
                    }
                    Opcode::And => {
                        bit_flip(rng, success_rate, 1000, depth, v1)
                            & bit_flip(rng, success_rate, 1000, depth, v2)
                    }
                    Opcode::Xor => {
                        bit_flip(rng, success_rate, 1000, depth, v1)
                            ^ bit_flip(rng, success_rate, 1000, depth, v2)
                    }
                    Opcode::Pow => v1.wrapping_pow(v2 as u32),
                };
                Ok(res)
            }
            Self::Prefix(o, e) => {
                let v = e.evaluate(rng, success_rate, depth + 1)?;
                let res = match o {
                    PrefixOpcode::Not => !v,
                };
                Ok(res)
            }
            Self::Postfix(e, o) => {
                let v = e.evaluate(rng, success_rate, depth + 1)?;
                let res = match o {
                    PostfixOpcode::Inc => v.wrapping_add(1),
                    PostfixOpcode::Dec => v.wrapping_sub(1),
                    PostfixOpcode::Factorial => factorial(rng, success_rate, depth, v),
                };
                Ok(res)
            }
            Self::Show(e) => Err(format!("{:?}", &e)),
            Self::Help(e) => {
                let res = match e {
                    Help::Opcode(o) => match o {
                        Opcode::Mul => format!("Multiplication. Multiplies the two given numbers."),
                        Opcode::Div => format!("Integer Division. Divides the first number with the second number and drops the remainder"),
                        Opcode::Mod => format!("Modulo. Like integer division, but returns the remainder instead."),
                        Opcode::Pow => format!("Power. Raises the first number to the power of the second number."),
                        Opcode::Add => format!("Addition. Adds the two given numbers together."),
                        Opcode::Sub => format!("Subtraction. Subtracts the second number from the first."),
                        Opcode::LShift => format!("Bit Shift Left. Shifts all bits from the first number to the left by the amount of the second number."),
                        Opcode::RShift => format!("Bit Shift Right. Shifts all bits from the first number to the righ by the amount of the second number."),
                        Opcode::Or => format!("Bitwise OR. Performs a bitwise OR between the two numbers."),
                        Opcode::And => format!("Bitwise AND. Performs a bitwise AND between the two numbers."),
                        Opcode::Xor => format!("Bitwise XOR. Performs a bitwise XOR between the two numbers."),
                    },
                    Help::PrefixOpcode(o) => match o {
                        PrefixOpcode::Not => format!("Bitwise Not. Flips every bit in the following number.")
                    },
                    Help::PostfixOpcode(o) => match o {
                        PostfixOpcode::Inc => format!("Increment. Increments the preceding number by 1."),
                        PostfixOpcode::Dec => format!("Decrement. Decrements the preceding number by 1."),
                        PostfixOpcode::Factorial => format!("Factorial. Calculates the factorial of the preceding number."),
                    },
                    Help::Number(_) => format!("A number."),
                    Help::Constant(c) => match c {
                        Constant::Min => format!("The lowest possible number."),
                        Constant::Max => format!("The highest possible number."),
                        Constant::C => format!("The speed of light."),
                    },
                    Help::Show => format!("Command to show an expression with parenthesis."),
                    Help::Help => {
                        let operators = ["!", "%", "&", "*", "+", "++", "-", "--", "/", "<<", ">>", "^", "c", "max", "min", "show", "xor", "|", "~"];
                        let operator_string = operators.join(", ");
                        format!("Show help for one of the following operators: {}", operator_string)
                    }

                };
                Err(res)
            },
            Self::Error => Err("Oh no, something went wrong!".to_string()),
        }
        .map(|v| bit_flip(rng, success_rate, 1000, depth, v))
    }
}

pub enum ExprSymbol<'input> {
    NumSymbol(&'input str),
    Op(Box<ExprSymbol<'input>>, Opcode, Box<ExprSymbol<'input>>),
    Error,
}

#[derive(Copy, Clone)]
pub enum Opcode {
    Mul,
    Div,
    Mod,
    Pow,
    Add,
    Sub,
    LShift,
    RShift,
    Or,
    And,
    Xor,
}

#[derive(Copy, Clone)]
pub enum PrefixOpcode {
    Not,
}

#[derive(Copy, Clone)]
pub enum PostfixOpcode {
    Inc,
    Dec,
    Factorial,
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            Number(n) => write!(fmt, "{:?}", n),
            Show(ref e) => write!(fmt, "show {:?}", e),
            Help(ref e) => write!(fmt, "? {:?}", e),
            Op(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
            Prefix(op, ref r) => write!(fmt, "({:?} {:?})", op, r),
            Postfix(ref l, op) => write!(fmt, "({:?} {:?})", l, op),
            Error => write!(fmt, "error"),
        }
    }
}

impl<'input> Debug for ExprSymbol<'input> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::ExprSymbol::*;
        match *self {
            NumSymbol(n) => write!(fmt, "{:?}", n),
            Op(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
            Error => write!(fmt, "error"),
        }
    }
}

impl Debug for Opcode {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Opcode::*;
        match *self {
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Mod => write!(fmt, "%"),
            Pow => write!(fmt, "^"),
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
            LShift => write!(fmt, "<<"),
            RShift => write!(fmt, ">>"),
            Or => write!(fmt, "|"),
            And => write!(fmt, "&"),
            Xor => write!(fmt, "xor"),
        }
    }
}

impl Debug for PrefixOpcode {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::PrefixOpcode::*;
        match *self {
            Not => write!(fmt, "~"),
        }
    }
}

impl Debug for PostfixOpcode {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::PostfixOpcode::*;
        match *self {
            Inc => write!(fmt, "++"),
            Dec => write!(fmt, "--"),
            Factorial => write!(fmt, "!"),
        }
    }
}

impl Debug for Constant {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Constant::*;
        match *self {
            Min => write!(fmt, "min"),
            Max => write!(fmt, "max"),
            C => write!(fmt, "c"),
        }
    }
}

impl Debug for Help {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Help::*;
        match *self {
            Opcode(o) => write!(fmt, "{:?}", o),
            PrefixOpcode(o) => write!(fmt, "{:?}", o),
            PostfixOpcode(o) => write!(fmt, "{:?}", o),
            Number(n) => write!(fmt, "{}", n),
            Constant(c) => write!(fmt, "{:?}", c),
            Show => write!(fmt, "show"),
            Help => write!(fmt, "?"),
        }
    }
}
