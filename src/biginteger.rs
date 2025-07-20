use crate::bigdecimal::BigDecimal;
use core::fmt::{Display, Formatter};
use core::str::FromStr;
use std::iter::Sum;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{StdError, StdResult, Uint128, Uint256};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[cw_serde]
#[derive(Copy, Default, Ord, PartialOrd, Eq)]
pub struct BigInteger(pub Uint256);

impl BigInteger {

    pub const MAX: Self = Self(Uint256::MAX);
    pub const MIN: Self = Self(Uint256::MIN);

    pub fn scale_down(&self, decimals: u32) -> BigDecimal {
        BigDecimal::from(*self, decimals)
    }

    pub fn scale_up(&self, decimals: u32) -> Self {
        Self(self.0 * Uint256::from(10u64).pow(decimals))
    }

    pub fn to_uint128(&self) -> StdResult<Uint128> {
        Ok(Uint128::try_from(self.0)?)
    }

    pub fn to_uint256(&self) -> Uint256 {
        self.0
    }

    pub fn create_with_scale(value: u128, decimals: u32) -> Self {
        Self::from(value).scale_up(decimals)
    }

    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    pub fn zero() -> Self {
        Self(Uint256::zero())
    }

    pub fn saturating_sub(&self, rhs: Self) -> Self {
        Self(self.0.saturating_sub(rhs.0))
    }

    pub fn pow(&self, exp: u32) -> Self {
        Self(self.0.pow(exp))
    }

    pub fn from_be_bytes(bytes: [u8; 32]) -> Self {
        Self(Uint256::from_be_bytes(bytes))
    }

    pub fn from_le_bytes(bytes: [u8; 32]) -> Self {
        Self(Uint256::from_le_bytes(bytes))
    }

    pub fn to_be_bytes(&self) -> [u8; 32] {
        self.0.to_be_bytes()
    }

    pub fn to_le_bytes(&self) -> [u8; 32] {
        self.0.to_le_bytes()
    }
}

impl From<BigInteger> for String {
    fn from(value: BigInteger) -> Self {
        Self::from(value.0)
    }
}

impl FromStr for BigInteger {
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(BigInteger(Uint256::from_str(s)?))
    }
}

impl From<BigDecimal> for BigInteger {
    fn from(value: BigDecimal) -> Self {
        Self(value.0.to_uint_floor())
    }
}

impl From<u128> for BigInteger {
    fn from(value: u128) -> Self {
        Self(Uint256::from_u128(value))
    }
}

impl From<Uint128> for BigInteger {
    fn from(value: Uint128) -> Self {
        Self::from(value.u128())
    }
}

impl From<BigInteger> for Uint256 {
    fn from(value: BigInteger) -> Self {
        value.0
    }
}

impl From<u64> for BigInteger {
    fn from(value: u64) -> Self {
        Self(Uint256::from(value))
    }
}

impl From<u32> for BigInteger {
    fn from(value: u32) -> Self {
        Self(Uint256::from(value))
    }
}

impl From<u16> for BigInteger {
    fn from(value: u16) -> Self {
        Self(Uint256::from(value))
    }
}

impl From<u8> for BigInteger {
    fn from(value: u8) -> Self {
        Self(Uint256::from(value))
    }
}

impl Sub<BigInteger> for BigInteger {
    type Output = BigInteger;

    fn sub(self, rhs: BigInteger) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Add<BigInteger> for BigInteger {
    type Output = BigInteger;

    fn add(self, rhs: BigInteger) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Div<BigInteger> for BigInteger {
    type Output = BigInteger;

    fn div(self, rhs: BigInteger) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl Div<BigDecimal> for BigInteger {
    type Output = BigDecimal;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: BigDecimal) -> Self::Output {
        BigDecimal::from(self, 0) / rhs
    }
}

impl Mul<BigInteger> for BigInteger {
    type Output = BigInteger;

    fn mul(self, rhs: BigInteger) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Mul<BigDecimal> for BigInteger {
    type Output = BigDecimal;

    fn mul(self, rhs: BigDecimal) -> Self::Output {
        rhs * self
    }
}

impl AddAssign for BigInteger {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl SubAssign for BigInteger {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl MulAssign for BigInteger {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl DivAssign for BigInteger {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
    }
}

impl Display for BigInteger {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl Sum for BigInteger {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), Add::add)
    }
}

impl <'a> Sum<&'a BigInteger> for BigInteger {
    fn sum<I: Iterator<Item=&'a Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |a, b| a + *b)
    }
}

#[cfg(test)]
mod tests {
    use crate::bigdecimal::BigDecimal;
    use crate::biginteger::BigInteger;
    use cosmwasm_std::{Decimal256, Uint256};

    #[test]
    fn test_scale_down() {
        let bigint = BigInteger(Uint256::from(1000000u64));
        let bigdecimal = bigint.scale_down(6);
        assert_eq!(bigdecimal, BigDecimal(Decimal256::one()));
    }

    #[test]
    fn test_div() {
        let d = BigDecimal::from(BigInteger::from(100000000000000000000u128), 0);
        let i = BigInteger::from(100000000000000000000u128);
        assert_eq!(i / d, BigDecimal::one());
    }

    #[test]
    fn test_sum() {
        let vector: Vec<BigInteger> = vec![BigInteger::from(1u64), BigInteger::from(2u64), BigInteger::from(3u64)];
        assert_eq!(vector.clone().into_iter().sum::<BigInteger>(), BigInteger::from(6u64));
        assert_eq!(vector.iter().sum::<BigInteger>(), BigInteger::from(6u64));
    }
}
