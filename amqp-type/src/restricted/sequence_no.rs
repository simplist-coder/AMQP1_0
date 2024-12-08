use crate::error::AppError;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use std::cmp::Ordering;
use std::num::Wrapping;
use std::ops::{Add, AddAssign};
use std::vec::IntoIter;

/// # Sequence Number
/// A 32-bit RFC-1982 serial number.
/// ```xml
/// <type name="sequence-no" class="restricted" source="uint"/>
/// ```
/// A sequence-no encodes a serial number as defined in RFC-1982. The arithmetic, and operators for
/// these numbers are defined by RFC-1982.
#[derive(Debug, Clone, Copy, Default)]
pub struct SequenceNumber(u32);

const HALF_MAX: u32 = 1 << 30;

impl SequenceNumber {
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    pub fn inner(&self) -> u32 {
        self.0
    }
}

impl From<u32> for SequenceNumber {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl Add for SequenceNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self((Wrapping(self.0) + Wrapping(rhs.0)).0)
    }
}

impl AddAssign for SequenceNumber {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl PartialEq<Self> for SequenceNumber {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd<Self> for SequenceNumber {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let i1 = self.0;
        let i2 = other.0;
        if i1 == i2 {
            Some(Ordering::Equal)
        } else if (i1 < i2 && i2 - i1 < HALF_MAX) || (i1 > i2 && i1 - i2 > HALF_MAX) {
            Some(Ordering::Less)
        } else if (i1 < i2 && i2 - i1 > HALF_MAX) || (i1 > i2 && i1 - i2 < HALF_MAX) {
            Some(Ordering::Greater)
        } else {
            // This case is undefined behaviour according to RFC 1982.
            None
        }
    }
}

impl Encode for SequenceNumber {
    fn encode(self) -> Encoded {
        self.0.encode()
    }
}

impl Decode for SequenceNumber {
    fn try_decode(constructor: u8, stream: &mut IntoIter<u8>) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        u32::try_decode(constructor, stream).map(Self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAX: u32 = 1 << 31;

    #[test]
    fn test_default_is_zero() {
        assert_eq!(SequenceNumber::default(), 0.into());
        assert_eq!(SequenceNumber::new(15), 15.into());
    }

    #[test]
    fn test_addition_wraps_around() {
        let max = SequenceNumber::from(u32::MAX);
        let one = SequenceNumber::from(1_u32);
        assert_eq!((max + one).0, 0)
    }

    #[test]
    fn test_ordering() {
        assert!(SequenceNumber::from(0) > SequenceNumber::from(u32::MAX));
        assert!(SequenceNumber::from(44) > SequenceNumber::from(0));
        assert!(SequenceNumber::from(200) > SequenceNumber::from(0));
        assert!(SequenceNumber::from(HALF_MAX + 1) < SequenceNumber::from(0));
        assert!(SequenceNumber::from(HALF_MAX - 1) > SequenceNumber::from(0));
        assert!(SequenceNumber::from(HALF_MAX - 1) > SequenceNumber::from(MAX));
    }

    #[test]
    fn test_undefined_cases() {
        assert_eq!(SequenceNumber::from(HALF_MAX).partial_cmp(&0.into()), None);
        assert_eq!(SequenceNumber::from(1).partial_cmp(&(HALF_MAX + 1).into()), None);
        assert_eq!(SequenceNumber::from(0).partial_cmp(&HALF_MAX.into()), None);
    }
}
