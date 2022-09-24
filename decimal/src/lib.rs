use std::ops::{Add, Mul, Sub};

use num_bigint::BigInt;

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Debug)]
pub struct Decimal {
    num: BigInt,
    denum: BigInt,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        let input = &input.trim();
        let parts = input.split('.').collect::<Vec<&str>>();
        Some(Self {
            num: BigInt::parse_bytes(parts.join("").as_bytes(), 10)?,
            denum: BigInt::from(10).pow(parts.get(1).unwrap_or(&"").len() as u32),
        })
    }
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        (self.num.clone() * other.denum.clone()) == (other.num.clone() * self.denum.clone())
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.num.clone() * other.denum.clone()).partial_cmp(&(other.num.clone() * self.denum.clone()))
    }
}

impl Add for Decimal {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            num: (self.num * rhs.denum.clone()) + (rhs.num * self.denum.clone()),
            denum: (self.denum * rhs.denum),
        }
    }
}

impl Sub for Decimal {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            num: (self.num * rhs.denum.clone()) - (rhs.num * self.denum.clone()),
            denum: (self.denum * rhs.denum),
        }
    }
}

impl Mul for Decimal {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            num: self.num * rhs.num,
            denum: self.denum * rhs.denum,
        }
    }
}
