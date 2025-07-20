use crate::biginteger::BigInteger;
use core::fmt::{Display, Formatter};
use core::str::FromStr;
use std::iter::Sum;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Decimal256, StdError, Uint128, Uint256};
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign};

#[cw_serde]
#[derive(Copy, Default, Ord, PartialOrd, Eq)]
pub struct BigDecimal(pub Decimal256);

impl BigDecimal {

    pub const MAX: Self = Self(Decimal256::MAX);
    pub const MIN: Self = Self(Decimal256::MIN);

    pub fn new(bigint: BigInteger) -> Self {
        Self(Decimal256::new(bigint.0))
    }

    pub fn from(bigint: BigInteger, decimals: u32) -> Self {
        Self(Decimal256::from_ratio(
            bigint.0,
            Uint128::from(10u64).pow(decimals),
        ))
    }

    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    pub fn percent(x: u64) -> BigDecimal {
        Self(Decimal256::percent(x))
    }

    pub fn zero() -> Self {
        Self(Decimal256::zero())
    }

    pub fn one() -> Self {
        Self(Decimal256::one())
    }

    pub fn from_ratio(numerator: impl Into<Uint256>, denominator: impl Into<Uint256>) -> Self {
        Self(Decimal256::from_ratio(numerator, denominator))
    }

    pub fn saturating_sub(&self, rhs: Self) -> Self {
        Self(Decimal256::saturating_sub(self.0, rhs.0))
    }

    pub fn scale_up(&self, decimals: u32) -> BigInteger {
        BigInteger((self.0 * Decimal256::from_ratio(10u128.pow(decimals), 1u128)).to_uint_floor())
    }

    pub fn move_point_right(&self, decimals: u32) -> BigDecimal {
        *self * BigDecimal::from_ratio(10u128.pow(decimals), 1u128)
    }

    pub fn move_point_left(&self, decimals: u32) -> BigDecimal {
        *self / BigDecimal::from_ratio(10u128.pow(decimals), 1u128)
    }

    pub fn is_ratio(&self) -> bool {
        *self >= BigDecimal::zero() && *self <= BigDecimal::one()
    }
}

impl Sub<BigDecimal> for BigDecimal {
    type Output = BigDecimal;

    fn sub(self, rhs: BigDecimal) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Add<BigDecimal> for BigDecimal {
    type Output = BigDecimal;

    fn add(self, rhs: BigDecimal) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Div<BigDecimal> for BigDecimal {
    type Output = BigDecimal;

    fn div(self, rhs: BigDecimal) -> Self::Output {
        BigDecimal(self.0 / rhs.0)
    }
}

impl Div<BigInteger> for BigDecimal {
    type Output = BigDecimal;

    fn div(self, rhs: BigInteger) -> Self::Output {
        self / BigDecimal::from(rhs, 0)
    }
}

impl Mul<BigDecimal> for BigDecimal {
    type Output = BigDecimal;

    fn mul(self, rhs: BigDecimal) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Mul<BigInteger> for BigDecimal {
    type Output = BigDecimal;

    fn mul(self, rhs: BigInteger) -> Self::Output {
        self * BigDecimal::from(rhs, 0)
    }
}

impl AddAssign for BigDecimal {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl SubAssign for BigDecimal {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl MulAssign for BigDecimal {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl Display for BigDecimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl FromStr for BigDecimal {
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const DECIMAL_FRACTIONAL: Uint256 = // 1*10**18
            Uint256::from_u128(1_000_000_000_000_000_000);
        let mut parts_iter = s.split('.');

        let whole_part = parts_iter.next().unwrap(); // split always returns at least one element
        let whole = whole_part
            .parse::<Uint256>()
            .map_err(|_| StdError::generic_err("Error parsing whole"))?;
        let mut atomics = whole
            .checked_mul(DECIMAL_FRACTIONAL)
            .map_err(|_| StdError::generic_err("Value too big"))?;

        if let Some(fractional_part) = parts_iter.next() {
            let fractional = fractional_part
                .parse::<Uint256>()
                .map_err(|_| StdError::generic_err("Error parsing fractional"))?;
            let exp = (Decimal256::DECIMAL_PLACES.checked_sub(fractional_part.len() as u32))
                .ok_or_else(|| {
                    StdError::generic_err(format!(
                        "Cannot parse more than {} fractional digits",
                        Decimal256::DECIMAL_PLACES
                    ))
                })?;
            let fractional_factor = Uint256::from(10u128).pow(exp);
            atomics = atomics
                .checked_add(
                    // The inner multiplication can't overflow because
                    // fractional < 10^DECIMAL_PLACES && fractional_factor <= 10^DECIMAL_PLACES
                    fractional.checked_mul(fractional_factor).unwrap(),
                )
                .map_err(|_| StdError::generic_err("Value too big"))?;
        }

        if parts_iter.next().is_some() {
            return Err(StdError::generic_err("Unexpected number of dots"));
        }

        Ok(Self(Decimal256::new(atomics)))
    }
}


impl Sum for BigDecimal {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), Add::add)
    }
}

impl <'a> Sum<&'a BigDecimal> for BigDecimal {
    fn sum<I: Iterator<Item=&'a Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |a, b| a + *b)
    }
}